# ARCHITECTURE NOTE (updated):
# gen_methods.rs methods are now PUBLIC and have clean names (no _with_params / _exec suffix).
# The fluent builder layer (fluent.rs) has been removed.
# Public API: bot.method_name(required_args, params: Option<MethodParams>)
# Ergonomic helper: p!(MethodParams { field: value }) macro in macros.rs
# To regenerate: run this script, then apply:
#   s/pub\(crate\) async fn (\w+)_with_params/pub async fn /g
#   s/pub\(crate\) async fn (\w+)_exec/pub async fn /g
# (or update the codegen templates directly to emit pub async fn with clean names)

#!/usr/bin/env python3
"""
ferobot - Code Generator

Generates Rust source files from the Telegram Bot API spec (api.json).

Spec source: https://github.com/ankit-chaubey/api-spec
Project:     https://github.com/ankit-chaubey/ferobot
Author:      Ankit Chaubey <ankitchaubey.dev@gmail.com>
License:     MIT

Usage:
    python3 codegen.py <api.json> <output_directory>

Example:
    python3 codegen/codegen.py api.json ferobot/src/

Generates:
    gen_types.rs   - All Telegram Bot API types
    gen_methods.rs - All Telegram Bot API methods (raw, with _with_params suffix for those that have options)
    fluent.rs      - Fluent IntoFuture builder wrappers for EVERY method

No external dependencies required. Pure Python 3.6+.
"""

import json
import re
import sys
import os
from pathlib import Path

COPYRIGHT_HEADER = """\
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
"""

# Methods whose bare name is reserved for the fluent wrapper.
# gen_methods.rs generates these with a _with_params suffix.
# This set is computed automatically from the spec - all methods that have
# optional fields. Do NOT edit by hand; it is rebuilt on every codegen run.
FLUENT_METHODS = set()  # populated in main() from the spec

SKIP_TYPES = {
    "InputFile",   # rich enum in ferobot/src/input_file.rs
    "InputMedia",  # ergonomic wrapper enum in ferobot/src/lib.rs
}

FORCE_DEFAULT = {
    "ForceReply",
    "InputMediaAnimation",
    "InputMediaAudio",
    "InputMediaDocument",
    "InputMediaPhoto",
    "InputMediaVideo",
    "InputPollOption",
    "InputTextMessageContent",
    "KeyboardButton",
    "ReplyKeyboardMarkup",
    "ReplyParameters",
    "WriteAccessAllowed",
}

def load_spec(path):
    with open(path) as f:
        return json.load(f)

def snake_case(name):
    s = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', name)
    s = re.sub(r'([a-z0-9])([A-Z])', r'\1_\2', s)
    return s.lower()

def pascal_case(name):
    """camelCase or snake_case -> PascalCase"""
    if '_' in name:
        return ''.join(w.capitalize() for w in name.split('_'))
    return name[0].upper() + name[1:]

def safe_field_name(name):
    keywords = {'type', 'self', 'move', 'use', 'in', 'fn', 'let', 'mut', 'ref',
                'where', 'loop', 'if', 'else', 'match', 'return'}
    s = snake_case(name)
    return 'r#' + s if s in keywords else s

def method_fn_name(name):
    return snake_case(name)

def method_params_struct(name):
    return name[0].upper() + name[1:] + 'Params'

def request_struct_name(method_name):
    return pascal_case(method_name) + 'Request'

BASE_TYPE_MAP = {
    'Integer': 'i64',
    'Float': 'f64',
    'Boolean': 'bool',
    'String': 'String',
    'InputFile': 'InputFile',
}

def is_array(t):
    return t.startswith('Array of ')

def strip_array(t):
    return t[len('Array of '):]

def tg_to_rust(t, optional, types_map):
    if is_array(t):
        inner = strip_array(t)
        inner_rust = tg_to_rust(inner, False, types_map)
        rust = f'Vec<{inner_rust}>'
        return f'Option<{rust}>' if optional else rust
    base = BASE_TYPE_MAP.get(t, t)
    if optional:
        tg = types_map.get(t)
        if tg and t not in BASE_TYPE_MAP:
            return f'Option<Box<{base}>>'
        return f'Option<{base}>'
    return base

