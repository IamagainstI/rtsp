#[derive(Clone, Copy)]
#[repr(packed)]
pub struct TpktPayloadHeader {
    version: u8,
    pre_reserved: u8,
    header: u8,
    packet_length: u8
}