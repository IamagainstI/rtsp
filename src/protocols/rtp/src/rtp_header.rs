use abstractions::extensions::cast_error::CastError;
use byteorder::{BigEndian, ByteOrder};
use std::fmt;

pub(crate) const RTP_HEADER_FIXED_SIZE: usize = 12;

// Bit masks for extracting fields from the v_p_x_cc byte
const VERSION_MASK: u8 = 0b1100_0000;
const PADDING_MASK: u8 = 0b0010_0000;
const EXTENSION_MASK: u8 = 0b0001_0000;
const CSRC_COUNT_MASK: u8 = 0b0000_1111;

// Bit masks for extracting fields from the marker_payload_type byte
const MARKER_MASK: u8 = 0b10000000;
const PAYLOAD_TYPE_MASK: u8 = 0b01111111;

/// Represents the RTP (Real-time Transport Protocol) header.
#[derive(PartialEq)]
pub struct RtpHeader<'a> {
    v_p_x_cc: u8, // Version, Padding, Extension, and CSRC Count combined
    marker_payload_type: u8, // Marker and Payload Type combined
    sequence_number: u16, // Sequence number
    timestamp: u32, // Timestamp
    ssrc: u32, // SSRC identifier
    csrc_list: &'a [u8], // CSRC list
}

impl<'a> TryFrom<&'a [u8]> for RtpHeader<'a> {
    type Error = CastError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let len = value.len();
        if len < RTP_HEADER_FIXED_SIZE {
            dbg!(value);
            return Err(CastError::from_str("Buffer too short to contain RTP header"));
        }
        let v_p_x_cc = value[0];
        let csrc_count = value[0] & CSRC_COUNT_MASK;
        let marker_payload_type = value[1];
        
        let sequence_number = BigEndian::read_u16(&value[2..4]);
        let timestamp = BigEndian::read_u32(&value[4..8]);
        let ssrc = BigEndian::read_u32(&value[8..12]);

        let csrc_list_start = RTP_HEADER_FIXED_SIZE;
        let csrc_list_end = csrc_list_start + (csrc_count as usize * 4);
        if len < csrc_list_end {
            dbg!(value);
            return Err(CastError::from_str("Buffer too short to contain RTP header"));
        }
        let csrc_list = &value[csrc_list_start..csrc_list_end];

        Ok(Self {
            v_p_x_cc,
            marker_payload_type,
            sequence_number,
            timestamp,
            ssrc,
            csrc_list,
        })
    }
}

impl<'a> RtpHeader<'a> {
    /// Creates a new `RtpHeader`.
    ///
    /// # Arguments
    ///
    /// * `version` - The RTP version.
    /// * `padding` - Indicates if padding is present.
    /// * `extension` - Indicates if an extension header is present.
    /// * `csrc_count` - The number of CSRC identifiers.
    /// * `marker` - The marker bit.
    /// * `payload_type` - The payload type.
    /// * `sequence_number` - The sequence number.
    /// * `timestamp` - The timestamp.
    /// * `ssrc` - The SSRC identifier.
    /// * `csrc_list` - The list of CSRC identifiers.
    ///
    /// # Returns
    ///
    /// A new `RtpHeader` instance.
    pub fn new(
        version: u8,
        padding: bool,
        extension: bool,
        csrc_count: u8,
        marker: bool,
        payload_type: u8,
        sequence_number: u16,
        timestamp: u32,
        ssrc: u32,
        csrc_list: &'a [u8],
    ) -> Self {
        Self {
            v_p_x_cc: (version << 6) | ((padding as u8) << 5) | ((extension as u8) << 4) | csrc_count,
            marker_payload_type: ((marker as u8) << 7) | payload_type,
            sequence_number,
            timestamp,
            ssrc,
            csrc_list,
        }
    }

    /// Write `RtpHeader` to a buffer byte vector.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The buffer to write the RTP packet to.
    ///
    /// # Returns
    ///
    /// The number of bytes written to the buffer.
    pub fn write(&self, buffer: &mut [u8]) -> usize {
        buffer[0] = self.v_p_x_cc;
        buffer[1] = self.marker_payload_type;
        BigEndian::write_u16(&mut buffer[2..4], self.sequence_number);
        BigEndian::write_u32(&mut buffer[4..8], self.timestamp);
        BigEndian::write_u32(&mut buffer[8..12], self.ssrc);
        buffer[12..12 + self.csrc_list.len()].copy_from_slice(self.csrc_list);
        RTP_HEADER_FIXED_SIZE + self.csrc_list.len()
    }

    /// Returns the RTP version.
    ///
    /// # Returns
    ///
    /// The RTP version.
    pub fn version(&self) -> u8 {
        (self.v_p_x_cc & VERSION_MASK) >> 6
    }

    /// Indicates if padding is present.
    ///
    /// # Returns
    ///
    /// `true` if padding is present, `false` otherwise.
    pub fn padding(&self) -> bool {
        (self.v_p_x_cc & PADDING_MASK) != 0
    }

    /// Indicates if an extension header is present.
    ///
    /// # Returns
    ///
    /// `true` if an extension header is present, `false` otherwise.
    pub fn extension(&self) -> bool {
        (self.v_p_x_cc & EXTENSION_MASK) != 0
    }

    /// Returns the number of CSRC identifiers.
    ///
    /// # Returns
    ///
    /// The number of CSRC identifiers.
    pub fn csrc_count(&self) -> u8 {
        self.v_p_x_cc & CSRC_COUNT_MASK
    }

    /// Returns the marker bit.
    ///
    /// # Returns
    ///
    /// `true` if the marker bit is set, `false` otherwise.
    pub fn marker(&self) -> bool {
        (self.marker_payload_type & MARKER_MASK) != 0
    }

    /// Returns the payload type.
    ///
    /// # Returns
    ///
    /// The payload type.
    pub fn payload_type(&self) -> u8 {
        self.marker_payload_type & PAYLOAD_TYPE_MASK
    }

    /// Returns the sequence number.
    ///
    /// # Returns
    ///
    /// The sequence number.
    pub fn sequence_number(&self) -> u16 {
        self.sequence_number
    }

    /// Returns the timestamp.
    ///
    /// # Returns
    ///
    /// The timestamp.
    pub fn timestamp(&self) -> u32 {
        self.timestamp
    }

    /// Returns the SSRC identifier.
    ///
    /// # Returns
    ///
    /// The SSRC identifier.
    pub fn ssrc(&self) -> u32 {
        self.ssrc
    }

    /// Returns the list of CSRC identifiers.
    ///
    /// # Returns
    ///
    /// A slice containing the CSRC identifiers.
    pub fn csrc_list(&self) -> &[u8] {
        self.csrc_list
    }

    /// Returns the total byte size of the `RtpHeader`.
    ///
    /// # Returns
    ///
    /// The total byte size of the `RtpHeader`.
    pub fn byte_size(&self) -> usize {
        RTP_HEADER_FIXED_SIZE + (self.csrc_count() as usize * 4)
    }
}

impl<'a> fmt::Debug for RtpHeader<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RtpHeader")
            .field("version", &self.version())
            .field("padding", &self.padding())
            .field("extension", &self.extension())
            .field("csrc_count", &self.csrc_count())
            .field("marker", &self.marker())
            .field("payload_type", &self.payload_type())
            .field("sequence_number", &self.sequence_number())
            .field("timestamp", &self.timestamp())
            .field("ssrc", &self.ssrc())
            .field("csrc_list", &self.csrc_list())
            .field("byte_size", &self.byte_size())
            .finish()
    }
}