def field_rust_type(field, types_map):
    types = field['types']
    required = field['required']
    name = field['name']
    if len(types) == 0:
        return 'serde_json::Value'
    if len(types) == 1:
        return tg_to_rust(types[0], not required, types_map)
    sorted_types = sorted(types)
    if sorted_types == ['Integer', 'String']:
        return 'ChatId' if required else 'Option<ChatId>'
    if 'InputFile' in types and 'String' in types and len(types) == 2:
        return 'InputFileOrString' if required else 'Option<InputFileOrString>'
    if name == 'reply_markup' and len(types) >= 2:
        return 'ReplyMarkup' if required else 'Option<ReplyMarkup>'
    if name == 'media':
        if any('InputMedia' in t for t in types) or any('InputPaidMedia' in t for t in types):
            return 'InputMedia' if required else 'Option<InputMedia>'
    return tg_to_rust(types[0], not required, types_map)

def opt_wrap(rust_type, optional):
    if optional and not rust_type.startswith('Option<'):
        return f'Option<{rust_type}>'
    return rust_type

def return_rust_type(returns, types_map):
    if not returns:
        return 'bool'
    if len(returns) == 1:
        t = returns[0]
        return tg_to_rust(t, False, types_map)
    return 'serde_json::Value'

def doc_comment(lines, indent=''):
    return '\n'.join(f'{indent}/// {line}' for line in lines)

# Strip Option<> wrapper from a Rust type string
def unwrap_option(t):
    if t.startswith('Option<') and t.endswith('>'):
        return t[7:-1]
    return t

# Strip Box<> wrapper
def unwrap_box(t):
    if t.startswith('Box<') and t.endswith('>'):
        return t[4:-1]
    return t

def generate_setter(fname, rust_type_with_option):
    """Generate a fluent setter method for an optional param field."""
    inner = unwrap_option(rust_type_with_option)

    if inner.startswith('Box<'):
        inner_unboxed = unwrap_box(inner)
        # Special ergonomic: accept T, auto-box
        return (
            f'    pub fn {fname}(mut self, v: {inner_unboxed}) -> Self {{\n'
            f'        self.params.{fname} = Some(Box::new(v));\n'
            f'        self\n'
            f'    }}'
        )
    elif inner == 'String':
        return (
            f'    pub fn {fname}(mut self, v: impl Into<String>) -> Self {{\n'
            f'        self.params.{fname} = Some(v.into());\n'
            f'        self\n'
            f'    }}'
        )
    elif inner in ('bool', 'i64', 'f64'):
        return (
            f'    pub fn {fname}(mut self, v: {inner}) -> Self {{\n'
            f'        self.params.{fname} = Some(v);\n'
            f'        self\n'
            f'    }}'
        )
    elif inner == 'ChatId':
        return (
            f'    pub fn {fname}(mut self, v: impl Into<ChatId>) -> Self {{\n'
            f'        self.params.{fname} = Some(v.into());\n'
            f'        self\n'
            f'    }}'
        )
    elif inner == 'InputFileOrString':
        return (
            f'    pub fn {fname}(mut self, v: impl Into<InputFileOrString>) -> Self {{\n'
            f'        self.params.{fname} = Some(v.into());\n'
            f'        self\n'
            f'    }}'
        )
    elif inner == 'ReplyMarkup':
        return (
            f'    pub fn {fname}(mut self, v: impl Into<ReplyMarkup>) -> Self {{\n'
            f'        self.params.{fname} = Some(v.into());\n'
            f'        self\n'
            f'    }}'
        )
    elif inner == 'InputMedia':
        return (
            f'    pub fn {fname}(mut self, v: InputMedia) -> Self {{\n'
            f'        self.params.{fname} = Some(v);\n'
            f'        self\n'
            f'    }}'
        )
    else:
        # Generic fallback - pass T directly
        return (
            f'    pub fn {fname}(mut self, v: {inner}) -> Self {{\n'
            f'        self.params.{fname} = Some(v);\n'
            f'        self\n'
            f'    }}'
        )

# -------------------------------------------------------------------------
# generate_types  (unchanged from original)
# -------------------------------------------------------------------------

