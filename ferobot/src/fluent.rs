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

//! Fluent builder API for the most commonly used Bot methods.
//!
//! Every builder is returned by a method on [`Bot`] and implements
//! [`IntoFuture`], so you can `.await` it directly with or without
//! optional parameters:
//!
//! ```rust,no_run
//! use ferobot::{Bot, InputFile};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), ferobot::BotError> {
//! let bot = Bot::new("TOKEN").await?;
//!
//! // Simplest possible call no trailing None needed
//! let msg = bot.send_message(123456789_i64, "Hello!").await?;
//!
//! // Chain as many options as you like, then await
//! let msg = bot
//!     .send_message(123456789_i64, "Hello!")
//!     .html()                          // parse_mode = "HTML"
//!     .reply_to(msg.message_id)        // reply_parameters shorthand
//!     .silent()                        // disable_notification = true
//!     .await?;
//!
//! // Send a photo with a caption
//! let msg = bot
//!     .send_photo(123456789_i64, InputFile::memory("cat.jpg", b"fake image data" as &[u8]))
//!     .caption("Look at this cat 🐱")
//!     .html()
//!     .await?;
//!
//! // Edit a message
//! bot.edit_message_text(123456789_i64, msg.message_id, "Updated text")
//!    .html()
//!    .await?;
//!
//! // Answer a callback query
//! bot.answer_callback_query("query_id_here")
//!    .text("Done!")
//!    .show_alert(true)
//!    .await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Power-user escape hatch
//!
//! The underlying `_with_params` variants are always available for full
//! control or programmatic param construction:
//!
//! ```rust,no_run
//! # async fn example(bot: ferobot::Bot) -> Result<(), ferobot::BotError> {
//! use ferobot::gen_methods::SendMessageParams;
//!
//! let params = SendMessageParams::new()
//!     .parse_mode("HTML")
//!     .protect_content(true);
//!
//! bot.send_message_with_params(123_i64, "Hi", Some(params)).await?;
//! # Ok(())
//! # }
//! ```

use std::future::IntoFuture;
use std::pin::Pin;

use crate::{
    gen_methods::{
        AnswerCallbackQueryParams, CopyMessageParams, EditMessageTextParams, ForwardMessageParams,
        PinChatMessageParams, SendChatActionParams, SendDocumentParams, SendMessageParams,
        SendPhotoParams,
    },
    input_file::InputFileOrString,
    types::{InlineKeyboardMarkup, Message, MessageEntity, ReplyParameters},
    Bot, BotError, ChatId, ReplyMarkup,
};

/// Generates a fluent setter on a request struct that delegates to its inner
/// `params` field.
macro_rules! delegate {
    // bool field
    ($name:ident, $field:ident, bool) => {
        pub fn $name(mut self, v: bool) -> Self {
            self.params.$field = Some(v);
            self
        }
    };
    // i64 field
    ($name:ident, $field:ident, i64) => {
        pub fn $name(mut self, v: i64) -> Self {
            self.params.$field = Some(v);
            self
        }
    };
    // String field
    ($name:ident, $field:ident, String) => {
        pub fn $name(mut self, v: impl Into<String>) -> Self {
            self.params.$field = Some(v.into());
            self
        }
    };
    // Vec<MessageEntity> field
    ($name:ident, $field:ident, Vec<MessageEntity>) => {
        pub fn $name(mut self, v: Vec<MessageEntity>) -> Self {
            self.params.$field = Some(v);
            self
        }
    };
}

fn make_reply_params(message_id: i64) -> Box<ReplyParameters> {
    Box::new(ReplyParameters {
        message_id,
        chat_id: None,
        allow_sending_without_reply: None,
        quote: None,
        quote_parse_mode: None,
        quote_entities: None,
        quote_position: None,
        checklist_task_id: None,
        poll_option_id: None,
    })
}

/// Fluent builder for [`Bot::send_message`]. Implements [`IntoFuture`].
pub struct SendMessageRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    text: String,
    params: SendMessageParams,
}

