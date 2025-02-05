use rstest::rstest;
use sdp::payload_type::PayloadType;

#[rstest]
#[case("video", Some(PayloadType::Video))]
#[case("audio", Some(PayloadType::Audio))]
#[case("application", Some(PayloadType::Application))]
#[case("data", Some(PayloadType::Data))]
#[case("control", Some(PayloadType::Control))]
#[case("invalid", None)]
fn test_from_str(#[case] input: &str, #[case] expected: Option<PayloadType>) {
    let result = PayloadType::from_str(input);
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