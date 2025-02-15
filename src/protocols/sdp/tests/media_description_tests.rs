use media::video::h264_codec::H264Codec;
use media::video::h265_codec::H265Codec;
use sdp::media_description::MediaDescription;
use sdp::payload_type::PayloadType;
use sdp::transport_protocol::MediaTransportProtocol;
use abstractions::parsing::payload_parser::PayloadParser;
use sdp::sdp_port::SdpPort;
use media::codec_type::CodecType;
use media::{audio::aac_codec::AacCodec, codec::Codec};
use rstest::rstest;

#[rstest]
#[case(
    b"video 51372 RTP/AVP 97 98\r
      c=IN IP4 192.168.1.1/1/2\r
      a=rtpmap:97 H264/90000\r
      a=fmtp:97 packetization-mode=1; profile-level-id=42e01f; sprop-parameter-sets=Z0IAH5WoFAFuQA==,aM4G4g==\r\n
      a=rtpmap:98 H265/90000\r
      a=fmtp:98 profile-id=1; sprop-vps=Z0IAH5WoFAFuQA==; sprop-sps=Z0IAH5WoFAFuQA==; sprop-pps=aM4G4g==\r\n",
    Ok(MediaDescription::new(
        None,
        vec![
            CodecType::H264(H264Codec::parse(90000, None, b"a=fmtp:97 packetization-mode=1; profile-level-id=42e01f; sprop-parameter-sets=Z0IAH5WoFAFuQA==,aM4G4g==\r\n").unwrap()), 
            CodecType::H265(H265Codec::parse(90000, None, b"a=fmtp:98 profile-id=1; sprop-vps=Z0IAH5WoFAFuQA==; sprop-sps=Z0IAH5WoFAFuQA==; sprop-pps=aM4G4g==\r\n").unwrap()), 
        ],
        None,
        PayloadType::Video,
        vec![SdpPort::new(51372, 51373)],
        1,
        MediaTransportProtocol::RtpAvp,
        Some(abstractions::net::connection_addresses::ConnectionAddresses::new(
            abstractions::net::address_type::AddressType::Ipv4,
            vec![
                "192.168.1.1".parse().unwrap(),
                "192.168.1.2".parse().unwrap(),
            ],
            Some(1),
        ))),
    )
)]
#[case(
    b"audio 49170 RTP/AVP 97\r
      a=rtpmap:97 MPEG4-GENERIC/48000/2\r
      a=fmtp:97 streamtype=5; profile-level-id=15; mode=AAC-hbr; config=1190; sizeLength=13; indexLength=3; indexDeltaLength=3; profile=1;\r",
    Ok(MediaDescription::new(
        None,
        vec![
            CodecType::Aac(AacCodec::parse(48000, Some(2), b"a=fmtp:97 streamtype=5; profile-level-id=15; mode=AAC-hbr; config=1190; sizeLength=13; indexLength=3; indexDeltaLength=3; profile=1;\r\n").unwrap())],
        None,
        PayloadType::Audio,
        vec![SdpPort::new(49170, 49171)],
        1,
        MediaTransportProtocol::RtpAvp,
        None
    ))
)]
#[case(
    b"invalid sdp message",
    Err(())
)]
fn test_parse_media_description(#[case] sdp_message: &[u8], #[case] expected: Result<MediaDescription, ()>) {
    let result = MediaDescription::parse(sdp_message).map_err(|_| ());
    assert_eq!(result, expected);
}