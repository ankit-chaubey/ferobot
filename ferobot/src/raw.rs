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

//! Raw / escape-hatch API caller.
//!
//! Use [`Bot::raw`] to call **any** Telegram Bot API method by name, using a
//! chainable builder. This lets you use new or niche Bot API features without
//! waiting for a typed wrapper to be generated.
//!
//! # Example
//!
//! ```rust,no_run
//! use ferobot::Bot;
//! use serde_json::Value;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), ferobot::BotError> {
//! let bot = Bot::new("YOUR_TOKEN").await?;
//!
//! // Call sendMessage with chained params
//! let msg: Value = bot
//!     .raw("sendMessage")
//!     .param("chat_id", 123456789_i64)
//!     .param("text", "Hello from raw!")
//!     .param("parse_mode", "HTML")
//!     .call()
//!     .await?;
//!
//! println!("message_id = {}", msg["message_id"]);
//!
//! // Use any feature flag e.g. sendMessage with reply_parameters
//! let _: Value = bot
//!     .raw("sendMessage")
//!     .param("chat_id", 123456789_i64)
//!     .param("text", "Replying!")
//!     .param("reply_parameters", serde_json::json!({ "message_id": 42 }))
//!     .call()
//!     .await?;
//!
//! // Upload a file via multipart
//! let photo = ferobot::InputFile::memory("photo.jpg", std::fs::read("photo.jpg").unwrap());
//! let _: Value = bot
//!     .raw("sendPhoto")
//!     .param("chat_id", 123456789_i64)
//!     .file("photo", photo)
//!     .call()
//!     .await?;
//! # Ok(())
//! # }
//! ```

use serde::{de::DeserializeOwned, Serialize};

use crate::{
    input_file::{InputFile, InputFileOrString},
    Bot, BotError,
};

/// A chainable raw API request builder returned by [`Bot::raw`].
///
/// Build up parameters with [`.param()`](RawRequest::param), optionally attach
/// a file upload with [`.file()`](RawRequest::file), then execute with
/// [`.call::<T>()`](RawRequest::call).
pub struct RawRequest<'bot> {
    bot: &'bot Bot,
    method: String,
    params: serde_json::Map<String, serde_json::Value>,
    file: Option<(String, InputFile)>,
}

impl<'bot> RawRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, method: impl Into<String>) -> Self {
        Self {
            bot,
            method: method.into(),
            params: serde_json::Map::new(),
            file: None,
        }
    }

    /// Add a parameter. `value` can be anything serializable primitives,
    /// strings, booleans, `serde_json::json!{...}` blobs, or your own types.
    ///
    /// Calling `.param()` with the same key twice overwrites the previous value.
    ///
    /// ```rust,no_run
    /// # async fn example(bot: ferobot::Bot) -> Result<(), ferobot::BotError> {
    /// use serde_json::Value;
    /// let _: Value = bot
    ///     .raw("sendMessage")
    ///     .param("chat_id", 123_i64)
    ///     .param("text", "hi")
    ///     .param("disable_notification", true)
    ///     .call()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn param(mut self, key: impl Into<String>, value: impl Serialize) -> Self {
        let v = serde_json::to_value(value).unwrap_or(serde_json::Value::Null);
        if !v.is_null() {
            self.params.insert(key.into(), v);
        }
        self
    }

    /// Attach an [`InputFile`] for methods that accept file uploads (e.g.
    /// `sendPhoto`, `sendDocument`, `sendVideo`).
    ///
    /// `field` is the Bot API parameter name for the file (e.g. `"photo"`,
    /// `"document"`, `"video"`).
    ///
    /// For `FileId` / `Url` variants the file reference is serialized as a
    /// normal JSON string; for `Memory` uploads the request is automatically
    /// switched to `multipart/form-data`.
    pub fn file(mut self, field: impl Into<String>, file: InputFile) -> Self {
        self.file = Some((field.into(), file));
        self
    }

    /// Execute the request and deserialize the Telegram `result` field as `T`.
    ///
    /// Use `serde_json::Value` as `T` to accept any response without a typed
    /// struct:
    /// ```rust,no_run
    /// # async fn example(bot: ferobot::Bot) -> Result<(), ferobot::BotError> {
    /// let resp: serde_json::Value = bot
    ///     .raw("getMe")
    ///     .call()
    ///     .await?;
    /// println!("{}", resp["username"]);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn call<T>(self) -> Result<T, BotError>
    where
        T: DeserializeOwned,
    {
        let Self {
            bot,
            method,
            params,
            file,
        } = self;

        match file {
            None => {
                bot.call_api(&method, serde_json::Value::Object(params))
                    .await
            }
            Some((field, input_file)) => {
                bot.call_api_with_file(&method, params, &field, InputFileOrString::File(input_file))
                    .await
            }
        }
    }

    /// Execute the request and return the raw `serde_json::Value` result.
    ///
    /// Equivalent to `.call::<serde_json::Value>()` a convenience alias.
    pub async fn call_raw(self) -> Result<serde_json::Value, BotError> {
        self.call::<serde_json::Value>().await
    }

    /// Execute the request and discard the result (`ok=true` check only).
    ///
    /// Useful for fire-and-forget calls like `deleteMessage`.
    pub async fn send(self) -> Result<(), BotError> {
        self.call::<serde_json::Value>().await.map(|_| ())
    }
}
