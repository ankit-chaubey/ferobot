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

use bytes::Bytes;
use serde::{Deserialize, Serialize, Serializer};

/// A file to be sent via the Telegram Bot API.
///

#[derive(Debug, Clone)]
pub enum InputFile {
    FileId(String),
    Url(String),
    Memory { filename: String, data: Bytes },
}

impl InputFile {
    pub fn file_id(id: impl Into<String>) -> Self {
        InputFile::FileId(id.into())
    }

    pub fn url(url: impl Into<String>) -> Self {
        InputFile::Url(url.into())
    }

    pub fn memory(filename: impl Into<String>, data: impl Into<Bytes>) -> Self {
        InputFile::Memory {
            filename: filename.into(),
            data: data.into(),
        }
    }
}

// Memory files are handled separately when building multipart requests.
// FileId and Url serialize to their string value.
impl Serialize for InputFile {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            InputFile::FileId(id) => serializer.serialize_str(id),
            InputFile::Url(url) => serializer.serialize_str(url),
            InputFile::Memory { filename, .. } => {
                serializer.serialize_str(&format!("attach://{}", filename))
            }
        }
    }
}

impl<'de> Deserialize<'de> for InputFile {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        if s.starts_with("http://") || s.starts_with("https://") {
            Ok(InputFile::Url(s))
        } else {
            Ok(InputFile::FileId(s))
        }
    }
}

impl From<String> for InputFile {
    fn from(s: String) -> Self {
        InputFile::FileId(s)
    }
}
impl From<&str> for InputFile {
    fn from(s: &str) -> Self {
        InputFile::FileId(s.to_string())
    }
}

/// A field that accepts either an [`InputFile`] or a plain string (file_id / URL).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputFileOrString {
    File(InputFile),
    String(String),
}

impl From<InputFile> for InputFileOrString {
    fn from(f: InputFile) -> Self {
        InputFileOrString::File(f)
    }
}
impl From<String> for InputFileOrString {
    fn from(s: String) -> Self {
        InputFileOrString::String(s)
    }
}
impl From<&str> for InputFileOrString {
    fn from(s: &str) -> Self {
        InputFileOrString::String(s.to_string())
    }
}
