# Changelog

All notable changes to **ferobot** are documented here.

Format: [Semantic Versioning](https://semver.org/)  
Auto-generated API updates use the [Telegram Bot API spec](https://github.com/PaulSonOfLars/telegram-bot-api-spec).

---

## [0.2.2] - 2026-06-12

### Telegram Bot API: `Bot API 10.1`

**Changelog:** [core.telegram.org/bots/api#recent-changes](https://core.telegram.org/bots/api#recent-changes)

Manual release.

---
## [0.2.1] - 2026-06-03

### Telegram Bot API: `Bot API 10.0`

**Changelog:** [core.telegram.org/bots/api#recent-changes](https://core.telegram.org/bots/api#recent-changes)

Manual release.

---
## [0.2.0] - 2026-06-02

### Changed

- Rewrote `ReqwestClient::with_timeout` to build the inner client directly instead of delegating to `for_api_with_timeout`
- Simplified `ReqwestClient::for_api`: removed HTTP/2 settings (`http2_adaptive_window`, `http2_keep_alive_*`), gzip, and `connect_timeout`; pool reduced from 512 to 200; keepalive interval changed from 30s to 60s
- Simplified `ReqwestClient::for_polling`: removed `connect_timeout`, `pool_idle_timeout`, `tcp_nodelay`, and HTTP/2 settings; keepalive interval changed to 60s
- Removed `call_api_raw` from `Bot` (fast-path that serialized directly to bytes, bypassing `serde_json::Value`)
- Removed `post_json_raw` from `BotClient` trait and `ReqwestClient` (zero-copy raw bytes POST path)
- Webhook handler reverted from optimised single-spawn (`catch_unwind`) to double-spawn for panic isolation; body deserialization moved back to axum's `Json` extractor
- `reqwest` feature set reduced: removed `http2`, `rustls-tls`, and `gzip` from non-wasm target
- Version bumped to `0.2.0` across `Cargo.toml`, docs, and README

---

## [0.1.2] - 2026-05-08

### Telegram Bot API: `Bot API 10.0`

Auto-generated from latest Telegram Bot API spec.

---

## [0.1.0] - 2026-01-01

### Initial release

- 285 types generated from Telegram Bot API 9.4 (257 structs, 21 union/enum types, 7 marker types)
- 165 async methods, 100 optional param structs with builder pattern
- `Bot::new()`, `Bot::with_api_url()`, `Bot::new_unverified()`
- `ChatId`, `InputFile`, `ReplyMarkup`, `InputMedia`
- `Poller` for long-polling, `BotError` with flood-wait helpers, multipart upload
- `Dispatcher` with handler groups, `CommandHandler`, `MessageHandler`, `CallbackQueryHandler`
- `ConversationHandler` with `InMemoryStorage` and Redis storage support
- `Middleware` trait with `LoggingMiddleware` and `RateLimiter` built-in
- `RetryPolicy` with exponential back-off and flood-wait handling
- `Updater` with long-polling and webhook modes
- `Context` struct with `effective_chat()`, `effective_user()`, `effective_message()`
- Filter system with composable `.and()`, `.or()`, `.not()` operators
- Codegen: `codegen/codegen.py` generates `gen_types.rs`, `gen_methods.rs`, `fluent.rs` from `api.json`
- CI: auto-regenerate on Telegram spec changes, build/test/lint, release, docs workflows
- Examples: `echo_bot`, `command_bot`, `colourbutton`, `webhook`, `mock_client`

---