def generate_types(spec):
    types_map = spec['types']
    version = spec['version']
    lines = []
    lines.append(COPYRIGHT_HEADER.rstrip())
    lines.append('')
    lines.append(f'// THIS FILE IS AUTO-GENERATED. DO NOT EDIT.')
    lines.append(f'// Generated from Telegram Bot API {version}')
    lines.append(f'// Spec:    https://github.com/ankit-chaubey/api-spec')
    lines.append(f'// Project: https://github.com/ankit-chaubey/ferobot')
    lines.append(f'// Author:  Ankit Chaubey <ankitchaubey.dev@gmail.com>')
    lines.append(f'// License: MIT')
    lines.append(f'// See:     https://core.telegram.org/bots/api')
    lines.append(f'')
    lines.append(f'#![allow(clippy::all, dead_code, unused_imports)]')
    lines.append(f'')
    lines.append(f'use serde::{{Deserialize, Serialize}};')
    lines.append(f'#[rustfmt::skip]')
    lines.append(f'use crate::{{ChatId, InputFile, InputFileOrString, ReplyMarkup, InputMedia}};')
    lines.append(f'')
    for type_name in sorted(types_map.keys()):
        if type_name in SKIP_TYPES:
            continue
        tg_type = types_map[type_name]
        docs = tg_type.get('description', [])
        href = tg_type.get('href', '')
        subtypes = tg_type.get('subtypes', [])
        fields = tg_type.get('fields', [])
        lines.append(doc_comment(docs))
        lines.append(f'/// {href}')
        if subtypes:
            lines.append('#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]')
            lines.append('#[serde(untagged)]')
            lines.append(f'pub enum {type_name} {{')
            for variant in subtypes:
                lines.append(f'    {variant}({variant}),')
            lines.append('}')
            lines.append('')
        elif not fields:
            lines.append('#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]')
            lines.append(f'pub struct {type_name} {{}}')
            lines.append('')
        else:
            all_fields_optional = all(
                field_rust_type(field, types_map).startswith('Option<')
                for field in fields
            ) if fields else False
            want_default = all_fields_optional or type_name in FORCE_DEFAULT
            derive_line = '#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]' \
                if want_default else \
                '#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]'
            lines.append(derive_line)
            lines.append(f'pub struct {type_name} {{')
            for field in fields:
                fname = safe_field_name(field['name'])
                fdesc = field['description'].replace('\n', ' ')
                ftype = field_rust_type(field, types_map)
                lines.append(f'    /// {fdesc}')
                if fname != field['name']:
                    lines.append(f'    #[serde(rename = "{field["name"]}")]')
                if ftype.startswith('Option<'):
                    lines.append(f'    #[serde(skip_serializing_if = "Option::is_none")]')
                    lines.append(f'    pub {fname}: {ftype},')
                else:
                    lines.append(f'    pub {fname}: {ftype},')
            lines.append('}')
            lines.append('')
    return '\n'.join(lines)

# -------------------------------------------------------------------------
# generate_methods  (updated: FLUENT_METHODS now auto-computed = all with opts)
# -------------------------------------------------------------------------

