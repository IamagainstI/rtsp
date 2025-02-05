use abstractions::parsing::payload_parser::PayloadParser;
use chrono::Duration;
use rstest::rstest;
use sdp::time::repeat_times::RepeatTimes;

#[rstest]
#[case(b"7d 1h 0 25h", Ok((Duration::days(7), Duration::hours(1), Duration::seconds(0), Duration::hours(25))))]
#[case(b"1h 30m 0 15m", Ok((Duration::hours(1), Duration::minutes(30), Duration::seconds(0), Duration::minutes(15))))]
#[case(b"1d 2h 0 12h", Ok((Duration::days(1), Duration::hours(2), Duration::seconds(0), Duration::hours(12))))]
#[case(b"1h 1h 0 1h", Ok((Duration::hours(1), Duration::hours(1), Duration::seconds(0), Duration::hours(1))))]
fn test_parse_repeat_times(#[case] input: &[u8], #[case] expected: Result<(Duration, Duration, Duration, Duration), ()>) {
    let result = RepeatTimes::parse(input).map(|repeat_times| (
        repeat_times.repeat_interval(),
        repeat_times.active_duration(),
        repeat_times.offset1(),
        repeat_times.offset2(),
    )).map_err(|_| ());

    assert_eq!(result, expected);
}

#[rstest]
#[case(b"7d 1h", true)]
#[case(b"7d 1x 0 25h", true)]
#[case(b"1h 30x 0 15m", true)]
#[case(b"1d 2h 0 12x", true)]
#[case(b"1w 1d 0 1x", true)]
#[case(b"1h 1h 0 1x", true)]
fn test_parse_invalid_repeat_times(#[case] input: &[u8], #[case] should_fail: bool) {
    let result = RepeatTimes::parse(input).is_err();
    assert_eq!(result, should_fail);
}