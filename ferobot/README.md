<div align="center">

<img src="https://rustacean.net/assets/rustacean-orig-noshadow.svg" width="110" alt="Ferris the Crab"/>

<h1>ferobot</h1>

<p>Async Telegram Bot API library for Rust.</p>

[![Crates.io](https://img.shields.io/crates/v/ferobot?style=for-the-badge&logo=rust&color=f74c00&labelColor=1a1a2e)](https://crates.io/crates/ferobot)
[![docs.rs](https://img.shields.io/docsrs/ferobot?style=for-the-badge&logo=docs.rs&color=4a90d9&labelColor=1a1a2e)](https://docs.rs/ferobot)
[![CI](https://img.shields.io/github/actions/workflow/status/ankit-chaubey/ferobot/ci.yml?branch=main&style=for-the-badge&logo=github-actions&label=CI&color=2ea44f&labelColor=1a1a2e)](https://github.com/ankit-chaubey/ferobot/actions/workflows/ci.yml)
[![API Sync](https://img.shields.io/github/actions/workflow/status/ankit-chaubey/ferobot/auto-regenerate.yml?style=for-the-badge&logo=telegram&label=API+SYNC&color=0088cc&labelColor=1a1a2e)](https://github.com/ankit-chaubey/ferobot/actions/workflows/auto-regenerate.yml)

[![Bot API](https://img.shields.io/badge/Telegram%20Bot%20API-9.4-0088cc?style=flat-square&logo=telegram&logoColor=white)](https://core.telegram.org/bots/api)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-f74c00?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-MIT-eab308?style=flat-square)](LICENSE)

[Install](#installation) · [Quick Start](#quick-start) · [How It Works](#how-it-works) · [docs.rs](https://docs.rs/ferobot)

</div>

---

## What is ferobot?

ferobot is a Telegram Bot API library for Rust. It covers all methods and types from the official API out of the box, and keeps them in sync automatically.

When Telegram ships a new Bot API version, CI picks up the spec change, regenerates the types and methods, and opens a PR. You get the update without touching anything.

The library is built around Tokio. Every bot method is async, every update runs in its own task, and the dispatcher handles routing, concurrency, and panic recovery out of the box.

---

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ferobot = "0.2"
tokio   = { version = "1", features = ["full"] }
```

Optional features you can enable:

| Feature | What it adds |
|---|---|
| `webhook` | Built-in Axum webhook server |
| `per-chat` | Sequential update processing per chat ID |
| `redis-storage` | Redis-backed conversation state |

---

## Quick Start

Get your token from [@BotFather](https://t.me/BotFather), then:

```rust
use ferobot::{Bot, CommandHandler, Dispatcher, DispatcherOpts, HandlerResult, Updater};
use ferobot::framework::Context;

async fn start(bot: Bot, ctx: Context) -> HandlerResult {
    if let Some(msg) = ctx.effective_message() {
        msg.reply(&bot, "hello!", None).await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let bot = Bot::new(std::env::var("BOT_TOKEN").unwrap()).await.unwrap();

    let mut dp = Dispatcher::new(DispatcherOpts::default());
    dp.add_handler(CommandHandler::new("start", start));

    Updater::new(bot, dp).start_polling().await.unwrap();
}
```

---

## How It Works

### Calling methods

Every bot method takes required arguments directly. Optional parameters are passed through a params struct, or you can skip them entirely with `None`:

```rust
// send a plain message
bot.send_message(chat_id, "hello").await?;

// with optional params chained via builder
bot.send_message(chat_id, "<b>hello</b>")
    .html()
    .silent()
    .reply_to(msg.message_id)
    .await?;
```

Shortcuts like `.html()`, `.markdown()`, `.silent()`, and `.reply_to(id)` cover the most common options. For anything else, use the params struct directly:

```rust
use ferobot::gen_methods::SendMessageParams;

let params = SendMessageParams::new()
    .parse_mode("HTML")
    .disable_notification(true);

bot.send_message(chat_id, "<b>hello</b>", Some(params)).await?;
```

### Dispatcher and handlers

The dispatcher routes incoming updates to the first matching handler. Handlers are plain async functions:

```rust
dp.add_handler(CommandHandler::new("start", start));
dp.add_handler(MessageHandler::new("echo", mf::text(), echo));
dp.add_handler(CallbackQueryHandler::new("btn", |_| true, on_button));
```

Filters compose with `.and()`, `.or()`, `.not()`:

```rust
// text that is not a command
MessageHandler::new("msg", mf::text().and(mf::command().not()), handler)
```

### Conversations

For multi-step flows (registration, forms, wizards), use `ConversationHandler`. It manages state per user and routes each message to the right step:

```rust
let conv = ConversationHandler::new(
    ConversationOpts::default(),
    vec![CommandHandler::new("start", ask_name)],     // entry points
    vec![("waiting_name", vec![...])],                 // states
    vec![],                                            // fallbacks
);
```

State is stored in memory by default. Add the `redis-storage` feature for persistence across restarts.

### ChatId and InputFile

`ChatId` accepts integers, negative group IDs, and `@username` strings interchangeably:

```rust
bot.send_message(123456789_i64, "hi").await?;
bot.send_message(-100123456789_i64, "hi group").await?;
bot.send_message("@username", "hi").await?;
```

`InputFile` accepts a file ID, a URL, or raw bytes at the same call site:

```rust
bot.send_photo(chat_id, "AgACAgIAAxkBAAI...").await?;                          // file_id
bot.send_photo(chat_id, "https://example.com/img.jpg").await?;                  // URL
bot.send_photo(chat_id, InputFile::memory("photo.jpg", bytes)).await?;          // bytes
```

---

## Raw API

If Telegram ships a new method and you can't wait for the next ferobot release, `bot.raw()` lets you call any method by name with arbitrary params and get back a JSON value.

```rust
// call any method by name
let result: serde_json::Value = bot
    .raw("sendGift")
    .param("chat_id", 123456789_i64)
    .param("gift_id", "gift_abc123")
    .call()
    .await?;
```

File uploads work too:

```rust
bot.raw("sendPhoto")
    .param("chat_id", chat_id)
    .file("photo", InputFile::memory("photo.jpg", bytes))
    .call::<serde_json::Value>()
    .await?;
```

For fire-and-forget calls where you don't care about the response:

```rust
bot.raw("deleteMessage")
    .param("chat_id", chat_id)
    .param("message_id", msg_id)
    .send()
    .await?;
```

You can also deserialize into your own struct instead of `serde_json::Value`:

```rust
#[derive(serde::Deserialize)]
struct MyUser { id: i64, username: Option<String> }

let user: MyUser = bot.raw("getMe").call().await?;
```

In practice you rarely need this since CI keeps ferobot in sync within hours of a Telegram release. But it's a useful escape hatch when you're in a hurry or testing something new.

---

## Always Up to Date

ferobot's types and methods are generated from [tgapis/x](https://github.com/tgapis/x/tree/data), a machine-readable mirror of the official Telegram Bot API spec. A GitHub Actions workflow watches the spec repo and triggers regeneration whenever the API changes. The generated files (`gen_types.rs`, `gen_methods.rs`, `fluent.rs`) are never edited by hand.

---

## Contributing

```sh
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --all
```

One concern per PR. For bugs and features, open a GitHub issue. For security issues, email [ankitchaubey.dev@gmail.com](mailto:ankitchaubey.dev@gmail.com).

---

## License

MIT - Copyright (c) 2026 [Ankit Chaubey](https://github.com/ankit-chaubey)

---

<div align="center">

Built by [Ankit Chaubey](https://github.com/ankit-chaubey) · [Telegram](https://t.me/ankify) · [docs.rs](https://docs.rs/ferobot) · [crates.io](https://crates.io/crates/ferobot)

*If ferobot saved you time, a star on GitHub means a lot.*

</div>
