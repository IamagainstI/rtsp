/// SDP (Session Description Protocol) module for handling SDP messages and related components.
///
/// This module provides functionality for working with SDP messages, including parsing and
/// generating SDP fields. It includes submodules for various SDP components such as media
/// sessions, bandwidth, origin, media attributes, media descriptions, data transfer modes,
/// payload types, network types, address types, time descriptions, SDP ports, and transport protocols.
///
/// # Examples
///
/// ## Parsing an SDP Message
///
/// ```rust
/// use sdp::media_session::MediaSession;
/// use abstractions::parsing::payload_parser::PayloadParser;
///
/// let sdp_message = b"v=0\r\n\
///                     o=- 2890844526 2890842807 IN IP4 192.0.2.10\r\n\
///                     s=SDP Seminar\r\n\
///                     c=IN IP4 224.2.17.12/127\r\n\
///                     t=2873397496 2873404696\r\n\
///                     m=audio 49170 RTP/AVP 0 96\r\n\
///                     a=rtpmap:0 PCMU/8000\r\n\
///                     a=rtpmap:96 opus/48000/2\r\n";
///
/// let media_session = MediaSession::parse(sdp_message);
/// match media_session {
///     Ok(ms) => println!("Parsed media session: {:?}", ms),
///     Err(err) => println!("Failed to parse media session: {:?}", err),
/// }
/// ```
pub mod media_session;
pub mod bandwidth;
pub mod origin;
pub mod media_attribute;
pub mod media_description;
pub mod data_transfer_mode;
pub mod payload_type;
pub mod time;
pub mod sdp_port;
pub mod transport_protocol;