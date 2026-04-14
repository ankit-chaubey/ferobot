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

//! Dispatcher - routes updates to handler groups in ascending order.
//!
//! Within a group the first matching handler runs; dispatcher then moves to the next group.
//! Handlers can alter flow by returning `Err(ContinueGroups)` or `Err(EndGroups)`.
//!

use std::{collections::BTreeMap, error::Error, panic::AssertUnwindSafe, sync::Arc};

use arc_swap::ArcSwap;
use futures::FutureExt as _;
use tokio::sync::Semaphore;
use tracing::{debug, error, warn};
#[cfg(feature = "per-chat")]
use {dashmap::DashMap, tokio::sync::mpsc};

use crate::{
    framework::{
        context::Context,
        handler::{ContinueGroups, EndGroups, Handler},
    },
    types::Update,
    Bot,
};

/// What the dispatcher does after an error hook returns.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DispatcherAction {
    #[default]
    Noop, // stop current group, move to next
    ContinueGroups, // keep iterating handlers in current group
    EndGroups,      // abort all groups
}

pub type ErrorHook = Arc<
    dyn std::ops::Fn(&Bot, &Context, &(dyn Error + Send + Sync)) -> DispatcherAction + Send + Sync,
>;

pub type PanicHook = Arc<dyn std::ops::Fn(&Bot, &Context, String) + Send + Sync>;

/// Options for [`Dispatcher::new`].
#[derive(Clone, Default)]
pub struct DispatcherOpts {
    pub max_routines: Option<usize>,
    pub error_handler: Option<ErrorHook>,
    pub panic_handler: Option<PanicHook>,
    /// Middleware chain run before and after every update.
    pub middleware: Vec<crate::middleware::ArcMiddleware>,
    /// Route updates to per-chat sequential workers.
    ///
    /// When enabled, updates for the same `chat_id` are processed one-at-a-time
    /// in arrival order, while different chats run in parallel. This prevents
    /// message-ordering bugs under high concurrency.
    ///
    /// Requires the `per-chat` feature and a multi-threaded tokio runtime.
    #[cfg(feature = "per-chat")]
    pub per_chat_concurrency: bool,
    /// Per-chat worker channel buffer size (default: 256).
    #[cfg(feature = "per-chat")]
    pub per_chat_queue_size: usize,
}

impl DispatcherOpts {
    pub fn max_routines(mut self, n: usize) -> Self {
        self.max_routines = Some(n);
        self
    }

    pub fn on_error<F>(mut self, f: F) -> Self
    where
        F: std::ops::Fn(&Bot, &Context, &(dyn Error + Send + Sync)) -> DispatcherAction
            + Send
            + Sync
            + 'static,
    {
        self.error_handler = Some(Arc::new(f));
        self
    }

    pub fn on_panic<F>(mut self, f: F) -> Self
    where
        F: std::ops::Fn(&Bot, &Context, String) + Send + Sync + 'static,
    {
        self.panic_handler = Some(Arc::new(f));
        self
    }

    /// Enable per-chat sequential concurrency (requires `per-chat` feature).
    #[cfg(feature = "per-chat")]
    pub fn per_chat_concurrency(mut self) -> Self {
        self.per_chat_concurrency = true;
        self
    }

    /// Append a middleware to the chain.
    ///
    /// Middleware run in registration order before any handler, and in reverse
    /// order after. Return `false` from [`Middleware::before`] to drop the update.
    ///
    /// ```rust,no_run
    /// use ferobot::{DispatcherOpts, middleware::LoggingMiddleware};
    ///
    /// let opts = DispatcherOpts::default()
    ///     .middleware(LoggingMiddleware);
    /// ```
    pub fn middleware(mut self, m: impl crate::middleware::Middleware) -> Self {
        self.middleware.push(std::sync::Arc::new(m));
        self
    }

    /// Set the per-chat worker channel buffer depth (default: `256`).
    ///
    /// When a chat's buffer is full the excess update falls through to
    /// the standard concurrent pool so no updates are dropped.
    /// Requires the `per-chat` feature.
    #[cfg(feature = "per-chat")]
    pub fn per_chat_queue(mut self, size: usize) -> Self {
        self.per_chat_queue_size = if size == 0 { 256 } else { size };
        self
    }
}

type HandlerMap = BTreeMap<i32, Vec<Arc<dyn Handler>>>;

#[cfg(feature = "per-chat")]
struct ChatWork {
    bot: Bot,
    update: Update,
}

pub struct Dispatcher {
    handlers: Arc<ArcSwap<HandlerMap>>,
    error_handler: Option<ErrorHook>,
    panic_handler: Option<PanicHook>,
    semaphore: Option<Arc<Semaphore>>,
    middleware: Vec<crate::middleware::ArcMiddleware>,
    #[cfg(feature = "per-chat")]
    chat_workers: Option<Arc<DashMap<i64, mpsc::Sender<ChatWork>>>>,
    #[cfg(feature = "per-chat")]
    per_chat_queue_size: usize,
}

