use byteorder::{BigEndian, ByteOrder};
use rtp::rtp_header::RtpHeader;

#[test]
fn test_rtp_header_new() {
    let csrc_list = [1, 2, 3, 4];
    let header = RtpHeader::new(2, true, false, 1, true, 96, 12345, 67890, 1234567890, &csrc_list);

    assert_eq!(header.version(), 2);
    assert_eq!(header.padding(), true);
    assert_eq!(header.extension(), false);
    assert_eq!(header.csrc_count(), 1);
    assert_eq!(header.marker(), true);
    assert_eq!(header.payload_type(), 96);
    assert_eq!(header.sequence_number(), 12345);
    assert_eq!(header.timestamp(), 67890);
    assert_eq!(header.ssrc(), 1234567890);
    assert_eq!(header.csrc_list(), &csrc_list);
}

#[test]
fn test_rtp_header_write() {
    let csrc_list = [1, 2, 3, 4];
    let header = RtpHeader::new(2, true, false, 1, true, 64, 12345, 67890, 1234567890, &csrc_list);

    let mut buffer = vec![0u8; 16];
    let bytes_written = header.write(&mut buffer);

    assert_eq!(bytes_written, 16);
    assert_eq!(buffer[0], 0b10100001); // version, padding, extension, csrc_count
    assert_eq!(buffer[1], 0b11000000); // marker, payload_type
    assert_eq!(BigEndian::read_u16(&buffer[2..4]), 12345);
    assert_eq!(BigEndian::read_u32(&buffer[4..8]), 67890);
    assert_eq!(BigEndian::read_u32(&buffer[8..12]), 1234567890);
    assert_eq!(&buffer[12..16], &csrc_list);
}

#[test]
fn test_rtp_header_try_from() {
    let buffer = [
        0b10100001, // version, padding, extension, csrc_count
        0b11000000, // marker, payload_type
        0x30, 0x39, // sequence_number (12345)
        0x00, 0x01, 0x09, 0x32, // timestamp (67890)
        0x49, 0x96, 0x02, 0xd2, // ssrc (1234567890)
        1, 2, 3, 4, // csrc_list
    ];

    let header = RtpHeader::try_from(&buffer[..]).unwrap();
    dbg!(&header);
    assert_eq!(header.version(), 2);
    assert_eq!(header.padding(), true);
    assert_eq!(header.extension(), false);
    assert_eq!(header.csrc_count(), 1);
    assert_eq!(header.marker(), true);
    assert_eq!(header.payload_type(), 64);
    assert_eq!(header.sequence_number(), 12345);
    assert_eq!(header.timestamp(), 67890);
    assert_eq!(header.ssrc(), 1234567890);
    assert_eq!(header.csrc_list(), &buffer[12..16]);
}

#[test]
fn test_rtp_header_byte_size() {
    let csrc_list = [1, 2, 3, 4];
    let header = RtpHeader::new(2, true, false, 1, true, 96, 12345, 67890, 1234567890, &csrc_list);

    assert_eq!(header.byte_size(), 16);
}
