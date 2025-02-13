use rstest::rstest;
use sdp::transport_protocol::MediaTransportProtocol;

#[rstest]
#[case(b"RTP/AVP", Ok(MediaTransportProtocol::RtpAvp))]
#[case(b"RTP/SAVP", Ok(MediaTransportProtocol::RtpSavp))]
#[case(b"UNKNOWN/PROTOCOL", Ok(MediaTransportProtocol::Unknknown("UNKNOWN/PROTOCOL".to_string())))]
#[case(&[0, 159, 146, 150], Err(()))]
fn test_from_bytes(#[case] input: &[u8], #[case] expected: Result<MediaTransportProtocol, ()>) {
    let result = MediaTransportProtocol::from_bytes(input).map_err(|_| ());
    assert_eq!(result, expected);
}