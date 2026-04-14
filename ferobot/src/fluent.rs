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

// THIS FILE IS AUTO-GENERATED. DO NOT EDIT.
// Generated from Telegram Bot API Bot API 9.6
// Spec:    https://github.com/ankit-chaubey/api-spec
// Project: https://github.com/ankit-chaubey/ferobot
// Author:  Ankit Chaubey <ankitchaubey.dev@gmail.com>
// License: MIT
// See:     https://core.telegram.org/bots/api

#![allow(clippy::all, unused_imports)]

//! Fluent builder API for every Telegram Bot API method.
//!
//! Every `bot.method(required_args)` call returns a `MethodRequest` builder
//! that implements [`IntoFuture`]. Chain optional parameters then `.await`:
//!
//! ```rust,no_run
//! # #[tokio::main]
//! # async fn main() -> Result<(), ferobot::BotError> {
//! use ferobot::{Bot, InputFile};
//! let bot = Bot::new("TOKEN").await?;
//!
//! // No options needed - just await
//! let me = bot.get_me().await?;
//!
//! // Chain options before await
//! bot.send_message(123_i64, "Hello!")
//!    .html()
//!    .silent()
//!    .reply_to(42)
//!    .await?;
//!
//! // Works for every method
//! bot.send_video(123_i64, InputFile::memory("v.mp4", vec![]))
//!    .caption("Watch this")
//!    .duration(30)
//!    .silent()
//!    .await?;
//!
//! bot.ban_chat_member(123_i64, 456_i64)
//!    .revoke_messages(true)
//!    .await?;
//! # Ok(())
//! # }
//! ```

use std::future::IntoFuture;
use std::pin::Pin;

#[rustfmt::skip]
use crate::{Bot, BotError, ChatId, InputFile, InputFileOrString, ReplyMarkup, InputMedia};
use crate::gen_methods::*;
#[allow(unused_imports)]
use crate::types::*;

fn make_reply_params(message_id: i64) -> Box<ReplyParameters> {
    Box::new(ReplyParameters {
        message_id,
        ..Default::default()
    })
}

/// Fluent builder for [`Bot::add_sticker_to_set`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#addstickertoset
pub struct AddStickerToSetRequest<'bot> {
    bot: &'bot Bot,
    user_id: i64,
    name: String,
    sticker: InputSticker,
}

impl<'bot> AddStickerToSetRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        user_id: i64,
        name: impl Into<String>,
        sticker: InputSticker,
    ) -> Self {
        Self {
            bot,
            user_id,
            name: name.into(),
            sticker,
        }
    }
}

impl<'bot> IntoFuture for AddStickerToSetRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_add_sticker_to_set(self.user_id, self.name, self.sticker)
                .await
        })
    }
}

/// Fluent builder for [`Bot::answer_callback_query`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#answercallbackquery
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

    pub fn text(mut self, v: impl Into<String>) -> Self {
        self.params.text = Some(v.into());
        self
    }
    pub fn show_alert(mut self, v: bool) -> Self {
        self.params.show_alert = Some(v);
        self
    }
    pub fn url(mut self, v: impl Into<String>) -> Self {
        self.params.url = Some(v.into());
        self
    }
    pub fn cache_time(mut self, v: i64) -> Self {
        self.params.cache_time = Some(v);
        self
    }
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

/// Fluent builder for [`Bot::answer_inline_query`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#answerinlinequery
pub struct AnswerInlineQueryRequest<'bot> {
    bot: &'bot Bot,
    inline_query_id: String,
    results: Vec<InlineQueryResult>,
    params: AnswerInlineQueryParams,
}

impl<'bot> AnswerInlineQueryRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        inline_query_id: impl Into<String>,
        results: Vec<InlineQueryResult>,
    ) -> Self {
        Self {
            bot,
            inline_query_id: inline_query_id.into(),
            results,
            params: Default::default(),
        }
    }

    pub fn cache_time(mut self, v: i64) -> Self {
        self.params.cache_time = Some(v);
        self
    }
    pub fn is_personal(mut self, v: bool) -> Self {
        self.params.is_personal = Some(v);
        self
    }
    pub fn next_offset(mut self, v: impl Into<String>) -> Self {
        self.params.next_offset = Some(v.into());
        self
    }
    pub fn button(mut self, v: InlineQueryResultsButton) -> Self {
        self.params.button = Some(Box::new(v));
        self
    }
}

impl<'bot> IntoFuture for AnswerInlineQueryRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .answer_inline_query_with_params(
                    self.inline_query_id,
                    self.results,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::answer_pre_checkout_query`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#answerprecheckoutquery
pub struct AnswerPreCheckoutQueryRequest<'bot> {
    bot: &'bot Bot,
    pre_checkout_query_id: String,
    ok: bool,
    params: AnswerPreCheckoutQueryParams,
}

impl<'bot> AnswerPreCheckoutQueryRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, pre_checkout_query_id: impl Into<String>, ok: bool) -> Self {
        Self {
            bot,
            pre_checkout_query_id: pre_checkout_query_id.into(),
            ok,
            params: Default::default(),
        }
    }

    pub fn error_message(mut self, v: impl Into<String>) -> Self {
        self.params.error_message = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for AnswerPreCheckoutQueryRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .answer_pre_checkout_query_with_params(
                    self.pre_checkout_query_id,
                    self.ok,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::answer_shipping_query`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#answershippingquery
pub struct AnswerShippingQueryRequest<'bot> {
    bot: &'bot Bot,
    shipping_query_id: String,
    ok: bool,
    params: AnswerShippingQueryParams,
}

impl<'bot> AnswerShippingQueryRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, shipping_query_id: impl Into<String>, ok: bool) -> Self {
        Self {
            bot,
            shipping_query_id: shipping_query_id.into(),
            ok,
            params: Default::default(),
        }
    }

    pub fn shipping_options(mut self, v: Vec<ShippingOption>) -> Self {
        self.params.shipping_options = Some(v);
        self
    }
    pub fn error_message(mut self, v: impl Into<String>) -> Self {
        self.params.error_message = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for AnswerShippingQueryRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .answer_shipping_query_with_params(
                    self.shipping_query_id,
                    self.ok,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::answer_web_app_query`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#answerwebappquery
pub struct AnswerWebAppQueryRequest<'bot> {
    bot: &'bot Bot,
    web_app_query_id: String,
    result: InlineQueryResult,
}

impl<'bot> AnswerWebAppQueryRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        web_app_query_id: impl Into<String>,
        result: InlineQueryResult,
    ) -> Self {
        Self {
            bot,
            web_app_query_id: web_app_query_id.into(),
            result,
        }
    }
}

impl<'bot> IntoFuture for AnswerWebAppQueryRequest<'bot> {
    type Output = Result<SentWebAppMessage, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_answer_web_app_query(self.web_app_query_id, self.result)
                .await
        })
    }
}

/// Fluent builder for [`Bot::approve_chat_join_request`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#approvechatjoinrequest
pub struct ApproveChatJoinRequestRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    user_id: i64,
}

impl<'bot> ApproveChatJoinRequestRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>, user_id: i64) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            user_id,
        }
    }
}

impl<'bot> IntoFuture for ApproveChatJoinRequestRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_approve_chat_join_request(self.chat_id, self.user_id)
                .await
        })
    }
}

/// Fluent builder for [`Bot::approve_suggested_post`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#approvesuggestedpost
pub struct ApproveSuggestedPostRequest<'bot> {
    bot: &'bot Bot,
    chat_id: i64,
    message_id: i64,
    params: ApproveSuggestedPostParams,
}

impl<'bot> ApproveSuggestedPostRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: i64, message_id: i64) -> Self {
        Self {
            bot,
            chat_id,
            message_id,
            params: Default::default(),
        }
    }

    pub fn send_date(mut self, v: i64) -> Self {
        self.params.send_date = Some(v);
        self
    }
}

impl<'bot> IntoFuture for ApproveSuggestedPostRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .approve_suggested_post_with_params(
                    self.chat_id,
                    self.message_id,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::ban_chat_member`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#banchatmember
pub struct BanChatMemberRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    user_id: i64,
    params: BanChatMemberParams,
}

impl<'bot> BanChatMemberRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>, user_id: i64) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            user_id,
            params: Default::default(),
        }
    }

    pub fn until_date(mut self, v: i64) -> Self {
        self.params.until_date = Some(v);
        self
    }
    pub fn revoke_messages(mut self, v: bool) -> Self {
        self.params.revoke_messages = Some(v);
        self
    }
}

impl<'bot> IntoFuture for BanChatMemberRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .ban_chat_member_with_params(self.chat_id, self.user_id, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::ban_chat_sender_chat`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#banchatsenderchat
pub struct BanChatSenderChatRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    sender_chat_id: i64,
}

impl<'bot> BanChatSenderChatRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>, sender_chat_id: i64) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            sender_chat_id,
        }
    }
}

impl<'bot> IntoFuture for BanChatSenderChatRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_ban_chat_sender_chat(self.chat_id, self.sender_chat_id)
                .await
        })
    }
}

/// Fluent builder for [`Bot::close`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#close
pub struct CloseRequest<'bot> {
    bot: &'bot Bot,
}

impl<'bot> CloseRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot) -> Self {
        Self { bot }
    }
}

impl<'bot> IntoFuture for CloseRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.raw_close().await })
    }
}

/// Fluent builder for [`Bot::close_forum_topic`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#closeforumtopic
pub struct CloseForumTopicRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    message_thread_id: i64,
}

impl<'bot> CloseForumTopicRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>, message_thread_id: i64) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            message_thread_id,
        }
    }
}

impl<'bot> IntoFuture for CloseForumTopicRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_close_forum_topic(self.chat_id, self.message_thread_id)
                .await
        })
    }
}

/// Fluent builder for [`Bot::close_general_forum_topic`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#closegeneralforumtopic
pub struct CloseGeneralForumTopicRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
}

impl<'bot> CloseGeneralForumTopicRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
        }
    }
}

impl<'bot> IntoFuture for CloseGeneralForumTopicRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.raw_close_general_forum_topic(self.chat_id).await })
    }
}

/// Fluent builder for [`Bot::convert_gift_to_stars`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#convertgifttostars
pub struct ConvertGiftToStarsRequest<'bot> {
    bot: &'bot Bot,
    business_connection_id: String,
    owned_gift_id: String,
}

impl<'bot> ConvertGiftToStarsRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        business_connection_id: impl Into<String>,
        owned_gift_id: impl Into<String>,
    ) -> Self {
        Self {
            bot,
            business_connection_id: business_connection_id.into(),
            owned_gift_id: owned_gift_id.into(),
        }
    }
}

impl<'bot> IntoFuture for ConvertGiftToStarsRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_convert_gift_to_stars(self.business_connection_id, self.owned_gift_id)
                .await
        })
    }
}

/// Fluent builder for [`Bot::copy_message`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#copymessage
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

    /// Set `parse_mode` to `"HTML"`.
    pub fn html(self) -> Self {
        self.parse_mode("HTML")
    }
    /// Set `parse_mode` to `"MarkdownV2"`.
    pub fn markdown(self) -> Self {
        self.parse_mode("MarkdownV2")
    }

    /// Send silently (`disable_notification = true`).
    pub fn silent(self) -> Self {
        self.disable_notification(true)
    }

    /// Reply to a message by id.
    pub fn reply_to(mut self, message_id: i64) -> Self {
        self.params.reply_parameters = Some(make_reply_params(message_id));
        self
    }

    pub fn message_thread_id(mut self, v: i64) -> Self {
        self.params.message_thread_id = Some(v);
        self
    }
    pub fn direct_messages_topic_id(mut self, v: i64) -> Self {
        self.params.direct_messages_topic_id = Some(v);
        self
    }
    pub fn video_start_timestamp(mut self, v: i64) -> Self {
        self.params.video_start_timestamp = Some(v);
        self
    }
    pub fn caption(mut self, v: impl Into<String>) -> Self {
        self.params.caption = Some(v.into());
        self
    }
    pub fn parse_mode(mut self, v: impl Into<String>) -> Self {
        self.params.parse_mode = Some(v.into());
        self
    }
    pub fn caption_entities(mut self, v: Vec<MessageEntity>) -> Self {
        self.params.caption_entities = Some(v);
        self
    }
    pub fn show_caption_above_media(mut self, v: bool) -> Self {
        self.params.show_caption_above_media = Some(v);
        self
    }
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
    pub fn allow_paid_broadcast(mut self, v: bool) -> Self {
        self.params.allow_paid_broadcast = Some(v);
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.params.message_effect_id = Some(v.into());
        self
    }
    pub fn suggested_post_parameters(mut self, v: SuggestedPostParameters) -> Self {
        self.params.suggested_post_parameters = Some(Box::new(v));
        self
    }
    pub fn reply_parameters(mut self, v: ReplyParameters) -> Self {
        self.params.reply_parameters = Some(Box::new(v));
        self
    }
    pub fn reply_markup(mut self, v: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for CopyMessageRequest<'bot> {
    type Output = Result<MessageId, BotError>;
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

/// Fluent builder for [`Bot::copy_messages`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#copymessages
pub struct CopyMessagesRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    from_chat_id: ChatId,
    message_ids: Vec<i64>,
    params: CopyMessagesParams,
}

impl<'bot> CopyMessagesRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        chat_id: impl Into<ChatId>,
        from_chat_id: impl Into<ChatId>,
        message_ids: Vec<i64>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            from_chat_id: from_chat_id.into(),
            message_ids,
            params: Default::default(),
        }
    }

    /// Send silently (`disable_notification = true`).
    pub fn silent(self) -> Self {
        self.disable_notification(true)
    }

    pub fn message_thread_id(mut self, v: i64) -> Self {
        self.params.message_thread_id = Some(v);
        self
    }
    pub fn direct_messages_topic_id(mut self, v: i64) -> Self {
        self.params.direct_messages_topic_id = Some(v);
        self
    }
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
    pub fn remove_caption(mut self, v: bool) -> Self {
        self.params.remove_caption = Some(v);
        self
    }
}

impl<'bot> IntoFuture for CopyMessagesRequest<'bot> {
    type Output = Result<Vec<MessageId>, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .copy_messages_with_params(
                    self.chat_id,
                    self.from_chat_id,
                    self.message_ids,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::create_chat_invite_link`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#createchatinvitelink
pub struct CreateChatInviteLinkRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    params: CreateChatInviteLinkParams,
}

impl<'bot> CreateChatInviteLinkRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            params: Default::default(),
        }
    }

    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.params.name = Some(v.into());
        self
    }
    pub fn expire_date(mut self, v: i64) -> Self {
        self.params.expire_date = Some(v);
        self
    }
    pub fn member_limit(mut self, v: i64) -> Self {
        self.params.member_limit = Some(v);
        self
    }
    pub fn creates_join_request(mut self, v: bool) -> Self {
        self.params.creates_join_request = Some(v);
        self
    }
}

impl<'bot> IntoFuture for CreateChatInviteLinkRequest<'bot> {
    type Output = Result<ChatInviteLink, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .create_chat_invite_link_with_params(self.chat_id, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::create_chat_subscription_invite_link`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#createchatsubscriptioninvitelink
pub struct CreateChatSubscriptionInviteLinkRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    subscription_period: i64,
    subscription_price: i64,
    params: CreateChatSubscriptionInviteLinkParams,
}

impl<'bot> CreateChatSubscriptionInviteLinkRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        chat_id: impl Into<ChatId>,
        subscription_period: i64,
        subscription_price: i64,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            subscription_period,
            subscription_price,
            params: Default::default(),
        }
    }

    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.params.name = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for CreateChatSubscriptionInviteLinkRequest<'bot> {
    type Output = Result<ChatInviteLink, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .create_chat_subscription_invite_link_with_params(
                    self.chat_id,
                    self.subscription_period,
                    self.subscription_price,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::create_forum_topic`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#createforumtopic
pub struct CreateForumTopicRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    name: String,
    params: CreateForumTopicParams,
}

impl<'bot> CreateForumTopicRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>, name: impl Into<String>) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            name: name.into(),
            params: Default::default(),
        }
    }

    pub fn icon_color(mut self, v: i64) -> Self {
        self.params.icon_color = Some(v);
        self
    }
    pub fn icon_custom_emoji_id(mut self, v: impl Into<String>) -> Self {
        self.params.icon_custom_emoji_id = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for CreateForumTopicRequest<'bot> {
    type Output = Result<ForumTopic, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .create_forum_topic_with_params(self.chat_id, self.name, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::create_invoice_link`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#createinvoicelink
pub struct CreateInvoiceLinkRequest<'bot> {
    bot: &'bot Bot,
    title: String,
    description: String,
    payload: String,
    currency: String,
    prices: Vec<LabeledPrice>,
    params: CreateInvoiceLinkParams,
}

impl<'bot> CreateInvoiceLinkRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        title: impl Into<String>,
        description: impl Into<String>,
        payload: impl Into<String>,
        currency: impl Into<String>,
        prices: Vec<LabeledPrice>,
    ) -> Self {
        Self {
            bot,
            title: title.into(),
            description: description.into(),
            payload: payload.into(),
            currency: currency.into(),
            prices,
            params: Default::default(),
        }
    }

    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(v.into());
        self
    }
    pub fn provider_token(mut self, v: impl Into<String>) -> Self {
        self.params.provider_token = Some(v.into());
        self
    }
    pub fn subscription_period(mut self, v: i64) -> Self {
        self.params.subscription_period = Some(v);
        self
    }
    pub fn max_tip_amount(mut self, v: i64) -> Self {
        self.params.max_tip_amount = Some(v);
        self
    }
    pub fn suggested_tip_amounts(mut self, v: Vec<i64>) -> Self {
        self.params.suggested_tip_amounts = Some(v);
        self
    }
    pub fn provider_data(mut self, v: impl Into<String>) -> Self {
        self.params.provider_data = Some(v.into());
        self
    }
    pub fn photo_url(mut self, v: impl Into<String>) -> Self {
        self.params.photo_url = Some(v.into());
        self
    }
    pub fn photo_size(mut self, v: i64) -> Self {
        self.params.photo_size = Some(v);
        self
    }
    pub fn photo_width(mut self, v: i64) -> Self {
        self.params.photo_width = Some(v);
        self
    }
    pub fn photo_height(mut self, v: i64) -> Self {
        self.params.photo_height = Some(v);
        self
    }
    pub fn need_name(mut self, v: bool) -> Self {
        self.params.need_name = Some(v);
        self
    }
    pub fn need_phone_number(mut self, v: bool) -> Self {
        self.params.need_phone_number = Some(v);
        self
    }
    pub fn need_email(mut self, v: bool) -> Self {
        self.params.need_email = Some(v);
        self
    }
    pub fn need_shipping_address(mut self, v: bool) -> Self {
        self.params.need_shipping_address = Some(v);
        self
    }
    pub fn send_phone_number_to_provider(mut self, v: bool) -> Self {
        self.params.send_phone_number_to_provider = Some(v);
        self
    }
    pub fn send_email_to_provider(mut self, v: bool) -> Self {
        self.params.send_email_to_provider = Some(v);
        self
    }
    pub fn is_flexible(mut self, v: bool) -> Self {
        self.params.is_flexible = Some(v);
        self
    }
}

impl<'bot> IntoFuture for CreateInvoiceLinkRequest<'bot> {
    type Output = Result<String, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .create_invoice_link_with_params(
                    self.title,
                    self.description,
                    self.payload,
                    self.currency,
                    self.prices,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::create_new_sticker_set`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#createnewstickerset
pub struct CreateNewStickerSetRequest<'bot> {
    bot: &'bot Bot,
    user_id: i64,
    name: String,
    title: String,
    stickers: Vec<InputSticker>,
    params: CreateNewStickerSetParams,
}

impl<'bot> CreateNewStickerSetRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        user_id: i64,
        name: impl Into<String>,
        title: impl Into<String>,
        stickers: Vec<InputSticker>,
    ) -> Self {
        Self {
            bot,
            user_id,
            name: name.into(),
            title: title.into(),
            stickers,
            params: Default::default(),
        }
    }

    pub fn sticker_type(mut self, v: impl Into<String>) -> Self {
        self.params.sticker_type = Some(v.into());
        self
    }
    pub fn needs_repainting(mut self, v: bool) -> Self {
        self.params.needs_repainting = Some(v);
        self
    }
}

impl<'bot> IntoFuture for CreateNewStickerSetRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .create_new_sticker_set_with_params(
                    self.user_id,
                    self.name,
                    self.title,
                    self.stickers,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::decline_chat_join_request`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#declinechatjoinrequest
pub struct DeclineChatJoinRequestRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    user_id: i64,
}

impl<'bot> DeclineChatJoinRequestRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>, user_id: i64) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            user_id,
        }
    }
}

impl<'bot> IntoFuture for DeclineChatJoinRequestRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_decline_chat_join_request(self.chat_id, self.user_id)
                .await
        })
    }
}

/// Fluent builder for [`Bot::decline_suggested_post`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#declinesuggestedpost
pub struct DeclineSuggestedPostRequest<'bot> {
    bot: &'bot Bot,
    chat_id: i64,
    message_id: i64,
    params: DeclineSuggestedPostParams,
}

impl<'bot> DeclineSuggestedPostRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: i64, message_id: i64) -> Self {
        Self {
            bot,
            chat_id,
            message_id,
            params: Default::default(),
        }
    }

    pub fn comment(mut self, v: impl Into<String>) -> Self {
        self.params.comment = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for DeclineSuggestedPostRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .decline_suggested_post_with_params(
                    self.chat_id,
                    self.message_id,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::delete_business_messages`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#deletebusinessmessages
pub struct DeleteBusinessMessagesRequest<'bot> {
    bot: &'bot Bot,
    business_connection_id: String,
    message_ids: Vec<i64>,
}

impl<'bot> DeleteBusinessMessagesRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        business_connection_id: impl Into<String>,
        message_ids: Vec<i64>,
    ) -> Self {
        Self {
            bot,
            business_connection_id: business_connection_id.into(),
            message_ids,
        }
    }
}

impl<'bot> IntoFuture for DeleteBusinessMessagesRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_delete_business_messages(self.business_connection_id, self.message_ids)
                .await
        })
    }
}

/// Fluent builder for [`Bot::delete_chat_photo`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#deletechatphoto
pub struct DeleteChatPhotoRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
}

impl<'bot> DeleteChatPhotoRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
        }
    }
}

impl<'bot> IntoFuture for DeleteChatPhotoRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.raw_delete_chat_photo(self.chat_id).await })
    }
}

/// Fluent builder for [`Bot::delete_chat_sticker_set`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#deletechatstickerset
pub struct DeleteChatStickerSetRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
}

impl<'bot> DeleteChatStickerSetRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
        }
    }
}

impl<'bot> IntoFuture for DeleteChatStickerSetRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.raw_delete_chat_sticker_set(self.chat_id).await })
    }
}

/// Fluent builder for [`Bot::delete_forum_topic`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#deleteforumtopic
pub struct DeleteForumTopicRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    message_thread_id: i64,
}

impl<'bot> DeleteForumTopicRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>, message_thread_id: i64) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            message_thread_id,
        }
    }
}

impl<'bot> IntoFuture for DeleteForumTopicRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_delete_forum_topic(self.chat_id, self.message_thread_id)
                .await
        })
    }
}

/// Fluent builder for [`Bot::delete_message`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#deletemessage
pub struct DeleteMessageRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    message_id: i64,
}

impl<'bot> DeleteMessageRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>, message_id: i64) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            message_id,
        }
    }
}

impl<'bot> IntoFuture for DeleteMessageRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_delete_message(self.chat_id, self.message_id)
                .await
        })
    }
}

/// Fluent builder for [`Bot::delete_messages`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#deletemessages
pub struct DeleteMessagesRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    message_ids: Vec<i64>,
}

impl<'bot> DeleteMessagesRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>, message_ids: Vec<i64>) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            message_ids,
        }
    }
}

impl<'bot> IntoFuture for DeleteMessagesRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_delete_messages(self.chat_id, self.message_ids)
                .await
        })
    }
}

/// Fluent builder for [`Bot::delete_my_commands`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#deletemycommands
pub struct DeleteMyCommandsRequest<'bot> {
    bot: &'bot Bot,
    params: DeleteMyCommandsParams,
}

impl<'bot> DeleteMyCommandsRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot) -> Self {
        Self {
            bot,
            params: Default::default(),
        }
    }

    pub fn scope(mut self, v: BotCommandScope) -> Self {
        self.params.scope = Some(Box::new(v));
        self
    }
    pub fn language_code(mut self, v: impl Into<String>) -> Self {
        self.params.language_code = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for DeleteMyCommandsRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .delete_my_commands_with_params(Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::delete_sticker_from_set`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#deletestickerfromset
pub struct DeleteStickerFromSetRequest<'bot> {
    bot: &'bot Bot,
    sticker: String,
}

impl<'bot> DeleteStickerFromSetRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, sticker: impl Into<String>) -> Self {
        Self {
            bot,
            sticker: sticker.into(),
        }
    }
}

impl<'bot> IntoFuture for DeleteStickerFromSetRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.raw_delete_sticker_from_set(self.sticker).await })
    }
}

/// Fluent builder for [`Bot::delete_sticker_set`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#deletestickerset
pub struct DeleteStickerSetRequest<'bot> {
    bot: &'bot Bot,
    name: String,
}

impl<'bot> DeleteStickerSetRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, name: impl Into<String>) -> Self {
        Self {
            bot,
            name: name.into(),
        }
    }
}

impl<'bot> IntoFuture for DeleteStickerSetRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.raw_delete_sticker_set(self.name).await })
    }
}

/// Fluent builder for [`Bot::delete_story`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#deletestory
pub struct DeleteStoryRequest<'bot> {
    bot: &'bot Bot,
    business_connection_id: String,
    story_id: i64,
}

impl<'bot> DeleteStoryRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        business_connection_id: impl Into<String>,
        story_id: i64,
    ) -> Self {
        Self {
            bot,
            business_connection_id: business_connection_id.into(),
            story_id,
        }
    }
}

impl<'bot> IntoFuture for DeleteStoryRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_delete_story(self.business_connection_id, self.story_id)
                .await
        })
    }
}

/// Fluent builder for [`Bot::delete_webhook`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#deletewebhook
pub struct DeleteWebhookRequest<'bot> {
    bot: &'bot Bot,
    params: DeleteWebhookParams,
}

impl<'bot> DeleteWebhookRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot) -> Self {
        Self {
            bot,
            params: Default::default(),
        }
    }

    pub fn drop_pending_updates(mut self, v: bool) -> Self {
        self.params.drop_pending_updates = Some(v);
        self
    }
}

impl<'bot> IntoFuture for DeleteWebhookRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.delete_webhook_with_params(Some(self.params)).await })
    }
}

/// Fluent builder for [`Bot::edit_chat_invite_link`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#editchatinvitelink
pub struct EditChatInviteLinkRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    invite_link: String,
    params: EditChatInviteLinkParams,
}

impl<'bot> EditChatInviteLinkRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        chat_id: impl Into<ChatId>,
        invite_link: impl Into<String>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            invite_link: invite_link.into(),
            params: Default::default(),
        }
    }

    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.params.name = Some(v.into());
        self
    }
    pub fn expire_date(mut self, v: i64) -> Self {
        self.params.expire_date = Some(v);
        self
    }
    pub fn member_limit(mut self, v: i64) -> Self {
        self.params.member_limit = Some(v);
        self
    }
    pub fn creates_join_request(mut self, v: bool) -> Self {
        self.params.creates_join_request = Some(v);
        self
    }
}

