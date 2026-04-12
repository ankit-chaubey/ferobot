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

//! Filters for `ChatMemberUpdated` updates.

use crate::types::{ChatMember, ChatMemberUpdated};

fn status(cm: &ChatMember) -> &str {
    match cm {
        ChatMember::ChatMemberOwner(m) => &m.status,
        ChatMember::ChatMemberAdministrator(m) => &m.status,
        ChatMember::ChatMemberMember(m) => &m.status,
        ChatMember::ChatMemberRestricted(m) => &m.status,
        ChatMember::ChatMemberLeft(m) => &m.status,
        ChatMember::ChatMemberBanned(m) => &m.status,
    }
}

pub fn all() -> impl super::Filter<ChatMemberUpdated> {
    |_: &ChatMemberUpdated| true
}

pub fn joined() -> impl super::Filter<ChatMemberUpdated> {
    |c: &ChatMemberUpdated| {
        matches!(
            status(&c.new_chat_member),
            "member" | "administrator" | "creator"
        )
    }
}

pub fn left() -> impl super::Filter<ChatMemberUpdated> {
    |c: &ChatMemberUpdated| matches!(status(&c.new_chat_member), "left" | "kicked")
}

pub fn promoted() -> impl super::Filter<ChatMemberUpdated> {
    |c: &ChatMemberUpdated| status(&c.new_chat_member) == "administrator"
}

pub fn chat_id(id: i64) -> impl super::Filter<ChatMemberUpdated> {
    move |c: &ChatMemberUpdated| c.chat.id == id
}
