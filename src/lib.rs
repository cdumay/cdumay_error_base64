//! [![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue)](./LICENSE)
//! [![cdumay_error_base64 on crates.io](https://img.shields.io/crates/v/cdumay_error_base64)](https://crates.io/crates/cdumay_error_base64)
//! [![cdumay_error_base64 on docs.rs](https://docs.rs/cdumay_error_base64/badge.svg)](https://docs.rs/cdumay_error_base64)
//! [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/cdumay_error_base64)
//!
//! Error wrapper for base64 encoding/decoding operations.
//!
//! This crate provides structured error types for handling
//! `base64::DecodeError` by categorizing them into descriptive,
//! typed error variants with associated codes and messages.

use base64::DecodeError;
use cdumay_error::{AsError, Error, define_errors, define_kinds, ErrorConverter};
use std::collections::BTreeMap;

/// Defines a custom error kind for base64 encoding / decoding issues.
/// The error kind has a code, an HTTP status code, and a description.
define_kinds! {
    Base64Decode = ("Base64-00001", 400, "Base64 decode error"),
    Base64Encode = ("Base64-00002", 400, "Base64 encode error"),
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
pub struct Base64Error;

impl ErrorConverter for Base64Error {
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
#[cfg(test)]
mod tests {
    use super::*;
    use base64::{Engine as _, engine::general_purpose};

    #[test]
    fn test_invalid_base64_decode_error() {
        // Attempt to decode an invalid Base64 string
        let invalid_data = "!!! not base64 !!!";
        let result = general_purpose::STANDARD.decode(invalid_data);

        // Ensure that decoding fails
        assert!(result.is_err());

        let err = result.unwrap_err();
        let ctx = BTreeMap::new();

        // Convert the base64 error into a structured application error
        let custom = Base64Error::convert_error(&err, Some("Failed to decode base64".to_string()), ctx);

        // Validate the custom error kind and message
        assert_eq!(custom.kind.message_id(), "Base64-00001");
        assert_eq!(custom.message, "Failed to decode base64");
    }

    #[test]
    fn test_valid_base64_decoding() {
        // Encode "hello world" into base64
        let valid_data = general_purpose::STANDARD.encode(b"hello world");

        // Attempt to decode it back
        let decoded = general_purpose::STANDARD.decode(&valid_data);

        // Ensure decoding succeeds and the content is correct
        assert!(decoded.is_ok());
        assert_eq!(decoded.unwrap(), b"hello world");
    }
}
