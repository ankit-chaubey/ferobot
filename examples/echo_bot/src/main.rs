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

// echo_bot - the simplest ferobot example.
//
// Run:
//   BOT_TOKEN=<your token> cargo run --example echo_bot
//
// The bot replies to every text message with the same text.
// It also responds to /start with a greeting.

use ferobot::framework::filters::message as mf;
use ferobot::{
    framework::{Context, HandlerResult},
    middleware::LoggingMiddleware,
    Bot, CommandHandler, Dispatcher, DispatcherOpts, MessageHandler, Updater,
};

async fn cmd_start(bot: Bot, ctx: Context) -> HandlerResult {
    if let Some(msg) = ctx.effective_message() {
        let name = msg
            .from
            .as_ref()
            .map(|u| u.first_name.as_str())
            .unwrap_or("stranger");
        msg.reply(
            &bot,
            format!("👋 Hello, {name}! Send me any text and I'll echo it back."),
            None,
        )
        .await?;
    }
    Ok(())
}

async fn echo(bot: Bot, ctx: Context) -> HandlerResult {
    if let Some(msg) = ctx.effective_message() {
        if let Some(text) = msg.get_text() {
            msg.reply(&bot, text.to_owned(), None).await?;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    // Initialise tracing so the LoggingMiddleware output is visible.
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter("info")
        .init();

    let token = std::env::var("BOT_TOKEN").expect("BOT_TOKEN env var must be set");
    let bot = Bot::new(token)
        .await
        .expect("failed to connect to Telegram");

    println!(
        "echo_bot running as @{}",
        bot.me.username.as_deref().unwrap_or("unknown")
    );

    let opts = DispatcherOpts::default().middleware(LoggingMiddleware); // log every incoming update

    let mut dp = Dispatcher::new(opts);

    // Handlers run in registration order within group 0.
    // CommandHandler matches before the generic text echo, so /start is handled cleanly.
    dp.add_handler(CommandHandler::new("start", cmd_start));
    dp.add_handler(MessageHandler::new("echo_text", mf::text(), echo));

    Updater::new(bot, dp)
        .poll_timeout(30)
        .start_polling()
        .await
        .expect("polling failed");
}
