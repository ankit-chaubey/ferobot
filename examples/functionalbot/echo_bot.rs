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

//! Minimal echo bot using the ferobot helper methods.
//!
//! Demonstrates: `msg.get_text()`, `msg.reply()`, `Updater`.

use ferobot::{Bot, CommandHandler, Dispatcher, DispatcherOpts, HandlerResult, Updater, Context};

async fn echo(bot: Bot, ctx: Context) -> HandlerResult {
    if let Some(msg) = ctx.effective_message() {
        if let Some(text) = msg.get_text() {
            msg.reply(&bot, text, None).await?;
        }
    }
    Ok(())
}

async fn start(bot: Bot, ctx: Context) -> HandlerResult {
    if let Some(msg) = ctx.effective_message() {
        let link = msg.get_link();
        let info = if link.is_empty() {
            "No public link (private/group chat)".into()
        } else {
            format!("Message link: {link}")
        };
        msg.reply(&bot, format!("Hello! I'll echo your messages.\n{info}"), None).await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = std::env::var("BOT_TOKEN").expect("BOT_TOKEN not set");
    let bot   = Bot::new(token).await.expect("failed to connect");

    let mut dp = Dispatcher::new(DispatcherOpts::default());
    dp.add_handler(CommandHandler::new("start", start));
    dp.add_handler(CommandHandler::new("echo",  echo));

    println!("Polling as @{}", bot.me.username.as_deref().unwrap_or("unknown"));
    Updater::new(bot, dp)
        .poll_timeout(30)
        .start_polling()
        .await
        .expect("polling failed");
}
