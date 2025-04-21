# cdumay_error_base64

[![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue)](./LICENSE)
[![cdumay_error_base64 on crates.io](https://img.shields.io/crates/v/cdumay_error_base64)](https://crates.io/crates/cdumay_error_base64)
[![cdumay_error_base64 on docs.rs](https://docs.rs/cdumay_error_base64/badge.svg)](https://docs.rs/cdumay_error_base64)
[![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/cdumay_error_base64)

A small utility crate for converting `base64::DecodeError` into structured, typed errors using the [`cdumay_error`](https://docs.rs/cdumay-error/) framework. This allows consistent, meaningful error reporting with custom codes, messages, and additional context.

### Features

- Maps all variants of `base64::DecodeError` into structured `cdumay_error::Error` types.
- Provides unique error codes, HTTP status codes, and human-readable messages.
- Easily attach contextual metadata for better debugging.
- Simple integration into any Rust project using `base64` and `cdumay_error`.

### Usage

```rust
use base64::{engine::general_purpose, Engine as _};
use cdumay_error::ErrorConverter;
use std::collections::BTreeMap;
use cdumay_error_base64::Base64DecodeErrorConverter;

fn decode_base64(input: &str) -> Result<Vec<u8>, cdumay_error::Error> {
    general_purpose::STANDARD.decode(input).map_err(|e| {
        let mut context = BTreeMap::new();
        context.insert("input".to_string(), serde_value::Value::String(input.to_string()));
        Base64DecodeErrorConverter::convert(&e, "Failed to decode base64".to_string(), context)
    })
}
