ferobot
=======

.. image:: https://img.shields.io/crates/v/ferobot?style=for-the-badge&logo=rust&color=f74c00&labelColor=1a1a2e
   :target: https://crates.io/crates/ferobot
   :alt: crates.io

.. image:: https://img.shields.io/docsrs/ferobot?style=for-the-badge&logo=docs.rs&color=4a90d9&labelColor=1a1a2e
   :target: https://docs.rs/ferobot
   :alt: docs.rs

.. image:: https://img.shields.io/badge/Telegram%20Bot%20API-9.6-0088cc?style=flat-square&logo=telegram
   :target: https://core.telegram.org/bots/api
   :alt: Bot API 9.6

----

Async Telegram Bot API framework for Rust. No proc macros. No hidden magic. Just clean async code.

All **165 methods** and **285 types** are auto-generated from the official Telegram spec: always current, never wrong.

.. code-block:: toml

   [dependencies]
   ferobot = { git = "https://github.com/ankit-chaubey/ferobot" }
   tokio   = { version = "1", features = ["full"] }

.. code-block:: rust

   use ferobot::{Bot, CommandHandler, Dispatcher, DispatcherOpts, HandlerResult, Updater, Context};

   async fn start(bot: Bot, ctx: Context) -> HandlerResult {
       if let Some(msg) = ctx.effective_message() {
           msg.reply(&bot, "Hello! I'm running on ferobot.", None).await?;
       }
       Ok(())
   }

   #[tokio::main]
   async fn main() {
       let bot = Bot::new(std::env::var("BOT_TOKEN").unwrap()).await.unwrap();

       let mut dp = Dispatcher::new(DispatcherOpts::default());
       dp.add_handler(CommandHandler::new("start", start));

       Updater::new(bot, dp).poll_timeout(30).start_polling().await.unwrap();
   }

Features
--------

- **Fully async**: built on Tokio; every update runs in its own task
- **Codegen API**: all 165 methods generated from the official Telegram spec
- **Dispatcher**: ``CommandHandler``, ``MessageHandler``, ``ConversationHandler`` with composable filters
- **Webhook + Polling**: production Axum webhook server and long-polling ``Updater``
- **Fluent builder**: chain optional params, then ``.await``; zero boilerplate
- **Termux ready**: compiles and runs on Android/ARM64

.. toctree::
   :maxdepth: 2
   :caption: Guide

   getting-started
   framework
   fluent
   contributing

.. toctree::
   :maxdepth: 1
   :caption: Reference

   reference/index
   changelog