impl<'bot> SendMessageRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>, text: impl Into<String>) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            text: text.into(),
            params: Default::default(),
        }
    }

    /// Set `parse_mode` to `"HTML"`.
    pub fn html(self) -> Self {
        self.parse_mode("HTML")
    }

    /// Set `parse_mode` to `"MarkdownV2"`.
    pub fn markdown(self) -> Self {
        self.parse_mode("MarkdownV2")
    }

    /// Reply to a message by id. Shorthand for `.reply_parameters(...)`.
    pub fn reply_to(mut self, message_id: i64) -> Self {
        self.params.reply_parameters = Some(make_reply_params(message_id));
        self
    }

    /// Set `disable_notification = true` (send silently).
    pub fn silent(self) -> Self {
        self.disable_notification(true)
    }

    /// Attach a reply keyboard / inline keyboard.
    pub fn keyboard(mut self, markup: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(markup.into());
        self
    }

    delegate!(parse_mode, parse_mode, String);
    delegate!(disable_notification, disable_notification, bool);
    delegate!(protect_content, protect_content, bool);
    delegate!(allow_paid_broadcast, allow_paid_broadcast, bool);
    delegate!(message_thread_id, message_thread_id, i64);
    delegate!(message_effect_id, message_effect_id, String);
    delegate!(business_connection_id, business_connection_id, String);
    delegate!(entities, entities, Vec<MessageEntity>);

    pub fn reply_parameters(mut self, v: Box<ReplyParameters>) -> Self {
        self.params.reply_parameters = Some(v);
        self
    }

    pub fn reply_markup(mut self, v: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for SendMessageRequest<'bot> {
    type Output = Result<Message, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .send_message_with_params(self.chat_id, self.text, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::send_photo`]. Implements [`IntoFuture`].
pub struct SendPhotoRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    photo: InputFileOrString,
    params: SendPhotoParams,
}

impl<'bot> SendPhotoRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        chat_id: impl Into<ChatId>,
        photo: impl Into<InputFileOrString>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            photo: photo.into(),
            params: Default::default(),
        }
    }

    pub fn html(self) -> Self {
        self.parse_mode("HTML")
    }
    pub fn markdown(self) -> Self {
        self.parse_mode("MarkdownV2")
    }
    pub fn reply_to(mut self, message_id: i64) -> Self {
        self.params.reply_parameters = Some(make_reply_params(message_id));
        self
    }
    pub fn silent(self) -> Self {
        self.disable_notification(true)
    }
    pub fn keyboard(mut self, markup: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(markup.into());
        self
    }

    delegate!(caption, caption, String);
    delegate!(parse_mode, parse_mode, String);
    delegate!(disable_notification, disable_notification, bool);
    delegate!(protect_content, protect_content, bool);
    delegate!(allow_paid_broadcast, allow_paid_broadcast, bool);
    delegate!(has_spoiler, has_spoiler, bool);
    delegate!(show_caption_above_media, show_caption_above_media, bool);
    delegate!(message_thread_id, message_thread_id, i64);
    delegate!(message_effect_id, message_effect_id, String);
    delegate!(business_connection_id, business_connection_id, String);

    pub fn reply_parameters(mut self, v: Box<ReplyParameters>) -> Self {
        self.params.reply_parameters = Some(v);
        self
    }
    pub fn reply_markup(mut self, v: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for SendPhotoRequest<'bot> {
    type Output = Result<Message, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .send_photo_with_params(self.chat_id, self.photo, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::send_document`]. Implements [`IntoFuture`].
pub struct SendDocumentRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    document: InputFileOrString,
    params: SendDocumentParams,
}

