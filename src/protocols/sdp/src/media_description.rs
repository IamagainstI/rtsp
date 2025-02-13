use std::net::IpAddr;

use abstractions::{
    extensions::{array_extensions::ArrayExt, utf8_array_extensions::U8ArrayExt},
    parsing::{
        parsing_error::ParsingError, payload_parser::PayloadParser, NEW_LINE, SLASH, TRIM_NEW_LINE, WHITESPACE
    },
};
use media::{codec::RTPMAP_KEY, codec_type::CodecType};

use crate::{
    bandwidth::Bandwidth, data_transfer_mode::DataTransferMode, media_session::get_connection_address, payload_type::PayloadType, sdp_port::SdpPort, transport_protocol::MediaTransportProtocol
};

const CONNECTION_KEY: &[u8] = b"c=";

/// Represents a media description in SDP.
///
/// The `MediaDescription` struct corresponds to the `m=` field in SDP, which specifies
/// the media type, transport port, transport protocol, and format list for a media description.
///
/// According to RFC 4566, the `m=` field has the following syntax:
///
/// ```text
/// m=<media> <port> <proto> <fmt> ...
/// ```
///
/// - `<media>`: The media type (e.g., `audio`, `video`, `application`, `data`, `control`).
/// - `<port>`: The transport port to which the media stream will be sent.
/// - `<proto>`: The transport protocol (e.g., `RTP/AVP`, `RTP/SAVP`).
/// - `<fmt>`: The format list, which specifies the payload types for the media stream.
///
/// Example:
///
/// ```text
/// m=audio 49170 RTP/AVP 0 96
/// ```
///
/// # Fields
///
/// * `bandwidth` - Optional bandwidth information for the media description.
/// * `codecs` - A list of codecs used in the media description.
/// * `data_transfer_mode` - Optional data transfer mode for the media description.
/// * `payload_type` - The payload type for the media description.
/// * `ports` - A list of ports used for the media description.
/// * `transport_protocol` - The transport protocol used for the media description.
#[derive(Debug, PartialEq)]
pub struct MediaDescription {
    bandwidth: Option<Bandwidth>,
    codecs: Vec<CodecType>,
    data_transfer_mode: Option<DataTransferMode>,
    payload_type: PayloadType,
    ports: Vec<SdpPort>,
    port_count: usize,
    transport_protocol: MediaTransportProtocol,
    connection_address: Option<IpAddr>,
}

impl MediaDescription {
    pub fn new(
        bandwidth: Option<Bandwidth>,
        codecs: Vec<CodecType>,
        data_transfer_mode: Option<DataTransferMode>,
        payload_type: PayloadType,
        ports: Vec<SdpPort>,
        port_count: usize,
        transport_protocol: MediaTransportProtocol,
        connection_address: Option<IpAddr>,
    ) -> Self {
        Self {
            bandwidth,
            codecs,
            data_transfer_mode,
            payload_type,
            ports,
            port_count,
            transport_protocol,
            connection_address,
        }
    }

    /// Returns the bandwidth information for the media description.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the `Bandwidth` if present, or `None` if not.
    pub fn bandwidth(&self) -> Option<&Bandwidth> {
        self.bandwidth.as_ref()
    }

    /// Returns the list of codecs used in the media description.
    ///
    /// # Returns
    ///
    /// A slice containing the `CodecType` instances.
    pub fn codecs(&self) -> &[CodecType] {
        &self.codecs
    }

    /// Returns the data transfer mode for the media description.
    ///
    /// # Returns
    ///
    /// An `Option` containing the `DataTransferMode` if present, or `None` if not.
    pub fn data_transfer_mode(&self) -> Option<DataTransferMode> {
        self.data_transfer_mode
    }

    /// Returns the payload type for the media description.
    ///
    /// # Returns
    ///
    /// A reference to the `PayloadType`.
    pub fn payload_type(&self) -> &PayloadType {
        &self.payload_type
    }

    /// Returns the list of ports used for the media description.
    ///
    /// # Returns
    ///
    /// A slice containing the `SdpPort` instances.
    pub fn ports(&self) -> &[SdpPort] {
        &self.ports
    }

