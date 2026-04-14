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

//! Pluggable storage backends for [`ConversationHandler`](crate::ConversationHandler).
//!
//! # Backends
//!
//! | Backend | Feature flag | Description |
//! |---------|-------------|-------------|
//! | [`InMemoryStorage`] | *(always available)* | Fast, process-local. Lost on restart. |
//! | [`RedisStorage`] | `redis-storage` | Persistent, multi-process safe. Survives restarts. |
//!
//! # Custom backend
//!
//! Implement [`ConversationStorage`] to use any backing store (Postgres, SQLite,
//! DynamoDB, etc.):
//!
//! ```rust,no_run
//! use std::sync::Arc;
//! use ferobot::storage::{ConversationStorage, KeyNotFound};
//!
//! struct MyDb;
//!
//! impl ConversationStorage for MyDb {
//!     fn get(&self, key: &str) -> Result<Arc<str>, KeyNotFound> {
//!         // query your DB ...
//!         Err(KeyNotFound)
//!     }
//!     fn set(&self, key: &str, state: &str) { /* upsert */ }
//!     fn delete(&self, key: &str) { /* delete row */ }
//! }
//! ```
//!
//! # Redis storage
//!
//! ```toml
//! ferobot = { version = "0.1", features = ["redis-storage"] }
//! ```
//!
//! ```rust,no_run
//! # #[cfg(feature = "redis-storage")]
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! use ferobot::storage::RedisStorage;
//! use ferobot::{ConversationOpts, ConversationHandler};
//!
//! let storage = RedisStorage::new("redis://127.0.0.1/")
//!     .await?
//!     .with_prefix("mybot:")   // namespace by bot name
//!     .with_ttl(86400);        // expire idle sessions after 24 h
//!
//! let opts = ConversationOpts {
//!     storage: Some(storage),
//!     ..Default::default()
//! };
//! # Ok(())
//! # }
//! ```

pub use crate::framework::handlers::conversation::{
    ConversationStorage, InMemoryStorage, KeyNotFound,
};

#[cfg(feature = "redis-storage")]
pub use crate::framework::handlers::conversation::redis_storage::RedisStorage;