impl<'bot> SendDocumentRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        chat_id: impl Into<ChatId>,
        document: impl Into<InputFileOrString>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            document: document.into(),
            params: Default::default(),
        }
    }

    pub fn html(self) -> Self {
        self.parse_mode("HTML")
    }
    pub fn markdown(self) -> Self {
        self.parse_mode("MarkdownV2")
    }
    pub fn reply_to(mut self, message_id: i64) -> Self {
        self.params.reply_parameters = Some(make_reply_params(message_id));
        self
    }
    pub fn silent(self) -> Self {
        self.disable_notification(true)
    }
    pub fn keyboard(mut self, markup: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(markup.into());
        self
    }

    delegate!(caption, caption, String);
    delegate!(parse_mode, parse_mode, String);
    delegate!(disable_notification, disable_notification, bool);
    delegate!(protect_content, protect_content, bool);
    delegate!(allow_paid_broadcast, allow_paid_broadcast, bool);
    delegate!(
        disable_content_type_detection,
        disable_content_type_detection,
        bool
    );
    delegate!(message_thread_id, message_thread_id, i64);
    delegate!(message_effect_id, message_effect_id, String);
    delegate!(business_connection_id, business_connection_id, String);

    pub fn reply_parameters(mut self, v: Box<ReplyParameters>) -> Self {
        self.params.reply_parameters = Some(v);
        self
    }
    pub fn reply_markup(mut self, v: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for SendDocumentRequest<'bot> {
    type Output = Result<Message, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .send_document_with_params(self.chat_id, self.document, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::edit_message_text`]. Implements [`IntoFuture`].
///
/// Requires either `(chat_id + message_id)` or `inline_message_id` to be set.
/// Use the required-params constructor from [`Bot::edit_message_text`] which
/// sets `chat_id` + `message_id` automatically.
pub struct EditMessageTextRequest<'bot> {
    bot: &'bot Bot,
    text: String,
    params: EditMessageTextParams,
}

impl<'bot> EditMessageTextRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        chat_id: impl Into<ChatId>,
        message_id: i64,
        text: impl Into<String>,
    ) -> Self {
        let params = EditMessageTextParams::new()
            .chat_id(chat_id)
            .message_id(message_id);
        Self {
            bot,
            text: text.into(),
            params,
        }
    }

    /// Edit an inline message (use instead of `chat_id` + `message_id`).
    pub(crate) fn new_inline(
        bot: &'bot Bot,
        inline_message_id: impl Into<String>,
        text: impl Into<String>,
    ) -> Self {
        let params = EditMessageTextParams::new().inline_message_id(inline_message_id);
        Self {
            bot,
            text: text.into(),
            params,
        }
    }

    pub fn html(self) -> Self {
        self.parse_mode("HTML")
    }
    pub fn markdown(self) -> Self {
        self.parse_mode("MarkdownV2")
    }

    delegate!(parse_mode, parse_mode, String);
    delegate!(business_connection_id, business_connection_id, String);
    delegate!(entities, entities, Vec<MessageEntity>);

    pub fn reply_markup(mut self, v: InlineKeyboardMarkup) -> Self {
        self.params.reply_markup = Some(Box::new(v));
        self
    }
}

impl<'bot> IntoFuture for EditMessageTextRequest<'bot> {
    type Output = Result<serde_json::Value, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .edit_message_text_with_params(self.text, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::answer_callback_query`]. Implements [`IntoFuture`].
pub struct AnswerCallbackQueryRequest<'bot> {
    bot: &'bot Bot,
    callback_query_id: String,
    params: AnswerCallbackQueryParams,
}

impl<'bot> AnswerCallbackQueryRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, callback_query_id: impl Into<String>) -> Self {
        Self {
            bot,
            callback_query_id: callback_query_id.into(),
            params: Default::default(),
        }
    }

    /// Show a toast notification (default behaviour).
    pub fn toast(self, text: impl Into<String>) -> Self {
        self.text(text).show_alert(false)
    }

    /// Show an alert dialog (user must dismiss it).
    pub fn alert(self, text: impl Into<String>) -> Self {
        self.text(text).show_alert(true)
    }

    delegate!(text, text, String);
    delegate!(show_alert, show_alert, bool);
    delegate!(url, url, String);
    delegate!(cache_time, cache_time, i64);
}

impl<'bot> IntoFuture for AnswerCallbackQueryRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .answer_callback_query_with_params(self.callback_query_id, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::forward_message`]. Implements [`IntoFuture`].
pub struct ForwardMessageRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    from_chat_id: ChatId,
    message_id: i64,
    params: ForwardMessageParams,
}

impl<'bot> ForwardMessageRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        chat_id: impl Into<ChatId>,
        from_chat_id: impl Into<ChatId>,
        message_id: i64,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            from_chat_id: from_chat_id.into(),
            message_id,
            params: Default::default(),
        }
    }

    pub fn silent(self) -> Self {
        self.disable_notification(true)
    }

    delegate!(disable_notification, disable_notification, bool);
    delegate!(protect_content, protect_content, bool);
    delegate!(message_thread_id, message_thread_id, i64);
}

