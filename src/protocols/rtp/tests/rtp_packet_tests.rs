use rtp::{rtp_header::RtpHeader, rtp_packet::RtpPacket};
#[test]
fn test_rtp_packet_new() {
    let header = RtpHeader::new(2, false, false, 0, false, 96, 12345, 67890, 1234567890, &[]);
    let payload = &[1, 2, 3, 4, 5];
    let packet = RtpPacket::new(header, payload);

    assert_eq!(packet.header().version(), 2);
    assert_eq!(packet.header().padding(), false);
    assert_eq!(packet.header().extension(), false);
    assert_eq!(packet.header().csrc_count(), 0);
    assert_eq!(packet.header().marker(), false);
    assert_eq!(packet.header().payload_type(), 96);
    assert_eq!(packet.header().sequence_number(), 12345);
    assert_eq!(packet.header().timestamp(), 67890);
    assert_eq!(packet.header().ssrc(), 1234567890);
    assert_eq!(packet.payload(), payload);
}

#[test]
fn test_rtp_packet_write() {
    let header = RtpHeader::new(2, false, false, 0, false, 96, 12345, 67890, 1234567890, &[]);
    let payload = &[1, 2, 3, 4, 5];
    let packet = RtpPacket::new(header, payload);

    let mut buffer = vec![0u8; 1024];
    let len = packet.write(&mut buffer);

    assert_eq!(len, packet.byte_size());
    assert_eq!(&buffer[..len], &[0x80, 0x60, 0x30, 0x39, 0x00, 0x01, 0x09, 0x32, 0x49, 0x96, 0x02, 0xd2, 1, 2, 3, 4, 5]);
}

#[test]
fn test_rtp_packet_try_from() {
    let buffer = [
        0x80, 0x60, 0x30, 0x39, 0x00, 0x01, 0x09, 0x32, 0x49, 0x96, 0x02, 0xd2, 1, 2, 3, 4, 5,
    ];

    let packet = RtpPacket::try_from(&buffer[..]).unwrap();

    assert_eq!(packet.header().version(), 2);
    assert_eq!(packet.header().padding(), false);
    assert_eq!(packet.header().extension(), false);
    assert_eq!(packet.header().csrc_count(), 0);
    assert_eq!(packet.header().marker(), false);
    assert_eq!(packet.header().payload_type(), 96);
    assert_eq!(packet.header().sequence_number(), 12345);
    assert_eq!(packet.header().timestamp(), 67890);
    assert_eq!(packet.header().ssrc(), 1234567890);
    assert_eq!(packet.payload(), &buffer[12..]);
}

#[test]
fn test_rtp_packet_byte_size() {
    let header = RtpHeader::new(2, false, false, 0, false, 96, 12345, 67890, 1234567890, &[]);
    let payload = &[1, 2, 3, 4, 5];
    let packet = RtpPacket::new(header, payload);

    assert_eq!(packet.byte_size(), 17);
}