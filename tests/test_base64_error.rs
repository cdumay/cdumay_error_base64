use base64::{Engine as _, engine::general_purpose};
use cdumay_error::ErrorConverter;
use cdumay_error_base64::Base64DecodeErrorConverter;
use std::collections::BTreeMap;

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
    let custom = Base64DecodeErrorConverter::convert_error(&err, Some("Failed to decode base64".to_string()), ctx);

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