def generate_methods(spec):
    types_map = spec['types']
    methods_map = spec['methods']
    version = spec['version']
    lines = []
    lines.append(COPYRIGHT_HEADER.rstrip())
    lines.append('')
    lines.append(f'// THIS FILE IS AUTO-GENERATED. DO NOT EDIT.')
    lines.append(f'// Generated from Telegram Bot API {version}')
    lines.append(f'// Spec:    https://github.com/ankit-chaubey/api-spec')
    lines.append(f'// Project: https://github.com/ankit-chaubey/ferobot')
    lines.append(f'// Author:  Ankit Chaubey <ankitchaubey.dev@gmail.com>')
    lines.append(f'// License: MIT')
    lines.append(f'// See:     https://core.telegram.org/bots/api')
    lines.append(f'')
    lines.append(f'#![allow(clippy::all, dead_code, unused_imports, unused_mut)]')
    lines.append(f'')
    lines.append(f'use serde::{{Deserialize, Serialize}};')
    lines.append(f'use crate::types::*;')
    lines.append(f'#[rustfmt::skip]')
    lines.append(f'use crate::{{Bot, BotError, ChatId, InputFile, InputFileOrString, ReplyMarkup, InputMedia}};')
    lines.append(f'')

    for method_name in sorted(methods_map.keys()):
        method = methods_map[method_name]
        fn_name = method_fn_name(method_name)
        has_opts = bool([f for f in method.get('fields', []) if not f['required']])
        if has_opts:
            fn_name = fn_name + "_with_params"
        params_name = method_params_struct(method_name)
        docs = method.get('description', [])
        href = method.get('href', '')
        all_fields = method.get('fields', [])
        returns = method.get('returns', [])
        required_fields = [f for f in all_fields if f['required']]
        optional_fields = [f for f in all_fields if not f['required']]

        if optional_fields:
            lines.append(f'/// Optional parameters for [`Bot::{fn_name}`]')
            lines.append('#[derive(Debug, Clone, Serialize, Deserialize, Default)]')
            lines.append(f'pub struct {params_name} {{')
            for field in optional_fields:
                fname = safe_field_name(field['name'])
                fdesc = field['description'].replace('\n', ' ')
                ftype = opt_wrap(field_rust_type(field, types_map), True)
                lines.append(f'    /// {fdesc}')
                if fname != field['name']:
                    lines.append(f'    #[serde(rename = "{field["name"]}")]')
                lines.append(f'    #[serde(skip_serializing_if = "Option::is_none")]')
                lines.append(f'    pub {fname}: {ftype},')
            lines.append('}')
            lines.append('')
            lines.append(f'impl {params_name} {{')
            lines.append(f'    pub fn new() -> Self {{ Self::default() }}')
            for field in optional_fields:
                fname = safe_field_name(field['name'])
                ftype = opt_wrap(field_rust_type(field, types_map), True)
                inner_type = unwrap_option(ftype)
                lines.append(f'    pub fn {fname}(mut self, v: impl Into<{inner_type}>) -> Self {{ self.{fname} = Some(v.into()); self }}')
            lines.append('}')
            lines.append('')

        ret = return_rust_type(returns, types_map)
        file_field = None
        for field in required_fields:
            if field_rust_type(field, types_map) == 'InputFileOrString':
                file_field = field['name']
                break

        sig_parts = []
        for field in required_fields:
            fname = safe_field_name(field['name'])
            ftype = field_rust_type(field, types_map)
            if ftype == 'String':
                sig_parts.append(f'{fname}: impl Into<String>')
            elif ftype == 'ChatId':
                sig_parts.append(f'{fname}: impl Into<ChatId>')
            elif ftype == 'InputFileOrString':
                sig_parts.append(f'{fname}: impl Into<InputFileOrString>')
            elif ftype == 'InputMedia':
                sig_parts.append(f'{fname}: Vec<InputMedia>')
            else:
                sig_parts.append(f'{fname}: {ftype}')
        if optional_fields:
            sig_parts.append(f'params: Option<{params_name}>')
        sig = ', '.join(sig_parts)

        lines.append(f'impl Bot {{')
        lines.append(doc_comment(docs, '    '))
        lines.append(f'    /// See: {href}')
        args = f'&self, {sig}' if sig else '&self'
        # Methods without optional params share their name with the fluent builder
        # in impl Bot, so emit them as pub(crate) raw_ to avoid duplicate definitions.
        if not has_opts:
            emit_fn_name = f'raw_{fn_name}'
            visibility = 'pub(crate)'
        else:
            emit_fn_name = fn_name
            visibility = 'pub'
        lines.append(f'    {visibility} async fn {emit_fn_name}({args}) -> Result<{ret}, BotError> {{')
        lines.append(f'        let mut req = serde_json::Map::new();')
        for field in required_fields:
            fname = safe_field_name(field['name'])
            ftype = field_rust_type(field, types_map)
            if ftype == 'InputFileOrString':
                pass
            elif ftype == 'InputMedia':
                lines.append(f'        req.insert("{field["name"]}".into(), serde_json::to_value(&{fname}).unwrap_or_default());')
            else:
                expr = f'{fname}.into()' if ftype in ('String', 'ChatId') else fname
                lines.append(f'        req.insert("{field["name"]}".into(), serde_json::to_value({expr}).unwrap_or_default());')
        if optional_fields:
            lines.append(f'        if let Some(p) = params {{')
            lines.append(f'            let extra = serde_json::to_value(&p).unwrap_or_default();')
            lines.append(f'            if let serde_json::Value::Object(m) = extra {{')
            lines.append(f'                for (k, v) in m {{ if !v.is_null() {{ req.insert(k, v); }} }}')
            lines.append(f'            }}')
            lines.append(f'        }}')
        if file_field:
            fn_arg = safe_field_name(file_field)
            lines.append(f'        self.call_api_with_file(\"{method_name}\", req, \"{file_field}\", {fn_arg}.into())')
            lines.append(f'            .await')
        else:
            lines.append(f'        self.call_api(\"{method_name}\", serde_json::Value::Object(req)).await')
        lines.append(f'    }}')
        lines.append(f'}}')
        lines.append(f'')
    return '\n'.join(lines)

