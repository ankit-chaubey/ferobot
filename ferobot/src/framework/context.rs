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

use std::collections::HashMap;

use crate::types::{Chat, MaybeInaccessibleMessage, Message, Update, User};

/// Per-update context passed to every handler.
#[derive(Debug, Clone)]
pub struct Context {
    pub update: Update,
    /// Shared data bag for passing values between handlers.
    pub data: HashMap<String, String>,
    pub(crate) args: Vec<String>,
}

impl Context {
    pub fn new(update: Update) -> Self {
        Self {
            update,
            data: HashMap::new(),
            args: Vec::new(),
        }
    }

    /// Chat this update belongs to.
    pub fn effective_chat(&self) -> Option<&Chat> {
        let u = &self.update;
        if let Some(m) = u.message.as_ref() {
            return Some(&m.chat);
        }
        if let Some(m) = u.edited_message.as_ref() {
            return Some(&m.chat);
        }
        if let Some(m) = u.channel_post.as_ref() {
            return Some(&m.chat);
        }
        if let Some(m) = u.edited_channel_post.as_ref() {
            return Some(&m.chat);
        }
        if let Some(cq) = u.callback_query.as_ref() {
            if let Some(msg) = cq.message.as_ref() {
                if let MaybeInaccessibleMessage::Message(m) = msg.as_ref() {
                    return Some(&m.chat);
                }
            }
        }
        if let Some(c) = u.my_chat_member.as_ref() {
            return Some(&c.chat);
        }
        if let Some(c) = u.chat_member.as_ref() {
            return Some(&c.chat);
        }
        if let Some(c) = u.chat_join_request.as_ref() {
            return Some(&c.chat);
        }
        None
    }

    /// User who triggered this update.
    pub fn effective_user(&self) -> Option<&User> {
        let u = &self.update;
        if let Some(m) = u.message.as_ref() {
            if let Some(f) = m.from.as_ref() {
                return Some(f);
            }
        }
        if let Some(m) = u.edited_message.as_ref() {
            if let Some(f) = m.from.as_ref() {
                return Some(f);
            }
        }
        if let Some(cq) = u.callback_query.as_ref() {
            return Some(&cq.from);
        }
        if let Some(iq) = u.inline_query.as_ref() {
            return Some(&iq.from);
        }
        if let Some(ci) = u.chosen_inline_result.as_ref() {
            return Some(&ci.from);
        }
        if let Some(m) = u.channel_post.as_ref() {
            if let Some(f) = m.from.as_ref() {
                return Some(f);
            }
        }
        if let Some(c) = u.my_chat_member.as_ref() {
            return Some(&c.from);
        }
        if let Some(c) = u.chat_member.as_ref() {
            return Some(&c.from);
        }
        if let Some(c) = u.chat_join_request.as_ref() {
            return Some(&c.from);
        }
        None
    }

    /// Message this update relates to.
    pub fn effective_message(&self) -> Option<&Message> {
        let u = &self.update;
        if let Some(m) = u.message.as_ref() {
            return Some(m);
        }
        if let Some(m) = u.edited_message.as_ref() {
            return Some(m);
        }
        if let Some(m) = u.channel_post.as_ref() {
            return Some(m);
        }
        if let Some(m) = u.edited_channel_post.as_ref() {
            return Some(m);
        }
        if let Some(cq) = u.callback_query.as_ref() {
            if let Some(msg) = cq.message.as_ref() {
                if let MaybeInaccessibleMessage::Message(m) = msg.as_ref() {
                    return Some(m);
                }
            }
        }
        None
    }

    /// Args split out of a command message, e.g. `["-n", "5"]` from `/cmd -n 5`.
    pub fn args(&self) -> &[String] {
        &self.args
    }

    /// Args joined back into a single string.
    pub fn args_str(&self) -> String {
        self.args.join(" ")
    }
}