impl<'bot> IntoFuture for EditChatInviteLinkRequest<'bot> {
    type Output = Result<ChatInviteLink, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .edit_chat_invite_link_with_params(
                    self.chat_id,
                    self.invite_link,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::edit_chat_subscription_invite_link`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#editchatsubscriptioninvitelink
pub struct EditChatSubscriptionInviteLinkRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    invite_link: String,
    params: EditChatSubscriptionInviteLinkParams,
}

impl<'bot> EditChatSubscriptionInviteLinkRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        chat_id: impl Into<ChatId>,
        invite_link: impl Into<String>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            invite_link: invite_link.into(),
            params: Default::default(),
        }
    }

    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.params.name = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for EditChatSubscriptionInviteLinkRequest<'bot> {
    type Output = Result<ChatInviteLink, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .edit_chat_subscription_invite_link_with_params(
                    self.chat_id,
                    self.invite_link,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::edit_forum_topic`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#editforumtopic
pub struct EditForumTopicRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    message_thread_id: i64,
    params: EditForumTopicParams,
}

impl<'bot> EditForumTopicRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>, message_thread_id: i64) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            message_thread_id,
            params: Default::default(),
        }
    }

    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.params.name = Some(v.into());
        self
    }
    pub fn icon_custom_emoji_id(mut self, v: impl Into<String>) -> Self {
        self.params.icon_custom_emoji_id = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for EditForumTopicRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .edit_forum_topic_with_params(
                    self.chat_id,
                    self.message_thread_id,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::edit_general_forum_topic`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#editgeneralforumtopic
pub struct EditGeneralForumTopicRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    name: String,
}

impl<'bot> EditGeneralForumTopicRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>, name: impl Into<String>) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            name: name.into(),
        }
    }
}

impl<'bot> IntoFuture for EditGeneralForumTopicRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_edit_general_forum_topic(self.chat_id, self.name)
                .await
        })
    }
}

/// Fluent builder for [`Bot::edit_message_caption`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#editmessagecaption
pub struct EditMessageCaptionRequest<'bot> {
    bot: &'bot Bot,
    params: EditMessageCaptionParams,
}

impl<'bot> EditMessageCaptionRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot) -> Self {
        Self {
            bot,
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

    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(v.into());
        self
    }
    pub fn chat_id(mut self, v: impl Into<ChatId>) -> Self {
        self.params.chat_id = Some(v.into());
        self
    }
    pub fn message_id(mut self, v: i64) -> Self {
        self.params.message_id = Some(v);
        self
    }
    pub fn inline_message_id(mut self, v: impl Into<String>) -> Self {
        self.params.inline_message_id = Some(v.into());
        self
    }
    pub fn caption(mut self, v: impl Into<String>) -> Self {
        self.params.caption = Some(v.into());
        self
    }
    pub fn parse_mode(mut self, v: impl Into<String>) -> Self {
        self.params.parse_mode = Some(v.into());
        self
    }
    pub fn caption_entities(mut self, v: Vec<MessageEntity>) -> Self {
        self.params.caption_entities = Some(v);
        self
    }
    pub fn show_caption_above_media(mut self, v: bool) -> Self {
        self.params.show_caption_above_media = Some(v);
        self
    }
    pub fn reply_markup(mut self, v: InlineKeyboardMarkup) -> Self {
        self.params.reply_markup = Some(Box::new(v));
        self
    }
}

impl<'bot> IntoFuture for EditMessageCaptionRequest<'bot> {
    type Output = Result<serde_json::Value, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .edit_message_caption_with_params(Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::edit_message_checklist`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#editmessagechecklist
pub struct EditMessageChecklistRequest<'bot> {
    bot: &'bot Bot,
    business_connection_id: String,
    chat_id: i64,
    message_id: i64,
    checklist: InputChecklist,
    params: EditMessageChecklistParams,
}

impl<'bot> EditMessageChecklistRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        business_connection_id: impl Into<String>,
        chat_id: i64,
        message_id: i64,
        checklist: InputChecklist,
    ) -> Self {
        Self {
            bot,
            business_connection_id: business_connection_id.into(),
            chat_id,
            message_id,
            checklist,
            params: Default::default(),
        }
    }

    pub fn reply_markup(mut self, v: InlineKeyboardMarkup) -> Self {
        self.params.reply_markup = Some(Box::new(v));
        self
    }
}

impl<'bot> IntoFuture for EditMessageChecklistRequest<'bot> {
    type Output = Result<Message, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .edit_message_checklist_with_params(
                    self.business_connection_id,
                    self.chat_id,
                    self.message_id,
                    self.checklist,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::edit_message_live_location`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#editmessagelivelocation
pub struct EditMessageLiveLocationRequest<'bot> {
    bot: &'bot Bot,
    latitude: f64,
    longitude: f64,
    params: EditMessageLiveLocationParams,
}

impl<'bot> EditMessageLiveLocationRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, latitude: f64, longitude: f64) -> Self {
        Self {
            bot,
            latitude,
            longitude,
            params: Default::default(),
        }
    }

    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(v.into());
        self
    }
    pub fn chat_id(mut self, v: impl Into<ChatId>) -> Self {
        self.params.chat_id = Some(v.into());
        self
    }
    pub fn message_id(mut self, v: i64) -> Self {
        self.params.message_id = Some(v);
        self
    }
    pub fn inline_message_id(mut self, v: impl Into<String>) -> Self {
        self.params.inline_message_id = Some(v.into());
        self
    }
    pub fn live_period(mut self, v: i64) -> Self {
        self.params.live_period = Some(v);
        self
    }
    pub fn horizontal_accuracy(mut self, v: f64) -> Self {
        self.params.horizontal_accuracy = Some(v);
        self
    }
    pub fn heading(mut self, v: i64) -> Self {
        self.params.heading = Some(v);
        self
    }
    pub fn proximity_alert_radius(mut self, v: i64) -> Self {
        self.params.proximity_alert_radius = Some(v);
        self
    }
    pub fn reply_markup(mut self, v: InlineKeyboardMarkup) -> Self {
        self.params.reply_markup = Some(Box::new(v));
        self
    }
}

impl<'bot> IntoFuture for EditMessageLiveLocationRequest<'bot> {
    type Output = Result<serde_json::Value, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .edit_message_live_location_with_params(
                    self.latitude,
                    self.longitude,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::edit_message_media`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#editmessagemedia
pub struct EditMessageMediaRequest<'bot> {
    bot: &'bot Bot,
    media: Vec<InputMedia>,
    params: EditMessageMediaParams,
}

impl<'bot> EditMessageMediaRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, media: Vec<InputMedia>) -> Self {
        Self {
            bot,
            media,
            params: Default::default(),
        }
    }

    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(v.into());
        self
    }
    pub fn chat_id(mut self, v: impl Into<ChatId>) -> Self {
        self.params.chat_id = Some(v.into());
        self
    }
    pub fn message_id(mut self, v: i64) -> Self {
        self.params.message_id = Some(v);
        self
    }
    pub fn inline_message_id(mut self, v: impl Into<String>) -> Self {
        self.params.inline_message_id = Some(v.into());
        self
    }
    pub fn reply_markup(mut self, v: InlineKeyboardMarkup) -> Self {
        self.params.reply_markup = Some(Box::new(v));
        self
    }
}

impl<'bot> IntoFuture for EditMessageMediaRequest<'bot> {
    type Output = Result<serde_json::Value, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .edit_message_media_with_params(self.media, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::edit_message_reply_markup`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#editmessagereplymarkup
pub struct EditMessageReplyMarkupRequest<'bot> {
    bot: &'bot Bot,
    params: EditMessageReplyMarkupParams,
}

impl<'bot> EditMessageReplyMarkupRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot) -> Self {
        Self {
            bot,
            params: Default::default(),
        }
    }

    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(v.into());
        self
    }
    pub fn chat_id(mut self, v: impl Into<ChatId>) -> Self {
        self.params.chat_id = Some(v.into());
        self
    }
    pub fn message_id(mut self, v: i64) -> Self {
        self.params.message_id = Some(v);
        self
    }
    pub fn inline_message_id(mut self, v: impl Into<String>) -> Self {
        self.params.inline_message_id = Some(v.into());
        self
    }
    pub fn reply_markup(mut self, v: InlineKeyboardMarkup) -> Self {
        self.params.reply_markup = Some(Box::new(v));
        self
    }
}

impl<'bot> IntoFuture for EditMessageReplyMarkupRequest<'bot> {
    type Output = Result<serde_json::Value, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .edit_message_reply_markup_with_params(Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::edit_message_text`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#editmessagetext
pub struct EditMessageTextRequest<'bot> {
    bot: &'bot Bot,
    text: String,
    params: EditMessageTextParams,
}

impl<'bot> EditMessageTextRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, text: impl Into<String>) -> Self {
        Self {
            bot,
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

    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(v.into());
        self
    }
    pub fn chat_id(mut self, v: impl Into<ChatId>) -> Self {
        self.params.chat_id = Some(v.into());
        self
    }
    pub fn message_id(mut self, v: i64) -> Self {
        self.params.message_id = Some(v);
        self
    }
    pub fn inline_message_id(mut self, v: impl Into<String>) -> Self {
        self.params.inline_message_id = Some(v.into());
        self
    }
    pub fn parse_mode(mut self, v: impl Into<String>) -> Self {
        self.params.parse_mode = Some(v.into());
        self
    }
    pub fn entities(mut self, v: Vec<MessageEntity>) -> Self {
        self.params.entities = Some(v);
        self
    }
    pub fn link_preview_options(mut self, v: LinkPreviewOptions) -> Self {
        self.params.link_preview_options = Some(Box::new(v));
        self
    }
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

/// Fluent builder for [`Bot::edit_story`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#editstory
pub struct EditStoryRequest<'bot> {
    bot: &'bot Bot,
    business_connection_id: String,
    story_id: i64,
    content: InputStoryContent,
    params: EditStoryParams,
}

impl<'bot> EditStoryRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        business_connection_id: impl Into<String>,
        story_id: i64,
        content: InputStoryContent,
    ) -> Self {
        Self {
            bot,
            business_connection_id: business_connection_id.into(),
            story_id,
            content,
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

    pub fn caption(mut self, v: impl Into<String>) -> Self {
        self.params.caption = Some(v.into());
        self
    }
    pub fn parse_mode(mut self, v: impl Into<String>) -> Self {
        self.params.parse_mode = Some(v.into());
        self
    }
    pub fn caption_entities(mut self, v: Vec<MessageEntity>) -> Self {
        self.params.caption_entities = Some(v);
        self
    }
    pub fn areas(mut self, v: Vec<StoryArea>) -> Self {
        self.params.areas = Some(v);
        self
    }
}

impl<'bot> IntoFuture for EditStoryRequest<'bot> {
    type Output = Result<Story, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .edit_story_with_params(
                    self.business_connection_id,
                    self.story_id,
                    self.content,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::edit_user_star_subscription`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#edituserstarsubscription
pub struct EditUserStarSubscriptionRequest<'bot> {
    bot: &'bot Bot,
    user_id: i64,
    telegram_payment_charge_id: String,
    is_canceled: bool,
}

impl<'bot> EditUserStarSubscriptionRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        user_id: i64,
        telegram_payment_charge_id: impl Into<String>,
        is_canceled: bool,
    ) -> Self {
        Self {
            bot,
            user_id,
            telegram_payment_charge_id: telegram_payment_charge_id.into(),
            is_canceled,
        }
    }
}

impl<'bot> IntoFuture for EditUserStarSubscriptionRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_edit_user_star_subscription(
                    self.user_id,
                    self.telegram_payment_charge_id,
                    self.is_canceled,
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::export_chat_invite_link`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#exportchatinvitelink
pub struct ExportChatInviteLinkRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
}

impl<'bot> ExportChatInviteLinkRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
        }
    }
}

impl<'bot> IntoFuture for ExportChatInviteLinkRequest<'bot> {
    type Output = Result<String, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.raw_export_chat_invite_link(self.chat_id).await })
    }
}

/// Fluent builder for [`Bot::forward_message`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#forwardmessage
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

    /// Send silently (`disable_notification = true`).
    pub fn silent(self) -> Self {
        self.disable_notification(true)
    }

    pub fn message_thread_id(mut self, v: i64) -> Self {
        self.params.message_thread_id = Some(v);
        self
    }
    pub fn direct_messages_topic_id(mut self, v: i64) -> Self {
        self.params.direct_messages_topic_id = Some(v);
        self
    }
    pub fn video_start_timestamp(mut self, v: i64) -> Self {
        self.params.video_start_timestamp = Some(v);
        self
    }
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.params.message_effect_id = Some(v.into());
        self
    }
    pub fn suggested_post_parameters(mut self, v: SuggestedPostParameters) -> Self {
        self.params.suggested_post_parameters = Some(Box::new(v));
        self
    }
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

/// Fluent builder for [`Bot::forward_messages`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#forwardmessages
pub struct ForwardMessagesRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    from_chat_id: ChatId,
    message_ids: Vec<i64>,
    params: ForwardMessagesParams,
}

impl<'bot> ForwardMessagesRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        chat_id: impl Into<ChatId>,
        from_chat_id: impl Into<ChatId>,
        message_ids: Vec<i64>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            from_chat_id: from_chat_id.into(),
            message_ids,
            params: Default::default(),
        }
    }

    /// Send silently (`disable_notification = true`).
    pub fn silent(self) -> Self {
        self.disable_notification(true)
    }

    pub fn message_thread_id(mut self, v: i64) -> Self {
        self.params.message_thread_id = Some(v);
        self
    }
    pub fn direct_messages_topic_id(mut self, v: i64) -> Self {
        self.params.direct_messages_topic_id = Some(v);
        self
    }
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
}

impl<'bot> IntoFuture for ForwardMessagesRequest<'bot> {
    type Output = Result<Vec<MessageId>, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .forward_messages_with_params(
                    self.chat_id,
                    self.from_chat_id,
                    self.message_ids,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::get_available_gifts`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#getavailablegifts
pub struct GetAvailableGiftsRequest<'bot> {
    bot: &'bot Bot,
}

impl<'bot> GetAvailableGiftsRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot) -> Self {
        Self { bot }
    }
}

impl<'bot> IntoFuture for GetAvailableGiftsRequest<'bot> {
    type Output = Result<Gifts, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.raw_get_available_gifts().await })
    }
}

/// Fluent builder for [`Bot::get_business_account_gifts`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#getbusinessaccountgifts
pub struct GetBusinessAccountGiftsRequest<'bot> {
    bot: &'bot Bot,
    business_connection_id: String,
    params: GetBusinessAccountGiftsParams,
}

impl<'bot> GetBusinessAccountGiftsRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, business_connection_id: impl Into<String>) -> Self {
        Self {
            bot,
            business_connection_id: business_connection_id.into(),
            params: Default::default(),
        }
    }

    pub fn exclude_unsaved(mut self, v: bool) -> Self {
        self.params.exclude_unsaved = Some(v);
        self
    }
    pub fn exclude_saved(mut self, v: bool) -> Self {
        self.params.exclude_saved = Some(v);
        self
    }
    pub fn exclude_unlimited(mut self, v: bool) -> Self {
        self.params.exclude_unlimited = Some(v);
        self
    }
    pub fn exclude_limited_upgradable(mut self, v: bool) -> Self {
        self.params.exclude_limited_upgradable = Some(v);
        self
    }
    pub fn exclude_limited_non_upgradable(mut self, v: bool) -> Self {
        self.params.exclude_limited_non_upgradable = Some(v);
        self
    }
    pub fn exclude_unique(mut self, v: bool) -> Self {
        self.params.exclude_unique = Some(v);
        self
    }
    pub fn exclude_from_blockchain(mut self, v: bool) -> Self {
        self.params.exclude_from_blockchain = Some(v);
        self
    }
    pub fn sort_by_price(mut self, v: bool) -> Self {
        self.params.sort_by_price = Some(v);
        self
    }
    pub fn offset(mut self, v: impl Into<String>) -> Self {
        self.params.offset = Some(v.into());
        self
    }
    pub fn limit(mut self, v: i64) -> Self {
        self.params.limit = Some(v);
        self
    }
}

impl<'bot> IntoFuture for GetBusinessAccountGiftsRequest<'bot> {
    type Output = Result<OwnedGifts, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .get_business_account_gifts_with_params(
                    self.business_connection_id,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::get_business_account_star_balance`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#getbusinessaccountstarbalance
pub struct GetBusinessAccountStarBalanceRequest<'bot> {
    bot: &'bot Bot,
    business_connection_id: String,
}

impl<'bot> GetBusinessAccountStarBalanceRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, business_connection_id: impl Into<String>) -> Self {
        Self {
            bot,
            business_connection_id: business_connection_id.into(),
        }
    }
}

impl<'bot> IntoFuture for GetBusinessAccountStarBalanceRequest<'bot> {
    type Output = Result<StarAmount, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_get_business_account_star_balance(self.business_connection_id)
                .await
        })
    }
}

/// Fluent builder for [`Bot::get_business_connection`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#getbusinessconnection
pub struct GetBusinessConnectionRequest<'bot> {
    bot: &'bot Bot,
    business_connection_id: String,
}

impl<'bot> GetBusinessConnectionRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, business_connection_id: impl Into<String>) -> Self {
        Self {
            bot,
            business_connection_id: business_connection_id.into(),
        }
    }
}

impl<'bot> IntoFuture for GetBusinessConnectionRequest<'bot> {
    type Output = Result<BusinessConnection, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_get_business_connection(self.business_connection_id)
                .await
        })
    }
}

/// Fluent builder for [`Bot::get_chat`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#getchat
pub struct GetChatRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
}

impl<'bot> GetChatRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
        }
    }
}

impl<'bot> IntoFuture for GetChatRequest<'bot> {
    type Output = Result<ChatFullInfo, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.raw_get_chat(self.chat_id).await })
    }
}

/// Fluent builder for [`Bot::get_chat_administrators`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#getchatadministrators
pub struct GetChatAdministratorsRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
}

impl<'bot> GetChatAdministratorsRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
        }
    }
}

impl<'bot> IntoFuture for GetChatAdministratorsRequest<'bot> {
    type Output = Result<Vec<ChatMember>, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.raw_get_chat_administrators(self.chat_id).await })
    }
}

/// Fluent builder for [`Bot::get_chat_gifts`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#getchatgifts
pub struct GetChatGiftsRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    params: GetChatGiftsParams,
}

impl<'bot> GetChatGiftsRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            params: Default::default(),
        }
    }

    pub fn exclude_unsaved(mut self, v: bool) -> Self {
        self.params.exclude_unsaved = Some(v);
        self
    }
    pub fn exclude_saved(mut self, v: bool) -> Self {
        self.params.exclude_saved = Some(v);
        self
    }
    pub fn exclude_unlimited(mut self, v: bool) -> Self {
        self.params.exclude_unlimited = Some(v);
        self
    }
    pub fn exclude_limited_upgradable(mut self, v: bool) -> Self {
        self.params.exclude_limited_upgradable = Some(v);
        self
    }
    pub fn exclude_limited_non_upgradable(mut self, v: bool) -> Self {
        self.params.exclude_limited_non_upgradable = Some(v);
        self
    }
    pub fn exclude_from_blockchain(mut self, v: bool) -> Self {
        self.params.exclude_from_blockchain = Some(v);
        self
    }
    pub fn exclude_unique(mut self, v: bool) -> Self {
        self.params.exclude_unique = Some(v);
        self
    }
    pub fn sort_by_price(mut self, v: bool) -> Self {
        self.params.sort_by_price = Some(v);
        self
    }
    pub fn offset(mut self, v: impl Into<String>) -> Self {
        self.params.offset = Some(v.into());
        self
    }
    pub fn limit(mut self, v: i64) -> Self {
        self.params.limit = Some(v);
        self
    }
}

impl<'bot> IntoFuture for GetChatGiftsRequest<'bot> {
    type Output = Result<OwnedGifts, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .get_chat_gifts_with_params(self.chat_id, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::get_chat_member`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#getchatmember
pub struct GetChatMemberRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    user_id: i64,
}

impl<'bot> GetChatMemberRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>, user_id: i64) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            user_id,
        }
    }
}

impl<'bot> IntoFuture for GetChatMemberRequest<'bot> {
    type Output = Result<ChatMember, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_get_chat_member(self.chat_id, self.user_id)
                .await
        })
    }
}

/// Fluent builder for [`Bot::get_chat_member_count`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#getchatmembercount
pub struct GetChatMemberCountRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
}

impl<'bot> GetChatMemberCountRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
        }
    }
}

impl<'bot> IntoFuture for GetChatMemberCountRequest<'bot> {
    type Output = Result<i64, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.raw_get_chat_member_count(self.chat_id).await })
    }
}

/// Fluent builder for [`Bot::get_chat_menu_button`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#getchatmenubutton
pub struct GetChatMenuButtonRequest<'bot> {
    bot: &'bot Bot,
    params: GetChatMenuButtonParams,
}

impl<'bot> GetChatMenuButtonRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot) -> Self {
        Self {
            bot,
            params: Default::default(),
        }
    }

    pub fn chat_id(mut self, v: i64) -> Self {
        self.params.chat_id = Some(v);
        self
    }
}

impl<'bot> IntoFuture for GetChatMenuButtonRequest<'bot> {
    type Output = Result<MenuButton, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .get_chat_menu_button_with_params(Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::get_custom_emoji_stickers`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#getcustomemojistickers
pub struct GetCustomEmojiStickersRequest<'bot> {
    bot: &'bot Bot,
    custom_emoji_ids: Vec<String>,
}

impl<'bot> GetCustomEmojiStickersRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, custom_emoji_ids: Vec<String>) -> Self {
        Self {
            bot,
            custom_emoji_ids,
        }
    }
}

impl<'bot> IntoFuture for GetCustomEmojiStickersRequest<'bot> {
    type Output = Result<Vec<Sticker>, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_get_custom_emoji_stickers(self.custom_emoji_ids)
                .await
        })
    }
}

/// Fluent builder for [`Bot::get_file`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#getfile
pub struct GetFileRequest<'bot> {
    bot: &'bot Bot,
    file_id: String,
}

impl<'bot> GetFileRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, file_id: impl Into<String>) -> Self {
        Self {
            bot,
            file_id: file_id.into(),
        }
    }
}

impl<'bot> IntoFuture for GetFileRequest<'bot> {
    type Output = Result<File, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.raw_get_file(self.file_id).await })
    }
}

/// Fluent builder for [`Bot::get_forum_topic_icon_stickers`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#getforumtopiciconstickers
pub struct GetForumTopicIconStickersRequest<'bot> {
    bot: &'bot Bot,
}

impl<'bot> GetForumTopicIconStickersRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot) -> Self {
        Self { bot }
    }
}

impl<'bot> IntoFuture for GetForumTopicIconStickersRequest<'bot> {
    type Output = Result<Vec<Sticker>, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.raw_get_forum_topic_icon_stickers().await })
    }
}

/// Fluent builder for [`Bot::get_game_high_scores`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#getgamehighscores
pub struct GetGameHighScoresRequest<'bot> {
    bot: &'bot Bot,
    user_id: i64,
    params: GetGameHighScoresParams,
}

impl<'bot> GetGameHighScoresRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, user_id: i64) -> Self {
        Self {
            bot,
            user_id,
            params: Default::default(),
        }
    }

    pub fn chat_id(mut self, v: i64) -> Self {
        self.params.chat_id = Some(v);
        self
    }
    pub fn message_id(mut self, v: i64) -> Self {
        self.params.message_id = Some(v);
        self
    }
    pub fn inline_message_id(mut self, v: impl Into<String>) -> Self {
        self.params.inline_message_id = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for GetGameHighScoresRequest<'bot> {
    type Output = Result<Vec<GameHighScore>, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .get_game_high_scores_with_params(self.user_id, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::get_managed_bot_token`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#getmanagedbottoken
pub struct GetManagedBotTokenRequest<'bot> {
    bot: &'bot Bot,
    user_id: i64,
}

impl<'bot> GetManagedBotTokenRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, user_id: i64) -> Self {
        Self { bot, user_id }
    }
}

impl<'bot> IntoFuture for GetManagedBotTokenRequest<'bot> {
    type Output = Result<String, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.raw_get_managed_bot_token(self.user_id).await })
    }
}

/// Fluent builder for [`Bot::get_me`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#getme
pub struct GetMeRequest<'bot> {
    bot: &'bot Bot,
}

impl<'bot> GetMeRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot) -> Self {
        Self { bot }
    }
}

impl<'bot> IntoFuture for GetMeRequest<'bot> {
    type Output = Result<User, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.raw_get_me().await })
    }
}

/// Fluent builder for [`Bot::get_my_commands`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#getmycommands
pub struct GetMyCommandsRequest<'bot> {
    bot: &'bot Bot,
    params: GetMyCommandsParams,
}

impl<'bot> GetMyCommandsRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot) -> Self {
        Self {
            bot,
            params: Default::default(),
        }
    }

    pub fn scope(mut self, v: BotCommandScope) -> Self {
        self.params.scope = Some(Box::new(v));
        self
    }
    pub fn language_code(mut self, v: impl Into<String>) -> Self {
        self.params.language_code = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for GetMyCommandsRequest<'bot> {
    type Output = Result<Vec<BotCommand>, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .get_my_commands_with_params(Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::get_my_default_administrator_rights`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#getmydefaultadministratorrights
pub struct GetMyDefaultAdministratorRightsRequest<'bot> {
    bot: &'bot Bot,
    params: GetMyDefaultAdministratorRightsParams,
}

impl<'bot> GetMyDefaultAdministratorRightsRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot) -> Self {
        Self {
            bot,
            params: Default::default(),
        }
    }

    pub fn for_channels(mut self, v: bool) -> Self {
        self.params.for_channels = Some(v);
        self
    }
}

impl<'bot> IntoFuture for GetMyDefaultAdministratorRightsRequest<'bot> {
    type Output = Result<ChatAdministratorRights, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .get_my_default_administrator_rights_with_params(Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::get_my_description`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#getmydescription
pub struct GetMyDescriptionRequest<'bot> {
    bot: &'bot Bot,
    params: GetMyDescriptionParams,
}

impl<'bot> GetMyDescriptionRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot) -> Self {
        Self {
            bot,
            params: Default::default(),
        }
    }

    pub fn language_code(mut self, v: impl Into<String>) -> Self {
        self.params.language_code = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for GetMyDescriptionRequest<'bot> {
    type Output = Result<BotDescription, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .get_my_description_with_params(Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::get_my_name`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#getmyname
pub struct GetMyNameRequest<'bot> {
    bot: &'bot Bot,
    params: GetMyNameParams,
}

impl<'bot> GetMyNameRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot) -> Self {
        Self {
            bot,
            params: Default::default(),
        }
    }

    pub fn language_code(mut self, v: impl Into<String>) -> Self {
        self.params.language_code = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for GetMyNameRequest<'bot> {
    type Output = Result<BotName, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.get_my_name_with_params(Some(self.params)).await })
    }
}

/// Fluent builder for [`Bot::get_my_short_description`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#getmyshortdescription
pub struct GetMyShortDescriptionRequest<'bot> {
    bot: &'bot Bot,
    params: GetMyShortDescriptionParams,
}

impl<'bot> GetMyShortDescriptionRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot) -> Self {
        Self {
            bot,
            params: Default::default(),
        }
    }

    pub fn language_code(mut self, v: impl Into<String>) -> Self {
        self.params.language_code = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for GetMyShortDescriptionRequest<'bot> {
    type Output = Result<BotShortDescription, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .get_my_short_description_with_params(Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::get_my_star_balance`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#getmystarbalance
pub struct GetMyStarBalanceRequest<'bot> {
    bot: &'bot Bot,
}

impl<'bot> GetMyStarBalanceRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot) -> Self {
        Self { bot }
    }
}

impl<'bot> IntoFuture for GetMyStarBalanceRequest<'bot> {
    type Output = Result<StarAmount, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.raw_get_my_star_balance().await })
    }
}

/// Fluent builder for [`Bot::get_star_transactions`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#getstartransactions
pub struct GetStarTransactionsRequest<'bot> {
    bot: &'bot Bot,
    params: GetStarTransactionsParams,
}

impl<'bot> GetStarTransactionsRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot) -> Self {
        Self {
            bot,
            params: Default::default(),
        }
    }

    pub fn offset(mut self, v: i64) -> Self {
        self.params.offset = Some(v);
        self
    }
    pub fn limit(mut self, v: i64) -> Self {
        self.params.limit = Some(v);
        self
    }
}

impl<'bot> IntoFuture for GetStarTransactionsRequest<'bot> {
    type Output = Result<StarTransactions, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .get_star_transactions_with_params(Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::get_sticker_set`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#getstickerset
pub struct GetStickerSetRequest<'bot> {
    bot: &'bot Bot,
    name: String,
}

impl<'bot> GetStickerSetRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, name: impl Into<String>) -> Self {
        Self {
            bot,
            name: name.into(),
        }
    }
}

impl<'bot> IntoFuture for GetStickerSetRequest<'bot> {
    type Output = Result<StickerSet, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.raw_get_sticker_set(self.name).await })
    }
}

/// Fluent builder for [`Bot::get_updates`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#getupdates
pub struct GetUpdatesRequest<'bot> {
    bot: &'bot Bot,
    params: GetUpdatesParams,
}

impl<'bot> GetUpdatesRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot) -> Self {
        Self {
            bot,
            params: Default::default(),
        }
    }

    pub fn offset(mut self, v: i64) -> Self {
        self.params.offset = Some(v);
        self
    }
    pub fn limit(mut self, v: i64) -> Self {
        self.params.limit = Some(v);
        self
    }
    pub fn timeout(mut self, v: i64) -> Self {
        self.params.timeout = Some(v);
        self
    }
    pub fn allowed_updates(mut self, v: Vec<String>) -> Self {
        self.params.allowed_updates = Some(v);
        self
    }
}

impl<'bot> IntoFuture for GetUpdatesRequest<'bot> {
    type Output = Result<Vec<Update>, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.get_updates_with_params(Some(self.params)).await })
    }
}

/// Fluent builder for [`Bot::get_user_chat_boosts`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#getuserchatboosts
pub struct GetUserChatBoostsRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    user_id: i64,
}

impl<'bot> GetUserChatBoostsRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>, user_id: i64) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            user_id,
        }
    }
}

impl<'bot> IntoFuture for GetUserChatBoostsRequest<'bot> {
    type Output = Result<UserChatBoosts, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_get_user_chat_boosts(self.chat_id, self.user_id)
                .await
        })
    }
}

/// Fluent builder for [`Bot::get_user_gifts`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#getusergifts
pub struct GetUserGiftsRequest<'bot> {
    bot: &'bot Bot,
    user_id: i64,
    params: GetUserGiftsParams,
}

impl<'bot> GetUserGiftsRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, user_id: i64) -> Self {
        Self {
            bot,
            user_id,
            params: Default::default(),
        }
    }

    pub fn exclude_unlimited(mut self, v: bool) -> Self {
        self.params.exclude_unlimited = Some(v);
        self
    }
    pub fn exclude_limited_upgradable(mut self, v: bool) -> Self {
        self.params.exclude_limited_upgradable = Some(v);
        self
    }
    pub fn exclude_limited_non_upgradable(mut self, v: bool) -> Self {
        self.params.exclude_limited_non_upgradable = Some(v);
        self
    }
    pub fn exclude_from_blockchain(mut self, v: bool) -> Self {
        self.params.exclude_from_blockchain = Some(v);
        self
    }
    pub fn exclude_unique(mut self, v: bool) -> Self {
        self.params.exclude_unique = Some(v);
        self
    }
    pub fn sort_by_price(mut self, v: bool) -> Self {
        self.params.sort_by_price = Some(v);
        self
    }
    pub fn offset(mut self, v: impl Into<String>) -> Self {
        self.params.offset = Some(v.into());
        self
    }
    pub fn limit(mut self, v: i64) -> Self {
        self.params.limit = Some(v);
        self
    }
}

impl<'bot> IntoFuture for GetUserGiftsRequest<'bot> {
    type Output = Result<OwnedGifts, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .get_user_gifts_with_params(self.user_id, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::get_user_profile_audios`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#getuserprofileaudios
pub struct GetUserProfileAudiosRequest<'bot> {
    bot: &'bot Bot,
    user_id: i64,
    params: GetUserProfileAudiosParams,
}

impl<'bot> GetUserProfileAudiosRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, user_id: i64) -> Self {
        Self {
            bot,
            user_id,
            params: Default::default(),
        }
    }

    pub fn offset(mut self, v: i64) -> Self {
        self.params.offset = Some(v);
        self
    }
    pub fn limit(mut self, v: i64) -> Self {
        self.params.limit = Some(v);
        self
    }
}

impl<'bot> IntoFuture for GetUserProfileAudiosRequest<'bot> {
    type Output = Result<UserProfileAudios, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .get_user_profile_audios_with_params(self.user_id, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::get_user_profile_photos`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#getuserprofilephotos
pub struct GetUserProfilePhotosRequest<'bot> {
    bot: &'bot Bot,
    user_id: i64,
    params: GetUserProfilePhotosParams,
}

impl<'bot> GetUserProfilePhotosRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, user_id: i64) -> Self {
        Self {
            bot,
            user_id,
            params: Default::default(),
        }
    }

    pub fn offset(mut self, v: i64) -> Self {
        self.params.offset = Some(v);
        self
    }
    pub fn limit(mut self, v: i64) -> Self {
        self.params.limit = Some(v);
        self
    }
}

impl<'bot> IntoFuture for GetUserProfilePhotosRequest<'bot> {
    type Output = Result<UserProfilePhotos, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .get_user_profile_photos_with_params(self.user_id, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::get_webhook_info`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#getwebhookinfo
pub struct GetWebhookInfoRequest<'bot> {
    bot: &'bot Bot,
}

impl<'bot> GetWebhookInfoRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot) -> Self {
        Self { bot }
    }
}

impl<'bot> IntoFuture for GetWebhookInfoRequest<'bot> {
    type Output = Result<WebhookInfo, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.raw_get_webhook_info().await })
    }
}

/// Fluent builder for [`Bot::gift_premium_subscription`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#giftpremiumsubscription
pub struct GiftPremiumSubscriptionRequest<'bot> {
    bot: &'bot Bot,
    user_id: i64,
    month_count: i64,
    star_count: i64,
    params: GiftPremiumSubscriptionParams,
}

impl<'bot> GiftPremiumSubscriptionRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, user_id: i64, month_count: i64, star_count: i64) -> Self {
        Self {
            bot,
            user_id,
            month_count,
            star_count,
            params: Default::default(),
        }
    }

    pub fn text(mut self, v: impl Into<String>) -> Self {
        self.params.text = Some(v.into());
        self
    }
    pub fn text_parse_mode(mut self, v: impl Into<String>) -> Self {
        self.params.text_parse_mode = Some(v.into());
        self
    }
    pub fn text_entities(mut self, v: Vec<MessageEntity>) -> Self {
        self.params.text_entities = Some(v);
        self
    }
}

impl<'bot> IntoFuture for GiftPremiumSubscriptionRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .gift_premium_subscription_with_params(
                    self.user_id,
                    self.month_count,
                    self.star_count,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::hide_general_forum_topic`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#hidegeneralforumtopic
pub struct HideGeneralForumTopicRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
}

impl<'bot> HideGeneralForumTopicRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
        }
    }
}

impl<'bot> IntoFuture for HideGeneralForumTopicRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.raw_hide_general_forum_topic(self.chat_id).await })
    }
}

/// Fluent builder for [`Bot::leave_chat`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#leavechat
pub struct LeaveChatRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
}

impl<'bot> LeaveChatRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
        }
    }
}

impl<'bot> IntoFuture for LeaveChatRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.raw_leave_chat(self.chat_id).await })
    }
}

/// Fluent builder for [`Bot::log_out`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#logout
pub struct LogOutRequest<'bot> {
    bot: &'bot Bot,
}

impl<'bot> LogOutRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot) -> Self {
        Self { bot }
    }
}

impl<'bot> IntoFuture for LogOutRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.raw_log_out().await })
    }
}

/// Fluent builder for [`Bot::pin_chat_message`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#pinchatmessage
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

    /// Send silently (`disable_notification = true`).
    pub fn silent(self) -> Self {
        self.disable_notification(true)
    }

    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(v.into());
        self
    }
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
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

/// Fluent builder for [`Bot::post_story`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#poststory
pub struct PostStoryRequest<'bot> {
    bot: &'bot Bot,
    business_connection_id: String,
    content: InputStoryContent,
    active_period: i64,
    params: PostStoryParams,
}

impl<'bot> PostStoryRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        business_connection_id: impl Into<String>,
        content: InputStoryContent,
        active_period: i64,
    ) -> Self {
        Self {
            bot,
            business_connection_id: business_connection_id.into(),
            content,
            active_period,
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

    pub fn caption(mut self, v: impl Into<String>) -> Self {
        self.params.caption = Some(v.into());
        self
    }
    pub fn parse_mode(mut self, v: impl Into<String>) -> Self {
        self.params.parse_mode = Some(v.into());
        self
    }
    pub fn caption_entities(mut self, v: Vec<MessageEntity>) -> Self {
        self.params.caption_entities = Some(v);
        self
    }
    pub fn areas(mut self, v: Vec<StoryArea>) -> Self {
        self.params.areas = Some(v);
        self
    }
    pub fn post_to_chat_page(mut self, v: bool) -> Self {
        self.params.post_to_chat_page = Some(v);
        self
    }
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
}

impl<'bot> IntoFuture for PostStoryRequest<'bot> {
    type Output = Result<Story, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .post_story_with_params(
                    self.business_connection_id,
                    self.content,
                    self.active_period,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::promote_chat_member`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#promotechatmember
pub struct PromoteChatMemberRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    user_id: i64,
    params: PromoteChatMemberParams,
}

impl<'bot> PromoteChatMemberRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>, user_id: i64) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            user_id,
            params: Default::default(),
        }
    }

    pub fn is_anonymous(mut self, v: bool) -> Self {
        self.params.is_anonymous = Some(v);
        self
    }
    pub fn can_manage_chat(mut self, v: bool) -> Self {
        self.params.can_manage_chat = Some(v);
        self
    }
    pub fn can_delete_messages(mut self, v: bool) -> Self {
        self.params.can_delete_messages = Some(v);
        self
    }
    pub fn can_manage_video_chats(mut self, v: bool) -> Self {
        self.params.can_manage_video_chats = Some(v);
        self
    }
    pub fn can_restrict_members(mut self, v: bool) -> Self {
        self.params.can_restrict_members = Some(v);
        self
    }
    pub fn can_promote_members(mut self, v: bool) -> Self {
        self.params.can_promote_members = Some(v);
        self
    }
    pub fn can_change_info(mut self, v: bool) -> Self {
        self.params.can_change_info = Some(v);
        self
    }
    pub fn can_invite_users(mut self, v: bool) -> Self {
        self.params.can_invite_users = Some(v);
        self
    }
    pub fn can_post_stories(mut self, v: bool) -> Self {
        self.params.can_post_stories = Some(v);
        self
    }
    pub fn can_edit_stories(mut self, v: bool) -> Self {
        self.params.can_edit_stories = Some(v);
        self
    }
    pub fn can_delete_stories(mut self, v: bool) -> Self {
        self.params.can_delete_stories = Some(v);
        self
    }
    pub fn can_post_messages(mut self, v: bool) -> Self {
        self.params.can_post_messages = Some(v);
        self
    }
    pub fn can_edit_messages(mut self, v: bool) -> Self {
        self.params.can_edit_messages = Some(v);
        self
    }
    pub fn can_pin_messages(mut self, v: bool) -> Self {
        self.params.can_pin_messages = Some(v);
        self
    }
    pub fn can_manage_topics(mut self, v: bool) -> Self {
        self.params.can_manage_topics = Some(v);
        self
    }
    pub fn can_manage_direct_messages(mut self, v: bool) -> Self {
        self.params.can_manage_direct_messages = Some(v);
        self
    }
    pub fn can_manage_tags(mut self, v: bool) -> Self {
        self.params.can_manage_tags = Some(v);
        self
    }
}

impl<'bot> IntoFuture for PromoteChatMemberRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .promote_chat_member_with_params(self.chat_id, self.user_id, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::read_business_message`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#readbusinessmessage
pub struct ReadBusinessMessageRequest<'bot> {
    bot: &'bot Bot,
    business_connection_id: String,
    chat_id: i64,
    message_id: i64,
}

impl<'bot> ReadBusinessMessageRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        business_connection_id: impl Into<String>,
        chat_id: i64,
        message_id: i64,
    ) -> Self {
        Self {
            bot,
            business_connection_id: business_connection_id.into(),
            chat_id,
            message_id,
        }
    }
}

impl<'bot> IntoFuture for ReadBusinessMessageRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_read_business_message(
                    self.business_connection_id,
                    self.chat_id,
                    self.message_id,
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::refund_star_payment`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#refundstarpayment
pub struct RefundStarPaymentRequest<'bot> {
    bot: &'bot Bot,
    user_id: i64,
    telegram_payment_charge_id: String,
}

impl<'bot> RefundStarPaymentRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        user_id: i64,
        telegram_payment_charge_id: impl Into<String>,
    ) -> Self {
        Self {
            bot,
            user_id,
            telegram_payment_charge_id: telegram_payment_charge_id.into(),
        }
    }
}

impl<'bot> IntoFuture for RefundStarPaymentRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_refund_star_payment(self.user_id, self.telegram_payment_charge_id)
                .await
        })
    }
}

/// Fluent builder for [`Bot::remove_business_account_profile_photo`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#removebusinessaccountprofilephoto
pub struct RemoveBusinessAccountProfilePhotoRequest<'bot> {
    bot: &'bot Bot,
    business_connection_id: String,
    params: RemoveBusinessAccountProfilePhotoParams,
}

impl<'bot> RemoveBusinessAccountProfilePhotoRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, business_connection_id: impl Into<String>) -> Self {
        Self {
            bot,
            business_connection_id: business_connection_id.into(),
            params: Default::default(),
        }
    }

    pub fn is_public(mut self, v: bool) -> Self {
        self.params.is_public = Some(v);
        self
    }
}

impl<'bot> IntoFuture for RemoveBusinessAccountProfilePhotoRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .remove_business_account_profile_photo_with_params(
                    self.business_connection_id,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::remove_chat_verification`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#removechatverification
pub struct RemoveChatVerificationRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
}

impl<'bot> RemoveChatVerificationRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
        }
    }
}

impl<'bot> IntoFuture for RemoveChatVerificationRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.raw_remove_chat_verification(self.chat_id).await })
    }
}

/// Fluent builder for [`Bot::remove_my_profile_photo`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#removemyprofilephoto
pub struct RemoveMyProfilePhotoRequest<'bot> {
    bot: &'bot Bot,
}

impl<'bot> RemoveMyProfilePhotoRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot) -> Self {
        Self { bot }
    }
}

impl<'bot> IntoFuture for RemoveMyProfilePhotoRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.raw_remove_my_profile_photo().await })
    }
}

/// Fluent builder for [`Bot::remove_user_verification`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#removeuserverification
pub struct RemoveUserVerificationRequest<'bot> {
    bot: &'bot Bot,
    user_id: i64,
}

impl<'bot> RemoveUserVerificationRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, user_id: i64) -> Self {
        Self { bot, user_id }
    }
}

impl<'bot> IntoFuture for RemoveUserVerificationRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.raw_remove_user_verification(self.user_id).await })
    }
}

/// Fluent builder for [`Bot::reopen_forum_topic`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#reopenforumtopic
pub struct ReopenForumTopicRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    message_thread_id: i64,
}

impl<'bot> ReopenForumTopicRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>, message_thread_id: i64) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            message_thread_id,
        }
    }
}

impl<'bot> IntoFuture for ReopenForumTopicRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_reopen_forum_topic(self.chat_id, self.message_thread_id)
                .await
        })
    }
}

/// Fluent builder for [`Bot::reopen_general_forum_topic`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#reopengeneralforumtopic
pub struct ReopenGeneralForumTopicRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
}

impl<'bot> ReopenGeneralForumTopicRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
        }
    }
}

impl<'bot> IntoFuture for ReopenGeneralForumTopicRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.raw_reopen_general_forum_topic(self.chat_id).await })
    }
}

/// Fluent builder for [`Bot::replace_managed_bot_token`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#replacemanagedbottoken
pub struct ReplaceManagedBotTokenRequest<'bot> {
    bot: &'bot Bot,
    user_id: i64,
}

impl<'bot> ReplaceManagedBotTokenRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, user_id: i64) -> Self {
        Self { bot, user_id }
    }
}

impl<'bot> IntoFuture for ReplaceManagedBotTokenRequest<'bot> {
    type Output = Result<String, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.raw_replace_managed_bot_token(self.user_id).await })
    }
}

/// Fluent builder for [`Bot::replace_sticker_in_set`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#replacestickerinset
pub struct ReplaceStickerInSetRequest<'bot> {
    bot: &'bot Bot,
    user_id: i64,
    name: String,
    old_sticker: String,
    sticker: InputSticker,
}

impl<'bot> ReplaceStickerInSetRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        user_id: i64,
        name: impl Into<String>,
        old_sticker: impl Into<String>,
        sticker: InputSticker,
    ) -> Self {
        Self {
            bot,
            user_id,
            name: name.into(),
            old_sticker: old_sticker.into(),
            sticker,
        }
    }
}

impl<'bot> IntoFuture for ReplaceStickerInSetRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_replace_sticker_in_set(self.user_id, self.name, self.old_sticker, self.sticker)
                .await
        })
    }
}

/// Fluent builder for [`Bot::repost_story`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#repoststory
pub struct RepostStoryRequest<'bot> {
    bot: &'bot Bot,
    business_connection_id: String,
    from_chat_id: i64,
    from_story_id: i64,
    active_period: i64,
    params: RepostStoryParams,
}

impl<'bot> RepostStoryRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        business_connection_id: impl Into<String>,
        from_chat_id: i64,
        from_story_id: i64,
        active_period: i64,
    ) -> Self {
        Self {
            bot,
            business_connection_id: business_connection_id.into(),
            from_chat_id,
            from_story_id,
            active_period,
            params: Default::default(),
        }
    }

    pub fn post_to_chat_page(mut self, v: bool) -> Self {
        self.params.post_to_chat_page = Some(v);
        self
    }
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
}

impl<'bot> IntoFuture for RepostStoryRequest<'bot> {
    type Output = Result<Story, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .repost_story_with_params(
                    self.business_connection_id,
                    self.from_chat_id,
                    self.from_story_id,
                    self.active_period,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::restrict_chat_member`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#restrictchatmember
pub struct RestrictChatMemberRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    user_id: i64,
    permissions: ChatPermissions,
    params: RestrictChatMemberParams,
}

impl<'bot> RestrictChatMemberRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        chat_id: impl Into<ChatId>,
        user_id: i64,
        permissions: ChatPermissions,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            user_id,
            permissions,
            params: Default::default(),
        }
    }

    pub fn use_independent_chat_permissions(mut self, v: bool) -> Self {
        self.params.use_independent_chat_permissions = Some(v);
        self
    }
    pub fn until_date(mut self, v: i64) -> Self {
        self.params.until_date = Some(v);
        self
    }
}

impl<'bot> IntoFuture for RestrictChatMemberRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .restrict_chat_member_with_params(
                    self.chat_id,
                    self.user_id,
                    self.permissions,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::revoke_chat_invite_link`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#revokechatinvitelink
pub struct RevokeChatInviteLinkRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    invite_link: String,
}

impl<'bot> RevokeChatInviteLinkRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        chat_id: impl Into<ChatId>,
        invite_link: impl Into<String>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            invite_link: invite_link.into(),
        }
    }
}

impl<'bot> IntoFuture for RevokeChatInviteLinkRequest<'bot> {
    type Output = Result<ChatInviteLink, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_revoke_chat_invite_link(self.chat_id, self.invite_link)
                .await
        })
    }
}

/// Fluent builder for [`Bot::save_prepared_inline_message`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#savepreparedinlinemessage
pub struct SavePreparedInlineMessageRequest<'bot> {
    bot: &'bot Bot,
    user_id: i64,
    result: InlineQueryResult,
    params: SavePreparedInlineMessageParams,
}

impl<'bot> SavePreparedInlineMessageRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, user_id: i64, result: InlineQueryResult) -> Self {
        Self {
            bot,
            user_id,
            result,
            params: Default::default(),
        }
    }

    pub fn allow_user_chats(mut self, v: bool) -> Self {
        self.params.allow_user_chats = Some(v);
        self
    }
    pub fn allow_bot_chats(mut self, v: bool) -> Self {
        self.params.allow_bot_chats = Some(v);
        self
    }
    pub fn allow_group_chats(mut self, v: bool) -> Self {
        self.params.allow_group_chats = Some(v);
        self
    }
    pub fn allow_channel_chats(mut self, v: bool) -> Self {
        self.params.allow_channel_chats = Some(v);
        self
    }
}

impl<'bot> IntoFuture for SavePreparedInlineMessageRequest<'bot> {
    type Output = Result<PreparedInlineMessage, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .save_prepared_inline_message_with_params(
                    self.user_id,
                    self.result,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::save_prepared_keyboard_button`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#savepreparedkeyboardbutton
pub struct SavePreparedKeyboardButtonRequest<'bot> {
    bot: &'bot Bot,
    user_id: i64,
    button: KeyboardButton,
}

impl<'bot> SavePreparedKeyboardButtonRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, user_id: i64, button: KeyboardButton) -> Self {
        Self {
            bot,
            user_id,
            button,
        }
    }
}

impl<'bot> IntoFuture for SavePreparedKeyboardButtonRequest<'bot> {
    type Output = Result<PreparedKeyboardButton, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_save_prepared_keyboard_button(self.user_id, self.button)
                .await
        })
    }
}

/// Fluent builder for [`Bot::send_animation`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#sendanimation
pub struct SendAnimationRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    animation: InputFileOrString,
    params: SendAnimationParams,
}

impl<'bot> SendAnimationRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        chat_id: impl Into<ChatId>,
        animation: impl Into<InputFileOrString>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            animation: animation.into(),
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

    /// Send silently (`disable_notification = true`).
    pub fn silent(self) -> Self {
        self.disable_notification(true)
    }

    /// Reply to a message by id.
    pub fn reply_to(mut self, message_id: i64) -> Self {
        self.params.reply_parameters = Some(make_reply_params(message_id));
        self
    }

    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: i64) -> Self {
        self.params.message_thread_id = Some(v);
        self
    }
    pub fn direct_messages_topic_id(mut self, v: i64) -> Self {
        self.params.direct_messages_topic_id = Some(v);
        self
    }
    pub fn duration(mut self, v: i64) -> Self {
        self.params.duration = Some(v);
        self
    }
    pub fn width(mut self, v: i64) -> Self {
        self.params.width = Some(v);
        self
    }
    pub fn height(mut self, v: i64) -> Self {
        self.params.height = Some(v);
        self
    }
    pub fn thumbnail(mut self, v: impl Into<InputFileOrString>) -> Self {
        self.params.thumbnail = Some(v.into());
        self
    }
    pub fn caption(mut self, v: impl Into<String>) -> Self {
        self.params.caption = Some(v.into());
        self
    }
    pub fn parse_mode(mut self, v: impl Into<String>) -> Self {
        self.params.parse_mode = Some(v.into());
        self
    }
    pub fn caption_entities(mut self, v: Vec<MessageEntity>) -> Self {
        self.params.caption_entities = Some(v);
        self
    }
    pub fn show_caption_above_media(mut self, v: bool) -> Self {
        self.params.show_caption_above_media = Some(v);
        self
    }
    pub fn has_spoiler(mut self, v: bool) -> Self {
        self.params.has_spoiler = Some(v);
        self
    }
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
    pub fn allow_paid_broadcast(mut self, v: bool) -> Self {
        self.params.allow_paid_broadcast = Some(v);
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.params.message_effect_id = Some(v.into());
        self
    }
    pub fn suggested_post_parameters(mut self, v: SuggestedPostParameters) -> Self {
        self.params.suggested_post_parameters = Some(Box::new(v));
        self
    }
    pub fn reply_parameters(mut self, v: ReplyParameters) -> Self {
        self.params.reply_parameters = Some(Box::new(v));
        self
    }
    pub fn reply_markup(mut self, v: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for SendAnimationRequest<'bot> {
    type Output = Result<Message, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .send_animation_with_params(self.chat_id, self.animation, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::send_audio`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#sendaudio
pub struct SendAudioRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    audio: InputFileOrString,
    params: SendAudioParams,
}

impl<'bot> SendAudioRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        chat_id: impl Into<ChatId>,
        audio: impl Into<InputFileOrString>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            audio: audio.into(),
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

    /// Send silently (`disable_notification = true`).
    pub fn silent(self) -> Self {
        self.disable_notification(true)
    }

