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

//! Middleware hooks that run before and after every update.
//!
//! Register middleware on [`DispatcherOpts`](crate::DispatcherOpts):
//!
//! ```rust,no_run
//! use ferobot::{Bot, Dispatcher, DispatcherOpts, Updater};
//! use ferobot::middleware::Middleware;
//! use ferobot::types::Update;
//! use async_trait::async_trait;
//!
//! struct Logger;
//!
//! #[async_trait]
//! impl Middleware for Logger {
//!     async fn before(&self, _bot: &Bot, update: &Update) -> bool {
//!         tracing::info!(update_id = update.update_id, "incoming update");
//!         true  // return false to drop the update
//!     }
//! }
//!
//! # #[tokio::main]
//! # async fn main() {
//! let bot = Bot::new("TOKEN").await.unwrap();
//! let opts = DispatcherOpts::default().middleware(Logger);
//! let dp   = Dispatcher::new(opts);
//! Updater::new(bot, dp).start_polling().await.unwrap();
//! # }
//! ```
//!
//! ## Built-in middleware
//!
//! | Type | What it does |
//! |------|-------------|
//! | [`LoggingMiddleware`] | Logs every update at `INFO` level with `tracing`. |
//! | [`RateLimiter`] | Drops updates from a chat that arrive faster than a threshold. |

use std::sync::Arc;

use async_trait::async_trait;

use crate::{types::Update, Bot};

/// A hook that runs around every dispatched update.
///
/// Implement at least [`before`](Middleware::before); the defaults for the
/// other methods are no-ops.
#[async_trait]
pub trait Middleware: Send + Sync + 'static {
    /// Called before any handler runs for this update.
    ///
    /// Return `true` to let the update continue, `false` to drop it silently.
    async fn before(&self, bot: &Bot, update: &Update) -> bool {
        let _ = (bot, update);
        true
    }

    /// Called after all handlers finish (regardless of errors or panics).
    async fn after(&self, bot: &Bot, update: &Update) {
        let _ = (bot, update);
    }
}

/// Type-erased, cheaply cloneable middleware handle.
pub type ArcMiddleware = Arc<dyn Middleware>;

/// Run all `before` hooks in order. Returns `false` if any hook drops the update.
pub async fn run_before(chain: &[ArcMiddleware], bot: &Bot, update: &Update) -> bool {
    for mw in chain {
        if !mw.before(bot, update).await {
            return false;
        }
    }
    true
}

/// Run all `after` hooks in order.
pub async fn run_after(chain: &[ArcMiddleware], bot: &Bot, update: &Update) {
    for mw in chain {
        mw.after(bot, update).await;
    }
}

/// Logs every update at `INFO` level using [`tracing`].
///
/// Emits: `update update_id=<n> kind=<message|callback_query|...>`
pub struct LoggingMiddleware;

#[async_trait]
impl Middleware for LoggingMiddleware {
    async fn before(&self, _bot: &Bot, update: &Update) -> bool {
        let kind = update_kind(update);
        tracing::info!(update_id = update.update_id, kind, "update received");
        true
    }

    async fn after(&self, _bot: &Bot, update: &Update) {
        tracing::debug!(update_id = update.update_id, "update handled");
    }
}

fn update_kind(u: &Update) -> &'static str {
    if u.message.is_some() {
        "message"
    } else if u.edited_message.is_some() {
        "edited_message"
    } else if u.callback_query.is_some() {
        "callback_query"
    } else if u.inline_query.is_some() {
        "inline_query"
    } else if u.channel_post.is_some() {
        "channel_post"
    } else if u.chat_member.is_some() {
        "chat_member"
    } else {
        "other"
    }
}

/// Simple per-chat rate limiter.
///
/// Drops an update if its chat has sent more than `max_per_second` updates
/// in the last second. Uses an in-memory sliding-window counter.
///
/// # Example
///
/// ```rust,no_run
/// use ferobot::middleware::RateLimiter;
/// use ferobot::DispatcherOpts;
///
/// let opts = DispatcherOpts::default()
///     .middleware(RateLimiter::new(3)); // max 3 updates/s per chat
/// ```
pub struct RateLimiter {
    max_per_second: u32,
    counters: Arc<dashmap_rl::DashMap>,
}

// We re-use dashmap behind the per-chat feature gate so we don't add a new dep.
// If per-chat isn't enabled we fall back to std Mutex.
mod dashmap_rl {
    // We always bundle a lightweight version using std primitives so the
    // RateLimiter compiles regardless of feature flags.
    use std::{collections::HashMap, sync::Mutex, time::Instant};

    pub struct DashMap(Mutex<HashMap<i64, (Instant, u32)>>);

    impl DashMap {
        pub fn new() -> Self {
            Self(Mutex::new(HashMap::new()))
        }

        pub fn check_and_increment(&self, id: i64, max: u32) -> bool {
            let mut m = self.0.lock().unwrap_or_else(|e| e.into_inner());
            let now = Instant::now();
            let entry = m.entry(id).or_insert((now, 0));
            if now.duration_since(entry.0).as_secs() >= 1 {
                *entry = (now, 1);
                true
            } else if entry.1 < max {
                entry.1 += 1;
                true
            } else {
                false
            }
        }
    }
}

impl RateLimiter {
    /// Create a rate limiter that allows up to `max_per_second` updates per
    /// chat per second.
    pub fn new(max_per_second: u32) -> Self {
        Self {
            max_per_second,
            counters: Arc::new(dashmap_rl::DashMap::new()),
        }
    }

    fn chat_id(update: &Update) -> Option<i64> {
        update
            .message
            .as_ref()
            .map(|m| m.chat.id)
            .or_else(|| update.edited_message.as_ref().map(|m| m.chat.id))
            .or_else(|| update.channel_post.as_ref().map(|m| m.chat.id))
    }
}

#[async_trait]
impl Middleware for RateLimiter {
    async fn before(&self, _bot: &Bot, update: &Update) -> bool {
        let id = match Self::chat_id(update) {
            Some(id) => id,
            None => return true, // non-chat updates are not rate-limited
        };
        let allowed = self.counters.check_and_increment(id, self.max_per_second);
        if !allowed {
            tracing::debug!(chat_id = id, "rate limit: update dropped");
        }
        allowed
    }
}
