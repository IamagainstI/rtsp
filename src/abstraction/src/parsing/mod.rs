/// Module containing parsing utilities and error handling.
///
/// This module provides traits and error types for parsing data into specific types.
/// It includes the `PayloadParser` trait for parsing data from byte slices and the
/// `ParsingError` enum for representing errors that can occur during parsing.
pub mod payload_parser;
pub mod parsing_error;

/// `&[U8]` representation of characters `\r\n`.
pub const NEW_LINE: &[u8] = b"\r\n";

/// `&[U8]` representation of characters `\r\n`.
pub const TRIM_NEW_LINE: &[u8] = b"\r\n ";

/// `&[U8]` representation of characters `=`.
pub const EQUAL: &[u8] = b"=";

/// `&[U8]` representation of characters `:`.
pub const COLON: &[u8] = b":";

/// `&[U8]` representation of characters ` `.
pub const WHITESPACE: &[u8] = b" ";

/// `&[U8]` representation of characters `/`.
pub const SLASH: &[u8] = b"/";

/// `&[U8]` representation of characters `;`.
pub const SEMICOLON: &[u8] = b";";

/// `&[U8]` representation of characters `;`.
pub const COMMA: &[u8] = b",";