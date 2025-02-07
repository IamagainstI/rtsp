/// Module containing parsing utilities and error handling.
///
/// This module provides traits and error types for parsing data into specific types.
/// It includes the `PayloadParser` trait for parsing data from byte slices and the
/// `ParsingError` enum for representing errors that can occur during parsing.
pub mod payload_parser;
pub mod parsing_error;