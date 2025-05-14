//! [![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue)](./LICENSE)
//! [![cdumay_error_base64 on crates.io](https://img.shields.io/crates/v/cdumay_error_base64)](https://crates.io/crates/cdumay_error_base64)
//! [![cdumay_error_base64 on docs.rs](https://docs.rs/cdumay_error_base64/badge.svg)](https://docs.rs/cdumay_error_base64)
//! [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/cdumay_error_base64)
//!
//! A small utility crate for converting `base64::DecodeError` into structured, typed errors using the [`cdumay_error`](https://docs.rs/cdumay-error/) framework. This allows consistent, meaningful error reporting with custom codes, messages, and additional context.
//!
//! ## Features
//!
//! - Maps all variants of `base64::DecodeError` into structured `cdumay_error::Error` types.
//! - Provides unique error codes, HTTP status codes, and human-readable messages.
//! - Easily attach contextual metadata for better debugging.
//! - Simple integration into any Rust project using `base64` and `cdumay_error`.
//! - Provides a convenient `convert_result!` macro for error conversion
//!
//! ## Usage
//!
//! Using the `Base64DecodeErrorConverter` directly:
//! ```rust
//! use base64::{engine::general_purpose, Engine as _};
//! use cdumay_error::ErrorConverter;
//! use std::collections::BTreeMap;
//! use cdumay_error_base64::Base64DecodeErrorConverter;
//!
//! fn decode_base64(input: &str) -> Result<Vec<u8>, cdumay_error::Error> {
//!     general_purpose::STANDARD.decode(input).map_err(|e| {
//!         let mut context = BTreeMap::new();
//!         context.insert("input".to_string(), serde_value::Value::String(input.to_string()));
//!         Base64DecodeErrorConverter::convert(&e, "Failed to decode base64".to_string(), context)
//!     })
//! }
//! ```
//! Using the `convert_result!` macro:
//! ```rust
//! use base64::{engine::general_purpose, Engine as _};
//! use cdumay_error::ErrorConverter;
//! use std::collections::BTreeMap;
//! use cdumay_error_base64::convert_result;
//!
//! fn decode_base64(input: &str) -> Result<Vec<u8>, cdumay_error::Error> {
//!     convert_result!(general_purpose::STANDARD.decode(input), "Failed to decode base64")
//! }
//! ```
//!
#[macro_use]
mod macros;

use base64::DecodeError;
use cdumay_error::{define_errors, define_kinds, AsError, Error, ErrorConverter};
use std::collections::BTreeMap;

/// Defines a custom error kind for base64 decoding issues.
/// The error kind has a code, an HTTP status code, and a description.
define_kinds! {
    Base64Decode = ("Base64-00001", 400, "Base64 decode error"),
}

/// Associates the defined kind with a typed error.
///
/// This macro creates new error types based on the `Base64Decode`.
define_errors! {
    InvalidByteError = Base64Decode,
    InvalidLengthError = Base64Decode,
    InvalidLastSymbolError = Base64Decode,
    InvalidPaddingError = Base64Decode,
}

/// A helper structure that converts `base64::DecodeError` into a structured `Error`.
pub struct Base64DecodeErrorConverter;

impl ErrorConverter for Base64DecodeErrorConverter {
    type Error = DecodeError;
    /// Converts a `base64::DecodeError` into a custom structured `Error`.
    ///
    /// # Arguments
    ///
    /// * `err` - The error returned by the `base64` crate.
    /// * `text` - A human-readable error message.
    /// * `context` - A mutable reference to a `Context` containing additional metadata.
    ///
    /// # Returns
    ///
    /// Returns a structured `Error` with kind `DecodeBase64`, message, and details.
    fn convert(error: &DecodeError, text: String, context: BTreeMap<String, serde_value::Value>) -> Error {
        match error {
            DecodeError::InvalidByte(_, _) => InvalidByteError::new().set_message(text).set_details(context).into(),
            DecodeError::InvalidLength(_) => InvalidLengthError::new().set_message(text).set_details(context).into(),
            DecodeError::InvalidLastSymbol(_, _) => InvalidLastSymbolError::new().set_message(text).set_details(context).into(),
            DecodeError::InvalidPadding => InvalidPaddingError::new().set_message(text).set_details(context).into(),
        }
    }
}