# -------------------------------------------------------------------------
# generate_fluent  (NEW - generates fluent wrappers for ALL 169 methods)
# -------------------------------------------------------------------------

def generate_fluent(spec):
    types_map = spec['types']
    methods_map = spec['methods']
    version = spec['version']
    lines = []

    lines.append(COPYRIGHT_HEADER.rstrip())
    lines.append('')
    lines.append(f'// THIS FILE IS AUTO-GENERATED. DO NOT EDIT.')
    lines.append(f'// Generated from Telegram Bot API {version}')
    lines.append(f'// Spec:    https://github.com/ankit-chaubey/api-spec')
    lines.append(f'// Project: https://github.com/ankit-chaubey/ferobot')
    lines.append(f'// Author:  Ankit Chaubey <ankitchaubey.dev@gmail.com>')
    lines.append(f'// License: MIT')
    lines.append(f'// See:     https://core.telegram.org/bots/api')
    lines.append('')
    lines.append('#![allow(clippy::all, unused_imports)]')
    lines.append('')
    lines.append('//! Fluent builder API for every Telegram Bot API method.')
    lines.append('//!')
    lines.append('//! Every `bot.method(required_args)` call returns a `MethodRequest` builder')
    lines.append('//! that implements [`IntoFuture`]. Chain optional parameters then `.await`:')
    lines.append('//!')
    lines.append('//! ```rust,no_run')
    lines.append('//! # #[tokio::main]')
    lines.append('//! # async fn main() -> Result<(), ferobot::BotError> {')
    lines.append('//! use ferobot::{Bot, InputFile};')
    lines.append('//! let bot = Bot::new("TOKEN").await?;')
    lines.append('//!')
    lines.append('//! // No options needed - just await')
    lines.append('//! let me = bot.get_me().await?;')
    lines.append('//!')
    lines.append('//! // Chain options before await')
    lines.append('//! bot.send_message(123_i64, "Hello!")')
    lines.append('//!    .html()')
    lines.append('//!    .silent()')
    lines.append('//!    .reply_to(42)')
    lines.append('//!    .await?;')
    lines.append('//!')
    lines.append('//! // Works for every method')
    lines.append('//! bot.send_video(123_i64, InputFile::memory("v.mp4", vec![]))')
    lines.append('//!    .caption("Watch this")')
    lines.append('//!    .duration(30)')
    lines.append('//!    .silent()')
    lines.append('//!    .await?;')
    lines.append('//!')
    lines.append('//! bot.ban_chat_member(123_i64, 456_i64)')
    lines.append('//!    .revoke_messages(true)')
    lines.append('//!    .await?;')
    lines.append('//! # Ok(())')
    lines.append('//! # }')
    lines.append('//! ```')
    lines.append('')
    lines.append('use std::future::IntoFuture;')
    lines.append('use std::pin::Pin;')
    lines.append('')
    lines.append('#[rustfmt::skip]')
    lines.append('use crate::{Bot, BotError, ChatId, InputFile, InputFileOrString, ReplyMarkup, InputMedia};')
    lines.append('use crate::gen_methods::*;')
    lines.append('#[allow(unused_imports)]')
    lines.append('use crate::types::*;')
    lines.append('')
    # Helper fn
    lines.append('fn make_reply_params(message_id: i64) -> Box<ReplyParameters> {')
    lines.append('    Box::new(ReplyParameters {')
    lines.append('        message_id,')
    lines.append('        ..Default::default()')
    lines.append('    })')
    lines.append('}')
    lines.append('')

    # ---- generate per-method Request structs ----
    bot_impl_lines = ['impl Bot {']

    for method_name in sorted(methods_map.keys()):
        method = methods_map[method_name]
        fn_name = method_fn_name(method_name)
        struct_name = request_struct_name(method_name)
        params_name = method_params_struct(method_name)
        all_fields = method.get('fields', [])
        returns = method.get('returns', [])
        docs = method.get('description', [])
        href = method.get('href', '')
        required_fields = [f for f in all_fields if f['required']]
        optional_fields = [f for f in all_fields if not f['required']]
        has_opts = bool(optional_fields)
        ret = return_rust_type(returns, types_map)

        # Determine the underlying fn name in gen_methods
        raw_fn = fn_name + '_with_params' if has_opts else 'raw_' + fn_name

        # Identify file upload field
        file_field = None
        for field in required_fields:
            if field_rust_type(field, types_map) == 'InputFileOrString':
                file_field = field['name']
                break

        # --- Struct definition ---
        lines.append(f'/// Fluent builder for [`Bot::{fn_name}`]. Implements [`IntoFuture`].')
        lines.append(f'///')
        lines.append(f'/// See: {href}')
        lines.append(f'pub struct {struct_name}<\'bot> {{')
        lines.append(f'    bot: &\'bot Bot,')
        for field in required_fields:
            fname = safe_field_name(field['name'])
            ftype = field_rust_type(field, types_map)
            # Store concrete owned types in struct
            if ftype == 'InputFileOrString':
                lines.append(f'    {fname}: InputFileOrString,')
            elif ftype == 'String':
                lines.append(f'    {fname}: String,')
            elif ftype == 'ChatId':
                lines.append(f'    {fname}: ChatId,')
            elif ftype == 'InputMedia':
                lines.append(f'    {fname}: Vec<InputMedia>,')
            else:
                lines.append(f'    {fname}: {ftype},')
        if has_opts:
            lines.append(f'    params: {params_name},')
        lines.append('}')
        lines.append('')

        # --- impl block ---
        lines.append(f'impl<\'bot> {struct_name}<\'bot> {{')

        # Constructor
        cons_args = ['bot: &\'bot Bot']
        for field in required_fields:
            fname = safe_field_name(field['name'])
            ftype = field_rust_type(field, types_map)
            if ftype == 'String':
                cons_args.append(f'{fname}: impl Into<String>')
            elif ftype == 'ChatId':
                cons_args.append(f'{fname}: impl Into<ChatId>')
            elif ftype == 'InputFileOrString':
                cons_args.append(f'{fname}: impl Into<InputFileOrString>')
            elif ftype == 'InputMedia':
                cons_args.append(f'{fname}: Vec<InputMedia>')
            else:
                cons_args.append(f'{fname}: {ftype}')
        lines.append(f'    pub(crate) fn new({", ".join(cons_args)}) -> Self {{')
        lines.append(f'        Self {{')
        lines.append(f'            bot,')
        for field in required_fields:
            fname = safe_field_name(field['name'])
            ftype = field_rust_type(field, types_map)
            if ftype in ('String', 'ChatId', 'InputFileOrString'):
                lines.append(f'            {fname}: {fname}.into(),')
            else:
                lines.append(f'            {fname},')
        if has_opts:
            lines.append(f'            params: Default::default(),')
        lines.append(f'        }}')
        lines.append(f'    }}')

        if has_opts:
            # Collect which optional field names exist for ergonomic shortcuts
            opt_fnames = {safe_field_name(f['name']) for f in optional_fields}

            # Ergonomic shortcuts
            if 'parse_mode' in opt_fnames:
                lines.append('')
                lines.append('    /// Set `parse_mode` to `"HTML"`.')
                lines.append('    pub fn html(self) -> Self { self.parse_mode("HTML") }')
                lines.append('    /// Set `parse_mode` to `"MarkdownV2"`.')
                lines.append('    pub fn markdown(self) -> Self { self.parse_mode("MarkdownV2") }')
            if 'disable_notification' in opt_fnames:
                lines.append('')
                lines.append('    /// Send silently (`disable_notification = true`).')
                lines.append('    pub fn silent(self) -> Self { self.disable_notification(true) }')
            if 'reply_parameters' in opt_fnames:
                lines.append('')
                lines.append('    /// Reply to a message by id.')
                lines.append('    pub fn reply_to(mut self, message_id: i64) -> Self {')
                lines.append('        self.params.reply_parameters = Some(make_reply_params(message_id));')
                lines.append('        self')
                lines.append('    }')
            # reply_markup setter is generated by generate_setter below; no duplicate keyboard() shortcut

            # All optional setters
            lines.append('')
            for field in optional_fields:
                fname = safe_field_name(field['name'])
                ftype = opt_wrap(field_rust_type(field, types_map), True)
                setter = generate_setter(fname, ftype)
                lines.append(setter)

        lines.append('}')
        lines.append('')

        # --- IntoFuture ---
        lines.append(f'impl<\'bot> IntoFuture for {struct_name}<\'bot> {{')
        lines.append(f'    type Output = Result<{ret}, BotError>;')
        lines.append(f'    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + \'bot>>;')
        lines.append(f'')
        lines.append(f'    fn into_future(self) -> Self::IntoFuture {{')
        lines.append(f'        Box::pin(async move {{')

        # Build the call to the underlying raw method
        call_args = ['self.' + safe_field_name(f['name']) for f in required_fields
                     if field_rust_type(f, types_map) != 'InputFileOrString']
        # InputFileOrString required field: passed as last positional, no .into() needed
        iff_fields = [f for f in required_fields if field_rust_type(f, types_map) == 'InputFileOrString']
        call_args_with_file = ['self.' + safe_field_name(f['name']) for f in required_fields]

        if has_opts:
            args_str = ', '.join(call_args_with_file + ['Some(self.params)'])
        else:
            args_str = ', '.join(call_args_with_file)

        lines.append(f'            self.bot.{raw_fn}({args_str}).await')
        lines.append(f'        }})')
        lines.append(f'    }}')
        lines.append(f'}}')
        lines.append('')

        # --- Bot impl entry ---
        sig_args = []
        for field in required_fields:
            fname = safe_field_name(field['name'])
            ftype = field_rust_type(field, types_map)
            if ftype == 'String':
                sig_args.append(f'{fname}: impl Into<String>')
            elif ftype == 'ChatId':
                sig_args.append(f'{fname}: impl Into<ChatId>')
            elif ftype == 'InputFileOrString':
                sig_args.append(f'{fname}: impl Into<InputFileOrString>')
            elif ftype == 'InputMedia':
                sig_args.append(f'{fname}: Vec<InputMedia>')
            else:
                sig_args.append(f'{fname}: {ftype}')

        sig = ', '.join(sig_args)
        call_fields = ', '.join(['self'] + [safe_field_name(f['name']) for f in required_fields])

        bot_impl_lines.append(f'    /// {docs[0] if docs else method_name}')
        bot_impl_lines.append(f'    /// See: {href}')
        if sig:
            bot_impl_lines.append(f'    pub fn {fn_name}(&self, {sig}) -> {struct_name}<\'_> {{')
        else:
            bot_impl_lines.append(f'    pub fn {fn_name}(&self) -> {struct_name}<\'_> {{')
        bot_impl_lines.append(f'        {struct_name}::new({call_fields})')
        bot_impl_lines.append(f'    }}')
        bot_impl_lines.append('')

    bot_impl_lines.append('}')
    lines.extend(bot_impl_lines)

    return '\n'.join(lines)

