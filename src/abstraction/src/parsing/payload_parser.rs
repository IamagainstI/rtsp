use super::parsing_error::ParsingError;

/// A trait for parsing data into a specific type.
///
/// The `PayloadParser` trait defines a method for parsing data from a byte slice
/// into a specific type. This trait is intended to be used for parsing various
/// payloads, including those in the RTSP (Real-Time Streaming Protocol) and other
/// related protocols.
///
/// Implementors of this trait must provide an implementation for the `parse` method,
/// which takes a byte slice as input and returns a `Result` containing the parsed
/// instance or a `ParsingError` if the parsing fails.
///
/// # Example
///
/// ```
/// use abstractions::parsing::payload_parser::PayloadParser;
/// use abstractions::parsing::parsing_error::ParsingError;
///
/// #[derive(Debug)]
/// struct MyPayload {
///     // Fields for MyPayload
/// }
///
/// impl PayloadParser for MyPayload {
///     fn parse(data: &[u8]) -> Result<Self, ParsingError> {
///         // Implementation for parsing MyPayload from data
///         Ok(MyPayload {
///             // Initialize fields
///         })
///     }
/// }
///
/// let data = b"example data";
/// let result = MyPayload::parse(data);
/// match result {
///     Ok(payload) => println!("Parsed payload: {:?}", payload),
///     Err(err) => println!("Failed to parse payload: {:?}", err),
/// }
/// ```
pub trait PayloadParser {
    /// Parses data from a byte slice into a specific type.
    ///
    /// # Arguments
    ///
    /// * `data` - A byte slice containing the data to be parsed.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed instance if successful, or a `ParsingError` if the parsing fails.
    fn parse(data: &[u8]) -> Result<Self, ParsingError> where Self: Sized;
}