    /// Reply to a message by id.
    pub fn reply_to(mut self, message_id: i64) -> Self {
        self.params.reply_parameters = Some(make_reply_params(message_id));
        self
    }

    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: i64) -> Self {
        self.params.message_thread_id = Some(v);
        self
    }
    pub fn direct_messages_topic_id(mut self, v: i64) -> Self {
        self.params.direct_messages_topic_id = Some(v);
        self
    }
    pub fn caption(mut self, v: impl Into<String>) -> Self {
        self.params.caption = Some(v.into());
        self
    }
    pub fn parse_mode(mut self, v: impl Into<String>) -> Self {
        self.params.parse_mode = Some(v.into());
        self
    }
    pub fn caption_entities(mut self, v: Vec<MessageEntity>) -> Self {
        self.params.caption_entities = Some(v);
        self
    }
    pub fn duration(mut self, v: i64) -> Self {
        self.params.duration = Some(v);
        self
    }
    pub fn performer(mut self, v: impl Into<String>) -> Self {
        self.params.performer = Some(v.into());
        self
    }
    pub fn title(mut self, v: impl Into<String>) -> Self {
        self.params.title = Some(v.into());
        self
    }
    pub fn thumbnail(mut self, v: impl Into<InputFileOrString>) -> Self {
        self.params.thumbnail = Some(v.into());
        self
    }
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
    pub fn allow_paid_broadcast(mut self, v: bool) -> Self {
        self.params.allow_paid_broadcast = Some(v);
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.params.message_effect_id = Some(v.into());
        self
    }
    pub fn suggested_post_parameters(mut self, v: SuggestedPostParameters) -> Self {
        self.params.suggested_post_parameters = Some(Box::new(v));
        self
    }
    pub fn reply_parameters(mut self, v: ReplyParameters) -> Self {
        self.params.reply_parameters = Some(Box::new(v));
        self
    }
    pub fn reply_markup(mut self, v: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for SendAudioRequest<'bot> {
    type Output = Result<Message, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .send_audio_with_params(self.chat_id, self.audio, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::send_chat_action`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#sendchataction
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

    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: i64) -> Self {
        self.params.message_thread_id = Some(v);
        self
    }
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

/// Fluent builder for [`Bot::send_checklist`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#sendchecklist
pub struct SendChecklistRequest<'bot> {
    bot: &'bot Bot,
    business_connection_id: String,
    chat_id: i64,
    checklist: InputChecklist,
    params: SendChecklistParams,
}

impl<'bot> SendChecklistRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        business_connection_id: impl Into<String>,
        chat_id: i64,
        checklist: InputChecklist,
    ) -> Self {
        Self {
            bot,
            business_connection_id: business_connection_id.into(),
            chat_id,
            checklist,
            params: Default::default(),
        }
    }

    /// Send silently (`disable_notification = true`).
    pub fn silent(self) -> Self {
        self.disable_notification(true)
    }

    /// Reply to a message by id.
    pub fn reply_to(mut self, message_id: i64) -> Self {
        self.params.reply_parameters = Some(make_reply_params(message_id));
        self
    }

    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.params.message_effect_id = Some(v.into());
        self
    }
    pub fn reply_parameters(mut self, v: ReplyParameters) -> Self {
        self.params.reply_parameters = Some(Box::new(v));
        self
    }
    pub fn reply_markup(mut self, v: InlineKeyboardMarkup) -> Self {
        self.params.reply_markup = Some(Box::new(v));
        self
    }
}

impl<'bot> IntoFuture for SendChecklistRequest<'bot> {
    type Output = Result<Message, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .send_checklist_with_params(
                    self.business_connection_id,
                    self.chat_id,
                    self.checklist,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::send_contact`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#sendcontact
pub struct SendContactRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    phone_number: String,
    first_name: String,
    params: SendContactParams,
}

impl<'bot> SendContactRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        chat_id: impl Into<ChatId>,
        phone_number: impl Into<String>,
        first_name: impl Into<String>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            phone_number: phone_number.into(),
            first_name: first_name.into(),
            params: Default::default(),
        }
    }

    /// Send silently (`disable_notification = true`).
    pub fn silent(self) -> Self {
        self.disable_notification(true)
    }

    /// Reply to a message by id.
    pub fn reply_to(mut self, message_id: i64) -> Self {
        self.params.reply_parameters = Some(make_reply_params(message_id));
        self
    }

    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: i64) -> Self {
        self.params.message_thread_id = Some(v);
        self
    }
    pub fn direct_messages_topic_id(mut self, v: i64) -> Self {
        self.params.direct_messages_topic_id = Some(v);
        self
    }
    pub fn last_name(mut self, v: impl Into<String>) -> Self {
        self.params.last_name = Some(v.into());
        self
    }
    pub fn vcard(mut self, v: impl Into<String>) -> Self {
        self.params.vcard = Some(v.into());
        self
    }
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
    pub fn allow_paid_broadcast(mut self, v: bool) -> Self {
        self.params.allow_paid_broadcast = Some(v);
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.params.message_effect_id = Some(v.into());
        self
    }
    pub fn suggested_post_parameters(mut self, v: SuggestedPostParameters) -> Self {
        self.params.suggested_post_parameters = Some(Box::new(v));
        self
    }
    pub fn reply_parameters(mut self, v: ReplyParameters) -> Self {
        self.params.reply_parameters = Some(Box::new(v));
        self
    }
    pub fn reply_markup(mut self, v: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for SendContactRequest<'bot> {
    type Output = Result<Message, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .send_contact_with_params(
                    self.chat_id,
                    self.phone_number,
                    self.first_name,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::send_dice`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#senddice
pub struct SendDiceRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    params: SendDiceParams,
}

impl<'bot> SendDiceRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            params: Default::default(),
        }
    }

    /// Send silently (`disable_notification = true`).
    pub fn silent(self) -> Self {
        self.disable_notification(true)
    }

    /// Reply to a message by id.
    pub fn reply_to(mut self, message_id: i64) -> Self {
        self.params.reply_parameters = Some(make_reply_params(message_id));
        self
    }

    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: i64) -> Self {
        self.params.message_thread_id = Some(v);
        self
    }
    pub fn direct_messages_topic_id(mut self, v: i64) -> Self {
        self.params.direct_messages_topic_id = Some(v);
        self
    }
    pub fn emoji(mut self, v: impl Into<String>) -> Self {
        self.params.emoji = Some(v.into());
        self
    }
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
    pub fn allow_paid_broadcast(mut self, v: bool) -> Self {
        self.params.allow_paid_broadcast = Some(v);
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.params.message_effect_id = Some(v.into());
        self
    }
    pub fn suggested_post_parameters(mut self, v: SuggestedPostParameters) -> Self {
        self.params.suggested_post_parameters = Some(Box::new(v));
        self
    }
    pub fn reply_parameters(mut self, v: ReplyParameters) -> Self {
        self.params.reply_parameters = Some(Box::new(v));
        self
    }
    pub fn reply_markup(mut self, v: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for SendDiceRequest<'bot> {
    type Output = Result<Message, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .send_dice_with_params(self.chat_id, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::send_document`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#senddocument
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

    /// Set `parse_mode` to `"HTML"`.
    pub fn html(self) -> Self {
        self.parse_mode("HTML")
    }
    /// Set `parse_mode` to `"MarkdownV2"`.
    pub fn markdown(self) -> Self {
        self.parse_mode("MarkdownV2")
    }

    /// Send silently (`disable_notification = true`).
    pub fn silent(self) -> Self {
        self.disable_notification(true)
    }

    /// Reply to a message by id.
    pub fn reply_to(mut self, message_id: i64) -> Self {
        self.params.reply_parameters = Some(make_reply_params(message_id));
        self
    }

    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: i64) -> Self {
        self.params.message_thread_id = Some(v);
        self
    }
    pub fn direct_messages_topic_id(mut self, v: i64) -> Self {
        self.params.direct_messages_topic_id = Some(v);
        self
    }
    pub fn thumbnail(mut self, v: impl Into<InputFileOrString>) -> Self {
        self.params.thumbnail = Some(v.into());
        self
    }
    pub fn caption(mut self, v: impl Into<String>) -> Self {
        self.params.caption = Some(v.into());
        self
    }
    pub fn parse_mode(mut self, v: impl Into<String>) -> Self {
        self.params.parse_mode = Some(v.into());
        self
    }
    pub fn caption_entities(mut self, v: Vec<MessageEntity>) -> Self {
        self.params.caption_entities = Some(v);
        self
    }
    pub fn disable_content_type_detection(mut self, v: bool) -> Self {
        self.params.disable_content_type_detection = Some(v);
        self
    }
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
    pub fn allow_paid_broadcast(mut self, v: bool) -> Self {
        self.params.allow_paid_broadcast = Some(v);
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.params.message_effect_id = Some(v.into());
        self
    }
    pub fn suggested_post_parameters(mut self, v: SuggestedPostParameters) -> Self {
        self.params.suggested_post_parameters = Some(Box::new(v));
        self
    }
    pub fn reply_parameters(mut self, v: ReplyParameters) -> Self {
        self.params.reply_parameters = Some(Box::new(v));
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

/// Fluent builder for [`Bot::send_game`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#sendgame
pub struct SendGameRequest<'bot> {
    bot: &'bot Bot,
    chat_id: i64,
    game_short_name: String,
    params: SendGameParams,
}

impl<'bot> SendGameRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: i64, game_short_name: impl Into<String>) -> Self {
        Self {
            bot,
            chat_id,
            game_short_name: game_short_name.into(),
            params: Default::default(),
        }
    }

    /// Send silently (`disable_notification = true`).
    pub fn silent(self) -> Self {
        self.disable_notification(true)
    }

    /// Reply to a message by id.
    pub fn reply_to(mut self, message_id: i64) -> Self {
        self.params.reply_parameters = Some(make_reply_params(message_id));
        self
    }

    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: i64) -> Self {
        self.params.message_thread_id = Some(v);
        self
    }
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
    pub fn allow_paid_broadcast(mut self, v: bool) -> Self {
        self.params.allow_paid_broadcast = Some(v);
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.params.message_effect_id = Some(v.into());
        self
    }
    pub fn reply_parameters(mut self, v: ReplyParameters) -> Self {
        self.params.reply_parameters = Some(Box::new(v));
        self
    }
    pub fn reply_markup(mut self, v: InlineKeyboardMarkup) -> Self {
        self.params.reply_markup = Some(Box::new(v));
        self
    }
}

impl<'bot> IntoFuture for SendGameRequest<'bot> {
    type Output = Result<Message, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .send_game_with_params(self.chat_id, self.game_short_name, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::send_gift`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#sendgift
pub struct SendGiftRequest<'bot> {
    bot: &'bot Bot,
    gift_id: String,
    params: SendGiftParams,
}

impl<'bot> SendGiftRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, gift_id: impl Into<String>) -> Self {
        Self {
            bot,
            gift_id: gift_id.into(),
            params: Default::default(),
        }
    }

    pub fn user_id(mut self, v: i64) -> Self {
        self.params.user_id = Some(v);
        self
    }
    pub fn chat_id(mut self, v: impl Into<ChatId>) -> Self {
        self.params.chat_id = Some(v.into());
        self
    }
    pub fn pay_for_upgrade(mut self, v: bool) -> Self {
        self.params.pay_for_upgrade = Some(v);
        self
    }
    pub fn text(mut self, v: impl Into<String>) -> Self {
        self.params.text = Some(v.into());
        self
    }
    pub fn text_parse_mode(mut self, v: impl Into<String>) -> Self {
        self.params.text_parse_mode = Some(v.into());
        self
    }
    pub fn text_entities(mut self, v: Vec<MessageEntity>) -> Self {
        self.params.text_entities = Some(v);
        self
    }
}

impl<'bot> IntoFuture for SendGiftRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .send_gift_with_params(self.gift_id, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::send_invoice`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#sendinvoice
pub struct SendInvoiceRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    title: String,
    description: String,
    payload: String,
    currency: String,
    prices: Vec<LabeledPrice>,
    params: SendInvoiceParams,
}

impl<'bot> SendInvoiceRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        chat_id: impl Into<ChatId>,
        title: impl Into<String>,
        description: impl Into<String>,
        payload: impl Into<String>,
        currency: impl Into<String>,
        prices: Vec<LabeledPrice>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            title: title.into(),
            description: description.into(),
            payload: payload.into(),
            currency: currency.into(),
            prices,
            params: Default::default(),
        }
    }

    /// Send silently (`disable_notification = true`).
    pub fn silent(self) -> Self {
        self.disable_notification(true)
    }

    /// Reply to a message by id.
    pub fn reply_to(mut self, message_id: i64) -> Self {
        self.params.reply_parameters = Some(make_reply_params(message_id));
        self
    }

    pub fn message_thread_id(mut self, v: i64) -> Self {
        self.params.message_thread_id = Some(v);
        self
    }
    pub fn direct_messages_topic_id(mut self, v: i64) -> Self {
        self.params.direct_messages_topic_id = Some(v);
        self
    }
    pub fn provider_token(mut self, v: impl Into<String>) -> Self {
        self.params.provider_token = Some(v.into());
        self
    }
    pub fn max_tip_amount(mut self, v: i64) -> Self {
        self.params.max_tip_amount = Some(v);
        self
    }
    pub fn suggested_tip_amounts(mut self, v: Vec<i64>) -> Self {
        self.params.suggested_tip_amounts = Some(v);
        self
    }
    pub fn start_parameter(mut self, v: impl Into<String>) -> Self {
        self.params.start_parameter = Some(v.into());
        self
    }
    pub fn provider_data(mut self, v: impl Into<String>) -> Self {
        self.params.provider_data = Some(v.into());
        self
    }
    pub fn photo_url(mut self, v: impl Into<String>) -> Self {
        self.params.photo_url = Some(v.into());
        self
    }
    pub fn photo_size(mut self, v: i64) -> Self {
        self.params.photo_size = Some(v);
        self
    }
    pub fn photo_width(mut self, v: i64) -> Self {
        self.params.photo_width = Some(v);
        self
    }
    pub fn photo_height(mut self, v: i64) -> Self {
        self.params.photo_height = Some(v);
        self
    }
    pub fn need_name(mut self, v: bool) -> Self {
        self.params.need_name = Some(v);
        self
    }
    pub fn need_phone_number(mut self, v: bool) -> Self {
        self.params.need_phone_number = Some(v);
        self
    }
    pub fn need_email(mut self, v: bool) -> Self {
        self.params.need_email = Some(v);
        self
    }
    pub fn need_shipping_address(mut self, v: bool) -> Self {
        self.params.need_shipping_address = Some(v);
        self
    }
    pub fn send_phone_number_to_provider(mut self, v: bool) -> Self {
        self.params.send_phone_number_to_provider = Some(v);
        self
    }
    pub fn send_email_to_provider(mut self, v: bool) -> Self {
        self.params.send_email_to_provider = Some(v);
        self
    }
    pub fn is_flexible(mut self, v: bool) -> Self {
        self.params.is_flexible = Some(v);
        self
    }
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
    pub fn allow_paid_broadcast(mut self, v: bool) -> Self {
        self.params.allow_paid_broadcast = Some(v);
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.params.message_effect_id = Some(v.into());
        self
    }
    pub fn suggested_post_parameters(mut self, v: SuggestedPostParameters) -> Self {
        self.params.suggested_post_parameters = Some(Box::new(v));
        self
    }
    pub fn reply_parameters(mut self, v: ReplyParameters) -> Self {
        self.params.reply_parameters = Some(Box::new(v));
        self
    }
    pub fn reply_markup(mut self, v: InlineKeyboardMarkup) -> Self {
        self.params.reply_markup = Some(Box::new(v));
        self
    }
}

impl<'bot> IntoFuture for SendInvoiceRequest<'bot> {
    type Output = Result<Message, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .send_invoice_with_params(
                    self.chat_id,
                    self.title,
                    self.description,
                    self.payload,
                    self.currency,
                    self.prices,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::send_location`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#sendlocation
pub struct SendLocationRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    latitude: f64,
    longitude: f64,
    params: SendLocationParams,
}

impl<'bot> SendLocationRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        chat_id: impl Into<ChatId>,
        latitude: f64,
        longitude: f64,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            latitude,
            longitude,
            params: Default::default(),
        }
    }

    /// Send silently (`disable_notification = true`).
    pub fn silent(self) -> Self {
        self.disable_notification(true)
    }

    /// Reply to a message by id.
    pub fn reply_to(mut self, message_id: i64) -> Self {
        self.params.reply_parameters = Some(make_reply_params(message_id));
        self
    }

    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: i64) -> Self {
        self.params.message_thread_id = Some(v);
        self
    }
    pub fn direct_messages_topic_id(mut self, v: i64) -> Self {
        self.params.direct_messages_topic_id = Some(v);
        self
    }
    pub fn horizontal_accuracy(mut self, v: f64) -> Self {
        self.params.horizontal_accuracy = Some(v);
        self
    }
    pub fn live_period(mut self, v: i64) -> Self {
        self.params.live_period = Some(v);
        self
    }
    pub fn heading(mut self, v: i64) -> Self {
        self.params.heading = Some(v);
        self
    }
    pub fn proximity_alert_radius(mut self, v: i64) -> Self {
        self.params.proximity_alert_radius = Some(v);
        self
    }
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
    pub fn allow_paid_broadcast(mut self, v: bool) -> Self {
        self.params.allow_paid_broadcast = Some(v);
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.params.message_effect_id = Some(v.into());
        self
    }
    pub fn suggested_post_parameters(mut self, v: SuggestedPostParameters) -> Self {
        self.params.suggested_post_parameters = Some(Box::new(v));
        self
    }
    pub fn reply_parameters(mut self, v: ReplyParameters) -> Self {
        self.params.reply_parameters = Some(Box::new(v));
        self
    }
    pub fn reply_markup(mut self, v: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for SendLocationRequest<'bot> {
    type Output = Result<Message, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .send_location_with_params(
                    self.chat_id,
                    self.latitude,
                    self.longitude,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::send_media_group`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#sendmediagroup
pub struct SendMediaGroupRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    media: Vec<InputMedia>,
    params: SendMediaGroupParams,
}

impl<'bot> SendMediaGroupRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>, media: Vec<InputMedia>) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            media,
            params: Default::default(),
        }
    }

    /// Send silently (`disable_notification = true`).
    pub fn silent(self) -> Self {
        self.disable_notification(true)
    }

    /// Reply to a message by id.
    pub fn reply_to(mut self, message_id: i64) -> Self {
        self.params.reply_parameters = Some(make_reply_params(message_id));
        self
    }

    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: i64) -> Self {
        self.params.message_thread_id = Some(v);
        self
    }
    pub fn direct_messages_topic_id(mut self, v: i64) -> Self {
        self.params.direct_messages_topic_id = Some(v);
        self
    }
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
    pub fn allow_paid_broadcast(mut self, v: bool) -> Self {
        self.params.allow_paid_broadcast = Some(v);
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.params.message_effect_id = Some(v.into());
        self
    }
    pub fn reply_parameters(mut self, v: ReplyParameters) -> Self {
        self.params.reply_parameters = Some(Box::new(v));
        self
    }
}

impl<'bot> IntoFuture for SendMediaGroupRequest<'bot> {
    type Output = Result<Vec<Message>, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .send_media_group_with_params(self.chat_id, self.media, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::send_message`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#sendmessage
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

    /// Send silently (`disable_notification = true`).
    pub fn silent(self) -> Self {
        self.disable_notification(true)
    }

    /// Reply to a message by id.
    pub fn reply_to(mut self, message_id: i64) -> Self {
        self.params.reply_parameters = Some(make_reply_params(message_id));
        self
    }

    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: i64) -> Self {
        self.params.message_thread_id = Some(v);
        self
    }
    pub fn direct_messages_topic_id(mut self, v: i64) -> Self {
        self.params.direct_messages_topic_id = Some(v);
        self
    }
    pub fn parse_mode(mut self, v: impl Into<String>) -> Self {
        self.params.parse_mode = Some(v.into());
        self
    }
    pub fn entities(mut self, v: Vec<MessageEntity>) -> Self {
        self.params.entities = Some(v);
        self
    }
    pub fn link_preview_options(mut self, v: LinkPreviewOptions) -> Self {
        self.params.link_preview_options = Some(Box::new(v));
        self
    }
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
    pub fn allow_paid_broadcast(mut self, v: bool) -> Self {
        self.params.allow_paid_broadcast = Some(v);
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.params.message_effect_id = Some(v.into());
        self
    }
    pub fn suggested_post_parameters(mut self, v: SuggestedPostParameters) -> Self {
        self.params.suggested_post_parameters = Some(Box::new(v));
        self
    }
    pub fn reply_parameters(mut self, v: ReplyParameters) -> Self {
        self.params.reply_parameters = Some(Box::new(v));
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

/// Fluent builder for [`Bot::send_message_draft`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#sendmessagedraft
pub struct SendMessageDraftRequest<'bot> {
    bot: &'bot Bot,
    chat_id: i64,
    draft_id: i64,
    text: String,
    params: SendMessageDraftParams,
}

impl<'bot> SendMessageDraftRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        chat_id: i64,
        draft_id: i64,
        text: impl Into<String>,
    ) -> Self {
        Self {
            bot,
            chat_id,
            draft_id,
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

    pub fn message_thread_id(mut self, v: i64) -> Self {
        self.params.message_thread_id = Some(v);
        self
    }
    pub fn parse_mode(mut self, v: impl Into<String>) -> Self {
        self.params.parse_mode = Some(v.into());
        self
    }
    pub fn entities(mut self, v: Vec<MessageEntity>) -> Self {
        self.params.entities = Some(v);
        self
    }
}

impl<'bot> IntoFuture for SendMessageDraftRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .send_message_draft_with_params(
                    self.chat_id,
                    self.draft_id,
                    self.text,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::send_paid_media`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#sendpaidmedia
pub struct SendPaidMediaRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    star_count: i64,
    media: Vec<InputPaidMedia>,
    params: SendPaidMediaParams,
}

impl<'bot> SendPaidMediaRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        chat_id: impl Into<ChatId>,
        star_count: i64,
        media: Vec<InputPaidMedia>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            star_count,
            media,
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

    /// Send silently (`disable_notification = true`).
    pub fn silent(self) -> Self {
        self.disable_notification(true)
    }

