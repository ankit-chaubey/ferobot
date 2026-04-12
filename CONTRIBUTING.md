# Contributing to ferobot

Contributions are welcome! ferobot is an auto-generated Telegram Bot API library for Rust, understanding the architecture will help you contribute effectively.

---

## Architecture

```
api.json (Telegram spec)
 |
 v
codegen/codegen.py
 |
 +---> ferobot/src/gen_types.rs (auto-generated)
 +---> ferobot/src/gen_methods.rs (auto-generated)

Handwritten files:
 ferobot/src/bot.rs
 ferobot/src/error.rs
 ferobot/src/chat_id.rs
 ferobot/src/input_file.rs
 ferobot/src/reply_markup.rs
 ferobot/src/polling.rs
 ferobot/src/lib.rs
```

**Key rule:** Never edit `gen_types.rs` or `gen_methods.rs` by hand. Edit `codegen.py` instead and re-run the generator.

---

## Getting Started

**Prerequisites:** Rust 1.75+, Python 3.8+, Git

```sh
git clone https://github.com/ankit-chaubey/ferobot
cd ferobot

cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets
cargo fmt --all -- --check
```

---

## Development Workflow

### Changing the Codegen

```sh
$EDITOR codegen/codegen.py

python3 codegen/codegen.py api.json ferobot/src/

python3 .github/scripts/validate_generated.py \
 api.json \
 ferobot/src/gen_types.rs \
 ferobot/src/gen_methods.rs

cargo build --workspace
cargo test --workspace
```

### Changing the Runtime

For changes to `bot.rs`, `error.rs`, `polling.rs`, etc.:

```sh
$EDITOR ferobot/src/bot.rs

cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

### Testing with a Real Bot

```sh
export BOT_TOKEN=your_test_bot_token

cargo run --example echo_bot
cargo run --example advanced_bot
```

---

## What to Contribute

**High value:**
- Tests, unit and integration tests
- Examples, more example bots in `examples/`
- Codegen improvements, better type generation, edge case handling
- Runtime improvements, better error messages, helper methods
- Documentation, doc comments on public types and methods

**Do not change:**
- `gen_types.rs` and `gen_methods.rs`, auto-generated, edit `codegen.py` instead
- `api.json`, auto-updated by the workflow

---

## Reporting Bugs

Use the [bug report template](https://github.com/ankit-chaubey/ferobot/issues/new?template=bug_report.md).

Include: minimal reproduction code, full error output, `rustc --version`, and ferobot version.

---

## Pull Request Process

1. Fork the repo and create a branch: `git checkout -b my-feature`
2. Make your changes
3. Run the checks:
 ```sh
 cargo build --workspace
 cargo test --workspace
 cargo clippy --workspace --all-targets -- -D warnings
 cargo fmt --all -- --check
 ```
4. Open a PR against `main`

### Commit Message Format

```
type(scope): short description

[optional body]
[optional: Closes #issue]
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

Examples:
```
feat(polling): add graceful shutdown support
fix(bot): handle empty response body from Telegram
docs(readme): add webhook example
```

---

## Security

Do not open a public issue for security vulnerabilities.
Email directly: [ankitchaubey.dev@gmail.com](mailto:ankitchaubey.dev@gmail.com)

---

## Contact

- **Email:** [ankitchaubey.dev@gmail.com](mailto:ankitchaubey.dev@gmail.com)
- **Telegram:** [@ankify](https://t.me/ankify)
- **GitHub:** [@ankit-chaubey](https://github.com/ankit-chaubey)