impl<'bot> IntoFuture for ForwardMessageRequest<'bot> {
    type Output = Result<Message, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .forward_message_with_params(
                    self.chat_id,
                    self.from_chat_id,
                    self.message_id,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::copy_message`]. Implements [`IntoFuture`].
pub struct CopyMessageRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    from_chat_id: ChatId,
    message_id: i64,
    params: CopyMessageParams,
}

impl<'bot> CopyMessageRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        chat_id: impl Into<ChatId>,
        from_chat_id: impl Into<ChatId>,
        message_id: i64,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            from_chat_id: from_chat_id.into(),
            message_id,
            params: Default::default(),
        }
    }

    pub fn html(self) -> Self {
        self.parse_mode("HTML")
    }
    pub fn markdown(self) -> Self {
        self.parse_mode("MarkdownV2")
    }
    pub fn reply_to(mut self, message_id: i64) -> Self {
        self.params.reply_parameters = Some(make_reply_params(message_id));
        self
    }
    pub fn silent(self) -> Self {
        self.disable_notification(true)
    }
    pub fn keyboard(mut self, markup: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(markup.into());
        self
    }

    delegate!(caption, caption, String);
    delegate!(parse_mode, parse_mode, String);
    delegate!(disable_notification, disable_notification, bool);
    delegate!(protect_content, protect_content, bool);
    delegate!(allow_paid_broadcast, allow_paid_broadcast, bool);
    delegate!(show_caption_above_media, show_caption_above_media, bool);
    delegate!(message_thread_id, message_thread_id, i64);

    pub fn reply_parameters(mut self, v: Box<ReplyParameters>) -> Self {
        self.params.reply_parameters = Some(v);
        self
    }
    pub fn reply_markup(mut self, v: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for CopyMessageRequest<'bot> {
    type Output = Result<crate::types::MessageId, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .copy_message_with_params(
                    self.chat_id,
                    self.from_chat_id,
                    self.message_id,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::pin_chat_message`]. Implements [`IntoFuture`].
pub struct PinChatMessageRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    message_id: i64,
    params: PinChatMessageParams,
}

impl<'bot> PinChatMessageRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>, message_id: i64) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            message_id,
            params: Default::default(),
        }
    }

    /// Pin silently (no notification).
    pub fn silent(self) -> Self {
        self.disable_notification(true)
    }

    delegate!(disable_notification, disable_notification, bool);
    delegate!(business_connection_id, business_connection_id, String);
}

impl<'bot> IntoFuture for PinChatMessageRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .pin_chat_message_with_params(self.chat_id, self.message_id, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::send_chat_action`]. Implements [`IntoFuture`].
pub struct SendChatActionRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    action: String,
    params: SendChatActionParams,
}

impl<'bot> SendChatActionRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        chat_id: impl Into<ChatId>,
        action: impl Into<String>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            action: action.into(),
            params: Default::default(),
        }
    }

    delegate!(message_thread_id, message_thread_id, i64);
    delegate!(business_connection_id, business_connection_id, String);
}

impl<'bot> IntoFuture for SendChatActionRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .send_chat_action_with_params(self.chat_id, self.action, Some(self.params))
                .await
        })
    }
}

