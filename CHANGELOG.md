# Changelog

All notable changes to ferobot are documented here.
Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
Versioning follows [Semantic Versioning](https://semver.org/).

## [0.1.0] - 2026-01-01

Initial release.

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
- Codegen: `codegen/codegen.py` generates `gen_types.rs` and `gen_methods.rs` from `api.json`
- CI: auto-regenerate on Telegram spec changes, build/test/lint, release, docs workflows
- Examples: `echo_bot`, `command_bot`, `colourbutton`, `webhook`, `mock_client`
