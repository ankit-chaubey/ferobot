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

// command_bot - demonstrates CommandHandler, filters, middleware, and RetryPolicy.
//
// Run:
//   BOT_TOKEN=<your token> cargo run --example command_bot
//
// Supported commands:
//   /start        – greeting
//   /help         – list commands
//   /ping         – pong
//   /shout <text> – replies in uppercase
//   /about        – library info

use ferobot::{
    framework::{Context, HandlerResult},
    middleware::{LoggingMiddleware, RateLimiter},
    retry::RetryPolicy,
    Bot, CommandHandler, Dispatcher, DispatcherOpts, Updater,
};

fn policy() -> RetryPolicy {
    RetryPolicy::new()
        .max_attempts(3)
        .base_delay(std::time::Duration::from_millis(300))
}

async fn cmd_start(bot: Bot, ctx: Context) -> HandlerResult {
    let Some(msg) = ctx.effective_message() else {
        return Ok(());
    };
    let name = msg
        .from
        .as_ref()
        .map(|u| u.first_name.as_str())
        .unwrap_or("there");
    let text = format!(
        "👋 Hello, {name}!\n\
         I'm a demo bot built with <b>ferobot</b>.\n\n\
         Try /help to see what I can do."
    );
    policy().run(|| msg.reply(&bot, text.clone(), None)).await?;
    Ok(())
}

async fn cmd_help(bot: Bot, ctx: Context) -> HandlerResult {
    let Some(msg) = ctx.effective_message() else {
        return Ok(());
    };
    let text = "\
/start        – greeting\n\
/help         – this list\n\
/ping         – latency check\n\
/shout <text> – YELL BACK\n\
/about        – library info";
    policy().run(|| msg.reply(&bot, text, None)).await?;
    Ok(())
}

async fn cmd_ping(bot: Bot, ctx: Context) -> HandlerResult {
    let Some(msg) = ctx.effective_message() else {
        return Ok(());
    };
    policy().run(|| msg.reply(&bot, "🏓 Pong!", None)).await?;
    Ok(())
}

async fn cmd_shout(bot: Bot, ctx: Context) -> HandlerResult {
    let Some(msg) = ctx.effective_message() else {
        return Ok(());
    };
    let reply = if ctx.args().is_empty() {
        "Usage: /shout <something to shout>".to_owned()
    } else {
        ctx.args().join(" ").to_uppercase()
    };
    policy()
        .run(|| msg.reply(&bot, reply.clone(), None))
        .await?;
    Ok(())
}

async fn cmd_about(bot: Bot, ctx: Context) -> HandlerResult {
    let Some(msg) = ctx.effective_message() else {
        return Ok(());
    };
    let text = "\
🦀 <b>ferobot v0.1.0</b>\n\
Async Telegram Bot API framework for Rust.\n\n\
• All 285 types and 165 methods, fully generated\n\
• Dispatcher with group-ordered handler routing\n\
• Conversation (FSM) support with Redis storage\n\
• Built-in webhook server (axum)\n\
• Per-chat sequential concurrency\n\
• Middleware + RetryPolicy\n\n\
<a href=\"https://github.com/ankit-chaubey/ferobot\">github.com/ankit-chaubey/ferobot</a>";
    policy().run(|| msg.reply(&bot, text, None)).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter("info")
        .init();

    let token = std::env::var("BOT_TOKEN").expect("BOT_TOKEN env var must be set");
    let bot = Bot::new(token)
        .await
        .expect("failed to connect to Telegram");

    println!(
        "command_bot running as @{}",
        bot.me.username.as_deref().unwrap_or("unknown")
    );

    let opts = DispatcherOpts::default()
        .middleware(LoggingMiddleware)
        .middleware(RateLimiter::new(5)); // max 5 updates / second per chat

    let mut dp = Dispatcher::new(opts);
    dp.add_handler(CommandHandler::new("start", cmd_start));
    dp.add_handler(CommandHandler::new("help", cmd_help));
    dp.add_handler(CommandHandler::new("ping", cmd_ping));
    dp.add_handler(CommandHandler::new("shout", cmd_shout));
    dp.add_handler(CommandHandler::new("about", cmd_about));

    Updater::new(bot, dp)
        .poll_timeout(30)
        .start_polling()
        .await
        .expect("polling failed");
}
