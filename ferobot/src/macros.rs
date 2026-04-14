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

//! Ergonomic macros for building optional parameter structs.
//!
//! # `p!` - build params inline
//!
//! Instead of writing:
//! ```rust,ignore
//! bot.send_message(chat_id, "Hi", Some(SendMessageParams {
//!     parse_mode: Some("HTML".into()),
//!     disable_notification: Some(true),
//!     ..Default::default()
//! })).await?;
//! ```
//!
//! Write:
//! ```rust,ignore
//! use ferobot::p;
//! use ferobot::gen_methods::SendMessageParams;
//!
//! bot.send_message(chat_id, "Hi", p!(SendMessageParams {
//!     parse_mode: "HTML",
//!     disable_notification: true,
//! })).await?;
//! ```

/// Build an optional Telegram params struct with named fields.
///
/// Each field value is wrapped in `Some(value.into())` automatically.
/// All other fields are filled with `Default::default()`.
/// The whole expression evaluates to `Some(ParamsStruct { ... })`.
///
/// Use `p!(T)` (no braces) to produce `None::<T>` (no options set).
///
/// # Example
/// ```rust,ignore
/// use ferobot::{p, gen_methods::{SendMessageParams, BanChatMemberParams}};
///
/// // With options
/// bot.send_message(chat_id, "Hello!", p!(SendMessageParams {
///     parse_mode: "HTML",
///     disable_notification: true,
/// })).await?;
///
/// // No options
/// bot.send_message(chat_id, "Hello!", p!(SendMessageParams)).await?;
///
/// // Or just use None directly
/// bot.send_message(chat_id, "Hello!", None).await?;
/// ```
#[macro_export]
macro_rules! p {
    // With fields
    ($T:ident { $($key:ident : $val:expr),* $(,)? }) => {
        Some($crate::gen_methods::$T {
            $($key: Some($val.into()),)*
            ..Default::default()
        })
    };
    // No fields - produces None
    ($T:ident) => {
        None::<$crate::gen_methods::$T>
    };
}