impl Dispatcher {
    pub fn new(opts: DispatcherOpts) -> Self {
        Self {
            handlers: Arc::new(ArcSwap::from_pointee(BTreeMap::new())),
            error_handler: opts.error_handler,
            panic_handler: opts.panic_handler,
            semaphore: opts.max_routines.map(|n| Arc::new(Semaphore::new(n))),
            middleware: opts.middleware,
            #[cfg(feature = "per-chat")]
            chat_workers: {
                #[allow(clippy::needless_bool)]
                if opts.per_chat_concurrency {
                    Some(Arc::new(DashMap::new()))
                } else {
                    None
                }
            },
            #[cfg(feature = "per-chat")]
            per_chat_queue_size: if opts.per_chat_queue_size == 0 {
                256
            } else {
                opts.per_chat_queue_size
            },
        }
    }

    pub fn add_handler<H: Handler + 'static>(&mut self, handler: H) {
        self.add_handler_to_group(handler, 0);
    }

    pub fn add_handler_to_group<H: Handler + 'static>(&mut self, handler: H, group: i32) {
        // Create the Arc once outside the rcu closure so retries don't double-allocate.
        let h: Arc<dyn Handler> = Arc::new(handler);
        self.handlers.rcu(|map| {
            let mut m: HandlerMap = (**map).clone();
            m.entry(group).or_default().push(Arc::clone(&h));
            m
        });
    }

    pub fn remove_handler(&mut self, name: &str, group: i32) -> bool {
        let mut removed = false;
        self.handlers.rcu(|map| {
            let mut m: HandlerMap = (**map).clone();
            if let Some(vec) = m.get_mut(&group) {
                if let Some(pos) = vec.iter().position(|h| h.name() == name) {
                    vec.remove(pos);
                    removed = true;
                }
            }
            m
        });
        removed
    }

    pub fn remove_handler_any_group(&mut self, name: &str) -> Option<i32> {
        let mut found_group: Option<i32> = None;
        self.handlers.rcu(|map| {
            let mut m: HandlerMap = (**map).clone();
            'search: for (&group, vec) in m.iter_mut() {
                if let Some(pos) = vec.iter().position(|h| h.name() == name) {
                    vec.remove(pos);
                    found_group = Some(group);
                    break 'search;
                }
            }
            m
        });
        found_group
    }

    pub fn remove_group(&mut self, group: i32) -> bool {
        let mut removed = false;
        self.handlers.rcu(|map| {
            let mut m: HandlerMap = (**map).clone();
            removed = m.remove(&group).is_some();
            m
        });
        removed
    }

    /// Dispatch an update. The update is handled in a spawned task.
    pub fn dispatch(&self, bot: Bot, update: Update) {
        let handlers_arc = Arc::clone(&self.handlers);
        let error_hook = self.error_handler.clone();
        let panic_hook = self.panic_handler.clone();
        let semaphore = self.semaphore.clone();
        let middleware = self.middleware.clone();

        // per-chat routing send to per-chat sequential worker if enabled.
        #[cfg(feature = "per-chat")]
        if let Some(ref workers) = self.chat_workers {
            let chat_id = Context::new(update.clone()).effective_chat().map(|c| c.id);
            if let Some(id) = chat_id {
                let queue_sz = self.per_chat_queue_size;
                let tx = get_or_spawn_worker(
                    workers,
                    id,
                    Arc::clone(&handlers_arc),
                    error_hook.clone(),
                    panic_hook.clone(),
                    semaphore.clone(),
                    middleware.clone(),
                    queue_sz,
                );
                // try_send is non-blocking: if the 256-slot buffer is full the
                // update falls through to regular parallel dispatch below.
                if tx
                    .try_send(ChatWork {
                        bot: bot.clone(),
                        update: update.clone(),
                    })
                    .is_ok()
                {
                    return;
                }
            }
        }

        tokio::spawn(async move {
            // Run before-hooks; any hook returning false drops the update.
            if !crate::middleware::run_before(&middleware, &bot, &update).await {
                return;
            }

            let _permit = if let Some(sem) = &semaphore {
                Some(sem.clone().acquire_owned().await.ok())
            } else {
                None
            };

            let ctx = Context::new(update.clone());
            let snapshot = handlers_arc.load_full();
            run_handlers(snapshot, bot.clone(), ctx, error_hook, panic_hook).await;

            // Run after-hooks regardless of handler outcome.
            crate::middleware::run_after(&middleware, &bot, &update).await;
        });
    }

    /// Run an update synchronously in the calling task (no panic recovery).
    ///
    /// Useful for tests and for wrappers that already manage their own task.
    pub async fn process_update(&self, bot: &Bot, update: Update) {
        let ctx = Context::new(update);
        let snapshot = self.handlers.load_full();

        'groups: for (_, handlers) in snapshot.iter() {
            for handler in handlers {
                if !handler.check_update(&ctx) {
                    continue;
                }
                match handler.handle_update(bot.clone(), ctx.clone()).await {
                    Err(e) if e.is::<ContinueGroups>() => continue,
                    Err(e) if e.is::<EndGroups>() => break 'groups,
                    Err(e) => {
                        let action = self
                            .error_handler
                            .as_ref()
                            .map(|h| h(bot, &ctx, e.as_ref()))
                            .unwrap_or_default();
                        match action {
                            DispatcherAction::Noop => break,
                            DispatcherAction::ContinueGroups => continue,
                            DispatcherAction::EndGroups => break 'groups,
                        }
                    }
                    Ok(()) => break,
                }
            }
        }
    }
}

