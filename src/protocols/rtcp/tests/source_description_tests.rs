use rtcp::rtcp_packet_type::RtcpPacketType;
use rtcp::source_description::SourceDescription;
use std::convert::TryFrom;

#[test]
fn test_source_description_try_from() {
    let buffer = [
        0b10000001, 202, 0x00, 0x06, // RTCP header
        0x12, 0x34, 0x56, 0x78, // SSRC
        1, 3, b'c', b's', b'r', // CNAME item
        0, // End of items
    ];

    let source_description = SourceDescription::try_from(&buffer[..]).unwrap();

    assert_eq!(source_description.header().version(), 2);
    assert_eq!(source_description.header().padding(), false);
    assert_eq!(source_description.header().report_count(), 1);
    assert_eq!(source_description.header().packet_type(), &RtcpPacketType::SourceDescription);
    assert_eq!(source_description.header().length(), 6);

    let chunks = source_description.chunks();
    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0].ssrc(), 0x12345678);
    let items = chunks[0].items();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].item_type(), 1);
    assert_eq!(items[0].length(), 3);
    //assert_eq!(chunks[0].items()[0].data(), b"csr");
}

#[test]
fn test_source_description_try_from_short_buffer() {
    let buffer = [
        0b10000001, 202, 0x00, // Too short buffer
    ];

    let result = SourceDescription::try_from(&buffer[..]);
    assert!(result.is_err());
}

#[test]
fn test_source_description_try_from_insufficient_chunks() {
    let buffer = [
        0b10000010, 202, 0x00, 0x06, // RTCP header with report_count = 2
        0x12, 0x34, 0x56, 0x78, // Only one chunk provided
    ];

    let result = SourceDescription::try_from(&buffer[..]);
    assert!(result.is_err());
}