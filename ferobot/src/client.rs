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

    /// POST pre-serialized JSON bytes to `url`. Avoids re-serializing a
    /// `serde_json::Value` that was already rendered to bytes by the caller.
    /// The default delegates to `post_json` via deserialization; override for
    /// zero-copy performance.
    async fn post_json_raw(&self, url: &str, body: Vec<u8>) -> Result<bytes::Bytes, BotError> {
        let v: serde_json::Value =
            serde_json::from_slice(&body).unwrap_or(serde_json::Value::Object(Default::default()));
        self.post_json(url, v).await
    }

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
    /// Uses the same optimized settings as `for_api` so every Bot,
    /// regardless of how it was constructed, gets keepalive + TCP_NODELAY.
    pub fn with_timeout(timeout: std::time::Duration) -> Result<Self, BotError> {
        Self::for_api_with_timeout(timeout)
    }

    /// HTTP client for outbound API calls (sendMessage, etc.).
    /// 10 s timeout, HTTP/2, large connection pool, TCP_NODELAY, keepalive.
    pub fn for_api() -> Result<Self, BotError> {
        Self::for_api_with_timeout(std::time::Duration::from_secs(10))
    }

    /// Like `for_api` but with a custom read timeout.
    pub fn for_api_with_timeout(timeout: std::time::Duration) -> Result<Self, BotError> {
        let inner = reqwest::Client::builder()
            .timeout(timeout)
            .connect_timeout(std::time::Duration::from_secs(5))
            .pool_max_idle_per_host(512)
            .pool_idle_timeout(std::time::Duration::from_secs(55))
            .tcp_keepalive(std::time::Duration::from_secs(30))
            .tcp_nodelay(true)
            // HTTP/2 is negotiated via ALPN over TLS (rustls-tls feature).
            // No prior_knowledge here: that is for plaintext h2 only.
            .http2_adaptive_window(true)
            .http2_keep_alive_interval(std::time::Duration::from_secs(20))
            .http2_keep_alive_timeout(std::time::Duration::from_secs(5))
            .http2_keep_alive_while_idle(true)
            .gzip(true)
            .build()
            .map_err(BotError::Http)?;
        Ok(Self { inner })
    }

    /// HTTP client for long-polling (`getUpdates`).
    /// 65 s read timeout (slightly over the max 60 s poll), single-connection
    /// pool so it never contends with outbound API calls.
    pub fn for_polling() -> Result<Self, BotError> {
        let inner = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(65))
            .connect_timeout(std::time::Duration::from_secs(5))
            .pool_max_idle_per_host(1)
            .pool_idle_timeout(std::time::Duration::from_secs(55))
            .tcp_keepalive(std::time::Duration::from_secs(30))
            .tcp_nodelay(true)
            .http2_adaptive_window(true)
            .http2_keep_alive_interval(std::time::Duration::from_secs(20))
            .http2_keep_alive_timeout(std::time::Duration::from_secs(5))
            .http2_keep_alive_while_idle(true)
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

    /// Zero-copy fast path: body is already serialized, send raw bytes directly
    /// without re-parsing into a `serde_json::Value` tree.
    async fn post_json_raw(&self, url: &str, body: Vec<u8>) -> Result<bytes::Bytes, BotError> {
        self.inner
            .post(url)
            .header("Content-Type", "application/json")
            .body(body)
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
