use rtcp::rtcp_packet_type::RtcpPacketType;
use rtcp::application_defined::ApplicationDefined;
use std::convert::TryFrom;

#[test]
fn test_application_defined_try_from() {
    let buffer = [
        0b10100001, 204, 0x00, 0x06, // RTCP header
        0x12, 0x34, 0x56, 0x78, // SSRC
        b'A', b'P', b'P', b'1', // Name
        1, 2, 3, 4, 5, 6, 7, 8, // Application-specific data
    ];

    let app_defined = ApplicationDefined::try_from(&buffer[..]).unwrap();

    dbg!(&app_defined);
    assert_eq!(app_defined.header().version(), 2);
    assert_eq!(app_defined.header().padding(), true);
    assert_eq!(app_defined.header().report_count(), 1);
    assert_eq!(app_defined.header().packet_type(), &RtcpPacketType::ApplicationDefined);
    assert_eq!(app_defined.header().length(), 6);
    assert_eq!(app_defined.subtype(), 1);
    assert_eq!(app_defined.ssrc(), 0x12345678);
    assert_eq!(app_defined.name(), b"APP1");
    assert_eq!(app_defined.data(), &[1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_application_defined_try_from_short_buffer() {
    let buffer = [
        0b10100001, 204, 0x00, // Too short buffer
    ];

    let result = ApplicationDefined::try_from(&buffer[..]);
    assert!(result.is_err());
}