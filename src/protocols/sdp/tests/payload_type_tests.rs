use rstest::rstest;
use sdp::payload_type::PayloadType;

#[rstest]
#[case("video", Ok(PayloadType::Video))]
#[case("audio", Ok(PayloadType::Audio))]
#[case("application", Ok(PayloadType::Application))]
#[case("data", Ok(PayloadType::Data))]
#[case("control", Ok(PayloadType::Control))]
#[case("invalid", Err(()))]
fn test_from_str(#[case] input: &str, #[case] expected: Result<PayloadType, ()>) {
    let result = PayloadType::from_str(input).map_err(|_| ());
    assert_eq!(result, expected);
}

#[rstest]
#[case(b"video", Ok(PayloadType::Video))]
#[case(b"audio", Ok(PayloadType::Audio))]
#[case(b"application", Ok(PayloadType::Application))]
#[case(b"data", Ok(PayloadType::Data))]
#[case(b"control", Ok(PayloadType::Control))]
#[case(b"invalid", Err(()))]
fn test_from_bytes(#[case] input: &[u8], #[case] expected: Result<PayloadType, ()>) {
    let result = PayloadType::from_bytes(input).map_err(|_| ());
    assert_eq!(result, expected);
}

#[rstest]
#[case(PayloadType::Video, "video")]
#[case(PayloadType::Audio, "audio")]
#[case(PayloadType::Application, "application")]
#[case(PayloadType::Data, "data")]
#[case(PayloadType::Control, "control")]
fn test_as_str(#[case] payload_type: PayloadType, #[case] expected: &str) {
    let result = payload_type.as_str();
    assert_eq!(result, expected);
}