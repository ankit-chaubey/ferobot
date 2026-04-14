Framework
=========

The dispatcher routes incoming updates to handlers.
Handlers are matched in group order; within a group the first match wins.

Dispatcher setup
----------------

.. code-block:: rust

   use ferobot::{Bot, Dispatcher, DispatcherOpts, DispatcherAction, Updater};

   let mut dp = Dispatcher::new(
       DispatcherOpts::default()
           .max_routines(128)
           .on_error(|_bot, _ctx, err| {
               eprintln!("handler error: {err}");
               DispatcherAction::Noop
           })
           .on_panic(|_bot, _ctx, msg| eprintln!("panic: {msg}"))
   );

   Updater::new(bot, dp).poll_timeout(30).start_polling().await.unwrap();

CommandHandler
--------------

Routes ``/command`` messages, optionally with args.

.. code-block:: rust

   use ferobot::{CommandHandler, HandlerResult, Context, Bot};

   async fn greet(bot: Bot, ctx: Context) -> HandlerResult {
       let name = ctx.args().first().map(String::as_str).unwrap_or("there");
       if let Some(msg) = ctx.effective_message() {
           msg.reply(&bot, format!("Hello, {name}!"), None).await?;
       }
       Ok(())
   }

   dp.add_handler(CommandHandler::new("greet", greet));
   // matches /greet @username: args = ["@username"]

MessageHandler with filters
----------------------------

.. code-block:: rust

   use ferobot::{MessageHandler, HandlerResult, Context, Bot};
   use ferobot::framework::filters::message as mf;

   async fn on_text(bot: Bot, ctx: Context) -> HandlerResult {
       if let Some(msg) = ctx.effective_message() {
           if let Some(text) = msg.get_text() {
               msg.reply(&bot, format!("You said: {text}"), None).await?;
           }
       }
       Ok(())
   }

   dp.add_handler(MessageHandler::new("text_echo", mf::text(),  on_text));
   dp.add_handler(MessageHandler::new("photos",    mf::photo(), on_photo));

Available filters:

.. list-table::
   :header-rows: 1
   :widths: 30 70

   * - Filter
     - Matches
   * - ``mf::text()``
     - Any message with text
   * - ``mf::command()``
     - Messages starting with ``/``
   * - ``mf::photo()``
     - Messages containing a photo
   * - ``mf::video()``
     - Messages containing a video
   * - ``mf::audio()``
     - Messages containing audio
   * - ``mf::document()``
     - Messages containing a document
   * - ``mf::sticker()``
     - Messages containing a sticker
   * - ``mf::caption()``
     - Media messages with a caption
   * - ``mf::reply()``
     - Messages that reply to another message
   * - ``mf::forward()``
     - Forwarded messages

Filters compose with ``.and()``, ``.or()``, ``.not()``:

.. code-block:: rust

   // text that is NOT a command
   mf::text().and(mf::command().not())

CallbackQueryHandler
--------------------

.. code-block:: rust

   use ferobot::{CallbackQueryHandler, HandlerResult, Context, Bot};

   async fn on_button(bot: Bot, ctx: Context) -> HandlerResult {
       if let Some(cq) = ctx.update.callback_query.as_ref() {
           if cq.data.as_deref() == Some("confirm") {
               bot.answer_callback_query(&cq.id, None).await?;
           }
       }
       Ok(())
   }

   dp.add_handler(CallbackQueryHandler::new("btn", |_| true, on_button));

Handler groups
--------------

Lower group numbers run first. Within a group the first matching handler fires and the rest are skipped.

.. code-block:: rust

   // Group 0 = admin-only, group 1 = catch-all
   dp.add_handler_to_group(CommandHandler::new("admin", admin_fn), 0);
   dp.add_handler_to_group(MessageHandler::new("catch_all", |_| true, echo_fn), 1);

   // Remove at runtime
   dp.remove_handler("catch_all", 1);

ConversationHandler
-------------------

Multi-step flows (FSM) with pluggable state storage.

.. code-block:: rust

   use ferobot::{
       ConversationHandler, ConversationOpts, InMemoryStorage,
       KeyStrategy, NextState, EndConversation,
       CommandHandler, MessageHandler,
   };
   use std::collections::HashMap;

   async fn ask_name(bot: Bot, ctx: Context) -> HandlerResult {
       if let Some(msg) = ctx.effective_message() {
           msg.reply(&bot, "What is your name?", None).await?;
       }
       Err(Box::new(NextState("ask_name".into())))
   }

   async fn save_name(bot: Bot, ctx: Context) -> HandlerResult {
       if let Some(msg) = ctx.effective_message() {
           let name = msg.text.clone().unwrap_or_default();
           msg.reply(&bot, format!("Nice to meet you, {name}!"), None).await?;
       }
       Err(Box::new(EndConversation))
   }

   let opts = ConversationOpts {
       storage:      Some(InMemoryStorage::new()),
       key_strategy: KeyStrategy::SenderAndChat,
       ..Default::default()
   };

   let conv = ConversationHandler::new(
       vec![Box::new(CommandHandler::new("register", ask_name))],
       HashMap::from([
           ("ask_name".to_string(), vec![
               Box::new(MessageHandler::new("name_input", |_| true, save_name))
                   as Box<dyn ferobot::Handler>
           ]),
       ]),
       opts,
   );
   dp.add_handler(conv);

Redis storage
~~~~~~~~~~~~~

.. code-block:: toml

   ferobot = { version = "0.1", features = ["redis-storage"] }

.. code-block:: rust

   use ferobot::storage::RedisStorage;

   let storage = RedisStorage::new("redis://127.0.0.1/")
       .await?
       .with_prefix("mybot:")
       .with_ttl(86400);

   let opts = ConversationOpts { storage: Some(storage), ..Default::default() };

Webhook
-------

Requires the ``webhook`` feature.

.. code-block:: rust

   use ferobot::{Bot, Dispatcher, DispatcherOpts, Updater};

   let bot = Bot::new(std::env::var("BOT_TOKEN").unwrap()).await.unwrap();
   let dp  = Dispatcher::new(DispatcherOpts::default());

   Updater::new(bot, dp)
       .webhook_port(8443)
       .webhook_secret("your-random-secret")
       .start_webhook("https://yourdomain.com/bot")
       .await
       .unwrap();

For local testing: ``ngrok http 8443``

Middleware
----------

Middleware runs before and after every update. Return ``false`` from ``before`` to drop the update.

.. code-block:: rust

   use ferobot::{Dispatcher, DispatcherOpts, Updater};
   use ferobot::middleware::{LoggingMiddleware, RateLimiter};

   let opts = DispatcherOpts::default()
       .middleware(LoggingMiddleware)     // logs every update
       .middleware(RateLimiter::new(5));  // max 5 updates/sec per chat

Custom middleware:

.. code-block:: rust

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

Panic recovery
--------------

Handler panics are caught via ``JoinHandle``, logged at ``error!`` level, and the polling loop continues unaffected.
The ``on_panic`` hook receives the panic message string.

.. code-block:: rust

   let opts = DispatcherOpts::default()
       .on_panic(|_bot, _ctx, msg| eprintln!("panic caught: {msg}"));
