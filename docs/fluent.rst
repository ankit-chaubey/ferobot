Fluent API
==========

Every ``bot.method(required_args)`` accepts an optional params struct for extra fields.
Use the builder pattern to chain options cleanly, then ``.await``.

Sending messages
----------------

.. code-block:: rust

   use ferobot::gen_methods::SendMessageParams;

   // Minimal
   bot.send_message(chat_id, "Hello!", None).await?;

   // With options
   let params = SendMessageParams::new()
       .parse_mode("HTML".to_string())
       .disable_notification(true);

   bot.send_message(chat_id, "<b>Bold text</b>", Some(params)).await?;

Sending media
-------------

.. code-block:: rust

   use ferobot::InputFile;
   use ferobot::gen_methods::SendPhotoParams;

   // File ID already on Telegram
   bot.send_photo(chat_id, "AgACAgIAAxkBAAI...", None).await?;

   // URL — Telegram fetches it
   bot.send_photo(chat_id, "https://example.com/img.jpg", None).await?;

   // Raw bytes from disk
   let data   = tokio::fs::read("photo.jpg").await?;
   let file   = InputFile::memory("photo.jpg", data);
   let params = SendPhotoParams::new().caption("Look at this!".to_string());
   bot.send_photo(chat_id, file, Some(params)).await?;

Inline keyboard
---------------

.. code-block:: rust

   use ferobot::{ReplyMarkup, gen_methods::SendMessageParams};
   use ferobot::types::{InlineKeyboardButton, InlineKeyboardMarkup};

   let kb = ReplyMarkup::InlineKeyboard(InlineKeyboardMarkup {
       inline_keyboard: vec![vec![
           InlineKeyboardButton {
               text:          "Yes".into(),
               callback_data: Some("yes".into()),
               ..Default::default()
           },
           InlineKeyboardButton {
               text:          "No".into(),
               callback_data: Some("no".into()),
               ..Default::default()
           },
       ]],
   });

   let params = SendMessageParams::new().reply_markup(kb);
   bot.send_message(chat_id, "Confirm?", Some(params)).await?;

Answering callbacks
-------------------

.. code-block:: rust

   use ferobot::gen_methods::AnswerCallbackQueryParams;

   let params = AnswerCallbackQueryParams::new()
       .text("Done!".to_string())
       .show_alert(false);

   bot.answer_callback_query(&cq.id, Some(params)).await?;

Editing messages
----------------

.. code-block:: rust

   use ferobot::gen_methods::EditMessageTextParams;
   use ferobot::types::MaybeInaccessibleMessage;

   if let Some(MaybeInaccessibleMessage::Message(m)) = cq.message.as_deref() {
       let params = EditMessageTextParams::new()
           .chat_id(m.chat.id)
           .message_id(m.message_id);
       bot.edit_message_text("Updated text", Some(params)).await?;
   }

Message helpers
---------------

.. code-block:: rust

   // Reply to a message
   msg.reply(&bot, "Thanks!", None).await?;

   // Get text (also reads caption for media)
   if let Some(text) = msg.get_text() { /* ... */ }

   // Get a t.me deep link to the message
   let link = msg.get_link(); // "https://t.me/c/1234/56"

   // Get sender
   let user = msg.get_sender(); // Option<&User>

   // File URL from a File struct
   let url = file.url(&bot); // Option<String>

InputFile
---------

.. code-block:: rust

   // File already on Telegram — pass file_id string directly
   bot.send_photo(chat_id, "AgACAgIAAxkBAAI...", None).await?;

   // URL — pass URL string directly
   bot.send_photo(chat_id, "https://example.com/img.jpg", None).await?;

   // Raw bytes
   InputFile::memory("photo.jpg", bytes)

Raw API access
--------------

Call any Bot API method by name with arbitrary params:

.. code-block:: rust

   let result: serde_json::Value = bot
       .raw("sendMessage")
       .param("chat_id",   123456789_i64)
       .param("text",      "Hello from raw!")
       .param("parse_mode","HTML")
       .call()
       .await?;

Sending files (full reference)
-------------------------------

.. list-table::
   :header-rows: 1
   :widths: 30 70

   * - Input type
     - How to pass it
   * - Existing ``file_id``
     - Pass the ID string directly
   * - HTTP URL
     - Pass the URL string directly (Telegram fetches)
   * - Bytes / disk file
     - ``InputFile::memory("name.ext", bytes)``

.. code-block:: rust

   // Reading from disk
   let bytes = tokio::fs::read("document.pdf").await?;
   bot.send_document(chat_id, InputFile::memory("doc.pdf", bytes), None).await?;

Webhook (built-in vs manual)
-----------------------------

.. list-table::
   :header-rows: 1
   :widths: 40 30 30

   * - Feature
     - Built-in ``WebhookServer``
     - Manual (bring your own)
   * - Zero boilerplate
     - ✓
     - ✗
   * - Secret token validation
     - Built-in
     - Manual
   * - Custom routing / middleware
     - ✗
     - ✓
   * - Works with existing server
     - ✗
     - ✓
   * - Feature flag needed
     - ``webhook``
     - None

Manual webhook with Axum:

.. code-block:: rust

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

   async fn handle(
       State(s): State<Arc<AppState>>,
       Json(update): Json<Update>,
   ) -> StatusCode {
       let bot = s.bot.clone();
       tokio::spawn(async move {
           if let Some(msg) = update.message {
               let _ = bot.send_message(msg.chat.id, "got it", None).await;
           }
       });
       StatusCode::OK
   }