    /// Returns the count of ports used for the media description.
    ///
    /// # Returns
    ///
    /// Count of ports.
    pub fn port_count(&self) -> &usize {
        &self.port_count
    }

    /// Returns the transport protocol used for the media description.
    ///
    /// # Returns
    ///
    /// A reference to the `MediaTransportProtocol`.
    pub fn transport_protocol(&self) -> &MediaTransportProtocol {
        &self.transport_protocol
    }
    
    /// Returns the connection address if it exists.
    ///
    /// # Returns
    ///
    /// `Option` for `IpAddr`.
    pub fn connection_address(&self) -> Option<IpAddr> {
        self.connection_address
    }
}

impl PayloadParser for MediaDescription {
    fn parse(data: &[u8]) -> Result<Self, ParsingError>
    where
        Self: Sized,
    {
        let mut connection_address: Option<IpAddr> = None;
        let (top, bot) = data
            .separate_trimmed(NEW_LINE, TRIM_NEW_LINE)
            .ok_or_else(|| ParsingError::from_bytes(data))?;

        let (r#type, right) = top
            .separate(WHITESPACE)
            .ok_or_else(|| ParsingError::from_bytes(data))?;
        let r#type = PayloadType::from_bytes(r#type)?;

        let (ports_block, right) = right
            .separate(WHITESPACE)
            .ok_or_else(|| ParsingError::from_bytes(data))?;
        let ports = get_ports(ports_block)?;
        let port_count = ports.len();

        let (transport_protocol, right) = right
            .separate(WHITESPACE)
            .ok_or_else(|| ParsingError::from_bytes(data))?;
        let transport_protocol = MediaTransportProtocol::from_bytes(transport_protocol)?;

        let mut codec_formats = Vec::<u16>::new();
        let mut current = right;
        while let Some((left, right)) = current.while_separate_trimmed(WHITESPACE, WHITESPACE) {
            let format = left.utf8_to_number::<u16>()?;
            codec_formats.push(format);
            current = right;
        }
        if codec_formats.is_empty() {
            return Err(ParsingError::from_bytes(data));
        }

        let (_, bot) = bot
            .separate_trimmed(RTPMAP_KEY, WHITESPACE)
            .ok_or_else(|| ParsingError::from_bytes(data))?;

        let mut slice = bot;
        let mut codecs: Vec<CodecType> = Vec::new();
        if let Some((_, all)) = slice.separate_trimmed(CONNECTION_KEY, TRIM_NEW_LINE) {
            let (connection, b) = all
                .separate_trimmed(NEW_LINE, TRIM_NEW_LINE)
                .ok_or_else(|| ParsingError::from_bytes(data))?;
            connection_address = Some(get_connection_address(connection)?);
            slice = b;
        }
        while let Some((top, bot)) = slice.while_separate_trimmed(RTPMAP_KEY, TRIM_NEW_LINE) {
            codecs.push(CodecType::parse(top)?);
            slice = bot;
        }

        if codecs.is_empty() || codecs.len() != codec_formats.len() {
            return Err(ParsingError::from_bytes(data));
        }

        Ok(MediaDescription {
            bandwidth: None,
            codecs,
            data_transfer_mode: None,
            payload_type: r#type,
            ports,
            port_count,
            transport_protocol,
            connection_address,
        })
    }
}

/// Parses the ports from a byte slice.
///
/// # Arguments
///
/// * `ports_block` - A byte slice containing the ports information.
///
/// # Returns
///
/// A `Result` containing a vector of `SdpPort` instances if successful, or a `ParsingError` if the parsing fails.
fn get_ports(ports_block: &[u8]) -> Result<Vec<SdpPort>, ParsingError> {
    let mut ports: Vec<SdpPort> = Vec::new();
    if let Some((port, count)) = ports_block.separate(SLASH) {
        let mut port = port.utf8_to_number::<u16>()?;
        let count = count.utf8_to_number::<u8>()?;

        for _ in 1..count {
            ports.push(SdpPort::new(port, port + 1));
            port += 2;
        }
    } else {
        let rtp_port = ports_block.utf8_to_number::<u16>()?;
        ports.push(SdpPort::new(rtp_port, rtp_port + 1));
    }
    Ok(ports)
}
