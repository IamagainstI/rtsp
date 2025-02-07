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
/// use sdp::payload_parser::PayloadParser;
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
///
/// ## Generating an SDP Message
///
/// ```rust
/// use sdp::media_session::MediaSession;
/// use sdp::payload_type::PayloadType;
/// use sdp::transport_protocol::MediaTransportProtocol;
/// use sdp::sdp_port::SdpPort;
///
/// let media_session = MediaSession {
///     version: 0,
///     origin: None,
///     session_name: "SDP Seminar".to_string(),
///     connection_info: None,
///     timing: None,
///     media_descriptions: vec![
///         MediaDescription {
///             bandwidth: None,
///             codecs: vec![],
///             data_transfer_mode: None,
///             payload_type: PayloadType::Audio,
///             ports: vec![SdpPort::new(49170, 49171)],
///             transport_protocol: MediaTransportProtocol::RtpAvp,
///         }
///     ],
/// };
///
/// let sdp_message = format!(
///     "v={}\r\ns={}\r\nm={} {} {} {}\r\n",
///     media_session.version,
///     media_session.session_name,
///     media_session.media_descriptions[0].payload_type.as_str(),
///     media_session.media_descriptions[0].ports[0].port,
///     media_session.media_descriptions[0].transport_protocol.as_str(),
///     "0 96"
/// );
///
/// println!("Generated SDP message: {}", sdp_message);
/// ```
pub mod media_session;
pub mod bandwidth;
pub mod origin;
pub mod media_attribute;
pub mod media_description;
pub mod data_transfer_mode;
pub mod payload_type;
pub mod network_type;
pub mod address_type;
pub mod time;
mod sdp_port;
mod transport_protocol;

const RAW_SEPARATOR: &[u8] = b"\r\n";
const KEY_VALUE_SEPARATOR: &[u8] = b"=";
const COLON_SEPARATOR: &[u8] = b":";
const TRIM: &[u8] = b" ";