    /// Reply to a message by id.
    pub fn reply_to(mut self, message_id: i64) -> Self {
        self.params.reply_parameters = Some(make_reply_params(message_id));
        self
    }

    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: i64) -> Self {
        self.params.message_thread_id = Some(v);
        self
    }
    pub fn direct_messages_topic_id(mut self, v: i64) -> Self {
        self.params.direct_messages_topic_id = Some(v);
        self
    }
    pub fn payload(mut self, v: impl Into<String>) -> Self {
        self.params.payload = Some(v.into());
        self
    }
    pub fn caption(mut self, v: impl Into<String>) -> Self {
        self.params.caption = Some(v.into());
        self
    }
    pub fn parse_mode(mut self, v: impl Into<String>) -> Self {
        self.params.parse_mode = Some(v.into());
        self
    }
    pub fn caption_entities(mut self, v: Vec<MessageEntity>) -> Self {
        self.params.caption_entities = Some(v);
        self
    }
    pub fn show_caption_above_media(mut self, v: bool) -> Self {
        self.params.show_caption_above_media = Some(v);
        self
    }
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
    pub fn allow_paid_broadcast(mut self, v: bool) -> Self {
        self.params.allow_paid_broadcast = Some(v);
        self
    }
    pub fn suggested_post_parameters(mut self, v: SuggestedPostParameters) -> Self {
        self.params.suggested_post_parameters = Some(Box::new(v));
        self
    }
    pub fn reply_parameters(mut self, v: ReplyParameters) -> Self {
        self.params.reply_parameters = Some(Box::new(v));
        self
    }
    pub fn reply_markup(mut self, v: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for SendPaidMediaRequest<'bot> {
    type Output = Result<Message, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .send_paid_media_with_params(
                    self.chat_id,
                    self.star_count,
                    self.media,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::send_photo`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#sendphoto
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

    /// Set `parse_mode` to `"HTML"`.
    pub fn html(self) -> Self {
        self.parse_mode("HTML")
    }
    /// Set `parse_mode` to `"MarkdownV2"`.
    pub fn markdown(self) -> Self {
        self.parse_mode("MarkdownV2")
    }

    /// Send silently (`disable_notification = true`).
    pub fn silent(self) -> Self {
        self.disable_notification(true)
    }

    /// Reply to a message by id.
    pub fn reply_to(mut self, message_id: i64) -> Self {
        self.params.reply_parameters = Some(make_reply_params(message_id));
        self
    }

    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: i64) -> Self {
        self.params.message_thread_id = Some(v);
        self
    }
    pub fn direct_messages_topic_id(mut self, v: i64) -> Self {
        self.params.direct_messages_topic_id = Some(v);
        self
    }
    pub fn caption(mut self, v: impl Into<String>) -> Self {
        self.params.caption = Some(v.into());
        self
    }
    pub fn parse_mode(mut self, v: impl Into<String>) -> Self {
        self.params.parse_mode = Some(v.into());
        self
    }
    pub fn caption_entities(mut self, v: Vec<MessageEntity>) -> Self {
        self.params.caption_entities = Some(v);
        self
    }
    pub fn show_caption_above_media(mut self, v: bool) -> Self {
        self.params.show_caption_above_media = Some(v);
        self
    }
    pub fn has_spoiler(mut self, v: bool) -> Self {
        self.params.has_spoiler = Some(v);
        self
    }
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
    pub fn allow_paid_broadcast(mut self, v: bool) -> Self {
        self.params.allow_paid_broadcast = Some(v);
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.params.message_effect_id = Some(v.into());
        self
    }
    pub fn suggested_post_parameters(mut self, v: SuggestedPostParameters) -> Self {
        self.params.suggested_post_parameters = Some(Box::new(v));
        self
    }
    pub fn reply_parameters(mut self, v: ReplyParameters) -> Self {
        self.params.reply_parameters = Some(Box::new(v));
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

/// Fluent builder for [`Bot::send_poll`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#sendpoll
pub struct SendPollRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    question: String,
    options: Vec<InputPollOption>,
    params: SendPollParams,
}

impl<'bot> SendPollRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        chat_id: impl Into<ChatId>,
        question: impl Into<String>,
        options: Vec<InputPollOption>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            question: question.into(),
            options,
            params: Default::default(),
        }
    }

    /// Send silently (`disable_notification = true`).
    pub fn silent(self) -> Self {
        self.disable_notification(true)
    }

    /// Reply to a message by id.
    pub fn reply_to(mut self, message_id: i64) -> Self {
        self.params.reply_parameters = Some(make_reply_params(message_id));
        self
    }

    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: i64) -> Self {
        self.params.message_thread_id = Some(v);
        self
    }
    pub fn question_parse_mode(mut self, v: impl Into<String>) -> Self {
        self.params.question_parse_mode = Some(v.into());
        self
    }
    pub fn question_entities(mut self, v: Vec<MessageEntity>) -> Self {
        self.params.question_entities = Some(v);
        self
    }
    pub fn is_anonymous(mut self, v: bool) -> Self {
        self.params.is_anonymous = Some(v);
        self
    }
    pub fn r#type(mut self, v: impl Into<String>) -> Self {
        self.params.r#type = Some(v.into());
        self
    }
    pub fn allows_multiple_answers(mut self, v: bool) -> Self {
        self.params.allows_multiple_answers = Some(v);
        self
    }
    pub fn allows_revoting(mut self, v: bool) -> Self {
        self.params.allows_revoting = Some(v);
        self
    }
    pub fn shuffle_options(mut self, v: bool) -> Self {
        self.params.shuffle_options = Some(v);
        self
    }
    pub fn allow_adding_options(mut self, v: bool) -> Self {
        self.params.allow_adding_options = Some(v);
        self
    }
    pub fn hide_results_until_closes(mut self, v: bool) -> Self {
        self.params.hide_results_until_closes = Some(v);
        self
    }
    pub fn correct_option_ids(mut self, v: Vec<i64>) -> Self {
        self.params.correct_option_ids = Some(v);
        self
    }
    pub fn explanation(mut self, v: impl Into<String>) -> Self {
        self.params.explanation = Some(v.into());
        self
    }
    pub fn explanation_parse_mode(mut self, v: impl Into<String>) -> Self {
        self.params.explanation_parse_mode = Some(v.into());
        self
    }
    pub fn explanation_entities(mut self, v: Vec<MessageEntity>) -> Self {
        self.params.explanation_entities = Some(v);
        self
    }
    pub fn open_period(mut self, v: i64) -> Self {
        self.params.open_period = Some(v);
        self
    }
    pub fn close_date(mut self, v: i64) -> Self {
        self.params.close_date = Some(v);
        self
    }
    pub fn is_closed(mut self, v: bool) -> Self {
        self.params.is_closed = Some(v);
        self
    }
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.params.description = Some(v.into());
        self
    }
    pub fn description_parse_mode(mut self, v: impl Into<String>) -> Self {
        self.params.description_parse_mode = Some(v.into());
        self
    }
    pub fn description_entities(mut self, v: Vec<MessageEntity>) -> Self {
        self.params.description_entities = Some(v);
        self
    }
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
    pub fn allow_paid_broadcast(mut self, v: bool) -> Self {
        self.params.allow_paid_broadcast = Some(v);
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.params.message_effect_id = Some(v.into());
        self
    }
    pub fn reply_parameters(mut self, v: ReplyParameters) -> Self {
        self.params.reply_parameters = Some(Box::new(v));
        self
    }
    pub fn reply_markup(mut self, v: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for SendPollRequest<'bot> {
    type Output = Result<Message, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .send_poll_with_params(self.chat_id, self.question, self.options, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::send_sticker`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#sendsticker
pub struct SendStickerRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    sticker: InputFileOrString,
    params: SendStickerParams,
}

impl<'bot> SendStickerRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        chat_id: impl Into<ChatId>,
        sticker: impl Into<InputFileOrString>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            sticker: sticker.into(),
            params: Default::default(),
        }
    }

    /// Send silently (`disable_notification = true`).
    pub fn silent(self) -> Self {
        self.disable_notification(true)
    }

    /// Reply to a message by id.
    pub fn reply_to(mut self, message_id: i64) -> Self {
        self.params.reply_parameters = Some(make_reply_params(message_id));
        self
    }

    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: i64) -> Self {
        self.params.message_thread_id = Some(v);
        self
    }
    pub fn direct_messages_topic_id(mut self, v: i64) -> Self {
        self.params.direct_messages_topic_id = Some(v);
        self
    }
    pub fn emoji(mut self, v: impl Into<String>) -> Self {
        self.params.emoji = Some(v.into());
        self
    }
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
    pub fn allow_paid_broadcast(mut self, v: bool) -> Self {
        self.params.allow_paid_broadcast = Some(v);
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.params.message_effect_id = Some(v.into());
        self
    }
    pub fn suggested_post_parameters(mut self, v: SuggestedPostParameters) -> Self {
        self.params.suggested_post_parameters = Some(Box::new(v));
        self
    }
    pub fn reply_parameters(mut self, v: ReplyParameters) -> Self {
        self.params.reply_parameters = Some(Box::new(v));
        self
    }
    pub fn reply_markup(mut self, v: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for SendStickerRequest<'bot> {
    type Output = Result<Message, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .send_sticker_with_params(self.chat_id, self.sticker, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::send_venue`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#sendvenue
pub struct SendVenueRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    latitude: f64,
    longitude: f64,
    title: String,
    address: String,
    params: SendVenueParams,
}

impl<'bot> SendVenueRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        chat_id: impl Into<ChatId>,
        latitude: f64,
        longitude: f64,
        title: impl Into<String>,
        address: impl Into<String>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            latitude,
            longitude,
            title: title.into(),
            address: address.into(),
            params: Default::default(),
        }
    }

    /// Send silently (`disable_notification = true`).
    pub fn silent(self) -> Self {
        self.disable_notification(true)
    }

    /// Reply to a message by id.
    pub fn reply_to(mut self, message_id: i64) -> Self {
        self.params.reply_parameters = Some(make_reply_params(message_id));
        self
    }

    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: i64) -> Self {
        self.params.message_thread_id = Some(v);
        self
    }
    pub fn direct_messages_topic_id(mut self, v: i64) -> Self {
        self.params.direct_messages_topic_id = Some(v);
        self
    }
    pub fn foursquare_id(mut self, v: impl Into<String>) -> Self {
        self.params.foursquare_id = Some(v.into());
        self
    }
    pub fn foursquare_type(mut self, v: impl Into<String>) -> Self {
        self.params.foursquare_type = Some(v.into());
        self
    }
    pub fn google_place_id(mut self, v: impl Into<String>) -> Self {
        self.params.google_place_id = Some(v.into());
        self
    }
    pub fn google_place_type(mut self, v: impl Into<String>) -> Self {
        self.params.google_place_type = Some(v.into());
        self
    }
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
    pub fn allow_paid_broadcast(mut self, v: bool) -> Self {
        self.params.allow_paid_broadcast = Some(v);
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.params.message_effect_id = Some(v.into());
        self
    }
    pub fn suggested_post_parameters(mut self, v: SuggestedPostParameters) -> Self {
        self.params.suggested_post_parameters = Some(Box::new(v));
        self
    }
    pub fn reply_parameters(mut self, v: ReplyParameters) -> Self {
        self.params.reply_parameters = Some(Box::new(v));
        self
    }
    pub fn reply_markup(mut self, v: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for SendVenueRequest<'bot> {
    type Output = Result<Message, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .send_venue_with_params(
                    self.chat_id,
                    self.latitude,
                    self.longitude,
                    self.title,
                    self.address,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::send_video`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#sendvideo
pub struct SendVideoRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    video: InputFileOrString,
    params: SendVideoParams,
}

impl<'bot> SendVideoRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        chat_id: impl Into<ChatId>,
        video: impl Into<InputFileOrString>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            video: video.into(),
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

    /// Send silently (`disable_notification = true`).
    pub fn silent(self) -> Self {
        self.disable_notification(true)
    }

    /// Reply to a message by id.
    pub fn reply_to(mut self, message_id: i64) -> Self {
        self.params.reply_parameters = Some(make_reply_params(message_id));
        self
    }

    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: i64) -> Self {
        self.params.message_thread_id = Some(v);
        self
    }
    pub fn direct_messages_topic_id(mut self, v: i64) -> Self {
        self.params.direct_messages_topic_id = Some(v);
        self
    }
    pub fn duration(mut self, v: i64) -> Self {
        self.params.duration = Some(v);
        self
    }
    pub fn width(mut self, v: i64) -> Self {
        self.params.width = Some(v);
        self
    }
    pub fn height(mut self, v: i64) -> Self {
        self.params.height = Some(v);
        self
    }
    pub fn thumbnail(mut self, v: impl Into<InputFileOrString>) -> Self {
        self.params.thumbnail = Some(v.into());
        self
    }
    pub fn cover(mut self, v: impl Into<InputFileOrString>) -> Self {
        self.params.cover = Some(v.into());
        self
    }
    pub fn start_timestamp(mut self, v: i64) -> Self {
        self.params.start_timestamp = Some(v);
        self
    }
    pub fn caption(mut self, v: impl Into<String>) -> Self {
        self.params.caption = Some(v.into());
        self
    }
    pub fn parse_mode(mut self, v: impl Into<String>) -> Self {
        self.params.parse_mode = Some(v.into());
        self
    }
    pub fn caption_entities(mut self, v: Vec<MessageEntity>) -> Self {
        self.params.caption_entities = Some(v);
        self
    }
    pub fn show_caption_above_media(mut self, v: bool) -> Self {
        self.params.show_caption_above_media = Some(v);
        self
    }
    pub fn has_spoiler(mut self, v: bool) -> Self {
        self.params.has_spoiler = Some(v);
        self
    }
    pub fn supports_streaming(mut self, v: bool) -> Self {
        self.params.supports_streaming = Some(v);
        self
    }
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
    pub fn allow_paid_broadcast(mut self, v: bool) -> Self {
        self.params.allow_paid_broadcast = Some(v);
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.params.message_effect_id = Some(v.into());
        self
    }
    pub fn suggested_post_parameters(mut self, v: SuggestedPostParameters) -> Self {
        self.params.suggested_post_parameters = Some(Box::new(v));
        self
    }
    pub fn reply_parameters(mut self, v: ReplyParameters) -> Self {
        self.params.reply_parameters = Some(Box::new(v));
        self
    }
    pub fn reply_markup(mut self, v: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for SendVideoRequest<'bot> {
    type Output = Result<Message, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .send_video_with_params(self.chat_id, self.video, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::send_video_note`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#sendvideonote
pub struct SendVideoNoteRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    video_note: InputFileOrString,
    params: SendVideoNoteParams,
}

impl<'bot> SendVideoNoteRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        chat_id: impl Into<ChatId>,
        video_note: impl Into<InputFileOrString>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            video_note: video_note.into(),
            params: Default::default(),
        }
    }

    /// Send silently (`disable_notification = true`).
    pub fn silent(self) -> Self {
        self.disable_notification(true)
    }

    /// Reply to a message by id.
    pub fn reply_to(mut self, message_id: i64) -> Self {
        self.params.reply_parameters = Some(make_reply_params(message_id));
        self
    }

    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: i64) -> Self {
        self.params.message_thread_id = Some(v);
        self
    }
    pub fn direct_messages_topic_id(mut self, v: i64) -> Self {
        self.params.direct_messages_topic_id = Some(v);
        self
    }
    pub fn duration(mut self, v: i64) -> Self {
        self.params.duration = Some(v);
        self
    }
    pub fn length(mut self, v: i64) -> Self {
        self.params.length = Some(v);
        self
    }
    pub fn thumbnail(mut self, v: impl Into<InputFileOrString>) -> Self {
        self.params.thumbnail = Some(v.into());
        self
    }
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
    pub fn allow_paid_broadcast(mut self, v: bool) -> Self {
        self.params.allow_paid_broadcast = Some(v);
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.params.message_effect_id = Some(v.into());
        self
    }
    pub fn suggested_post_parameters(mut self, v: SuggestedPostParameters) -> Self {
        self.params.suggested_post_parameters = Some(Box::new(v));
        self
    }
    pub fn reply_parameters(mut self, v: ReplyParameters) -> Self {
        self.params.reply_parameters = Some(Box::new(v));
        self
    }
    pub fn reply_markup(mut self, v: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for SendVideoNoteRequest<'bot> {
    type Output = Result<Message, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .send_video_note_with_params(self.chat_id, self.video_note, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::send_voice`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#sendvoice
pub struct SendVoiceRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    voice: InputFileOrString,
    params: SendVoiceParams,
}

impl<'bot> SendVoiceRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        chat_id: impl Into<ChatId>,
        voice: impl Into<InputFileOrString>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            voice: voice.into(),
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

    /// Send silently (`disable_notification = true`).
    pub fn silent(self) -> Self {
        self.disable_notification(true)
    }

    /// Reply to a message by id.
    pub fn reply_to(mut self, message_id: i64) -> Self {
        self.params.reply_parameters = Some(make_reply_params(message_id));
        self
    }

    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(v.into());
        self
    }
    pub fn message_thread_id(mut self, v: i64) -> Self {
        self.params.message_thread_id = Some(v);
        self
    }
    pub fn direct_messages_topic_id(mut self, v: i64) -> Self {
        self.params.direct_messages_topic_id = Some(v);
        self
    }
    pub fn caption(mut self, v: impl Into<String>) -> Self {
        self.params.caption = Some(v.into());
        self
    }
    pub fn parse_mode(mut self, v: impl Into<String>) -> Self {
        self.params.parse_mode = Some(v.into());
        self
    }
    pub fn caption_entities(mut self, v: Vec<MessageEntity>) -> Self {
        self.params.caption_entities = Some(v);
        self
    }
    pub fn duration(mut self, v: i64) -> Self {
        self.params.duration = Some(v);
        self
    }
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
    pub fn allow_paid_broadcast(mut self, v: bool) -> Self {
        self.params.allow_paid_broadcast = Some(v);
        self
    }
    pub fn message_effect_id(mut self, v: impl Into<String>) -> Self {
        self.params.message_effect_id = Some(v.into());
        self
    }
    pub fn suggested_post_parameters(mut self, v: SuggestedPostParameters) -> Self {
        self.params.suggested_post_parameters = Some(Box::new(v));
        self
    }
    pub fn reply_parameters(mut self, v: ReplyParameters) -> Self {
        self.params.reply_parameters = Some(Box::new(v));
        self
    }
    pub fn reply_markup(mut self, v: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for SendVoiceRequest<'bot> {
    type Output = Result<Message, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .send_voice_with_params(self.chat_id, self.voice, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::set_business_account_bio`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#setbusinessaccountbio
pub struct SetBusinessAccountBioRequest<'bot> {
    bot: &'bot Bot,
    business_connection_id: String,
    params: SetBusinessAccountBioParams,
}

impl<'bot> SetBusinessAccountBioRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, business_connection_id: impl Into<String>) -> Self {
        Self {
            bot,
            business_connection_id: business_connection_id.into(),
            params: Default::default(),
        }
    }

    pub fn bio(mut self, v: impl Into<String>) -> Self {
        self.params.bio = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for SetBusinessAccountBioRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .set_business_account_bio_with_params(
                    self.business_connection_id,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::set_business_account_gift_settings`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#setbusinessaccountgiftsettings
pub struct SetBusinessAccountGiftSettingsRequest<'bot> {
    bot: &'bot Bot,
    business_connection_id: String,
    show_gift_button: bool,
    accepted_gift_types: AcceptedGiftTypes,
}

impl<'bot> SetBusinessAccountGiftSettingsRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        business_connection_id: impl Into<String>,
        show_gift_button: bool,
        accepted_gift_types: AcceptedGiftTypes,
    ) -> Self {
        Self {
            bot,
            business_connection_id: business_connection_id.into(),
            show_gift_button,
            accepted_gift_types,
        }
    }
}

impl<'bot> IntoFuture for SetBusinessAccountGiftSettingsRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_set_business_account_gift_settings(
                    self.business_connection_id,
                    self.show_gift_button,
                    self.accepted_gift_types,
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::set_business_account_name`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#setbusinessaccountname
pub struct SetBusinessAccountNameRequest<'bot> {
    bot: &'bot Bot,
    business_connection_id: String,
    first_name: String,
    params: SetBusinessAccountNameParams,
}

impl<'bot> SetBusinessAccountNameRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        business_connection_id: impl Into<String>,
        first_name: impl Into<String>,
    ) -> Self {
        Self {
            bot,
            business_connection_id: business_connection_id.into(),
            first_name: first_name.into(),
            params: Default::default(),
        }
    }

    pub fn last_name(mut self, v: impl Into<String>) -> Self {
        self.params.last_name = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for SetBusinessAccountNameRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .set_business_account_name_with_params(
                    self.business_connection_id,
                    self.first_name,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::set_business_account_profile_photo`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#setbusinessaccountprofilephoto
pub struct SetBusinessAccountProfilePhotoRequest<'bot> {
    bot: &'bot Bot,
    business_connection_id: String,
    photo: InputProfilePhoto,
    params: SetBusinessAccountProfilePhotoParams,
}

impl<'bot> SetBusinessAccountProfilePhotoRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        business_connection_id: impl Into<String>,
        photo: InputProfilePhoto,
    ) -> Self {
        Self {
            bot,
            business_connection_id: business_connection_id.into(),
            photo,
            params: Default::default(),
        }
    }

    pub fn is_public(mut self, v: bool) -> Self {
        self.params.is_public = Some(v);
        self
    }
}

impl<'bot> IntoFuture for SetBusinessAccountProfilePhotoRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .set_business_account_profile_photo_with_params(
                    self.business_connection_id,
                    self.photo,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::set_business_account_username`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#setbusinessaccountusername
pub struct SetBusinessAccountUsernameRequest<'bot> {
    bot: &'bot Bot,
    business_connection_id: String,
    params: SetBusinessAccountUsernameParams,
}

impl<'bot> SetBusinessAccountUsernameRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, business_connection_id: impl Into<String>) -> Self {
        Self {
            bot,
            business_connection_id: business_connection_id.into(),
            params: Default::default(),
        }
    }

    pub fn username(mut self, v: impl Into<String>) -> Self {
        self.params.username = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for SetBusinessAccountUsernameRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .set_business_account_username_with_params(
                    self.business_connection_id,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::set_chat_administrator_custom_title`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#setchatadministratorcustomtitle
pub struct SetChatAdministratorCustomTitleRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    user_id: i64,
    custom_title: String,
}

impl<'bot> SetChatAdministratorCustomTitleRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        chat_id: impl Into<ChatId>,
        user_id: i64,
        custom_title: impl Into<String>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            user_id,
            custom_title: custom_title.into(),
        }
    }
}

impl<'bot> IntoFuture for SetChatAdministratorCustomTitleRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_set_chat_administrator_custom_title(
                    self.chat_id,
                    self.user_id,
                    self.custom_title,
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::set_chat_description`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#setchatdescription
pub struct SetChatDescriptionRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    params: SetChatDescriptionParams,
}

impl<'bot> SetChatDescriptionRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            params: Default::default(),
        }
    }

    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.params.description = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for SetChatDescriptionRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .set_chat_description_with_params(self.chat_id, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::set_chat_member_tag`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#setchatmembertag
pub struct SetChatMemberTagRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    user_id: i64,
    params: SetChatMemberTagParams,
}

impl<'bot> SetChatMemberTagRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>, user_id: i64) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            user_id,
            params: Default::default(),
        }
    }

    pub fn tag(mut self, v: impl Into<String>) -> Self {
        self.params.tag = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for SetChatMemberTagRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .set_chat_member_tag_with_params(self.chat_id, self.user_id, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::set_chat_menu_button`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#setchatmenubutton
pub struct SetChatMenuButtonRequest<'bot> {
    bot: &'bot Bot,
    params: SetChatMenuButtonParams,
}

impl<'bot> SetChatMenuButtonRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot) -> Self {
        Self {
            bot,
            params: Default::default(),
        }
    }

    pub fn chat_id(mut self, v: i64) -> Self {
        self.params.chat_id = Some(v);
        self
    }
    pub fn menu_button(mut self, v: MenuButton) -> Self {
        self.params.menu_button = Some(Box::new(v));
        self
    }
}

impl<'bot> IntoFuture for SetChatMenuButtonRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .set_chat_menu_button_with_params(Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::set_chat_permissions`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#setchatpermissions
pub struct SetChatPermissionsRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    permissions: ChatPermissions,
    params: SetChatPermissionsParams,
}

impl<'bot> SetChatPermissionsRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        chat_id: impl Into<ChatId>,
        permissions: ChatPermissions,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            permissions,
            params: Default::default(),
        }
    }

    pub fn use_independent_chat_permissions(mut self, v: bool) -> Self {
        self.params.use_independent_chat_permissions = Some(v);
        self
    }
}

impl<'bot> IntoFuture for SetChatPermissionsRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .set_chat_permissions_with_params(self.chat_id, self.permissions, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::set_chat_photo`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#setchatphoto
pub struct SetChatPhotoRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    photo: InputFile,
}

impl<'bot> SetChatPhotoRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>, photo: InputFile) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            photo,
        }
    }
}

impl<'bot> IntoFuture for SetChatPhotoRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.raw_set_chat_photo(self.chat_id, self.photo).await })
    }
}

/// Fluent builder for [`Bot::set_chat_sticker_set`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#setchatstickerset
pub struct SetChatStickerSetRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    sticker_set_name: String,
}

impl<'bot> SetChatStickerSetRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        chat_id: impl Into<ChatId>,
        sticker_set_name: impl Into<String>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            sticker_set_name: sticker_set_name.into(),
        }
    }
}

impl<'bot> IntoFuture for SetChatStickerSetRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_set_chat_sticker_set(self.chat_id, self.sticker_set_name)
                .await
        })
    }
}

/// Fluent builder for [`Bot::set_chat_title`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#setchattitle
pub struct SetChatTitleRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    title: String,
}

impl<'bot> SetChatTitleRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        chat_id: impl Into<ChatId>,
        title: impl Into<String>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            title: title.into(),
        }
    }
}

impl<'bot> IntoFuture for SetChatTitleRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.raw_set_chat_title(self.chat_id, self.title).await })
    }
}

/// Fluent builder for [`Bot::set_custom_emoji_sticker_set_thumbnail`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#setcustomemojistickersetthumbnail
pub struct SetCustomEmojiStickerSetThumbnailRequest<'bot> {
    bot: &'bot Bot,
    name: String,
    params: SetCustomEmojiStickerSetThumbnailParams,
}

impl<'bot> SetCustomEmojiStickerSetThumbnailRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, name: impl Into<String>) -> Self {
        Self {
            bot,
            name: name.into(),
            params: Default::default(),
        }
    }

    pub fn custom_emoji_id(mut self, v: impl Into<String>) -> Self {
        self.params.custom_emoji_id = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for SetCustomEmojiStickerSetThumbnailRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .set_custom_emoji_sticker_set_thumbnail_with_params(self.name, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::set_game_score`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#setgamescore
pub struct SetGameScoreRequest<'bot> {
    bot: &'bot Bot,
    user_id: i64,
    score: i64,
    params: SetGameScoreParams,
}

impl<'bot> SetGameScoreRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, user_id: i64, score: i64) -> Self {
        Self {
            bot,
            user_id,
            score,
            params: Default::default(),
        }
    }

    pub fn force(mut self, v: bool) -> Self {
        self.params.force = Some(v);
        self
    }
    pub fn disable_edit_message(mut self, v: bool) -> Self {
        self.params.disable_edit_message = Some(v);
        self
    }
    pub fn chat_id(mut self, v: i64) -> Self {
        self.params.chat_id = Some(v);
        self
    }
    pub fn message_id(mut self, v: i64) -> Self {
        self.params.message_id = Some(v);
        self
    }
    pub fn inline_message_id(mut self, v: impl Into<String>) -> Self {
        self.params.inline_message_id = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for SetGameScoreRequest<'bot> {
    type Output = Result<serde_json::Value, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .set_game_score_with_params(self.user_id, self.score, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::set_message_reaction`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#setmessagereaction
pub struct SetMessageReactionRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    message_id: i64,
    params: SetMessageReactionParams,
}

impl<'bot> SetMessageReactionRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>, message_id: i64) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            message_id,
            params: Default::default(),
        }
    }

    pub fn reaction(mut self, v: Vec<ReactionType>) -> Self {
        self.params.reaction = Some(v);
        self
    }
    pub fn is_big(mut self, v: bool) -> Self {
        self.params.is_big = Some(v);
        self
    }
}

impl<'bot> IntoFuture for SetMessageReactionRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .set_message_reaction_with_params(self.chat_id, self.message_id, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::set_my_commands`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#setmycommands
pub struct SetMyCommandsRequest<'bot> {
    bot: &'bot Bot,
    commands: Vec<BotCommand>,
    params: SetMyCommandsParams,
}

impl<'bot> SetMyCommandsRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, commands: Vec<BotCommand>) -> Self {
        Self {
            bot,
            commands,
            params: Default::default(),
        }
    }

    pub fn scope(mut self, v: BotCommandScope) -> Self {
        self.params.scope = Some(Box::new(v));
        self
    }
    pub fn language_code(mut self, v: impl Into<String>) -> Self {
        self.params.language_code = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for SetMyCommandsRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .set_my_commands_with_params(self.commands, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::set_my_default_administrator_rights`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#setmydefaultadministratorrights
pub struct SetMyDefaultAdministratorRightsRequest<'bot> {
    bot: &'bot Bot,
    params: SetMyDefaultAdministratorRightsParams,
}

impl<'bot> SetMyDefaultAdministratorRightsRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot) -> Self {
        Self {
            bot,
            params: Default::default(),
        }
    }

    pub fn rights(mut self, v: ChatAdministratorRights) -> Self {
        self.params.rights = Some(Box::new(v));
        self
    }
    pub fn for_channels(mut self, v: bool) -> Self {
        self.params.for_channels = Some(v);
        self
    }
}

impl<'bot> IntoFuture for SetMyDefaultAdministratorRightsRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .set_my_default_administrator_rights_with_params(Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::set_my_description`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#setmydescription
pub struct SetMyDescriptionRequest<'bot> {
    bot: &'bot Bot,
    params: SetMyDescriptionParams,
}

impl<'bot> SetMyDescriptionRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot) -> Self {
        Self {
            bot,
            params: Default::default(),
        }
    }

    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.params.description = Some(v.into());
        self
    }
    pub fn language_code(mut self, v: impl Into<String>) -> Self {
        self.params.language_code = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for SetMyDescriptionRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .set_my_description_with_params(Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::set_my_name`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#setmyname
pub struct SetMyNameRequest<'bot> {
    bot: &'bot Bot,
    params: SetMyNameParams,
}

impl<'bot> SetMyNameRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot) -> Self {
        Self {
            bot,
            params: Default::default(),
        }
    }

    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.params.name = Some(v.into());
        self
    }
    pub fn language_code(mut self, v: impl Into<String>) -> Self {
        self.params.language_code = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for SetMyNameRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.set_my_name_with_params(Some(self.params)).await })
    }
}

/// Fluent builder for [`Bot::set_my_profile_photo`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#setmyprofilephoto
pub struct SetMyProfilePhotoRequest<'bot> {
    bot: &'bot Bot,
    photo: InputProfilePhoto,
}

impl<'bot> SetMyProfilePhotoRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, photo: InputProfilePhoto) -> Self {
        Self { bot, photo }
    }
}

impl<'bot> IntoFuture for SetMyProfilePhotoRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.raw_set_my_profile_photo(self.photo).await })
    }
}

/// Fluent builder for [`Bot::set_my_short_description`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#setmyshortdescription
pub struct SetMyShortDescriptionRequest<'bot> {
    bot: &'bot Bot,
    params: SetMyShortDescriptionParams,
}

impl<'bot> SetMyShortDescriptionRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot) -> Self {
        Self {
            bot,
            params: Default::default(),
        }
    }

    pub fn short_description(mut self, v: impl Into<String>) -> Self {
        self.params.short_description = Some(v.into());
        self
    }
    pub fn language_code(mut self, v: impl Into<String>) -> Self {
        self.params.language_code = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for SetMyShortDescriptionRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .set_my_short_description_with_params(Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::set_passport_data_errors`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#setpassportdataerrors
pub struct SetPassportDataErrorsRequest<'bot> {
    bot: &'bot Bot,
    user_id: i64,
    errors: Vec<PassportElementError>,
}

impl<'bot> SetPassportDataErrorsRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, user_id: i64, errors: Vec<PassportElementError>) -> Self {
        Self {
            bot,
            user_id,
            errors,
        }
    }
}

impl<'bot> IntoFuture for SetPassportDataErrorsRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_set_passport_data_errors(self.user_id, self.errors)
                .await
        })
    }
}

/// Fluent builder for [`Bot::set_sticker_emoji_list`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#setstickeremojilist
pub struct SetStickerEmojiListRequest<'bot> {
    bot: &'bot Bot,
    sticker: String,
    emoji_list: Vec<String>,
}

impl<'bot> SetStickerEmojiListRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, sticker: impl Into<String>, emoji_list: Vec<String>) -> Self {
        Self {
            bot,
            sticker: sticker.into(),
            emoji_list,
        }
    }
}

impl<'bot> IntoFuture for SetStickerEmojiListRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_set_sticker_emoji_list(self.sticker, self.emoji_list)
                .await
        })
    }
}

/// Fluent builder for [`Bot::set_sticker_keywords`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#setstickerkeywords
pub struct SetStickerKeywordsRequest<'bot> {
    bot: &'bot Bot,
    sticker: String,
    params: SetStickerKeywordsParams,
}

impl<'bot> SetStickerKeywordsRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, sticker: impl Into<String>) -> Self {
        Self {
            bot,
            sticker: sticker.into(),
            params: Default::default(),
        }
    }

    pub fn keywords(mut self, v: Vec<String>) -> Self {
        self.params.keywords = Some(v);
        self
    }
}

impl<'bot> IntoFuture for SetStickerKeywordsRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .set_sticker_keywords_with_params(self.sticker, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::set_sticker_mask_position`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#setstickermaskposition
pub struct SetStickerMaskPositionRequest<'bot> {
    bot: &'bot Bot,
    sticker: String,
    params: SetStickerMaskPositionParams,
}

impl<'bot> SetStickerMaskPositionRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, sticker: impl Into<String>) -> Self {
        Self {
            bot,
            sticker: sticker.into(),
            params: Default::default(),
        }
    }

    pub fn mask_position(mut self, v: MaskPosition) -> Self {
        self.params.mask_position = Some(Box::new(v));
        self
    }
}

impl<'bot> IntoFuture for SetStickerMaskPositionRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .set_sticker_mask_position_with_params(self.sticker, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::set_sticker_position_in_set`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#setstickerpositioninset
pub struct SetStickerPositionInSetRequest<'bot> {
    bot: &'bot Bot,
    sticker: String,
    position: i64,
}

impl<'bot> SetStickerPositionInSetRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, sticker: impl Into<String>, position: i64) -> Self {
        Self {
            bot,
            sticker: sticker.into(),
            position,
        }
    }
}

impl<'bot> IntoFuture for SetStickerPositionInSetRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_set_sticker_position_in_set(self.sticker, self.position)
                .await
        })
    }
}

/// Fluent builder for [`Bot::set_sticker_set_thumbnail`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#setstickersetthumbnail
pub struct SetStickerSetThumbnailRequest<'bot> {
    bot: &'bot Bot,
    name: String,
    user_id: i64,
    format: String,
    params: SetStickerSetThumbnailParams,
}

impl<'bot> SetStickerSetThumbnailRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        name: impl Into<String>,
        user_id: i64,
        format: impl Into<String>,
    ) -> Self {
        Self {
            bot,
            name: name.into(),
            user_id,
            format: format.into(),
            params: Default::default(),
        }
    }

    pub fn thumbnail(mut self, v: impl Into<InputFileOrString>) -> Self {
        self.params.thumbnail = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for SetStickerSetThumbnailRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .set_sticker_set_thumbnail_with_params(
                    self.name,
                    self.user_id,
                    self.format,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::set_sticker_set_title`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#setstickersettitle
pub struct SetStickerSetTitleRequest<'bot> {
    bot: &'bot Bot,
    name: String,
    title: String,
}

impl<'bot> SetStickerSetTitleRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, name: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            bot,
            name: name.into(),
            title: title.into(),
        }
    }
}

impl<'bot> IntoFuture for SetStickerSetTitleRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_set_sticker_set_title(self.name, self.title)
                .await
        })
    }
}

/// Fluent builder for [`Bot::set_user_emoji_status`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#setuseremojistatus
pub struct SetUserEmojiStatusRequest<'bot> {
    bot: &'bot Bot,
    user_id: i64,
    params: SetUserEmojiStatusParams,
}

impl<'bot> SetUserEmojiStatusRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, user_id: i64) -> Self {
        Self {
            bot,
            user_id,
            params: Default::default(),
        }
    }

    pub fn emoji_status_custom_emoji_id(mut self, v: impl Into<String>) -> Self {
        self.params.emoji_status_custom_emoji_id = Some(v.into());
        self
    }
    pub fn emoji_status_expiration_date(mut self, v: i64) -> Self {
        self.params.emoji_status_expiration_date = Some(v);
        self
    }
}

impl<'bot> IntoFuture for SetUserEmojiStatusRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .set_user_emoji_status_with_params(self.user_id, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::set_webhook`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#setwebhook
pub struct SetWebhookRequest<'bot> {
    bot: &'bot Bot,
    url: String,
    params: SetWebhookParams,
}

impl<'bot> SetWebhookRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, url: impl Into<String>) -> Self {
        Self {
            bot,
            url: url.into(),
            params: Default::default(),
        }
    }

    pub fn certificate(mut self, v: InputFile) -> Self {
        self.params.certificate = Some(v);
        self
    }
    pub fn ip_address(mut self, v: impl Into<String>) -> Self {
        self.params.ip_address = Some(v.into());
        self
    }
    pub fn max_connections(mut self, v: i64) -> Self {
        self.params.max_connections = Some(v);
        self
    }
    pub fn allowed_updates(mut self, v: Vec<String>) -> Self {
        self.params.allowed_updates = Some(v);
        self
    }
    pub fn drop_pending_updates(mut self, v: bool) -> Self {
        self.params.drop_pending_updates = Some(v);
        self
    }
    pub fn secret_token(mut self, v: impl Into<String>) -> Self {
        self.params.secret_token = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for SetWebhookRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .set_webhook_with_params(self.url, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::stop_message_live_location`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#stopmessagelivelocation
pub struct StopMessageLiveLocationRequest<'bot> {
    bot: &'bot Bot,
    params: StopMessageLiveLocationParams,
}

impl<'bot> StopMessageLiveLocationRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot) -> Self {
        Self {
            bot,
            params: Default::default(),
        }
    }

    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(v.into());
        self
    }
    pub fn chat_id(mut self, v: impl Into<ChatId>) -> Self {
        self.params.chat_id = Some(v.into());
        self
    }
    pub fn message_id(mut self, v: i64) -> Self {
        self.params.message_id = Some(v);
        self
    }
    pub fn inline_message_id(mut self, v: impl Into<String>) -> Self {
        self.params.inline_message_id = Some(v.into());
        self
    }
    pub fn reply_markup(mut self, v: InlineKeyboardMarkup) -> Self {
        self.params.reply_markup = Some(Box::new(v));
        self
    }
}

impl<'bot> IntoFuture for StopMessageLiveLocationRequest<'bot> {
    type Output = Result<serde_json::Value, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .stop_message_live_location_with_params(Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::stop_poll`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#stoppoll
pub struct StopPollRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    message_id: i64,
    params: StopPollParams,
}

impl<'bot> StopPollRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>, message_id: i64) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            message_id,
            params: Default::default(),
        }
    }

    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(v.into());
        self
    }
    pub fn reply_markup(mut self, v: InlineKeyboardMarkup) -> Self {
        self.params.reply_markup = Some(Box::new(v));
        self
    }
}

impl<'bot> IntoFuture for StopPollRequest<'bot> {
    type Output = Result<Poll, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .stop_poll_with_params(self.chat_id, self.message_id, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::transfer_business_account_stars`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#transferbusinessaccountstars
pub struct TransferBusinessAccountStarsRequest<'bot> {
    bot: &'bot Bot,
    business_connection_id: String,
    star_count: i64,
}

impl<'bot> TransferBusinessAccountStarsRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        business_connection_id: impl Into<String>,
        star_count: i64,
    ) -> Self {
        Self {
            bot,
            business_connection_id: business_connection_id.into(),
            star_count,
        }
    }
}

impl<'bot> IntoFuture for TransferBusinessAccountStarsRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_transfer_business_account_stars(self.business_connection_id, self.star_count)
                .await
        })
    }
}

/// Fluent builder for [`Bot::transfer_gift`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#transfergift
pub struct TransferGiftRequest<'bot> {
    bot: &'bot Bot,
    business_connection_id: String,
    owned_gift_id: String,
    new_owner_chat_id: i64,
    params: TransferGiftParams,
}

impl<'bot> TransferGiftRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        business_connection_id: impl Into<String>,
        owned_gift_id: impl Into<String>,
        new_owner_chat_id: i64,
    ) -> Self {
        Self {
            bot,
            business_connection_id: business_connection_id.into(),
            owned_gift_id: owned_gift_id.into(),
            new_owner_chat_id,
            params: Default::default(),
        }
    }

    pub fn star_count(mut self, v: i64) -> Self {
        self.params.star_count = Some(v);
        self
    }
}

impl<'bot> IntoFuture for TransferGiftRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .transfer_gift_with_params(
                    self.business_connection_id,
                    self.owned_gift_id,
                    self.new_owner_chat_id,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::unban_chat_member`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#unbanchatmember
pub struct UnbanChatMemberRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    user_id: i64,
    params: UnbanChatMemberParams,
}

impl<'bot> UnbanChatMemberRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>, user_id: i64) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            user_id,
            params: Default::default(),
        }
    }

    pub fn only_if_banned(mut self, v: bool) -> Self {
        self.params.only_if_banned = Some(v);
        self
    }
}

impl<'bot> IntoFuture for UnbanChatMemberRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .unban_chat_member_with_params(self.chat_id, self.user_id, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::unban_chat_sender_chat`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#unbanchatsenderchat
pub struct UnbanChatSenderChatRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    sender_chat_id: i64,
}

impl<'bot> UnbanChatSenderChatRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>, sender_chat_id: i64) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            sender_chat_id,
        }
    }
}

impl<'bot> IntoFuture for UnbanChatSenderChatRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_unban_chat_sender_chat(self.chat_id, self.sender_chat_id)
                .await
        })
    }
}

/// Fluent builder for [`Bot::unhide_general_forum_topic`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#unhidegeneralforumtopic
pub struct UnhideGeneralForumTopicRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
}

impl<'bot> UnhideGeneralForumTopicRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
        }
    }
}

impl<'bot> IntoFuture for UnhideGeneralForumTopicRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.raw_unhide_general_forum_topic(self.chat_id).await })
    }
}

/// Fluent builder for [`Bot::unpin_all_chat_messages`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#unpinallchatmessages
pub struct UnpinAllChatMessagesRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
}

impl<'bot> UnpinAllChatMessagesRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
        }
    }
}

impl<'bot> IntoFuture for UnpinAllChatMessagesRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.bot.raw_unpin_all_chat_messages(self.chat_id).await })
    }
}

/// Fluent builder for [`Bot::unpin_all_forum_topic_messages`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#unpinallforumtopicmessages
pub struct UnpinAllForumTopicMessagesRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    message_thread_id: i64,
}

impl<'bot> UnpinAllForumTopicMessagesRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>, message_thread_id: i64) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            message_thread_id,
        }
    }
}

impl<'bot> IntoFuture for UnpinAllForumTopicMessagesRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_unpin_all_forum_topic_messages(self.chat_id, self.message_thread_id)
                .await
        })
    }
}

/// Fluent builder for [`Bot::unpin_all_general_forum_topic_messages`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#unpinallgeneralforumtopicmessages
pub struct UnpinAllGeneralForumTopicMessagesRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
}

impl<'bot> UnpinAllGeneralForumTopicMessagesRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
        }
    }
}

impl<'bot> IntoFuture for UnpinAllGeneralForumTopicMessagesRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_unpin_all_general_forum_topic_messages(self.chat_id)
                .await
        })
    }
}

/// Fluent builder for [`Bot::unpin_chat_message`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#unpinchatmessage
pub struct UnpinChatMessageRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    params: UnpinChatMessageParams,
}

impl<'bot> UnpinChatMessageRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            params: Default::default(),
        }
    }

    pub fn business_connection_id(mut self, v: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(v.into());
        self
    }
    pub fn message_id(mut self, v: i64) -> Self {
        self.params.message_id = Some(v);
        self
    }
}

impl<'bot> IntoFuture for UnpinChatMessageRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .unpin_chat_message_with_params(self.chat_id, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::upgrade_gift`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#upgradegift
pub struct UpgradeGiftRequest<'bot> {
    bot: &'bot Bot,
    business_connection_id: String,
    owned_gift_id: String,
    params: UpgradeGiftParams,
}

impl<'bot> UpgradeGiftRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        business_connection_id: impl Into<String>,
        owned_gift_id: impl Into<String>,
    ) -> Self {
        Self {
            bot,
            business_connection_id: business_connection_id.into(),
            owned_gift_id: owned_gift_id.into(),
            params: Default::default(),
        }
    }

    pub fn keep_original_details(mut self, v: bool) -> Self {
        self.params.keep_original_details = Some(v);
        self
    }
    pub fn star_count(mut self, v: i64) -> Self {
        self.params.star_count = Some(v);
        self
    }
}

impl<'bot> IntoFuture for UpgradeGiftRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .upgrade_gift_with_params(
                    self.business_connection_id,
                    self.owned_gift_id,
                    Some(self.params),
                )
                .await
        })
    }
}

/// Fluent builder for [`Bot::upload_sticker_file`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#uploadstickerfile
pub struct UploadStickerFileRequest<'bot> {
    bot: &'bot Bot,
    user_id: i64,
    sticker: InputFile,
    sticker_format: String,
}

impl<'bot> UploadStickerFileRequest<'bot> {
    pub(crate) fn new(
        bot: &'bot Bot,
        user_id: i64,
        sticker: InputFile,
        sticker_format: impl Into<String>,
    ) -> Self {
        Self {
            bot,
            user_id,
            sticker,
            sticker_format: sticker_format.into(),
        }
    }
}

impl<'bot> IntoFuture for UploadStickerFileRequest<'bot> {
    type Output = Result<File, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .raw_upload_sticker_file(self.user_id, self.sticker, self.sticker_format)
                .await
        })
    }
}

/// Fluent builder for [`Bot::verify_chat`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#verifychat
pub struct VerifyChatRequest<'bot> {
    bot: &'bot Bot,
    chat_id: ChatId,
    params: VerifyChatParams,
}

impl<'bot> VerifyChatRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, chat_id: impl Into<ChatId>) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            params: Default::default(),
        }
    }

    pub fn custom_description(mut self, v: impl Into<String>) -> Self {
        self.params.custom_description = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for VerifyChatRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .verify_chat_with_params(self.chat_id, Some(self.params))
                .await
        })
    }
}

/// Fluent builder for [`Bot::verify_user`]. Implements [`IntoFuture`].
///
/// See: https://core.telegram.org/bots/api#verifyuser
pub struct VerifyUserRequest<'bot> {
    bot: &'bot Bot,
    user_id: i64,
    params: VerifyUserParams,
}

impl<'bot> VerifyUserRequest<'bot> {
    pub(crate) fn new(bot: &'bot Bot, user_id: i64) -> Self {
        Self {
            bot,
            user_id,
            params: Default::default(),
        }
    }

    pub fn custom_description(mut self, v: impl Into<String>) -> Self {
        self.params.custom_description = Some(v.into());
        self
    }
}

impl<'bot> IntoFuture for VerifyUserRequest<'bot> {
    type Output = Result<bool, BotError>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'bot>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.bot
                .verify_user_with_params(self.user_id, Some(self.params))
                .await
        })
    }
}

