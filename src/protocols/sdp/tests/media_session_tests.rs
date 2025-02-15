use sdp::media_session::MediaSession;
use rstest::rstest;
use abstractions::parsing::payload_parser::PayloadParser;

#[rstest]
#[case(
    b"v=0\r
      o=- 2890844526 2890842807 IN IP4 192.0.2.10\r
      s=SDP Seminar\r
      c=IN IP4 224.2.17.12/127\r
      t=2873397496 2873404696\r
      m=audio 49170 RTP/AVP 0 96\r
      a=rtpmap:0 PCMU/8000\r
      a=rtpmap:96 opus/48000/2\r",
    true
)]
#[case(
    b"v=0\r
      o=- 2890844526 2890842807 IN IP4 192.0.2.10\r
      s=SDP Seminar\r
      c=IN IP4 224.2.17.12/127\r
      t=2873397496 2873404696\r
      m=video 51372 RTP/AVP 97 98\r
      a=rtpmap:97 H264/90000\r
      a=fmtp:97 packetization-mode=1; profile-level-id=42e01f; sprop-parameter-sets=Z0IAH5WoFAFuQA==,aM4G4g==\r
      a=rtpmap:98 H265/90000\r
      a=fmtp:98 profile-id=1; sprop-vps=Z0IAH5WoFAFuQA==; sprop-sps=Z0IAH5WoFAFuQA==; sprop-pps=aM4G4g==\r",
    true
)]
#[case(
    b"v=0\r
      o=- 2890844526 2890842807 IN IP4 192.0.2.10\r
      s=SDP Seminar\r
      c=IN IP4 224.2.17.12/127\r
      t=2873397496 2873404696\r
      m=audio 49170 RTP/AVP 0\r
      a=rtpmap:0 PCMU/8000\r",
    true
)]
#[case(
    b"v=0\r
      o=- 2890844526 2890842807 IN IP4 192.0.2.10\r
      s=SDP Seminar\r
      c=IN IP4 224.2.17.12/127\r
      t=2873397496 2873404696\r
      m=audio 49170 RTP/AVP 97\r
      a=rtpmap:97 MPEG4-GENERIC/48000/2\r
      a=fmtp:97 streamtype=5; profile-level-id=15; mode=AAC-hbr; config=1190; sizeLength=13; indexLength=3; indexDeltaLength=3; profile=1;\r",
    true
)]
#[case(
    b"invalid sdp message",
    false
)]
fn test_parse_sdp(#[case] sdp_message: &[u8], #[case] is_valid: bool) {
    let result = MediaSession::parse(sdp_message);
    if is_valid {
        assert!(result.is_ok());
    } else {
        assert!(result.is_err());
    }
}