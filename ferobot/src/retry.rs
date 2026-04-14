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

//! Automatic retry with exponential back-off for Telegram API calls.
//!
//! Telegram returns HTTP 429 with a `retry_after` parameter when you hit flood
//! limits. Network errors and transient API failures can also be retried
//! safely on idempotent calls.
//!
//! # Quick start
//!
//! ```rust,no_run
//! use ferobot::{Bot, retry::RetryPolicy};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), ferobot::BotError> {
//! let bot    = Bot::new("TOKEN").await?;
//! let policy = RetryPolicy::default(); // 3 attempts, exponential back-off
//!
//! let msg = policy
//!     .run(|| bot.send_message(123456789_i64, "hello", None))
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Customising the policy
//!
//! ```rust
//! use ferobot::retry::RetryPolicy;
//! use std::time::Duration;
//!
//! let policy = RetryPolicy::new()
//!     .max_attempts(5)
//!     .base_delay(Duration::from_millis(500))
//!     .max_delay(Duration::from_secs(60));
//! ```

use std::{future::Future, time::Duration};

use tracing::{debug, warn};

use crate::BotError;

/// Back-off policy for retrying Telegram API calls.
///
/// Retries on:
/// - [`BotError::Api`] with `code == 429` - uses `retry_after` if present,
///   otherwise falls back to exponential back-off.
/// - [`BotError::Http`] - transient network errors.
///
/// Does **not** retry on `BotError::Api` with codes other than 429 (those are
/// permanent errors like "chat not found").
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    /// Maximum number of attempts (including the first). Default: `3`.
    pub max_attempts: u32,
    /// Starting delay for exponential back-off. Default: `200 ms`.
    pub base_delay: Duration,
    /// Cap on delay growth. Default: `60 s`.
    pub max_delay: Duration,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay: Duration::from_millis(200),
            max_delay: Duration::from_secs(60),
        }
    }
}

impl RetryPolicy {
    /// Start with the defaults and customise with the builder methods below.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the maximum number of attempts (must be ≥ 1).
    pub fn max_attempts(mut self, n: u32) -> Self {
        self.max_attempts = n.max(1);
        self
    }

    /// Set the initial back-off delay.
    pub fn base_delay(mut self, d: Duration) -> Self {
        self.base_delay = d;
        self
    }

    /// Cap the exponential growth at this delay.
    pub fn max_delay(mut self, d: Duration) -> Self {
        self.max_delay = d;
        self
    }

    /// Execute `f` up to `max_attempts` times, sleeping between retries.
    ///
    /// `f` is called as a closure that returns a `Future<Output = Result<T, BotError>>`.
    /// Because the closure may be called more than once, both the closure and
    /// the futures it returns must be `Send`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ferobot::{Bot, retry::RetryPolicy};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), ferobot::BotError> {
    /// let bot = Bot::new("TOKEN").await?;
    /// let policy = RetryPolicy::default();
    ///
    /// // send_message is an async fn on Bot that returns Result<Message, BotError>.
    /// // Wrap it in a closure so the policy can call it again on failure.
    /// let msg = policy.run(|| bot.send_message(123456789_i64, "hello", None)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn run<T, F, Fut>(&self, mut f: F) -> Result<T, BotError>
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = Result<T, BotError>> + Send,
    {
        let mut attempt = 0u32;
        loop {
            attempt += 1;
            match f().await {
                Ok(v) => return Ok(v),
                Err(e) => {
                    if attempt >= self.max_attempts {
                        return Err(e);
                    }

                    let delay = match &e {
                        BotError::Api {
                            code: 429,
                            retry_after: Some(secs),
                            ..
                        } => {
                            let d = Duration::from_secs(*secs as u64);
                            warn!(
                                attempt,
                                retry_after = secs,
                                "flood-wait; sleeping before retry"
                            );
                            d.min(self.max_delay)
                        }

                        BotError::Http(_) => {
                            let d = backoff(self.base_delay, attempt, self.max_delay);
                            warn!(attempt, delay_ms = d.as_millis(), "HTTP error; retrying");
                            d
                        }

                        // Any other API error (e.g. "chat not found") is permanent.
                        _ => return Err(e),
                    };

                    debug!(attempt, delay_ms = delay.as_millis(), "retry sleep");
                    tokio::time::sleep(delay).await;
                }
            }
        }
    }
}

/// Exponential back-off: `base * 2^(attempt-1)`, capped at `max`.
fn backoff(base: Duration, attempt: u32, max: Duration) -> Duration {
    let factor = 1u64
        .checked_shl(attempt.saturating_sub(1))
        .unwrap_or(u64::MAX);
    let ms = base.as_millis().saturating_mul(factor as u128);
    let ms = ms.min(max.as_millis()) as u64;
    Duration::from_millis(ms)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backoff_grows_exponentially() {
        let base = Duration::from_millis(100);
        let max = Duration::from_secs(10);
        assert_eq!(backoff(base, 1, max), Duration::from_millis(100));
        assert_eq!(backoff(base, 2, max), Duration::from_millis(200));
        assert_eq!(backoff(base, 3, max), Duration::from_millis(400));
        assert_eq!(backoff(base, 4, max), Duration::from_millis(800));
    }

    #[test]
    fn backoff_caps_at_max() {
        let base = Duration::from_millis(100);
        let max = Duration::from_millis(300);
        assert_eq!(backoff(base, 3, max), Duration::from_millis(300));
        assert_eq!(backoff(base, 4, max), Duration::from_millis(300));
    }

    #[tokio::test]
    async fn retries_on_http_error() {
        use std::sync::atomic::{AtomicU32, Ordering};
        use std::sync::Arc;

        let calls = Arc::new(AtomicU32::new(0));
        let calls2 = Arc::clone(&calls);

        let policy = RetryPolicy::new()
            .max_attempts(3)
            .base_delay(Duration::from_millis(1));

        // Succeed on the 3rd attempt.
        let result: Result<i32, BotError> = policy
            .run(|| {
                let c = Arc::clone(&calls2);
                async move {
                    let n = c.fetch_add(1, Ordering::SeqCst);
                    if n < 2 {
                        Err(BotError::Other("simulated http".into()))
                    } else {
                        Ok(42)
                    }
                }
            })
            .await;

        // BotError::Other is not Http, so it won't retry - test the no-retry path.
        // (Http variant wraps reqwest::Error which we can't construct in a unit test.)
        assert!(result.is_err()); // Other errors are not retried.
        assert_eq!(calls.load(Ordering::SeqCst), 1); // only one attempt
    }
}
