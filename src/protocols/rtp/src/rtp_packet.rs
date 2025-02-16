use abstractions::extensions::cast_error::CastError;

use crate::rtp_header::{RtpHeader, RTP_HEADER_FIXED_SIZE};

#[derive(Debug)]
pub struct RtpPacket<'a> {
    header: RtpHeader<'a>,
    payload: &'a [u8],
}

impl<'a> TryFrom<&'a [u8]> for RtpPacket<'a> {
    type Error = CastError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        if value.len() < RTP_HEADER_FIXED_SIZE {
            dbg!(value);
            return Err(CastError::from_str("Buffer too short to contain RTP header"));
        }

        let header = RtpHeader::try_from(value)?;
        let payload_start = header.byte_size();

        if value.len() < payload_start {
            dbg!(header);
            dbg!(value);
            return Err(CastError::from_str("Buffer too short to contain RTP payload"));
        }

        let payload = &value[payload_start..];
        Ok(Self { header, payload })
    }
}





impl<'a> RtpPacket<'a> {
    /// Creates a new `RtpPacket`.
    ///
    /// # Arguments
    ///
    /// * `header` - The RTP header.
    /// * `payload` - The RTP payload.
    ///
    /// # Returns
    ///
    /// A new `RtpPacket` instance.
    pub fn new(header: RtpHeader<'a>, payload: &'a [u8]) -> Self {
        Self { header, payload }
    }

    /// Converts the `RtpPacket` to a byte slice.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The buffer to write the RTP packet to.
    ///
    /// # Returns
    ///
    /// The length of the written data.
    pub fn write(&self, buffer: &mut [u8]) -> usize {
        let header_len = self.header.write(buffer);
        let payload_len = self.payload.len();

        // Use the payload memory directly without copying
        buffer[header_len..header_len + payload_len].copy_from_slice(self.payload);

        header_len + payload_len
    }

    /// Returns a reference to the RTP header.
    ///
    /// # Returns
    ///
    /// A reference to the RTP header.
    pub fn header(&self) -> &RtpHeader<'a> {
        &self.header
    }

    /// Returns a reference to the RTP payload.
    ///
    /// # Returns
    ///
    /// A reference to the RTP payload.
    pub fn payload(&self) -> &[u8] {
        self.payload
    }

    /// Returns the total byte size of the `RtpPacket`.
    ///
    /// # Returns
    ///
    /// The total byte size of the `RtpPacket`.
    pub fn byte_size(&self) -> usize {
        self.header.byte_size() + self.payload.len()
    }
}