# -------------------------------------------------------------------------
# Main
# -------------------------------------------------------------------------

def main():
    spec_path = sys.argv[1] if len(sys.argv) > 1 else 'api.json'
    out_dir = sys.argv[2] if len(sys.argv) > 2 else '../ferobot/src'

    print(f'Reading spec: {spec_path}')
    spec = load_spec(spec_path)
    print(f"Telegram Bot API {spec['version']} ({spec['release_date']})")
    print(f"Types: {len(spec['types'])}, Methods: {len(spec['methods'])}")

    # Auto-compute FLUENT_METHODS = every method that has optional fields.
    global FLUENT_METHODS
    FLUENT_METHODS = {
        name
        for name, m in spec['methods'].items()
        if any(not f['required'] for f in m.get('fields', []))
    }
    print(f"Fluent wrappers: {len(FLUENT_METHODS)} methods with optional params + "
          f"{len(spec['methods']) - len(FLUENT_METHODS)} without")

    Path(out_dir).mkdir(parents=True, exist_ok=True)

    types_code = generate_types(spec)
    with open(f'{out_dir}/gen_types.rs', 'w') as f:
        f.write(types_code)
    print(f'Written: {out_dir}/gen_types.rs')

    methods_code = generate_methods(spec)
    with open(f'{out_dir}/gen_methods.rs', 'w') as f:
        f.write(methods_code)
    print(f'Written: {out_dir}/gen_methods.rs')

    fluent_code = generate_fluent(spec)
    with open(f'{out_dir}/fluent.rs', 'w') as f:
        f.write(fluent_code)
    print(f'Written: {out_dir}/fluent.rs')

    import subprocess
    for fname in ['gen_types.rs', 'gen_methods.rs', 'fluent.rs']:
        fpath = f'{out_dir}/{fname}'
        result = subprocess.run(['rustfmt', '--edition', '2021', fpath])
        if result.returncode == 0:
            print(f'Formatted: {fpath}')
        else:
            print(f'rustfmt not available for {fname} (ok, will be formatted on cargo build)')

    print('Done.')

if __name__ == '__main__':
    main()
