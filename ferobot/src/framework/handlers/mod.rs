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

pub mod callback_query;
pub mod command;
pub mod conversation;
pub mod message;

pub use callback_query::CallbackQueryHandler;
pub use command::CommandHandler;
#[cfg(feature = "redis-storage")]
pub use conversation::redis_storage::RedisStorage;
pub use conversation::{
    ConversationHandler, ConversationOpts, ConversationStorage, EndConversation, InMemoryStorage,
    KeyStrategy, NextState,
};
pub use message::MessageHandler;
