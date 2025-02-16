use rtcp::rtcp_header::RtcpHeader;
use rtcp::rtcp_packet_type::RtcpPacketType;
use std::convert::TryFrom;

#[test]
fn test_rtcp_header_new() {
    let header = RtcpHeader::new(2, true, 1, RtcpPacketType::SenderReport, 6);

    assert_eq!(header.version(), 2);
    assert_eq!(header.padding(), true);
    assert_eq!(header.report_count(), 1);
    assert_eq!(header.packet_type(), &RtcpPacketType::SenderReport);
    assert_eq!(header.length(), 6);
}

#[test]
fn test_rtcp_header_try_from() {
    let buffer = [0b10000001, 200, 0x00, 0x06]; // Example buffer

    let header = RtcpHeader::try_from(&buffer[..]).unwrap();

    assert_eq!(header.version(), 2);
    assert_eq!(header.padding(), false);
    assert_eq!(header.report_count(), 1);
    assert_eq!(header.packet_type(), &RtcpPacketType::SenderReport);
    assert_eq!(header.length(), 6);
}

#[test]
fn test_rtcp_header_try_from_short_buffer() {
    let buffer = [0b10000001, 200]; // Too short buffer

    let result = RtcpHeader::try_from(&buffer[..]);
    assert!(result.is_err());
}