impl Bot {
    /// Send a text message. Returns a [`SendMessageRequest`] builder.
    ///
    /// Chain optional parameters before `.await`:
    /// ```rust,no_run
    /// # async fn example(bot: ferobot::Bot) -> Result<(), ferobot::BotError> {
    /// bot.send_message(123_i64, "Hello!")
    ///    .html()
    ///    .reply_to(42)
    ///    .silent()
    ///    .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn send_message(
        &self,
        chat_id: impl Into<ChatId>,
        text: impl Into<String>,
    ) -> SendMessageRequest<'_> {
        SendMessageRequest::new(self, chat_id, text)
    }

    /// Send a photo. Returns a [`SendPhotoRequest`] builder.
    ///
    /// ```rust,no_run
    /// # async fn example(bot: ferobot::Bot) -> Result<(), ferobot::BotError> {
    /// use ferobot::InputFile;
    /// bot.send_photo(123_i64, InputFile::memory("img.jpg", vec![]))
    ///    .caption("Caption here")
    ///    .html()
    ///    .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn send_photo(
        &self,
        chat_id: impl Into<ChatId>,
        photo: impl Into<InputFileOrString>,
    ) -> SendPhotoRequest<'_> {
        SendPhotoRequest::new(self, chat_id, photo)
    }

    /// Send a document/file. Returns a [`SendDocumentRequest`] builder.
    pub fn send_document(
        &self,
        chat_id: impl Into<ChatId>,
        document: impl Into<InputFileOrString>,
    ) -> SendDocumentRequest<'_> {
        SendDocumentRequest::new(self, chat_id, document)
    }

    /// Edit a message's text. Returns an [`EditMessageTextRequest`] builder.
    ///
    /// ```rust,no_run
    /// # async fn example(bot: ferobot::Bot) -> Result<(), ferobot::BotError> {
    /// bot.edit_message_text(123_i64, 456_i64, "New text")
    ///    .html()
    ///    .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn edit_message_text(
        &self,
        chat_id: impl Into<ChatId>,
        message_id: i64,
        text: impl Into<String>,
    ) -> EditMessageTextRequest<'_> {
        EditMessageTextRequest::new(self, chat_id, message_id, text)
    }

    /// Edit an inline message's text. Returns an [`EditMessageTextRequest`] builder.
    pub fn edit_inline_message_text(
        &self,
        inline_message_id: impl Into<String>,
        text: impl Into<String>,
    ) -> EditMessageTextRequest<'_> {
        EditMessageTextRequest::new_inline(self, inline_message_id, text)
    }

    /// Answer a callback query. Returns an [`AnswerCallbackQueryRequest`] builder.
    ///
    /// ```rust,no_run
    /// # async fn example(bot: ferobot::Bot) -> Result<(), ferobot::BotError> {
    /// // Toast (default)
    /// bot.answer_callback_query("id").toast("Done!").await?;
    ///
    /// // Alert dialog
    /// bot.answer_callback_query("id").alert("Are you sure?").await?;
    ///
    /// // Just dismiss with no text
    /// bot.answer_callback_query("id").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn answer_callback_query(
        &self,
        callback_query_id: impl Into<String>,
    ) -> AnswerCallbackQueryRequest<'_> {
        AnswerCallbackQueryRequest::new(self, callback_query_id)
    }

    /// Forward a message. Returns a [`ForwardMessageRequest`] builder.
    pub fn forward_message(
        &self,
        chat_id: impl Into<ChatId>,
        from_chat_id: impl Into<ChatId>,
        message_id: i64,
    ) -> ForwardMessageRequest<'_> {
        ForwardMessageRequest::new(self, chat_id, from_chat_id, message_id)
    }

    /// Copy a message (like forward but without the "forwarded from" header).
    /// Returns a [`CopyMessageRequest`] builder.
    pub fn copy_message(
        &self,
        chat_id: impl Into<ChatId>,
        from_chat_id: impl Into<ChatId>,
        message_id: i64,
    ) -> CopyMessageRequest<'_> {
        CopyMessageRequest::new(self, chat_id, from_chat_id, message_id)
    }

    /// Pin a message. Returns a [`PinChatMessageRequest`] builder.
    ///
    /// ```rust,no_run
    /// # async fn example(bot: ferobot::Bot) -> Result<(), ferobot::BotError> {
    /// bot.pin_chat_message(123_i64, 456_i64).silent().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn pin_chat_message(
        &self,
        chat_id: impl Into<ChatId>,
        message_id: i64,
    ) -> PinChatMessageRequest<'_> {
        PinChatMessageRequest::new(self, chat_id, message_id)
    }

    /// Send a chat action (typing, uploading, etc.).
    /// Returns a [`SendChatActionRequest`] builder.
    ///
    /// ```rust,no_run
    /// # async fn example(bot: ferobot::Bot) -> Result<(), ferobot::BotError> {
    /// bot.send_chat_action(123_i64, "typing").await?;
    /// bot.send_chat_action(123_i64, "upload_document").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn send_chat_action(
        &self,
        chat_id: impl Into<ChatId>,
        action: impl Into<String>,
    ) -> SendChatActionRequest<'_> {
        SendChatActionRequest::new(self, chat_id, action)
    }
}
