use rtcp::rtcp_packet_type::RtcpPacketType;
use rtcp::goodbye::Goodbye;
use std::convert::TryFrom;

#[test]
fn test_goodbye_try_from() {
    let buffer = [
        0b10000001, 203, 0x00, 0x01, // RTCP header
        0x12, 0x34, 0x56, 0x78, // SSRC
        4, b'T', b'e', b's', b't', // Reason
    ];

    let goodbye = Goodbye::try_from(&buffer[..]).unwrap();

    assert_eq!(goodbye.header().version(), 2);
    assert_eq!(goodbye.header().padding(), false);
    assert_eq!(goodbye.header().report_count(), 1);
    assert_eq!(goodbye.header().packet_type(), &RtcpPacketType::Goodbye);
    assert_eq!(goodbye.header().length(), 1);

    let sources = goodbye.sources();
    assert_eq!(sources.len(), 1);
    assert_eq!(sources[0], 0x12345678);

    assert_eq!(goodbye.reason(), Some("Test"));
}

#[test]
fn test_goodbye_try_from_no_reason() {
    let buffer = [
        0b10000001, 203, 0x00, 0x01, // RTCP header
        0x12, 0x34, 0x56, 0x78, // SSRC
    ];

    let goodbye = Goodbye::try_from(&buffer[..]).unwrap();

    assert_eq!(goodbye.header().version(), 2);
    assert_eq!(goodbye.header().padding(), false);
    assert_eq!(goodbye.header().report_count(), 1);
    assert_eq!(goodbye.header().packet_type(), &RtcpPacketType::Goodbye);
    assert_eq!(goodbye.header().length(), 1);

    let sources = goodbye.sources();
    assert_eq!(sources.len(), 1);
    assert_eq!(sources[0], 0x12345678);

    assert_eq!(goodbye.reason(), None);
}

#[test]
fn test_goodbye_try_from_short_buffer() {
    let buffer = [
        0b10000001, 203, 0x00, // Too short buffer
    ];

    let result = Goodbye::try_from(&buffer[..]);
    assert!(result.is_err());
}

#[test]
fn test_goodbye_try_from_insufficient_sources() {
    let buffer = [
        0b10000010, 203, 0x00, 0x01, // RTCP header with report_count = 2
        0x12, 0x34, 0x56, 0x78, // Only one source provided
    ];

    let result = Goodbye::try_from(&buffer[..]);
    assert!(result.is_err());
}