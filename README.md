<div align="center">

<img src="https://rustacean.net/assets/rustacean-orig-noshadow.svg" width="110" alt="Ferris the Crab"/>

<h1>ferobot</h1>

<p>Telegram Bot API library for Rust. All types and methods, fully async.</p>

[![Crates.io](https://img.shields.io/crates/v/ferobot?style=for-the-badge&logo=rust&color=f74c00&labelColor=1a1a2e)](https://crates.io/crates/ferobot)
[![docs.rs](https://img.shields.io/docsrs/ferobot?style=for-the-badge&logo=docs.rs&color=4a90d9&labelColor=1a1a2e)](https://docs.rs/ferobot)
[![CI](https://img.shields.io/github/actions/workflow/status/ankit-chaubey/ferobot/ci.yml?branch=main&style=for-the-badge&logo=github-actions&label=CI&color=2ea44f&labelColor=1a1a2e)](https://github.com/ankit-chaubey/ferobot/actions/workflows/ci.yml)
[![API Sync](https://img.shields.io/github/actions/workflow/status/ankit-chaubey/ferobot/auto-regenerate.yml?style=for-the-badge&logo=telegram&label=API+SYNC&color=0088cc&labelColor=1a1a2e)](https://github.com/ankit-chaubey/ferobot/actions/workflows/auto-regenerate.yml)

[![Bot API](https://img.shields.io/badge/Telegram%20Bot%20API-9.4-0088cc?style=flat-square&logo=telegram&logoColor=white)](https://core.telegram.org/bots/api)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-f74c00?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-MIT-eab308?style=flat-square)](LICENSE)

[Install](#installation) · [Quick Start](#quick-start) · [Examples](#examples) · [API Reference](#api-reference) · [docs.rs](https://docs.rs/ferobot)

</div>

---

## Installation

```toml
[dependencies]
ferobot = "0.1"
tokio   = { version = "1", features = ["full"] }
```

Optional features:

```toml
ferobot = { version = "0.1", features = ["webhook"] }       # built-in axum webhook server
ferobot = { version = "0.1", features = ["per-chat"] }      # sequential per-chat concurrency
ferobot = { version = "0.1", features = ["redis-storage"] } # Redis conversation storage
```

---

## Quick Start

```rust
use ferobot::Bot;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bot = Bot::new("YOUR_TOKEN").await?;
    println!("running as @{}", bot.me.username.as_deref().unwrap_or("?"));
    bot.send_message(123456789i64, "hello 🦀", None).await?;
    Ok(())
}
```

---

## Examples

### Dispatcher + commands

The recommended way to build a bot. Handlers are async functions; the dispatcher routes updates to the first match.

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

### Echo bot (low-level polling)

Use `Poller` directly when you don't need the dispatcher.

```rust
use ferobot::{Bot, Poller, UpdateHandler};

#[tokio::main]
async fn main() {
    let bot = Bot::new(std::env::var("BOT_TOKEN").unwrap()).await.unwrap();

    let handler: UpdateHandler = Box::new(|bot, update| {
        Box::pin(async move {
            let Some(msg) = update.message else { return };
            let Some(text) = msg.text else { return };
            let _ = bot.send_message(msg.chat.id, text, None).await;
        })
    });

    Poller::new(bot, handler).timeout(30).start().await.unwrap();
}
```

---

### Middleware

Middleware runs before and after every update. Return `false` from `before` to drop the update.

```rust
use ferobot::{Dispatcher, DispatcherOpts, Updater};
use ferobot::middleware::{LoggingMiddleware, RateLimiter};

let opts = DispatcherOpts::default()
    .middleware(LoggingMiddleware)    // logs every update
    .middleware(RateLimiter::new(5)); // max 5 updates/sec per chat
```

Custom middleware:

```rust
use ferobot::middleware::Middleware;
use ferobot::{Bot, types::Update};
use async_trait::async_trait;

struct AuthGuard;

#[async_trait]
impl Middleware for AuthGuard {
    async fn before(&self, _bot: &Bot, update: &Update) -> bool {
        let allowed = [111111111i64, 222222222i64];
        let uid = update.message.as_ref()
            .and_then(|m| m.from.as_ref())
            .map(|u| u.id);
        uid.map(|id| allowed.contains(&id)).unwrap_or(false)
    }
}
```

---

### Retry

Wraps any API call and retries on flood-wait (429) and network errors.

```rust
use ferobot::RetryPolicy;

let msg = RetryPolicy::new()
    .max_attempts(3)
    .run(|| bot.send_message(chat_id, "hello", None))
    .await?;
```

---

### Inline keyboards

```rust
use ferobot::{ReplyMarkup, gen_methods::SendMessageParams};
use ferobot::types::{InlineKeyboardButton, InlineKeyboardMarkup};

let keyboard = InlineKeyboardMarkup {
    inline_keyboard: vec![vec![
        InlineKeyboardButton {
            text: "yes".into(),
            callback_data: Some("yes".into()),
            ..Default::default()
        },
        InlineKeyboardButton {
            text: "no".into(),
            callback_data: Some("no".into()),
            ..Default::default()
        },
    ]],
};

let params = SendMessageParams::new()
    .reply_markup(ReplyMarkup::InlineKeyboard(keyboard));

bot.send_message(chat_id, "pick one", Some(params)).await?;
```

---

### Callback queries

```rust
use ferobot::gen_methods::{AnswerCallbackQueryParams, EditMessageTextParams};
use ferobot::types::MaybeInaccessibleMessage;

let Some(cq) = update.callback_query else { return };
let data = cq.data.as_deref().unwrap_or("");

bot.answer_callback_query(
    cq.id.clone(),
    Some(AnswerCallbackQueryParams::new().text(format!("got: {data}"))),
).await?;

if let Some(MaybeInaccessibleMessage::Message(m)) = cq.message.as_deref() {
    let params = EditMessageTextParams::new()
        .chat_id(m.chat.id)
        .message_id(m.message_id);
    bot.edit_message_text(format!("you chose: {data}"), Some(params)).await?;
}
```

---

### Send files

```rust
use ferobot::InputFile;

// file already on Telegram
bot.send_photo(chat_id, "AgACAgIAAxkBAAI...", None).await?;

// URL (Telegram fetches it)
bot.send_photo(chat_id, "https://example.com/img.jpg", None).await?;

// raw bytes
let data = tokio::fs::read("photo.jpg").await?;
bot.send_photo(chat_id, InputFile::memory("photo.jpg", data), None).await?;
```

---

### Webhook (built-in server)

Requires the `webhook` feature.

```rust
use ferobot::{Bot, Dispatcher, DispatcherOpts, Updater};

let bot = Bot::new(std::env::var("BOT_TOKEN").unwrap()).await.unwrap();
let dp  = Dispatcher::new(DispatcherOpts::default());

Updater::new(bot, dp)
    .webhook_port(8443)
    .webhook_secret("my_secret")
    .start_webhook("https://yourdomain.com/bot")
    .await
    .unwrap();
```

For local testing: `ngrok http 8443`.

---

### Webhook (manual, bring your own server)

```rust
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use std::sync::Arc;
use ferobot::{types::Update, Bot};

struct AppState { bot: Bot }

#[tokio::main]
async fn main() {
    let bot = Bot::new("YOUR_TOKEN").await.unwrap();
    bot.set_webhook("https://yourdomain.com/bot", None).await.unwrap();

    let app = Router::new()
        .route("/bot", post(handle))
        .with_state(Arc::new(AppState { bot }));

    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:8443").await.unwrap(),
        app,
    ).await.unwrap();
}

async fn handle(State(s): State<Arc<AppState>>, Json(update): Json<Update>) -> StatusCode {
    let bot = s.bot.clone();
    tokio::spawn(async move {
        if let Some(msg) = update.message {
            let _ = bot.send_message(msg.chat.id, "got it", None).await;
        }
    });
    StatusCode::OK
}
```

| | Built-in `WebhookServer` | Manual |
|---|---|---|
| Zero boilerplate | yes | no |
| Secret token validation | built-in | manual |
| Custom routing / middleware | no | yes |
| Works with existing server | no | yes |
| Feature flag needed | `webhook` | no |

---

### Error handling

```rust
use ferobot::BotError;

match bot.send_message(chat_id, "hello", None).await {
    Ok(msg) => println!("sent #{}", msg.message_id),
    Err(BotError::Api { code: 403, .. }) => eprintln!("bot was blocked"),
    Err(e) if e.is_api_error_code(429) => {
        let secs = e.flood_wait_seconds().unwrap_or(5);
        tokio::time::sleep(std::time::Duration::from_secs(secs as u64)).await;
    }
    Err(e) => eprintln!("error: {e}"),
}
```

---

### Conversation (FSM)

Multi-step flows with pluggable state storage (in-memory or Redis).

```rust
use ferobot::{
    Bot, CommandHandler, ConversationHandler, ConversationOpts,
    Dispatcher, DispatcherOpts, HandlerResult, MessageHandler, NextState, Updater,
};
use ferobot::framework::{Context, filters::message as mf};

async fn ask_name(bot: Bot, ctx: Context) -> HandlerResult {
    if let Some(msg) = ctx.effective_message() {
        msg.reply(&bot, "what's your name?", None).await?;
    }
    Ok(NextState::new("waiting_name").into())
}

async fn got_name(bot: Bot, ctx: Context) -> HandlerResult {
    if let Some(msg) = ctx.effective_message() {
        let name = msg.get_text().unwrap_or("?");
        msg.reply(&bot, format!("hi {name}!"), None).await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let bot = Bot::new(std::env::var("BOT_TOKEN").unwrap()).await.unwrap();

    let conv = ConversationHandler::new(
        ConversationOpts::default(),
        vec![CommandHandler::new("start", ask_name)],
        vec![("waiting_name", vec![
            Box::new(MessageHandler::new("name", mf::text(), got_name))
        ])],
        vec![],
    );

    let mut dp = Dispatcher::new(DispatcherOpts::default());
    dp.add_handler(conv);

    Updater::new(bot, dp).start_polling().await.unwrap();
}
```

Redis storage:

```toml
ferobot = { version = "0.1", features = ["redis-storage"] }
```

```rust
use ferobot::storage::RedisStorage;

let storage = RedisStorage::new("redis://127.0.0.1/")
    .await?
    .with_prefix("mybot:")
    .with_ttl(86400);

let opts = ConversationOpts { storage: Some(storage), ..Default::default() };
```

---

## API Reference

### Bot constructors

| Method | Description |
|--------|-------------|
| `Bot::new(token)` | Connect and verify token via `getMe` |
| `Bot::with_api_url(token, url)` | Use a custom or local Bot API server |
| `Bot::new_unverified(token)` | Skip `getMe` on startup |
| `bot.token()` | Get the raw token string |
| `bot.me` | `User` populated on creation |

---

### ChatId

Numeric IDs, negative group IDs, and `@username` strings all work anywhere a `ChatId` is expected:

```rust
bot.send_message(123456789i64, "user", None).await?;
bot.send_message(-100123456789i64, "group or channel", None).await?;
bot.send_message("@username", "by username", None).await?;
```

---

### InputFile

```rust
// file already on Telegram - pass the file_id string directly
bot.send_photo(chat_id, "AgACAgIAAxkBAAI...", None).await?;

// URL - pass the URL string directly
bot.send_photo(chat_id, "https://example.com/img.jpg", None).await?;

// bytes
InputFile::memory("photo.jpg", bytes)
```

---

### BotError

```rust
pub enum BotError {
    Http(reqwest::Error),
    Json(serde_json::Error),
    Api {
        code: i64,
        description: String,
        retry_after: Option<i64>,        // present on 429
        migrate_to_chat_id: Option<i64>, // present on migration errors
    },
    InvalidToken,
    Other(String),
}

err.is_api_error_code(429)  // bool
err.flood_wait_seconds()    // Option<i64>
```

---

### Optional params

Every method with optional fields has a `*Params` builder:

```rust
use ferobot::gen_methods::SendMessageParams;

let params = SendMessageParams::new()
    .parse_mode("HTML".to_string())
    .disable_notification(true);

bot.send_message(chat_id, "<b>hello</b>", Some(params)).await?;
```

---

## Auto-codegen

Types and methods are generated from [tgapis/x](https://github.com/tgapis/x/tree/data), which tracks the official Telegram Bot API. CI auto-regenerates on every upstream change.

Manual regeneration:

```sh
curl -sSf https://raw.githubusercontent.com/tgapis/x/data/botapi.json -o api.json
python3 codegen/codegen.py api.json ferobot/src/
cargo build
```

Never edit `gen_types.rs` or `gen_methods.rs` by hand.

---

## Contributing

```sh
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --all
```

One concern per PR. Run fmt and clippy before submitting. Open a GitHub issue for bugs and features, or email [ankitchaubey.dev@gmail.com](mailto:ankitchaubey.dev@gmail.com) for security issues.

---

## License

MIT License - Copyright (c) 2026 [Ankit Chaubey](https://github.com/ankit-chaubey)

---

<div align="center">

Built by [Ankit Chaubey](https://github.com/ankit-chaubey) · [Telegram](https://t.me/ankify) · [docs.rs](https://docs.rs/ferobot) · [crates.io](https://crates.io/crates/ferobot)

*If ferobot saved you time, a star on GitHub means a lot.*

</div>
