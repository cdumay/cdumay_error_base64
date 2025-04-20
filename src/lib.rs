//! [![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue)](./LICENSE)
//! [![cdumay_error_base64 on crates.io](https://img.shields.io/crates/v/cdumay_error_base64)](https://crates.io/crates/cdumay_error_base64)
//! [![cdumay_error_base64 on docs.rs](https://docs.rs/cdumay_error_base64/badge.svg)](https://docs.rs/cdumay_error_base64)
//! [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/cdumay_error_base64)
//!
//!

use base64::DecodeError;
use cdumay_error::{Error, define_errors, define_kinds};
use std::collections::BTreeMap;

define_kinds! {
    Base64Decode = ("Base64-00001", 400, "Base64 decode error"),
    Base64Encode = ("Base64-00002", 400, "Base64 encode error"),
}

define_errors! {
    InvalidByteError = Base64Decode,
    InvalidLengthError = Base64Decode,
    InvalidLastSymbolError = Base64Decode,
    InvalidPaddingError = Base64Decode,
}

pub struct Base64Error;

impl Base64Error {
    pub fn decode_error(error: &DecodeError, text: Option<String>, context: BTreeMap<String, serde_value::Value>) -> Error {
        let text = text.unwrap_or(error.to_string());
        match error {
            DecodeError::InvalidByte(_, _) => InvalidByteError::new().set_message(text).set_details(context).into(),
            DecodeError::InvalidLength(_) => InvalidLengthError::new().set_message(text).set_details(context).into(),
            DecodeError::InvalidLastSymbol(_, _) => InvalidLastSymbolError::new().set_message(text).set_details(context).into(),
            DecodeError::InvalidPadding => InvalidPaddingError::new().set_message(text).set_details(context).into(),
        }
    }
}
