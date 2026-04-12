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

use crate::types::{ForceReply, InlineKeyboardMarkup, ReplyKeyboardMarkup, ReplyKeyboardRemove};
use serde::{Deserialize, Serialize};

/// Unified `reply_markup` type covering all four keyboard variants.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ReplyMarkup {
    InlineKeyboard(InlineKeyboardMarkup),
    ReplyKeyboard(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}

impl From<InlineKeyboardMarkup> for ReplyMarkup {
    fn from(v: InlineKeyboardMarkup) -> Self {
        ReplyMarkup::InlineKeyboard(v)
    }
}
impl From<ReplyKeyboardMarkup> for ReplyMarkup {
    fn from(v: ReplyKeyboardMarkup) -> Self {
        ReplyMarkup::ReplyKeyboard(v)
    }
}
impl From<ReplyKeyboardRemove> for ReplyMarkup {
    fn from(v: ReplyKeyboardRemove) -> Self {
        ReplyMarkup::ReplyKeyboardRemove(v)
    }
}
impl From<ForceReply> for ReplyMarkup {
    fn from(v: ForceReply) -> Self {
        ReplyMarkup::ForceReply(v)
    }
}