impl Bot {
    /// Use this method to add a new sticker to a set created by the bot. Emoji sticker sets can have up to 200 stickers. Other sticker sets can have up to 120 stickers. Returns True on success.
    /// See: https://core.telegram.org/bots/api#addstickertoset
    pub fn add_sticker_to_set(
        &self,
        user_id: i64,
        name: impl Into<String>,
        sticker: InputSticker,
    ) -> AddStickerToSetRequest<'_> {
        AddStickerToSetRequest::new(self, user_id, name, sticker)
    }

    /// Use this method to send answers to callback queries sent from inline keyboards. The answer will be displayed to the user as a notification at the top of the chat screen or as an alert. On success, True is returned.
    /// See: https://core.telegram.org/bots/api#answercallbackquery
    pub fn answer_callback_query(
        &self,
        callback_query_id: impl Into<String>,
    ) -> AnswerCallbackQueryRequest<'_> {
        AnswerCallbackQueryRequest::new(self, callback_query_id)
    }

    /// Use this method to send answers to an inline query. On success, True is returned.
    /// See: https://core.telegram.org/bots/api#answerinlinequery
    pub fn answer_inline_query(
        &self,
        inline_query_id: impl Into<String>,
        results: Vec<InlineQueryResult>,
    ) -> AnswerInlineQueryRequest<'_> {
        AnswerInlineQueryRequest::new(self, inline_query_id, results)
    }

    /// Once the user has confirmed their payment and shipping details, the Bot API sends the final confirmation in the form of an Update with the field pre_checkout_query. Use this method to respond to such pre-checkout queries. On success, True is returned. Note: The Bot API must receive an answer within 10 seconds after the pre-checkout query was sent.
    /// See: https://core.telegram.org/bots/api#answerprecheckoutquery
    pub fn answer_pre_checkout_query(
        &self,
        pre_checkout_query_id: impl Into<String>,
        ok: bool,
    ) -> AnswerPreCheckoutQueryRequest<'_> {
        AnswerPreCheckoutQueryRequest::new(self, pre_checkout_query_id, ok)
    }

    /// If you sent an invoice requesting a shipping address and the parameter is_flexible was specified, the Bot API will send an Update with a shipping_query field to the bot. Use this method to reply to shipping queries. On success, True is returned.
    /// See: https://core.telegram.org/bots/api#answershippingquery
    pub fn answer_shipping_query(
        &self,
        shipping_query_id: impl Into<String>,
        ok: bool,
    ) -> AnswerShippingQueryRequest<'_> {
        AnswerShippingQueryRequest::new(self, shipping_query_id, ok)
    }

    /// Use this method to set the result of an interaction with a Web App and send a corresponding message on behalf of the user to the chat from which the query originated. On success, a SentWebAppMessage object is returned.
    /// See: https://core.telegram.org/bots/api#answerwebappquery
    pub fn answer_web_app_query(
        &self,
        web_app_query_id: impl Into<String>,
        result: InlineQueryResult,
    ) -> AnswerWebAppQueryRequest<'_> {
        AnswerWebAppQueryRequest::new(self, web_app_query_id, result)
    }

    /// Use this method to approve a chat join request. The bot must be an administrator in the chat for this to work and must have the can_invite_users administrator right. Returns True on success.
    /// See: https://core.telegram.org/bots/api#approvechatjoinrequest
    pub fn approve_chat_join_request(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: i64,
    ) -> ApproveChatJoinRequestRequest<'_> {
        ApproveChatJoinRequestRequest::new(self, chat_id, user_id)
    }

    /// Use this method to approve a suggested post in a direct messages chat. The bot must have the 'can_post_messages' administrator right in the corresponding channel chat. Returns True on success.
    /// See: https://core.telegram.org/bots/api#approvesuggestedpost
    pub fn approve_suggested_post(
        &self,
        chat_id: i64,
        message_id: i64,
    ) -> ApproveSuggestedPostRequest<'_> {
        ApproveSuggestedPostRequest::new(self, chat_id, message_id)
    }

    /// Use this method to ban a user in a group, a supergroup or a channel. In the case of supergroups and channels, the user will not be able to return to the chat on their own using invite links, etc., unless unbanned first. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
    /// See: https://core.telegram.org/bots/api#banchatmember
    pub fn ban_chat_member(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: i64,
    ) -> BanChatMemberRequest<'_> {
        BanChatMemberRequest::new(self, chat_id, user_id)
    }

    /// Use this method to ban a channel chat in a supergroup or a channel. Until the chat is unbanned, the owner of the banned chat won't be able to send messages on behalf of any of their channels. The bot must be an administrator in the supergroup or channel for this to work and must have the appropriate administrator rights. Returns True on success.
    /// See: https://core.telegram.org/bots/api#banchatsenderchat
    pub fn ban_chat_sender_chat(
        &self,
        chat_id: impl Into<ChatId>,
        sender_chat_id: i64,
    ) -> BanChatSenderChatRequest<'_> {
        BanChatSenderChatRequest::new(self, chat_id, sender_chat_id)
    }

    /// Use this method to close the bot instance before moving it from one local server to another. You need to delete the webhook before calling this method to ensure that the bot isn't launched again after server restart. The method will return error 429 in the first 10 minutes after the bot is launched. Returns True on success. Requires no parameters.
    /// See: https://core.telegram.org/bots/api#close
    pub fn close(&self) -> CloseRequest<'_> {
        CloseRequest::new(self)
    }

    /// Use this method to close an open topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights, unless it is the creator of the topic. Returns True on success.
    /// See: https://core.telegram.org/bots/api#closeforumtopic
    pub fn close_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
        message_thread_id: i64,
    ) -> CloseForumTopicRequest<'_> {
        CloseForumTopicRequest::new(self, chat_id, message_thread_id)
    }

    /// Use this method to close an open 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights. Returns True on success.
    /// See: https://core.telegram.org/bots/api#closegeneralforumtopic
    pub fn close_general_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> CloseGeneralForumTopicRequest<'_> {
        CloseGeneralForumTopicRequest::new(self, chat_id)
    }

    /// Converts a given regular gift to Telegram Stars. Requires the can_convert_gifts_to_stars business bot right. Returns True on success.
    /// See: https://core.telegram.org/bots/api#convertgifttostars
    pub fn convert_gift_to_stars(
        &self,
        business_connection_id: impl Into<String>,
        owned_gift_id: impl Into<String>,
    ) -> ConvertGiftToStarsRequest<'_> {
        ConvertGiftToStarsRequest::new(self, business_connection_id, owned_gift_id)
    }

    /// Use this method to copy messages of any kind. Service messages, paid media messages, giveaway messages, giveaway winners messages, and invoice messages can't be copied. A quiz poll can be copied only if the value of the field correct_option_id is known to the bot. The method is analogous to the method forwardMessage, but the copied message doesn't have a link to the original message. Returns the MessageId of the sent message on success.
    /// See: https://core.telegram.org/bots/api#copymessage
    pub fn copy_message(
        &self,
        chat_id: impl Into<ChatId>,
        from_chat_id: impl Into<ChatId>,
        message_id: i64,
    ) -> CopyMessageRequest<'_> {
        CopyMessageRequest::new(self, chat_id, from_chat_id, message_id)
    }

    /// Use this method to copy messages of any kind. If some of the specified messages can't be found or copied, they are skipped. Service messages, paid media messages, giveaway messages, giveaway winners messages, and invoice messages can't be copied. A quiz poll can be copied only if the value of the field correct_option_id is known to the bot. The method is analogous to the method forwardMessages, but the copied messages don't have a link to the original message. Album grouping is kept for copied messages. On success, an array of MessageId of the sent messages is returned.
    /// See: https://core.telegram.org/bots/api#copymessages
    pub fn copy_messages(
        &self,
        chat_id: impl Into<ChatId>,
        from_chat_id: impl Into<ChatId>,
        message_ids: Vec<i64>,
    ) -> CopyMessagesRequest<'_> {
        CopyMessagesRequest::new(self, chat_id, from_chat_id, message_ids)
    }

    /// Use this method to create an additional invite link for a chat. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. The link can be revoked using the method revokeChatInviteLink. Returns the new invite link as ChatInviteLink object.
    /// See: https://core.telegram.org/bots/api#createchatinvitelink
    pub fn create_chat_invite_link(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> CreateChatInviteLinkRequest<'_> {
        CreateChatInviteLinkRequest::new(self, chat_id)
    }

    /// Use this method to create a subscription invite link for a channel chat. The bot must have the can_invite_users administrator rights. The link can be edited using the method editChatSubscriptionInviteLink or revoked using the method revokeChatInviteLink. Returns the new invite link as a ChatInviteLink object.
    /// See: https://core.telegram.org/bots/api#createchatsubscriptioninvitelink
    pub fn create_chat_subscription_invite_link(
        &self,
        chat_id: impl Into<ChatId>,
        subscription_period: i64,
        subscription_price: i64,
    ) -> CreateChatSubscriptionInviteLinkRequest<'_> {
        CreateChatSubscriptionInviteLinkRequest::new(
            self,
            chat_id,
            subscription_period,
            subscription_price,
        )
    }

    /// Use this method to create a topic in a forum supergroup chat or a private chat with a user. In the case of a supergroup chat the bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator right. Returns information about the created topic as a ForumTopic object.
    /// See: https://core.telegram.org/bots/api#createforumtopic
    pub fn create_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
        name: impl Into<String>,
    ) -> CreateForumTopicRequest<'_> {
        CreateForumTopicRequest::new(self, chat_id, name)
    }

    /// Use this method to create a link for an invoice. Returns the created invoice link as String on success.
    /// See: https://core.telegram.org/bots/api#createinvoicelink
    pub fn create_invoice_link(
        &self,
        title: impl Into<String>,
        description: impl Into<String>,
        payload: impl Into<String>,
        currency: impl Into<String>,
        prices: Vec<LabeledPrice>,
    ) -> CreateInvoiceLinkRequest<'_> {
        CreateInvoiceLinkRequest::new(self, title, description, payload, currency, prices)
    }

    /// Use this method to create a new sticker set owned by a user. The bot will be able to edit the sticker set thus created. Returns True on success.
    /// See: https://core.telegram.org/bots/api#createnewstickerset
    pub fn create_new_sticker_set(
        &self,
        user_id: i64,
        name: impl Into<String>,
        title: impl Into<String>,
        stickers: Vec<InputSticker>,
    ) -> CreateNewStickerSetRequest<'_> {
        CreateNewStickerSetRequest::new(self, user_id, name, title, stickers)
    }

    /// Use this method to decline a chat join request. The bot must be an administrator in the chat for this to work and must have the can_invite_users administrator right. Returns True on success.
    /// See: https://core.telegram.org/bots/api#declinechatjoinrequest
    pub fn decline_chat_join_request(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: i64,
    ) -> DeclineChatJoinRequestRequest<'_> {
        DeclineChatJoinRequestRequest::new(self, chat_id, user_id)
    }

    /// Use this method to decline a suggested post in a direct messages chat. The bot must have the 'can_manage_direct_messages' administrator right in the corresponding channel chat. Returns True on success.
    /// See: https://core.telegram.org/bots/api#declinesuggestedpost
    pub fn decline_suggested_post(
        &self,
        chat_id: i64,
        message_id: i64,
    ) -> DeclineSuggestedPostRequest<'_> {
        DeclineSuggestedPostRequest::new(self, chat_id, message_id)
    }

    /// Delete messages on behalf of a business account. Requires the can_delete_sent_messages business bot right to delete messages sent by the bot itself, or the can_delete_all_messages business bot right to delete any message. Returns True on success.
    /// See: https://core.telegram.org/bots/api#deletebusinessmessages
    pub fn delete_business_messages(
        &self,
        business_connection_id: impl Into<String>,
        message_ids: Vec<i64>,
    ) -> DeleteBusinessMessagesRequest<'_> {
        DeleteBusinessMessagesRequest::new(self, business_connection_id, message_ids)
    }

    /// Use this method to delete a chat photo. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
    /// See: https://core.telegram.org/bots/api#deletechatphoto
    pub fn delete_chat_photo(&self, chat_id: impl Into<ChatId>) -> DeleteChatPhotoRequest<'_> {
        DeleteChatPhotoRequest::new(self, chat_id)
    }

    /// Use this method to delete a group sticker set from a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field can_set_sticker_set optionally returned in getChat requests to check if the bot can use this method. Returns True on success.
    /// See: https://core.telegram.org/bots/api#deletechatstickerset
    pub fn delete_chat_sticker_set(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> DeleteChatStickerSetRequest<'_> {
        DeleteChatStickerSetRequest::new(self, chat_id)
    }

    /// Use this method to delete a forum topic along with all its messages in a forum supergroup chat or a private chat with a user. In the case of a supergroup chat the bot must be an administrator in the chat for this to work and must have the can_delete_messages administrator rights. Returns True on success.
    /// See: https://core.telegram.org/bots/api#deleteforumtopic
    pub fn delete_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
        message_thread_id: i64,
    ) -> DeleteForumTopicRequest<'_> {
        DeleteForumTopicRequest::new(self, chat_id, message_thread_id)
    }

    /// Use this method to delete a message, including service messages, with the following limitations:
    /// See: https://core.telegram.org/bots/api#deletemessage
    pub fn delete_message(
        &self,
        chat_id: impl Into<ChatId>,
        message_id: i64,
    ) -> DeleteMessageRequest<'_> {
        DeleteMessageRequest::new(self, chat_id, message_id)
    }

    /// Use this method to delete multiple messages simultaneously. If some of the specified messages can't be found, they are skipped. Returns True on success.
    /// See: https://core.telegram.org/bots/api#deletemessages
    pub fn delete_messages(
        &self,
        chat_id: impl Into<ChatId>,
        message_ids: Vec<i64>,
    ) -> DeleteMessagesRequest<'_> {
        DeleteMessagesRequest::new(self, chat_id, message_ids)
    }

    /// Use this method to delete the list of the bot's commands for the given scope and user language. After deletion, higher level commands will be shown to affected users. Returns True on success.
    /// See: https://core.telegram.org/bots/api#deletemycommands
    pub fn delete_my_commands(&self) -> DeleteMyCommandsRequest<'_> {
        DeleteMyCommandsRequest::new(self)
    }

    /// Use this method to delete a sticker from a set created by the bot. Returns True on success.
    /// See: https://core.telegram.org/bots/api#deletestickerfromset
    pub fn delete_sticker_from_set(
        &self,
        sticker: impl Into<String>,
    ) -> DeleteStickerFromSetRequest<'_> {
        DeleteStickerFromSetRequest::new(self, sticker)
    }

    /// Use this method to delete a sticker set that was created by the bot. Returns True on success.
    /// See: https://core.telegram.org/bots/api#deletestickerset
    pub fn delete_sticker_set(&self, name: impl Into<String>) -> DeleteStickerSetRequest<'_> {
        DeleteStickerSetRequest::new(self, name)
    }

    /// Deletes a story previously posted by the bot on behalf of a managed business account. Requires the can_manage_stories business bot right. Returns True on success.
    /// See: https://core.telegram.org/bots/api#deletestory
    pub fn delete_story(
        &self,
        business_connection_id: impl Into<String>,
        story_id: i64,
    ) -> DeleteStoryRequest<'_> {
        DeleteStoryRequest::new(self, business_connection_id, story_id)
    }

    /// Use this method to remove webhook integration if you decide to switch back to getUpdates. Returns True on success.
    /// See: https://core.telegram.org/bots/api#deletewebhook
    pub fn delete_webhook(&self) -> DeleteWebhookRequest<'_> {
        DeleteWebhookRequest::new(self)
    }

    /// Use this method to edit a non-primary invite link created by the bot. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the edited invite link as a ChatInviteLink object.
    /// See: https://core.telegram.org/bots/api#editchatinvitelink
    pub fn edit_chat_invite_link(
        &self,
        chat_id: impl Into<ChatId>,
        invite_link: impl Into<String>,
    ) -> EditChatInviteLinkRequest<'_> {
        EditChatInviteLinkRequest::new(self, chat_id, invite_link)
    }

    /// Use this method to edit a subscription invite link created by the bot. The bot must have the can_invite_users administrator rights. Returns the edited invite link as a ChatInviteLink object.
    /// See: https://core.telegram.org/bots/api#editchatsubscriptioninvitelink
    pub fn edit_chat_subscription_invite_link(
        &self,
        chat_id: impl Into<ChatId>,
        invite_link: impl Into<String>,
    ) -> EditChatSubscriptionInviteLinkRequest<'_> {
        EditChatSubscriptionInviteLinkRequest::new(self, chat_id, invite_link)
    }

    /// Use this method to edit name and icon of a topic in a forum supergroup chat or a private chat with a user. In the case of a supergroup chat the bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights, unless it is the creator of the topic. Returns True on success.
    /// See: https://core.telegram.org/bots/api#editforumtopic
    pub fn edit_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
        message_thread_id: i64,
    ) -> EditForumTopicRequest<'_> {
        EditForumTopicRequest::new(self, chat_id, message_thread_id)
    }

    /// Use this method to edit the name of the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights. Returns True on success.
    /// See: https://core.telegram.org/bots/api#editgeneralforumtopic
    pub fn edit_general_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
        name: impl Into<String>,
    ) -> EditGeneralForumTopicRequest<'_> {
        EditGeneralForumTopicRequest::new(self, chat_id, name)
    }

    /// Use this method to edit captions of messages. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within 48 hours from the time they were sent.
    /// See: https://core.telegram.org/bots/api#editmessagecaption
    pub fn edit_message_caption(&self) -> EditMessageCaptionRequest<'_> {
        EditMessageCaptionRequest::new(self)
    }

    /// Use this method to edit a checklist on behalf of a connected business account. On success, the edited Message is returned.
    /// See: https://core.telegram.org/bots/api#editmessagechecklist
    pub fn edit_message_checklist(
        &self,
        business_connection_id: impl Into<String>,
        chat_id: i64,
        message_id: i64,
        checklist: InputChecklist,
    ) -> EditMessageChecklistRequest<'_> {
        EditMessageChecklistRequest::new(
            self,
            business_connection_id,
            chat_id,
            message_id,
            checklist,
        )
    }

    /// Use this method to edit live location messages. A location can be edited until its live_period expires or editing is explicitly disabled by a call to stopMessageLiveLocation. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned.
    /// See: https://core.telegram.org/bots/api#editmessagelivelocation
    pub fn edit_message_live_location(
        &self,
        latitude: f64,
        longitude: f64,
    ) -> EditMessageLiveLocationRequest<'_> {
        EditMessageLiveLocationRequest::new(self, latitude, longitude)
    }

    /// Use this method to edit animation, audio, document, photo, or video messages, or to add media to text messages. If a message is part of a message album, then it can be edited only to an audio for audio albums, only to a document for document albums and to a photo or a video otherwise. When an inline message is edited, a new file can't be uploaded; use a previously uploaded file via its file_id or specify a URL. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within 48 hours from the time they were sent.
    /// See: https://core.telegram.org/bots/api#editmessagemedia
    pub fn edit_message_media(&self, media: Vec<InputMedia>) -> EditMessageMediaRequest<'_> {
        EditMessageMediaRequest::new(self, media)
    }

    /// Use this method to edit only the reply markup of messages. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within 48 hours from the time they were sent.
    /// See: https://core.telegram.org/bots/api#editmessagereplymarkup
    pub fn edit_message_reply_markup(&self) -> EditMessageReplyMarkupRequest<'_> {
        EditMessageReplyMarkupRequest::new(self)
    }

    /// Use this method to edit text and game messages. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within 48 hours from the time they were sent.
    /// See: https://core.telegram.org/bots/api#editmessagetext
    pub fn edit_message_text(&self, text: impl Into<String>) -> EditMessageTextRequest<'_> {
        EditMessageTextRequest::new(self, text)
    }

    /// Edits a story previously posted by the bot on behalf of a managed business account. Requires the can_manage_stories business bot right. Returns Story on success.
    /// See: https://core.telegram.org/bots/api#editstory
    pub fn edit_story(
        &self,
        business_connection_id: impl Into<String>,
        story_id: i64,
        content: InputStoryContent,
    ) -> EditStoryRequest<'_> {
        EditStoryRequest::new(self, business_connection_id, story_id, content)
    }

    /// Allows the bot to cancel or re-enable extension of a subscription paid in Telegram Stars. Returns True on success.
    /// See: https://core.telegram.org/bots/api#edituserstarsubscription
    pub fn edit_user_star_subscription(
        &self,
        user_id: i64,
        telegram_payment_charge_id: impl Into<String>,
        is_canceled: bool,
    ) -> EditUserStarSubscriptionRequest<'_> {
        EditUserStarSubscriptionRequest::new(self, user_id, telegram_payment_charge_id, is_canceled)
    }

    /// Use this method to generate a new primary invite link for a chat; any previously generated primary link is revoked. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the new invite link as String on success.
    /// See: https://core.telegram.org/bots/api#exportchatinvitelink
    pub fn export_chat_invite_link(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> ExportChatInviteLinkRequest<'_> {
        ExportChatInviteLinkRequest::new(self, chat_id)
    }

    /// Use this method to forward messages of any kind. Service messages and messages with protected content can't be forwarded. On success, the sent Message is returned.
    /// See: https://core.telegram.org/bots/api#forwardmessage
    pub fn forward_message(
        &self,
        chat_id: impl Into<ChatId>,
        from_chat_id: impl Into<ChatId>,
        message_id: i64,
    ) -> ForwardMessageRequest<'_> {
        ForwardMessageRequest::new(self, chat_id, from_chat_id, message_id)
    }

    /// Use this method to forward multiple messages of any kind. If some of the specified messages can't be found or forwarded, they are skipped. Service messages and messages with protected content can't be forwarded. Album grouping is kept for forwarded messages. On success, an array of MessageId of the sent messages is returned.
    /// See: https://core.telegram.org/bots/api#forwardmessages
    pub fn forward_messages(
        &self,
        chat_id: impl Into<ChatId>,
        from_chat_id: impl Into<ChatId>,
        message_ids: Vec<i64>,
    ) -> ForwardMessagesRequest<'_> {
        ForwardMessagesRequest::new(self, chat_id, from_chat_id, message_ids)
    }

    /// Returns the list of gifts that can be sent by the bot to users and channel chats. Requires no parameters. Returns a Gifts object.
    /// See: https://core.telegram.org/bots/api#getavailablegifts
    pub fn get_available_gifts(&self) -> GetAvailableGiftsRequest<'_> {
        GetAvailableGiftsRequest::new(self)
    }

    /// Returns the gifts received and owned by a managed business account. Requires the can_view_gifts_and_stars business bot right. Returns OwnedGifts on success.
    /// See: https://core.telegram.org/bots/api#getbusinessaccountgifts
    pub fn get_business_account_gifts(
        &self,
        business_connection_id: impl Into<String>,
    ) -> GetBusinessAccountGiftsRequest<'_> {
        GetBusinessAccountGiftsRequest::new(self, business_connection_id)
    }

    /// Returns the amount of Telegram Stars owned by a managed business account. Requires the can_view_gifts_and_stars business bot right. Returns StarAmount on success.
    /// See: https://core.telegram.org/bots/api#getbusinessaccountstarbalance
    pub fn get_business_account_star_balance(
        &self,
        business_connection_id: impl Into<String>,
    ) -> GetBusinessAccountStarBalanceRequest<'_> {
        GetBusinessAccountStarBalanceRequest::new(self, business_connection_id)
    }

    /// Use this method to get information about the connection of the bot with a business account. Returns a BusinessConnection object on success.
    /// See: https://core.telegram.org/bots/api#getbusinessconnection
    pub fn get_business_connection(
        &self,
        business_connection_id: impl Into<String>,
    ) -> GetBusinessConnectionRequest<'_> {
        GetBusinessConnectionRequest::new(self, business_connection_id)
    }

    /// Use this method to get up-to-date information about the chat. Returns a ChatFullInfo object on success.
    /// See: https://core.telegram.org/bots/api#getchat
    pub fn get_chat(&self, chat_id: impl Into<ChatId>) -> GetChatRequest<'_> {
        GetChatRequest::new(self, chat_id)
    }

    /// Use this method to get a list of administrators in a chat, which aren't bots. Returns an Array of ChatMember objects.
    /// See: https://core.telegram.org/bots/api#getchatadministrators
    pub fn get_chat_administrators(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> GetChatAdministratorsRequest<'_> {
        GetChatAdministratorsRequest::new(self, chat_id)
    }

    /// Returns the gifts owned by a chat. Returns OwnedGifts on success.
    /// See: https://core.telegram.org/bots/api#getchatgifts
    pub fn get_chat_gifts(&self, chat_id: impl Into<ChatId>) -> GetChatGiftsRequest<'_> {
        GetChatGiftsRequest::new(self, chat_id)
    }

    /// Use this method to get information about a member of a chat. The method is only guaranteed to work for other users if the bot is an administrator in the chat. Returns a ChatMember object on success.
    /// See: https://core.telegram.org/bots/api#getchatmember
    pub fn get_chat_member(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: i64,
    ) -> GetChatMemberRequest<'_> {
        GetChatMemberRequest::new(self, chat_id, user_id)
    }

    /// Use this method to get the number of members in a chat. Returns Int on success.
    /// See: https://core.telegram.org/bots/api#getchatmembercount
    pub fn get_chat_member_count(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> GetChatMemberCountRequest<'_> {
        GetChatMemberCountRequest::new(self, chat_id)
    }

    /// Use this method to get the current value of the bot's menu button in a private chat, or the default menu button. Returns MenuButton on success.
    /// See: https://core.telegram.org/bots/api#getchatmenubutton
    pub fn get_chat_menu_button(&self) -> GetChatMenuButtonRequest<'_> {
        GetChatMenuButtonRequest::new(self)
    }

    /// Use this method to get information about custom emoji stickers by their identifiers. Returns an Array of Sticker objects.
    /// See: https://core.telegram.org/bots/api#getcustomemojistickers
    pub fn get_custom_emoji_stickers(
        &self,
        custom_emoji_ids: Vec<String>,
    ) -> GetCustomEmojiStickersRequest<'_> {
        GetCustomEmojiStickersRequest::new(self, custom_emoji_ids)
    }

    /// Use this method to get basic information about a file and prepare it for downloading. For the moment, bots can download files of up to 20MB in size. On success, a File object is returned. The file can then be downloaded via the link https://api.telegram.org/file/bot<token>/<file_path>, where <file_path> is taken from the response. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling getFile again.
    /// See: https://core.telegram.org/bots/api#getfile
    pub fn get_file(&self, file_id: impl Into<String>) -> GetFileRequest<'_> {
        GetFileRequest::new(self, file_id)
    }

    /// Use this method to get custom emoji stickers, which can be used as a forum topic icon by any user. Requires no parameters. Returns an Array of Sticker objects.
    /// See: https://core.telegram.org/bots/api#getforumtopiciconstickers
    pub fn get_forum_topic_icon_stickers(&self) -> GetForumTopicIconStickersRequest<'_> {
        GetForumTopicIconStickersRequest::new(self)
    }

    /// Use this method to get data for high score tables. Will return the score of the specified user and several of their neighbors in a game. Returns an Array of GameHighScore objects.
    /// See: https://core.telegram.org/bots/api#getgamehighscores
    pub fn get_game_high_scores(&self, user_id: i64) -> GetGameHighScoresRequest<'_> {
        GetGameHighScoresRequest::new(self, user_id)
    }

    /// Use this method to get the token of a managed bot. Returns the token as String on success.
    /// See: https://core.telegram.org/bots/api#getmanagedbottoken
    pub fn get_managed_bot_token(&self, user_id: i64) -> GetManagedBotTokenRequest<'_> {
        GetManagedBotTokenRequest::new(self, user_id)
    }

    /// A simple method for testing your bot's authentication token. Requires no parameters. Returns basic information about the bot in form of a User object.
    /// See: https://core.telegram.org/bots/api#getme
    pub fn get_me(&self) -> GetMeRequest<'_> {
        GetMeRequest::new(self)
    }

    /// Use this method to get the current list of the bot's commands for the given scope and user language. Returns an Array of BotCommand objects. If commands aren't set, an empty list is returned.
    /// See: https://core.telegram.org/bots/api#getmycommands
    pub fn get_my_commands(&self) -> GetMyCommandsRequest<'_> {
        GetMyCommandsRequest::new(self)
    }

    /// Use this method to get the current default administrator rights of the bot. Returns ChatAdministratorRights on success.
    /// See: https://core.telegram.org/bots/api#getmydefaultadministratorrights
    pub fn get_my_default_administrator_rights(
        &self,
    ) -> GetMyDefaultAdministratorRightsRequest<'_> {
        GetMyDefaultAdministratorRightsRequest::new(self)
    }

    /// Use this method to get the current bot description for the given user language. Returns BotDescription on success.
    /// See: https://core.telegram.org/bots/api#getmydescription
    pub fn get_my_description(&self) -> GetMyDescriptionRequest<'_> {
        GetMyDescriptionRequest::new(self)
    }

    /// Use this method to get the current bot name for the given user language. Returns BotName on success.
    /// See: https://core.telegram.org/bots/api#getmyname
    pub fn get_my_name(&self) -> GetMyNameRequest<'_> {
        GetMyNameRequest::new(self)
    }

    /// Use this method to get the current bot short description for the given user language. Returns BotShortDescription on success.
    /// See: https://core.telegram.org/bots/api#getmyshortdescription
    pub fn get_my_short_description(&self) -> GetMyShortDescriptionRequest<'_> {
        GetMyShortDescriptionRequest::new(self)
    }

    /// A method to get the current Telegram Stars balance of the bot. Requires no parameters. On success, returns a StarAmount object.
    /// See: https://core.telegram.org/bots/api#getmystarbalance
    pub fn get_my_star_balance(&self) -> GetMyStarBalanceRequest<'_> {
        GetMyStarBalanceRequest::new(self)
    }

    /// Returns the bot's Telegram Star transactions in chronological order. On success, returns a StarTransactions object.
    /// See: https://core.telegram.org/bots/api#getstartransactions
    pub fn get_star_transactions(&self) -> GetStarTransactionsRequest<'_> {
        GetStarTransactionsRequest::new(self)
    }

    /// Use this method to get a sticker set. On success, a StickerSet object is returned.
    /// See: https://core.telegram.org/bots/api#getstickerset
    pub fn get_sticker_set(&self, name: impl Into<String>) -> GetStickerSetRequest<'_> {
        GetStickerSetRequest::new(self, name)
    }

    /// Use this method to receive incoming updates using long polling (wiki). Returns an Array of Update objects.
    /// See: https://core.telegram.org/bots/api#getupdates
    pub fn get_updates(&self) -> GetUpdatesRequest<'_> {
        GetUpdatesRequest::new(self)
    }

    /// Use this method to get the list of boosts added to a chat by a user. Requires administrator rights in the chat. Returns a UserChatBoosts object.
    /// See: https://core.telegram.org/bots/api#getuserchatboosts
    pub fn get_user_chat_boosts(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: i64,
    ) -> GetUserChatBoostsRequest<'_> {
        GetUserChatBoostsRequest::new(self, chat_id, user_id)
    }

    /// Returns the gifts owned and hosted by a user. Returns OwnedGifts on success.
    /// See: https://core.telegram.org/bots/api#getusergifts
    pub fn get_user_gifts(&self, user_id: i64) -> GetUserGiftsRequest<'_> {
        GetUserGiftsRequest::new(self, user_id)
    }

    /// Use this method to get a list of profile audios for a user. Returns a UserProfileAudios object.
    /// See: https://core.telegram.org/bots/api#getuserprofileaudios
    pub fn get_user_profile_audios(&self, user_id: i64) -> GetUserProfileAudiosRequest<'_> {
        GetUserProfileAudiosRequest::new(self, user_id)
    }

    /// Use this method to get a list of profile pictures for a user. Returns a UserProfilePhotos object.
    /// See: https://core.telegram.org/bots/api#getuserprofilephotos
    pub fn get_user_profile_photos(&self, user_id: i64) -> GetUserProfilePhotosRequest<'_> {
        GetUserProfilePhotosRequest::new(self, user_id)
    }

    /// Use this method to get current webhook status. Requires no parameters. On success, returns a WebhookInfo object. If the bot is using getUpdates, will return an object with the url field empty.
    /// See: https://core.telegram.org/bots/api#getwebhookinfo
    pub fn get_webhook_info(&self) -> GetWebhookInfoRequest<'_> {
        GetWebhookInfoRequest::new(self)
    }

    /// Gifts a Telegram Premium subscription to the given user. Returns True on success.
    /// See: https://core.telegram.org/bots/api#giftpremiumsubscription
    pub fn gift_premium_subscription(
        &self,
        user_id: i64,
        month_count: i64,
        star_count: i64,
    ) -> GiftPremiumSubscriptionRequest<'_> {
        GiftPremiumSubscriptionRequest::new(self, user_id, month_count, star_count)
    }

    /// Use this method to hide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights. The topic will be automatically closed if it was open. Returns True on success.
    /// See: https://core.telegram.org/bots/api#hidegeneralforumtopic
    pub fn hide_general_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> HideGeneralForumTopicRequest<'_> {
        HideGeneralForumTopicRequest::new(self, chat_id)
    }

    /// Use this method for your bot to leave a group, supergroup or channel. Returns True on success.
    /// See: https://core.telegram.org/bots/api#leavechat
    pub fn leave_chat(&self, chat_id: impl Into<ChatId>) -> LeaveChatRequest<'_> {
        LeaveChatRequest::new(self, chat_id)
    }

    /// Use this method to log out from the cloud Bot API server before launching the bot locally. You must log out the bot before running it locally, otherwise there is no guarantee that the bot will receive updates. After a successful call, you can immediately log in on a local server, but will not be able to log in back to the cloud Bot API server for 10 minutes. Returns True on success. Requires no parameters.
    /// See: https://core.telegram.org/bots/api#logout
    pub fn log_out(&self) -> LogOutRequest<'_> {
        LogOutRequest::new(self)
    }

    /// Use this method to add a message to the list of pinned messages in a chat. In private chats and channel direct messages chats, all non-service messages can be pinned. Conversely, the bot must be an administrator with the 'can_pin_messages' right or the 'can_edit_messages' right to pin messages in groups and channels respectively. Returns True on success.
    /// See: https://core.telegram.org/bots/api#pinchatmessage
    pub fn pin_chat_message(
        &self,
        chat_id: impl Into<ChatId>,
        message_id: i64,
    ) -> PinChatMessageRequest<'_> {
        PinChatMessageRequest::new(self, chat_id, message_id)
    }

    /// Posts a story on behalf of a managed business account. Requires the can_manage_stories business bot right. Returns Story on success.
    /// See: https://core.telegram.org/bots/api#poststory
    pub fn post_story(
        &self,
        business_connection_id: impl Into<String>,
        content: InputStoryContent,
        active_period: i64,
    ) -> PostStoryRequest<'_> {
        PostStoryRequest::new(self, business_connection_id, content, active_period)
    }

    /// Use this method to promote or demote a user in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Pass False for all boolean parameters to demote a user. Returns True on success.
    /// See: https://core.telegram.org/bots/api#promotechatmember
    pub fn promote_chat_member(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: i64,
    ) -> PromoteChatMemberRequest<'_> {
        PromoteChatMemberRequest::new(self, chat_id, user_id)
    }

    /// Marks incoming message as read on behalf of a business account. Requires the can_read_messages business bot right. Returns True on success.
    /// See: https://core.telegram.org/bots/api#readbusinessmessage
    pub fn read_business_message(
        &self,
        business_connection_id: impl Into<String>,
        chat_id: i64,
        message_id: i64,
    ) -> ReadBusinessMessageRequest<'_> {
        ReadBusinessMessageRequest::new(self, business_connection_id, chat_id, message_id)
    }

    /// Refunds a successful payment in Telegram Stars. Returns True on success.
    /// See: https://core.telegram.org/bots/api#refundstarpayment
    pub fn refund_star_payment(
        &self,
        user_id: i64,
        telegram_payment_charge_id: impl Into<String>,
    ) -> RefundStarPaymentRequest<'_> {
        RefundStarPaymentRequest::new(self, user_id, telegram_payment_charge_id)
    }

    /// Removes the current profile photo of a managed business account. Requires the can_edit_profile_photo business bot right. Returns True on success.
    /// See: https://core.telegram.org/bots/api#removebusinessaccountprofilephoto
    pub fn remove_business_account_profile_photo(
        &self,
        business_connection_id: impl Into<String>,
    ) -> RemoveBusinessAccountProfilePhotoRequest<'_> {
        RemoveBusinessAccountProfilePhotoRequest::new(self, business_connection_id)
    }

    /// Removes verification from a chat that is currently verified on behalf of the organization represented by the bot. Returns True on success.
    /// See: https://core.telegram.org/bots/api#removechatverification
    pub fn remove_chat_verification(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> RemoveChatVerificationRequest<'_> {
        RemoveChatVerificationRequest::new(self, chat_id)
    }

    /// Removes the profile photo of the bot. Requires no parameters. Returns True on success.
    /// See: https://core.telegram.org/bots/api#removemyprofilephoto
    pub fn remove_my_profile_photo(&self) -> RemoveMyProfilePhotoRequest<'_> {
        RemoveMyProfilePhotoRequest::new(self)
    }

    /// Removes verification from a user who is currently verified on behalf of the organization represented by the bot. Returns True on success.
    /// See: https://core.telegram.org/bots/api#removeuserverification
    pub fn remove_user_verification(&self, user_id: i64) -> RemoveUserVerificationRequest<'_> {
        RemoveUserVerificationRequest::new(self, user_id)
    }

    /// Use this method to reopen a closed topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights, unless it is the creator of the topic. Returns True on success.
    /// See: https://core.telegram.org/bots/api#reopenforumtopic
    pub fn reopen_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
        message_thread_id: i64,
    ) -> ReopenForumTopicRequest<'_> {
        ReopenForumTopicRequest::new(self, chat_id, message_thread_id)
    }

    /// Use this method to reopen a closed 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights. The topic will be automatically unhidden if it was hidden. Returns True on success.
    /// See: https://core.telegram.org/bots/api#reopengeneralforumtopic
    pub fn reopen_general_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> ReopenGeneralForumTopicRequest<'_> {
        ReopenGeneralForumTopicRequest::new(self, chat_id)
    }

    /// Use this method to revoke the current token of a managed bot and generate a new one. Returns the new token as String on success.
    /// See: https://core.telegram.org/bots/api#replacemanagedbottoken
    pub fn replace_managed_bot_token(&self, user_id: i64) -> ReplaceManagedBotTokenRequest<'_> {
        ReplaceManagedBotTokenRequest::new(self, user_id)
    }

    /// Use this method to replace an existing sticker in a sticker set with a new one. The method is equivalent to calling deleteStickerFromSet, then addStickerToSet, then setStickerPositionInSet. Returns True on success.
    /// See: https://core.telegram.org/bots/api#replacestickerinset
    pub fn replace_sticker_in_set(
        &self,
        user_id: i64,
        name: impl Into<String>,
        old_sticker: impl Into<String>,
        sticker: InputSticker,
    ) -> ReplaceStickerInSetRequest<'_> {
        ReplaceStickerInSetRequest::new(self, user_id, name, old_sticker, sticker)
    }

    /// Reposts a story on behalf of a business account from another business account. Both business accounts must be managed by the same bot, and the story on the source account must have been posted (or reposted) by the bot. Requires the can_manage_stories business bot right for both business accounts. Returns Story on success.
    /// See: https://core.telegram.org/bots/api#repoststory
    pub fn repost_story(
        &self,
        business_connection_id: impl Into<String>,
        from_chat_id: i64,
        from_story_id: i64,
        active_period: i64,
    ) -> RepostStoryRequest<'_> {
        RepostStoryRequest::new(
            self,
            business_connection_id,
            from_chat_id,
            from_story_id,
            active_period,
        )
    }

    /// Use this method to restrict a user in a supergroup. The bot must be an administrator in the supergroup for this to work and must have the appropriate administrator rights. Pass True for all permissions to lift restrictions from a user. Returns True on success.
    /// See: https://core.telegram.org/bots/api#restrictchatmember
    pub fn restrict_chat_member(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: i64,
        permissions: ChatPermissions,
    ) -> RestrictChatMemberRequest<'_> {
        RestrictChatMemberRequest::new(self, chat_id, user_id, permissions)
    }

    /// Use this method to revoke an invite link created by the bot. If the primary link is revoked, a new link is automatically generated. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the revoked invite link as ChatInviteLink object.
    /// See: https://core.telegram.org/bots/api#revokechatinvitelink
    pub fn revoke_chat_invite_link(
        &self,
        chat_id: impl Into<ChatId>,
        invite_link: impl Into<String>,
    ) -> RevokeChatInviteLinkRequest<'_> {
        RevokeChatInviteLinkRequest::new(self, chat_id, invite_link)
    }

    /// Stores a message that can be sent by a user of a Mini App. Returns a PreparedInlineMessage object.
    /// See: https://core.telegram.org/bots/api#savepreparedinlinemessage
    pub fn save_prepared_inline_message(
        &self,
        user_id: i64,
        result: InlineQueryResult,
    ) -> SavePreparedInlineMessageRequest<'_> {
        SavePreparedInlineMessageRequest::new(self, user_id, result)
    }

    /// Stores a keyboard button that can be used by a user within a Mini App. Returns a PreparedKeyboardButton object.
    /// See: https://core.telegram.org/bots/api#savepreparedkeyboardbutton
    pub fn save_prepared_keyboard_button(
        &self,
        user_id: i64,
        button: KeyboardButton,
    ) -> SavePreparedKeyboardButtonRequest<'_> {
        SavePreparedKeyboardButtonRequest::new(self, user_id, button)
    }

    /// Use this method to send animation files (GIF or H.264/MPEG-4 AVC video without sound). On success, the sent Message is returned. Bots can currently send animation files of up to 50 MB in size, this limit may be changed in the future.
    /// See: https://core.telegram.org/bots/api#sendanimation
    pub fn send_animation(
        &self,
        chat_id: impl Into<ChatId>,
        animation: impl Into<InputFileOrString>,
    ) -> SendAnimationRequest<'_> {
        SendAnimationRequest::new(self, chat_id, animation)
    }

    /// Use this method to send audio files, if you want Telegram clients to display them in the music player. Your audio must be in the .MP3 or .M4A format. On success, the sent Message is returned. Bots can currently send audio files of up to 50 MB in size, this limit may be changed in the future.
    /// See: https://core.telegram.org/bots/api#sendaudio
    pub fn send_audio(
        &self,
        chat_id: impl Into<ChatId>,
        audio: impl Into<InputFileOrString>,
    ) -> SendAudioRequest<'_> {
        SendAudioRequest::new(self, chat_id, audio)
    }

    /// Use this method when you need to tell the user that something is happening on the bot's side. The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status). Returns True on success.
    /// See: https://core.telegram.org/bots/api#sendchataction
    pub fn send_chat_action(
        &self,
        chat_id: impl Into<ChatId>,
        action: impl Into<String>,
    ) -> SendChatActionRequest<'_> {
        SendChatActionRequest::new(self, chat_id, action)
    }

    /// Use this method to send a checklist on behalf of a connected business account. On success, the sent Message is returned.
    /// See: https://core.telegram.org/bots/api#sendchecklist
    pub fn send_checklist(
        &self,
        business_connection_id: impl Into<String>,
        chat_id: i64,
        checklist: InputChecklist,
    ) -> SendChecklistRequest<'_> {
        SendChecklistRequest::new(self, business_connection_id, chat_id, checklist)
    }

    /// Use this method to send phone contacts. On success, the sent Message is returned.
    /// See: https://core.telegram.org/bots/api#sendcontact
    pub fn send_contact(
        &self,
        chat_id: impl Into<ChatId>,
        phone_number: impl Into<String>,
        first_name: impl Into<String>,
    ) -> SendContactRequest<'_> {
        SendContactRequest::new(self, chat_id, phone_number, first_name)
    }

    /// Use this method to send an animated emoji that will display a random value. On success, the sent Message is returned.
    /// See: https://core.telegram.org/bots/api#senddice
    pub fn send_dice(&self, chat_id: impl Into<ChatId>) -> SendDiceRequest<'_> {
        SendDiceRequest::new(self, chat_id)
    }

    /// Use this method to send general files. On success, the sent Message is returned. Bots can currently send files of any type of up to 50 MB in size, this limit may be changed in the future.
    /// See: https://core.telegram.org/bots/api#senddocument
    pub fn send_document(
        &self,
        chat_id: impl Into<ChatId>,
        document: impl Into<InputFileOrString>,
    ) -> SendDocumentRequest<'_> {
        SendDocumentRequest::new(self, chat_id, document)
    }

    /// Use this method to send a game. On success, the sent Message is returned.
    /// See: https://core.telegram.org/bots/api#sendgame
    pub fn send_game(
        &self,
        chat_id: i64,
        game_short_name: impl Into<String>,
    ) -> SendGameRequest<'_> {
        SendGameRequest::new(self, chat_id, game_short_name)
    }

    /// Sends a gift to the given user or channel chat. The gift can't be converted to Telegram Stars by the receiver. Returns True on success.
    /// See: https://core.telegram.org/bots/api#sendgift
    pub fn send_gift(&self, gift_id: impl Into<String>) -> SendGiftRequest<'_> {
        SendGiftRequest::new(self, gift_id)
    }

    /// Use this method to send invoices. On success, the sent Message is returned.
    /// See: https://core.telegram.org/bots/api#sendinvoice
    pub fn send_invoice(
        &self,
        chat_id: impl Into<ChatId>,
        title: impl Into<String>,
        description: impl Into<String>,
        payload: impl Into<String>,
        currency: impl Into<String>,
        prices: Vec<LabeledPrice>,
    ) -> SendInvoiceRequest<'_> {
        SendInvoiceRequest::new(self, chat_id, title, description, payload, currency, prices)
    }

    /// Use this method to send point on the map. On success, the sent Message is returned.
    /// See: https://core.telegram.org/bots/api#sendlocation
    pub fn send_location(
        &self,
        chat_id: impl Into<ChatId>,
        latitude: f64,
        longitude: f64,
    ) -> SendLocationRequest<'_> {
        SendLocationRequest::new(self, chat_id, latitude, longitude)
    }

    /// Use this method to send a group of photos, videos, documents or audios as an album. Documents and audio files can be only grouped in an album with messages of the same type. On success, an array of Message objects that were sent is returned.
    /// See: https://core.telegram.org/bots/api#sendmediagroup
    pub fn send_media_group(
        &self,
        chat_id: impl Into<ChatId>,
        media: Vec<InputMedia>,
    ) -> SendMediaGroupRequest<'_> {
        SendMediaGroupRequest::new(self, chat_id, media)
    }

    /// Use this method to send text messages. On success, the sent Message is returned.
    /// See: https://core.telegram.org/bots/api#sendmessage
    pub fn send_message(
        &self,
        chat_id: impl Into<ChatId>,
        text: impl Into<String>,
    ) -> SendMessageRequest<'_> {
        SendMessageRequest::new(self, chat_id, text)
    }

    /// Use this method to stream a partial message to a user while the message is being generated. Returns True on success.
    /// See: https://core.telegram.org/bots/api#sendmessagedraft
    pub fn send_message_draft(
        &self,
        chat_id: i64,
        draft_id: i64,
        text: impl Into<String>,
    ) -> SendMessageDraftRequest<'_> {
        SendMessageDraftRequest::new(self, chat_id, draft_id, text)
    }

    /// Use this method to send paid media. On success, the sent Message is returned.
    /// See: https://core.telegram.org/bots/api#sendpaidmedia
    pub fn send_paid_media(
        &self,
        chat_id: impl Into<ChatId>,
        star_count: i64,
        media: Vec<InputPaidMedia>,
    ) -> SendPaidMediaRequest<'_> {
        SendPaidMediaRequest::new(self, chat_id, star_count, media)
    }

    /// Use this method to send photos. On success, the sent Message is returned.
    /// See: https://core.telegram.org/bots/api#sendphoto
    pub fn send_photo(
        &self,
        chat_id: impl Into<ChatId>,
        photo: impl Into<InputFileOrString>,
    ) -> SendPhotoRequest<'_> {
        SendPhotoRequest::new(self, chat_id, photo)
    }

    /// Use this method to send a native poll. On success, the sent Message is returned.
    /// See: https://core.telegram.org/bots/api#sendpoll
    pub fn send_poll(
        &self,
        chat_id: impl Into<ChatId>,
        question: impl Into<String>,
        options: Vec<InputPollOption>,
    ) -> SendPollRequest<'_> {
        SendPollRequest::new(self, chat_id, question, options)
    }

    /// Use this method to send static .WEBP, animated .TGS, or video .WEBM stickers. On success, the sent Message is returned.
    /// See: https://core.telegram.org/bots/api#sendsticker
    pub fn send_sticker(
        &self,
        chat_id: impl Into<ChatId>,
        sticker: impl Into<InputFileOrString>,
    ) -> SendStickerRequest<'_> {
        SendStickerRequest::new(self, chat_id, sticker)
    }

    /// Use this method to send information about a venue. On success, the sent Message is returned.
    /// See: https://core.telegram.org/bots/api#sendvenue
    pub fn send_venue(
        &self,
        chat_id: impl Into<ChatId>,
        latitude: f64,
        longitude: f64,
        title: impl Into<String>,
        address: impl Into<String>,
    ) -> SendVenueRequest<'_> {
        SendVenueRequest::new(self, chat_id, latitude, longitude, title, address)
    }

    /// Use this method to send video files, Telegram clients support MPEG4 videos (other formats may be sent as Document). On success, the sent Message is returned. Bots can currently send video files of up to 50 MB in size, this limit may be changed in the future.
    /// See: https://core.telegram.org/bots/api#sendvideo
    pub fn send_video(
        &self,
        chat_id: impl Into<ChatId>,
        video: impl Into<InputFileOrString>,
    ) -> SendVideoRequest<'_> {
        SendVideoRequest::new(self, chat_id, video)
    }

    /// As of v.4.0, Telegram clients support rounded square MPEG4 videos of up to 1 minute long. Use this method to send video messages. On success, the sent Message is returned.
    /// See: https://core.telegram.org/bots/api#sendvideonote
    pub fn send_video_note(
        &self,
        chat_id: impl Into<ChatId>,
        video_note: impl Into<InputFileOrString>,
    ) -> SendVideoNoteRequest<'_> {
        SendVideoNoteRequest::new(self, chat_id, video_note)
    }

    /// Use this method to send audio files, if you want Telegram clients to display the file as a playable voice message. For this to work, your audio must be in an .OGG file encoded with OPUS, or in .MP3 format, or in .M4A format (other formats may be sent as Audio or Document). On success, the sent Message is returned. Bots can currently send voice messages of up to 50 MB in size, this limit may be changed in the future.
    /// See: https://core.telegram.org/bots/api#sendvoice
    pub fn send_voice(
        &self,
        chat_id: impl Into<ChatId>,
        voice: impl Into<InputFileOrString>,
    ) -> SendVoiceRequest<'_> {
        SendVoiceRequest::new(self, chat_id, voice)
    }

    /// Changes the bio of a managed business account. Requires the can_change_bio business bot right. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setbusinessaccountbio
    pub fn set_business_account_bio(
        &self,
        business_connection_id: impl Into<String>,
    ) -> SetBusinessAccountBioRequest<'_> {
        SetBusinessAccountBioRequest::new(self, business_connection_id)
    }

    /// Changes the privacy settings pertaining to incoming gifts in a managed business account. Requires the can_change_gift_settings business bot right. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setbusinessaccountgiftsettings
    pub fn set_business_account_gift_settings(
        &self,
        business_connection_id: impl Into<String>,
        show_gift_button: bool,
        accepted_gift_types: AcceptedGiftTypes,
    ) -> SetBusinessAccountGiftSettingsRequest<'_> {
        SetBusinessAccountGiftSettingsRequest::new(
            self,
            business_connection_id,
            show_gift_button,
            accepted_gift_types,
        )
    }

    /// Changes the first and last name of a managed business account. Requires the can_change_name business bot right. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setbusinessaccountname
    pub fn set_business_account_name(
        &self,
        business_connection_id: impl Into<String>,
        first_name: impl Into<String>,
    ) -> SetBusinessAccountNameRequest<'_> {
        SetBusinessAccountNameRequest::new(self, business_connection_id, first_name)
    }

    /// Changes the profile photo of a managed business account. Requires the can_edit_profile_photo business bot right. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setbusinessaccountprofilephoto
    pub fn set_business_account_profile_photo(
        &self,
        business_connection_id: impl Into<String>,
        photo: InputProfilePhoto,
    ) -> SetBusinessAccountProfilePhotoRequest<'_> {
        SetBusinessAccountProfilePhotoRequest::new(self, business_connection_id, photo)
    }

    /// Changes the username of a managed business account. Requires the can_change_username business bot right. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setbusinessaccountusername
    pub fn set_business_account_username(
        &self,
        business_connection_id: impl Into<String>,
    ) -> SetBusinessAccountUsernameRequest<'_> {
        SetBusinessAccountUsernameRequest::new(self, business_connection_id)
    }

    /// Use this method to set a custom title for an administrator in a supergroup promoted by the bot. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setchatadministratorcustomtitle
    pub fn set_chat_administrator_custom_title(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: i64,
        custom_title: impl Into<String>,
    ) -> SetChatAdministratorCustomTitleRequest<'_> {
        SetChatAdministratorCustomTitleRequest::new(self, chat_id, user_id, custom_title)
    }

    /// Use this method to change the description of a group, a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setchatdescription
    pub fn set_chat_description(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> SetChatDescriptionRequest<'_> {
        SetChatDescriptionRequest::new(self, chat_id)
    }

    /// Use this method to set a tag for a regular member in a group or a supergroup. The bot must be an administrator in the chat for this to work and must have the can_manage_tags administrator right. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setchatmembertag
    pub fn set_chat_member_tag(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: i64,
    ) -> SetChatMemberTagRequest<'_> {
        SetChatMemberTagRequest::new(self, chat_id, user_id)
    }

    /// Use this method to change the bot's menu button in a private chat, or the default menu button. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setchatmenubutton
    pub fn set_chat_menu_button(&self) -> SetChatMenuButtonRequest<'_> {
        SetChatMenuButtonRequest::new(self)
    }

    /// Use this method to set default chat permissions for all members. The bot must be an administrator in the group or a supergroup for this to work and must have the can_restrict_members administrator rights. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setchatpermissions
    pub fn set_chat_permissions(
        &self,
        chat_id: impl Into<ChatId>,
        permissions: ChatPermissions,
    ) -> SetChatPermissionsRequest<'_> {
        SetChatPermissionsRequest::new(self, chat_id, permissions)
    }

    /// Use this method to set a new profile photo for the chat. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setchatphoto
    pub fn set_chat_photo(
        &self,
        chat_id: impl Into<ChatId>,
        photo: InputFile,
    ) -> SetChatPhotoRequest<'_> {
        SetChatPhotoRequest::new(self, chat_id, photo)
    }

    /// Use this method to set a new group sticker set for a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field can_set_sticker_set optionally returned in getChat requests to check if the bot can use this method. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setchatstickerset
    pub fn set_chat_sticker_set(
        &self,
        chat_id: impl Into<ChatId>,
        sticker_set_name: impl Into<String>,
    ) -> SetChatStickerSetRequest<'_> {
        SetChatStickerSetRequest::new(self, chat_id, sticker_set_name)
    }

    /// Use this method to change the title of a chat. Titles can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setchattitle
    pub fn set_chat_title(
        &self,
        chat_id: impl Into<ChatId>,
        title: impl Into<String>,
    ) -> SetChatTitleRequest<'_> {
        SetChatTitleRequest::new(self, chat_id, title)
    }

    /// Use this method to set the thumbnail of a custom emoji sticker set. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setcustomemojistickersetthumbnail
    pub fn set_custom_emoji_sticker_set_thumbnail(
        &self,
        name: impl Into<String>,
    ) -> SetCustomEmojiStickerSetThumbnailRequest<'_> {
        SetCustomEmojiStickerSetThumbnailRequest::new(self, name)
    }

    /// Use this method to set the score of the specified user in a game message. On success, if the message is not an inline message, the Message is returned, otherwise True is returned. Returns an error, if the new score is not greater than the user's current score in the chat and force is False.
    /// See: https://core.telegram.org/bots/api#setgamescore
    pub fn set_game_score(&self, user_id: i64, score: i64) -> SetGameScoreRequest<'_> {
        SetGameScoreRequest::new(self, user_id, score)
    }

    /// Use this method to change the chosen reactions on a message. Service messages of some types can't be reacted to. Automatically forwarded messages from a channel to its discussion group have the same available reactions as messages in the channel. Bots can't use paid reactions. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setmessagereaction
    pub fn set_message_reaction(
        &self,
        chat_id: impl Into<ChatId>,
        message_id: i64,
    ) -> SetMessageReactionRequest<'_> {
        SetMessageReactionRequest::new(self, chat_id, message_id)
    }

    /// Use this method to change the list of the bot's commands. See this manual for more details about bot commands. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setmycommands
    pub fn set_my_commands(&self, commands: Vec<BotCommand>) -> SetMyCommandsRequest<'_> {
        SetMyCommandsRequest::new(self, commands)
    }

    /// Use this method to change the default administrator rights requested by the bot when it's added as an administrator to groups or channels. These rights will be suggested to users, but they are free to modify the list before adding the bot. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setmydefaultadministratorrights
    pub fn set_my_default_administrator_rights(
        &self,
    ) -> SetMyDefaultAdministratorRightsRequest<'_> {
        SetMyDefaultAdministratorRightsRequest::new(self)
    }

    /// Use this method to change the bot's description, which is shown in the chat with the bot if the chat is empty. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setmydescription
    pub fn set_my_description(&self) -> SetMyDescriptionRequest<'_> {
        SetMyDescriptionRequest::new(self)
    }

    /// Use this method to change the bot's name. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setmyname
    pub fn set_my_name(&self) -> SetMyNameRequest<'_> {
        SetMyNameRequest::new(self)
    }

    /// Changes the profile photo of the bot. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setmyprofilephoto
    pub fn set_my_profile_photo(&self, photo: InputProfilePhoto) -> SetMyProfilePhotoRequest<'_> {
        SetMyProfilePhotoRequest::new(self, photo)
    }

    /// Use this method to change the bot's short description, which is shown on the bot's profile page and is sent together with the link when users share the bot. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setmyshortdescription
    pub fn set_my_short_description(&self) -> SetMyShortDescriptionRequest<'_> {
        SetMyShortDescriptionRequest::new(self)
    }

    /// Informs a user that some of the Telegram Passport elements they provided contains errors. The user will not be able to re-submit their Passport to you until the errors are fixed (the contents of the field for which you returned the error must change). Returns True on success.
    /// See: https://core.telegram.org/bots/api#setpassportdataerrors
    pub fn set_passport_data_errors(
        &self,
        user_id: i64,
        errors: Vec<PassportElementError>,
    ) -> SetPassportDataErrorsRequest<'_> {
        SetPassportDataErrorsRequest::new(self, user_id, errors)
    }

    /// Use this method to change the list of emoji assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setstickeremojilist
    pub fn set_sticker_emoji_list(
        &self,
        sticker: impl Into<String>,
        emoji_list: Vec<String>,
    ) -> SetStickerEmojiListRequest<'_> {
        SetStickerEmojiListRequest::new(self, sticker, emoji_list)
    }

    /// Use this method to change search keywords assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setstickerkeywords
    pub fn set_sticker_keywords(
        &self,
        sticker: impl Into<String>,
    ) -> SetStickerKeywordsRequest<'_> {
        SetStickerKeywordsRequest::new(self, sticker)
    }

    /// Use this method to change the mask position of a mask sticker. The sticker must belong to a sticker set that was created by the bot. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setstickermaskposition
    pub fn set_sticker_mask_position(
        &self,
        sticker: impl Into<String>,
    ) -> SetStickerMaskPositionRequest<'_> {
        SetStickerMaskPositionRequest::new(self, sticker)
    }

    /// Use this method to move a sticker in a set created by the bot to a specific position. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setstickerpositioninset
    pub fn set_sticker_position_in_set(
        &self,
        sticker: impl Into<String>,
        position: i64,
    ) -> SetStickerPositionInSetRequest<'_> {
        SetStickerPositionInSetRequest::new(self, sticker, position)
    }

    /// Use this method to set the thumbnail of a regular or mask sticker set. The format of the thumbnail file must match the format of the stickers in the set. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setstickersetthumbnail
    pub fn set_sticker_set_thumbnail(
        &self,
        name: impl Into<String>,
        user_id: i64,
        format: impl Into<String>,
    ) -> SetStickerSetThumbnailRequest<'_> {
        SetStickerSetThumbnailRequest::new(self, name, user_id, format)
    }

    /// Use this method to set the title of a created sticker set. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setstickersettitle
    pub fn set_sticker_set_title(
        &self,
        name: impl Into<String>,
        title: impl Into<String>,
    ) -> SetStickerSetTitleRequest<'_> {
        SetStickerSetTitleRequest::new(self, name, title)
    }

    /// Changes the emoji status for a given user that previously allowed the bot to manage their emoji status via the Mini App method requestEmojiStatusAccess. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setuseremojistatus
    pub fn set_user_emoji_status(&self, user_id: i64) -> SetUserEmojiStatusRequest<'_> {
        SetUserEmojiStatusRequest::new(self, user_id)
    }

    /// Use this method to specify a URL and receive incoming updates via an outgoing webhook. Whenever there is an update for the bot, we will send an HTTPS POST request to the specified URL, containing a JSON-serialized Update. In case of an unsuccessful request (a request with response HTTP status code different from 2XY), we will repeat the request and give up after a reasonable amount of attempts. Returns True on success.
    /// See: https://core.telegram.org/bots/api#setwebhook
    pub fn set_webhook(&self, url: impl Into<String>) -> SetWebhookRequest<'_> {
        SetWebhookRequest::new(self, url)
    }

    /// Use this method to stop updating a live location message before live_period expires. On success, if the message is not an inline message, the edited Message is returned, otherwise True is returned.
    /// See: https://core.telegram.org/bots/api#stopmessagelivelocation
    pub fn stop_message_live_location(&self) -> StopMessageLiveLocationRequest<'_> {
        StopMessageLiveLocationRequest::new(self)
    }

    /// Use this method to stop a poll which was sent by the bot. On success, the stopped Poll is returned.
    /// See: https://core.telegram.org/bots/api#stoppoll
    pub fn stop_poll(&self, chat_id: impl Into<ChatId>, message_id: i64) -> StopPollRequest<'_> {
        StopPollRequest::new(self, chat_id, message_id)
    }

    /// Transfers Telegram Stars from the business account balance to the bot's balance. Requires the can_transfer_stars business bot right. Returns True on success.
    /// See: https://core.telegram.org/bots/api#transferbusinessaccountstars
    pub fn transfer_business_account_stars(
        &self,
        business_connection_id: impl Into<String>,
        star_count: i64,
    ) -> TransferBusinessAccountStarsRequest<'_> {
        TransferBusinessAccountStarsRequest::new(self, business_connection_id, star_count)
    }

    /// Transfers an owned unique gift to another user. Requires the can_transfer_and_upgrade_gifts business bot right. Requires can_transfer_stars business bot right if the transfer is paid. Returns True on success.
    /// See: https://core.telegram.org/bots/api#transfergift
    pub fn transfer_gift(
        &self,
        business_connection_id: impl Into<String>,
        owned_gift_id: impl Into<String>,
        new_owner_chat_id: i64,
    ) -> TransferGiftRequest<'_> {
        TransferGiftRequest::new(
            self,
            business_connection_id,
            owned_gift_id,
            new_owner_chat_id,
        )
    }

    /// Use this method to unban a previously banned user in a supergroup or channel. The user will not return to the group or channel automatically, but will be able to join via link, etc. The bot must be an administrator for this to work. By default, this method guarantees that after the call the user is not a member of the chat, but will be able to join it. So if the user is a member of the chat they will also be removed from the chat. If you don't want this, use the parameter only_if_banned. Returns True on success.
    /// See: https://core.telegram.org/bots/api#unbanchatmember
    pub fn unban_chat_member(
        &self,
        chat_id: impl Into<ChatId>,
        user_id: i64,
    ) -> UnbanChatMemberRequest<'_> {
        UnbanChatMemberRequest::new(self, chat_id, user_id)
    }

    /// Use this method to unban a previously banned channel chat in a supergroup or channel. The bot must be an administrator for this to work and must have the appropriate administrator rights. Returns True on success.
    /// See: https://core.telegram.org/bots/api#unbanchatsenderchat
    pub fn unban_chat_sender_chat(
        &self,
        chat_id: impl Into<ChatId>,
        sender_chat_id: i64,
    ) -> UnbanChatSenderChatRequest<'_> {
        UnbanChatSenderChatRequest::new(self, chat_id, sender_chat_id)
    }

    /// Use this method to unhide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights. Returns True on success.
    /// See: https://core.telegram.org/bots/api#unhidegeneralforumtopic
    pub fn unhide_general_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> UnhideGeneralForumTopicRequest<'_> {
        UnhideGeneralForumTopicRequest::new(self, chat_id)
    }

    /// Use this method to clear the list of pinned messages in a chat. In private chats and channel direct messages chats, no additional rights are required to unpin all pinned messages. Conversely, the bot must be an administrator with the 'can_pin_messages' right or the 'can_edit_messages' right to unpin all pinned messages in groups and channels respectively. Returns True on success.
    /// See: https://core.telegram.org/bots/api#unpinallchatmessages
    pub fn unpin_all_chat_messages(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> UnpinAllChatMessagesRequest<'_> {
        UnpinAllChatMessagesRequest::new(self, chat_id)
    }

    /// Use this method to clear the list of pinned messages in a forum topic in a forum supergroup chat or a private chat with a user. In the case of a supergroup chat the bot must be an administrator in the chat for this to work and must have the can_pin_messages administrator right in the supergroup. Returns True on success.
    /// See: https://core.telegram.org/bots/api#unpinallforumtopicmessages
    pub fn unpin_all_forum_topic_messages(
        &self,
        chat_id: impl Into<ChatId>,
        message_thread_id: i64,
    ) -> UnpinAllForumTopicMessagesRequest<'_> {
        UnpinAllForumTopicMessagesRequest::new(self, chat_id, message_thread_id)
    }

    /// Use this method to clear the list of pinned messages in a General forum topic. The bot must be an administrator in the chat for this to work and must have the can_pin_messages administrator right in the supergroup. Returns True on success.
    /// See: https://core.telegram.org/bots/api#unpinallgeneralforumtopicmessages
    pub fn unpin_all_general_forum_topic_messages(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> UnpinAllGeneralForumTopicMessagesRequest<'_> {
        UnpinAllGeneralForumTopicMessagesRequest::new(self, chat_id)
    }

    /// Use this method to remove a message from the list of pinned messages in a chat. In private chats and channel direct messages chats, all messages can be unpinned. Conversely, the bot must be an administrator with the 'can_pin_messages' right or the 'can_edit_messages' right to unpin messages in groups and channels respectively. Returns True on success.
    /// See: https://core.telegram.org/bots/api#unpinchatmessage
    pub fn unpin_chat_message(&self, chat_id: impl Into<ChatId>) -> UnpinChatMessageRequest<'_> {
        UnpinChatMessageRequest::new(self, chat_id)
    }

    /// Upgrades a given regular gift to a unique gift. Requires the can_transfer_and_upgrade_gifts business bot right. Additionally requires the can_transfer_stars business bot right if the upgrade is paid. Returns True on success.
    /// See: https://core.telegram.org/bots/api#upgradegift
    pub fn upgrade_gift(
        &self,
        business_connection_id: impl Into<String>,
        owned_gift_id: impl Into<String>,
    ) -> UpgradeGiftRequest<'_> {
        UpgradeGiftRequest::new(self, business_connection_id, owned_gift_id)
    }

    /// Use this method to upload a file with a sticker for later use in the createNewStickerSet, addStickerToSet, or replaceStickerInSet methods (the file can be used multiple times). Returns the uploaded File on success.
    /// See: https://core.telegram.org/bots/api#uploadstickerfile
    pub fn upload_sticker_file(
        &self,
        user_id: i64,
        sticker: InputFile,
        sticker_format: impl Into<String>,
    ) -> UploadStickerFileRequest<'_> {
        UploadStickerFileRequest::new(self, user_id, sticker, sticker_format)
    }

    /// Verifies a chat on behalf of the organization which is represented by the bot. Returns True on success.
    /// See: https://core.telegram.org/bots/api#verifychat
    pub fn verify_chat(&self, chat_id: impl Into<ChatId>) -> VerifyChatRequest<'_> {
        VerifyChatRequest::new(self, chat_id)
    }

    /// Verifies a user on behalf of the organization which is represented by the bot. Returns True on success.
    /// See: https://core.telegram.org/bots/api#verifyuser
    pub fn verify_user(&self, user_id: i64) -> VerifyUserRequest<'_> {
        VerifyUserRequest::new(self, user_id)
    }
}