/// Execute the handler chain for one update.
/// Uses `catch_unwind` to isolate panics without spawning an extra task.
async fn run_handlers(
    snapshot: Arc<HandlerMap>,
    bot: Bot,
    ctx: Context,
    error_hook: Option<ErrorHook>,
    panic_hook: Option<PanicHook>,
) {
    'groups: for (group, handlers) in snapshot.iter() {
        for handler in handlers {
            if !handler.check_update(&ctx) {
                continue;
            }

            debug!(handler = handler.name(), group, "matched");

            let h = Arc::clone(handler);
            let bot2 = bot.clone();
            let ctx2 = ctx.clone();

            let result = AssertUnwindSafe(async move { h.handle_update(bot2, ctx2).await })
                .catch_unwind()
                .await;

            match result {
                Err(panic_payload) => {
                    let msg = panic_payload
                        .downcast::<String>()
                        .map(|s| *s)
                        .or_else(|p| p.downcast::<&str>().map(|s| s.to_string()))
                        .unwrap_or_else(|_| "<non-string panic>".into());
                    if let Some(ref hook) = panic_hook {
                        hook(&bot, &ctx, msg);
                    } else {
                        error!(handler = handler.name(), group, panic = %msg, "panicked");
                    }
                    break; // stop this group, let other groups continue
                }
                Ok(Err(e)) => {
                    if e.is::<ContinueGroups>() {
                        debug!(handler = handler.name(), "ContinueGroups");
                        continue;
                    }
                    if e.is::<EndGroups>() {
                        debug!(handler = handler.name(), "EndGroups");
                        break 'groups;
                    }
                    warn!(handler = handler.name(), group, error = %e);
                    let action = error_hook
                        .as_ref()
                        .map(|h| h(&bot, &ctx, e.as_ref()))
                        .unwrap_or_default();
                    match action {
                        DispatcherAction::Noop => break,
                        DispatcherAction::ContinueGroups => continue,
                        DispatcherAction::EndGroups => break 'groups,
                    }
                }
                Ok(Ok(())) => {
                    debug!(handler = handler.name(), group, "ok");
                    break;
                }
            }
        }
    }
}

/// Return the live sender for `chat_id`, spawning a new sequential worker if
/// the slot is empty or the previous worker has exited.
///
/// Each worker task processes updates for one chat sequentially (one `.await`
/// after another), so message order is always preserved. Chats run fully in
/// parallel with each other.
#[cfg(feature = "per-chat")]
fn get_or_spawn_worker(
    workers: &Arc<DashMap<i64, mpsc::Sender<ChatWork>>>,
    chat_id: i64,
    handlers_arc: Arc<ArcSwap<HandlerMap>>,
    error_hook: Option<ErrorHook>,
    panic_hook: Option<PanicHook>,
    semaphore: Option<Arc<Semaphore>>,
    middleware: Vec<crate::middleware::ArcMiddleware>,
    queue_size: usize,
) -> mpsc::Sender<ChatWork> {
    // Fast path: a live worker already exists for this chat.
    if let Some(entry) = workers.get(&chat_id) {
        if !entry.value().is_closed() {
            return entry.value().clone();
        }
    }

    // Slow path: spawn a fresh sequential worker task.
    let (tx, mut rx) = mpsc::channel::<ChatWork>(queue_size);
    workers.insert(chat_id, tx.clone());

    let workers_weak = Arc::downgrade(workers);
    tokio::spawn(async move {
        while let Some(work) = rx.recv().await {
            // Run before-middleware; drop update if any hook returns false.
            if !crate::middleware::run_before(&middleware, &work.bot, &work.update).await {
                continue;
            }

            // Each update in this chat is processed one at a time.
            let _permit = if let Some(sem) = &semaphore {
                Some(sem.clone().acquire_owned().await.ok())
            } else {
                None
            };

            let ctx = Context::new(work.update.clone());
            let snapshot = handlers_arc.load_full();
            run_handlers(
                snapshot,
                work.bot.clone(),
                ctx,
                error_hook.clone(),
                panic_hook.clone(),
            )
            .await;

            crate::middleware::run_after(&middleware, &work.bot, &work.update).await;
        }

        // all senders dropped, channel closed; remove the entry so the next
        // update for this chat spawns a fresh worker
        if let Some(w) = workers_weak.upgrade() {
            w.remove(&chat_id);
        }
    });

    tx
}
