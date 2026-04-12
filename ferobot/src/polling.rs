// Copyright (c) Ankit Chaubey <ankitchaubey.dev@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// ferobot: async Telegram Bot API framework written in Rust
// Repository: https://github.com/ankit-chaubey/ferobot
//
// Ferobot provides a fast and ergonomic framework for building Telegram bots
// using the official Telegram Bot API.
//
// Author: Ankit Chaubey
//
// If you use or modify this code, keep this notice at the top of your file
// and include the LICENSE-MIT or LICENSE-APACHE file from this repository.

//! Long-polling update loop with bounded concurrency and panic recovery.
//!
//! Each batch of updates is dispatched concurrently. A semaphore caps how many
//! handler tasks run at once so a flood of updates doesn't exhaust memory.
//!
//! `getUpdates` holds a connection open for up to 60 seconds. If you run
//! outbound calls (sendMessage etc.) through the same pool, they queue behind it.
//! Use two separate bot instances:
//!
//! ```rust,no_run
//! use ferobot::{Bot, Poller, UpdateHandler};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), ferobot::BotError> {
//! let verified  = Bot::new("TOKEN").await?;
//! let poll_bot  = verified.clone().into_polling_bot()?; // 65 s timeout, 1-conn pool
//! let api_bot   = verified.into_api_bot()?;             // 10 s timeout, 200-conn pool
//!
//! let handler: UpdateHandler = Box::new(|bot, update| {
//!     Box::pin(async move {
//!         if let Some(msg) = update.message {
//!             let _ = bot.send_message(msg.chat.id, "pong", None).await;
//!         }
//!     })
//! });
//!
//! Poller::new(poll_bot, handler)
//!     .api_bot(api_bot)        // handlers receive the fast API client
//!     .concurrency(512)        // max 512 concurrent handler tasks
//!     .start()
//!     .await?;
//! # Ok(())
//! # }
//! ```

use crate::gen_methods::GetUpdatesParams;
use crate::types::Update;
use crate::{Bot, BotError};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use tracing::{error, warn};

/// A function that handles an incoming update.
pub type UpdateHandler =
    Box<dyn Fn(Bot, Update) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

/// Long-polling update dispatcher with bounded concurrency and panic recovery.
///
/// See the [module docs](self) for the split-client pattern.
pub struct Poller {
    /// Bot used exclusively for `getUpdates` (long-poll client).
    poll_bot: Bot,
    /// Bot passed to handlers. Defaults to `poll_bot` if not set.
    api_bot: Option<Bot>,
    handler: Arc<UpdateHandler>,
    /// Seconds to long-poll per request (0 = short poll).
    timeout: i64,
    /// Max updates per `getUpdates` call.
    limit: i64,
    /// Update types to receive (empty = all).
    allowed_updates: Vec<String>,
    /// Maximum number of handler tasks that may run concurrently.
    /// When the limit is reached, new updates wait until a slot frees up.
    max_concurrent: usize,
}

impl Poller {
    /// Create a new `Poller`.
    ///
    /// `bot` is used for `getUpdates`. For production use, also call
    /// `.api_bot(bot.clone().into_api_bot()?)` so handlers get a separate
    /// fast-pool client.
    pub fn new(bot: Bot, handler: UpdateHandler) -> Self {
        Poller {
            poll_bot: bot,
            api_bot: None,
            handler: Arc::new(handler),
            timeout: 30,
            limit: 100,
            allowed_updates: vec![],
            max_concurrent: 512,
        }
    }

    /// Set the `Bot` instance passed to update handlers.
    /// If not set, handlers get a clone of the polling bot.
    pub fn api_bot(mut self, bot: Bot) -> Self {
        self.api_bot = Some(bot);
        self
    }

    /// Set the maximum number of handler tasks that may run at the same time.
    ///
    /// When this limit is reached, the polling loop waits until a slot frees up.
    /// Defaults to `512`.
    ///
    /// A value of `0` is treated as `1`.
    pub fn concurrency(mut self, max: usize) -> Self {
        self.max_concurrent = max.max(1);
        self
    }

    /// Set the long-polling timeout in seconds (default: `30`).
    ///
    /// Telegram holds the connection open for up to this many seconds before
    /// returning an empty update list. `0` means short-poll (instant return).
    pub fn timeout(mut self, t: i64) -> Self {
        self.timeout = t;
        self
    }

    /// Set the max number of updates per `getUpdates` call (default: `100`).
    pub fn limit(mut self, l: i64) -> Self {
        self.limit = l;
        self
    }

    /// Filter which update types to receive (empty = all).
    pub fn allowed_updates(mut self, updates: Vec<String>) -> Self {
        self.allowed_updates = updates;
        self
    }

    /// Start polling. Blocks until the process exits or an unrecoverable error.
    ///
    /// Each update runs in its own task. Panicking handlers are caught and logged;
    /// they cannot crash the polling loop.
    pub async fn start(self) -> Result<(), BotError> {
        let mut offset: i64 = 0;
        let allowed_updates = if self.allowed_updates.is_empty() {
            None
        } else {
            Some(self.allowed_updates.clone())
        };

        let semaphore = Arc::new(Semaphore::new(self.max_concurrent));

        // use api_bot if set, otherwise fall back to poll_bot
        let api_bot = self.api_bot.unwrap_or_else(|| self.poll_bot.clone());
        let handler = Arc::clone(&self.handler);

        tracing::debug!(
            max_concurrent = self.max_concurrent,
            timeout = self.timeout,
            "polling started"
        );

        loop {
            let mut params = GetUpdatesParams::new()
                .offset(offset)
                .timeout(self.timeout)
                .limit(self.limit);

            if let Some(ref au) = allowed_updates {
                params = params.allowed_updates(au.clone());
            }

            let updates = match self.poll_bot.get_updates(Some(params)).await {
                Ok(u) => u,
                Err(e) => {
                    let sleep_secs = match &e {
                        BotError::Api {
                            retry_after: Some(secs),
                            ..
                        } => {
                            warn!(retry_after = secs, "flood-wait on getUpdates");
                            *secs as u64
                        }
                        _ => {
                            error!(error = %e, "getUpdates error, retrying in 3 s");
                            3
                        }
                    };
                    tokio::time::sleep(Duration::from_secs(sleep_secs)).await;
                    continue;
                }
            };

            for update in updates {
                offset = update.update_id + 1;

                // block here until a slot is free, then move the permit into the task
                let permit = semaphore
                    .clone()
                    .acquire_owned()
                    .await
                    .expect("semaphore should not be closed");

                let bot = api_bot.clone();
                let handler = Arc::clone(&handler);

                tokio::spawn(async move {
                    let _permit = permit;

                    let result = tokio::spawn(async move {
                        (handler)(bot, update).await;
                    })
                    .await;

                    if let Err(join_err) = result {
                        if join_err.is_panic() {
                            error!("handler panicked on update - task isolated, poller continues");
                        }
                        // Cancellation (JoinError::is_cancelled): runtime is shutting down.
                    }
                });
            }
        }
    }
}
