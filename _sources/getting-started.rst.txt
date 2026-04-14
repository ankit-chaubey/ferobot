Getting Started
===============

1. Create a bot
---------------

Message `@BotFather <https://t.me/BotFather>`_ on Telegram, run ``/newbot``, and copy your token.

.. code-block:: bash

   export BOT_TOKEN="123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11"

2. Create a Rust project
------------------------

.. code-block:: bash

   cargo new my-bot
   cd my-bot

Add ferobot to ``Cargo.toml``:

.. code-block:: toml

   [dependencies]
   ferobot = "0.1"
   tokio   = { version = "1", features = ["full"] }

3. Echo bot
-----------

Replace ``src/main.rs``:

.. code-block:: rust

   use ferobot::{
       Bot, CommandHandler, Dispatcher, DispatcherOpts,
       HandlerResult, MessageHandler, Updater, Context,
   };
   use ferobot::framework::filters::message as mf;

   async fn cmd_start(bot: Bot, ctx: Context) -> HandlerResult {
       if let Some(msg) = ctx.effective_message() {
           let name = msg.from.as_ref()
               .map(|u| u.first_name.as_str())
               .unwrap_or("there");
           msg.reply(&bot, format!("Hello, {name}! Send me any text."), None).await?;
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
       let bot = Bot::new(std::env::var("BOT_TOKEN").unwrap()).await.unwrap();
       println!("Running @{}", bot.me.username.as_deref().unwrap_or("?"));

       let mut dp = Dispatcher::new(DispatcherOpts::default());
       dp.add_handler(CommandHandler::new("start", cmd_start));
       dp.add_handler(MessageHandler::new("echo_text", mf::text(), echo));

       Updater::new(bot, dp).poll_timeout(30).start_polling().await.unwrap();
   }

4. Run
------

.. code-block:: bash

   cargo run

Optional features
-----------------

Enable optional features in ``Cargo.toml``:

.. code-block:: toml

   [dependencies]
   ferobot = { version = "0.1", features = ["webhook", "per-chat", "redis-storage"] }

.. list-table::
   :header-rows: 1
   :widths: 25 75

   * - Feature
     - Description
   * - ``webhook``
     - Axum-based webhook server
   * - ``per-chat``
     - Serialize updates per ``chat_id`` (sequential per-chat concurrency)
   * - ``redis-storage``
     - Redis-backed conversation state storage
   * - ``bot-mapping``
     - Multi-bot dispatcher, route by URL path
   * - ``client-ureq``
     - Synchronous (blocking) HTTP client via ``ureq``

Bot constructors
----------------

.. list-table::
   :header-rows: 1
   :widths: 40 60

   * - Method
     - Description
   * - ``Bot::new(token)``
     - Connect and verify token via ``getMe``
   * - ``Bot::with_api_url(token, url)``
     - Use a custom or local Bot API server
   * - ``Bot::new_unverified(token)``
     - Skip ``getMe`` on startup; bot ID parsed from token
   * - ``Bot::with_timeout(token, url, dur)``
     - Custom HTTP timeout (default: 30s)
   * - ``bot.token()``
     - Return the raw token string
   * - ``bot.me``
     - ``User`` struct populated on creation

ChatId
------

Numeric IDs, negative group IDs, and ``@username`` strings all work anywhere a ``ChatId`` is expected:

.. code-block:: rust

   bot.send_message(123456789i64,       "user",             None).await?;
   bot.send_message(-100123456789i64,   "group or channel", None).await?;
   bot.send_message("@username",        "by username",      None).await?;

Error handling
--------------

.. code-block:: rust

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

``BotError`` variants:

.. code-block:: rust

   pub enum BotError {
       Http(reqwest::Error),
       Json(serde_json::Error),
       Api {
           code:               i64,
           description:        String,
           retry_after:        Option<i64>,   // present on 429
           migrate_to_chat_id: Option<i64>,   // present on migration errors
       },
       InvalidToken,
       Other(String),
   }

Retry policy
------------

Wraps any API call and retries on flood-wait (429) and network errors:

.. code-block:: rust

   use ferobot::RetryPolicy;

   let msg = RetryPolicy::new()
       .max_attempts(3)
       .run(|| bot.send_message(chat_id, "hello", None))
       .await?;

Auto-codegen
------------

Types and methods are generated from `tgapis/x <https://github.com/tgapis/x/tree/data>`_,
which tracks the official Telegram Bot API. CI auto-regenerates on every upstream change.

.. warning::

   Never edit ``gen_types.rs`` or ``gen_methods.rs`` by hand. Edit ``codegen/codegen.py`` instead.

Manual regeneration:

.. code-block:: bash

   curl -sSf https://raw.githubusercontent.com/tgapis/x/data/botapi.json -o api.json
   python3 codegen/codegen.py api.json ferobot/src/
   cargo build
