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

//! Filters for `CallbackQuery` updates.

use crate::types::CallbackQuery;
use regex::Regex;

pub fn all() -> impl super::Filter<CallbackQuery> {
    |_: &CallbackQuery| true
}

pub fn data_eq(expected: impl Into<String>) -> impl super::Filter<CallbackQuery> {
    let e = expected.into();
    move |cq: &CallbackQuery| cq.data.as_deref() == Some(&e)
}

pub fn data_starts_with(prefix: impl Into<String>) -> impl super::Filter<CallbackQuery> {
    let p = prefix.into();
    move |cq: &CallbackQuery| {
        cq.data
            .as_deref()
            .map(|d| d.starts_with(&p as &str))
            .unwrap_or(false)
    }
}

/// Panics if the pattern is invalid.
pub fn data_regex(pattern: impl AsRef<str>) -> impl super::Filter<CallbackQuery> {
    let re = Regex::new(pattern.as_ref()).expect("invalid regex");
    move |cq: &CallbackQuery| cq.data.as_deref().map(|d| re.is_match(d)).unwrap_or(false)
}

pub fn data_regex_safe(pattern: impl AsRef<str>) -> Option<impl super::Filter<CallbackQuery>> {
    let re = Regex::new(pattern.as_ref()).ok()?;
    Some(move |cq: &CallbackQuery| cq.data.as_deref().map(|d| re.is_match(d)).unwrap_or(false))
}

pub fn game() -> impl super::Filter<CallbackQuery> {
    |cq: &CallbackQuery| cq.game_short_name.is_some()
}

pub fn from_user_id(id: i64) -> impl super::Filter<CallbackQuery> {
    move |cq: &CallbackQuery| cq.from.id == id
}
