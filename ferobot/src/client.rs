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

//! Pluggable HTTP back-end for [`Bot`].
//!
//! By default `Bot` uses [`ReqwestClient`], which wraps `reqwest`. You can
//! replace it with any type that implements [`BotClient`] - useful for
//! unit-testing handlers without a live Telegram server, using a custom
//! proxy, or compiling to WASM with a different HTTP stack.
//!
//! # Custom client example
//!
//! ```rust,no_run
//! use async_trait::async_trait;
//! use ferobot::client::{BotClient, FormPart, FormBody};
//! use ferobot::BotError;
//!
//! #[derive(Debug)]
//! struct MockClient;
//!
//! #[async_trait]
//! impl BotClient for MockClient {
//!     async fn post_json(&self, _url: &str, _body: serde_json::Value) -> Result<bytes::Bytes, BotError> {
//!         // Return a fake ok response
//!         Ok(bytes::Bytes::from(r#"{"ok":true,"result":true}"#))
//!     }
//!     async fn post_form(&self, _url: &str, _parts: Vec<FormPart>) -> Result<bytes::Bytes, BotError> {
//!         Ok(bytes::Bytes::from(r#"{"ok":true,"result":true}"#))
//!     }
//! }
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), ferobot::BotError> {
//! let bot = ferobot::Bot::with_client("1234:TOKEN", "https://api.telegram.org", MockClient)?;
//! # Ok(())
//! # }
//! ```

use std::fmt;

use async_trait::async_trait;

use crate::BotError;

/// A single field in a `multipart/form-data` request.
#[derive(Debug)]
pub struct FormPart {
    /// Field name.
    pub name: String,
    /// Field content.
    pub body: FormBody,
}

impl FormPart {
    /// Convenience constructor for a plain-text part.
    pub fn text(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            body: FormBody::Text(value.into()),
        }
    }

    /// Convenience constructor for a binary file part.
    pub fn bytes(
        name: impl Into<String>,
        filename: impl Into<String>,
        mime: impl Into<String>,
        data: bytes::Bytes,
    ) -> Self {
        Self {
            name: name.into(),
            body: FormBody::Bytes {
                filename: filename.into(),
                mime: mime.into(),
                data,
            },
        }
    }
}

/// The body of a [`FormPart`].
#[derive(Debug)]
pub enum FormBody {
    /// A plain UTF-8 text value.
    Text(String),
    /// A binary file upload.
    Bytes {
        /// The file name sent to Telegram.
        filename: String,
        /// MIME type, e.g. `"image/jpeg"`.
        mime: String,
        /// Raw file bytes.
        data: bytes::Bytes,
    },
}

/// Pluggable HTTP transport for [`Bot`].
///
/// Implement this trait to replace the default `reqwest` client.
/// Both methods receive the full endpoint URL and must return the raw
/// response body as `bytes::Bytes`. `Bot` handles JSON deserialization
/// and Telegram error unwrapping.
#[async_trait]
pub trait BotClient: Send + Sync + fmt::Debug {
    /// POST a JSON body to `url` and return the raw response bytes.
    async fn post_json(&self, url: &str, body: serde_json::Value)
        -> Result<bytes::Bytes, BotError>;

    /// POST a `multipart/form-data` request to `url` and return the raw
    /// response bytes.
    async fn post_form(&self, url: &str, parts: Vec<FormPart>) -> Result<bytes::Bytes, BotError>;
}

/// The default [`BotClient`] backed by `reqwest`.
///
/// Created automatically by [`Bot::new`] and friends. Obtain one directly
/// only when you need to share an HTTP client across multiple `Bot` instances.
#[derive(Debug, Clone)]
pub struct ReqwestClient {
    pub(crate) inner: reqwest::Client,
}

impl ReqwestClient {
    /// Create a `ReqwestClient` with the given HTTP timeout.
    pub fn with_timeout(timeout: std::time::Duration) -> Result<Self, BotError> {
        let inner = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .map_err(BotError::Http)?;
        Ok(Self { inner })
    }

    /// HTTP client for outbound API calls (sendMessage, etc.).
    /// 10 s timeout, large connection pool, TCP_NODELAY, keepalive.
    pub fn for_api() -> Result<Self, BotError> {
        let inner = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .pool_max_idle_per_host(200)
            .tcp_keepalive(std::time::Duration::from_secs(60))
            .tcp_nodelay(true)
            .build()
            .map_err(BotError::Http)?;
        Ok(Self { inner })
    }

    /// HTTP client for long-polling (`getUpdates`).
    /// 65 s timeout (slightly over the max 60 s poll), pool of 1, keepalive.
    /// Pass a separate `for_api()` bot to handlers.
    pub fn for_polling() -> Result<Self, BotError> {
        let inner = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(65))
            .pool_max_idle_per_host(1)
            .tcp_keepalive(std::time::Duration::from_secs(60))
            .build()
            .map_err(BotError::Http)?;
        Ok(Self { inner })
    }
}

#[async_trait]
impl BotClient for ReqwestClient {
    async fn post_json(
        &self,
        url: &str,
        body: serde_json::Value,
    ) -> Result<bytes::Bytes, BotError> {
        self.inner
            .post(url)
            .json(&body)
            .send()
            .await
            .map_err(BotError::Http)?
            .bytes()
            .await
            .map_err(BotError::Http)
    }

    async fn post_form(&self, url: &str, parts: Vec<FormPart>) -> Result<bytes::Bytes, BotError> {
        let mut form = reqwest::multipart::Form::new();

        for part in parts {
            match part.body {
                FormBody::Text(text) => {
                    form = form.text(part.name, text);
                }
                FormBody::Bytes { .. } => {
                    // Binary file uploads are not supported on WASM targets because
                    // the multipart/form-data attachment API is unavailable in that
                    // environment.  Use InputFile::FileId or a URL string instead.
                    #[cfg(target_arch = "wasm32")]
                    return Err(BotError::Other(
                        "file uploads are not supported on WASM; use file_id or a URL".into(),
                    ));

                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        let FormBody::Bytes {
                            filename,
                            mime,
                            data,
                        } = part.body
                        else {
                            unreachable!()
                        };
                        let rpart = reqwest::multipart::Part::bytes(data.to_vec())
                            .file_name(filename)
                            .mime_str(&mime)
                            .map_err(|e| BotError::Other(e.to_string()))?;
                        form = form.part(part.name, rpart);
                    }
                }
            }
        }

        self.inner
            .post(url)
            .multipart(form)
            .send()
            .await
            .map_err(BotError::Http)?
            .bytes()
            .await
            .map_err(BotError::Http)
    }
}
