#[derive(Debug, PartialEq)]
pub struct SdpPort {
    rtp_port: u16,
    rtcp_port: u16,
}

impl SdpPort {
    /// Creates a new [`SdpPort`].
    pub fn new(rtp_port: u16, rtcp_port: u16) -> Self {
        Self { rtp_port, rtcp_port }
    }
    
    pub fn rtp_port(&self) -> u16 {
        self.rtp_port
    }
    pub fn rtcp_port(&self) -> u16 {
        self.rtcp_port
    }
}