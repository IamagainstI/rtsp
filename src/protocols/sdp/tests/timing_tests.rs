use chrono::Utc;
use rstest::rstest;
use sdp::time::timing::Timing;
use chrono::{TimeZone, Duration};

const SUBTRAHEND: i64 = 2208988800;

#[rstest]
#[case(b"3735928559 3735928560", Ok(Timing::new(
    Utc.timestamp_opt(3735928559 - SUBTRAHEND, 0).unwrap(),
    Some(Utc.timestamp_opt(3735928560 - SUBTRAHEND, 0).unwrap()),
)))]
#[case(b"3735928559 0", Ok(Timing::new(
    Utc.timestamp_opt(3735928559 - SUBTRAHEND, 0).unwrap(),
    None,
)))]
#[case(b"3735928559", Ok(Timing::new(
    Utc.timestamp_opt(3735928559 - SUBTRAHEND, 0).unwrap(),
    None,
)))]
#[case(b"invalid", Err(true))]
#[case(b"3735928559 invalid", Err(true))]
fn test_parse_timing(#[case] input: &[u8], #[case] expected: Result<Timing, bool>) {
    use abstractions::parsing::payload_parser::PayloadParser; 

    let result = Timing::parse(input).map_err(|_| true);
    assert_eq!(result, expected);
}

#[rstest]
fn test_default_timing() {
    let timing = Timing::default();
    assert!(timing.start_time() <= &Utc::now());
    assert!(timing.stop_time().is_none());
}

#[rstest]
#[case(
    Timing::new(
        Utc.timestamp_opt(3735928559 - SUBTRAHEND, 0).unwrap(),
        Some(Utc.timestamp_opt(3735928560 - SUBTRAHEND, 0).unwrap()),
    ),
    Some(Duration::seconds(1))
)]
#[case(
    Timing::new(
        Utc.timestamp_opt(3735928559 - SUBTRAHEND, 0).unwrap(),
        None,
    ),
    None
)]
fn test_get_duration(#[case] timing: Timing, #[case] expected: Option<Duration>) {
    assert_eq!(timing.get_duration(), expected);
}