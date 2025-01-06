use sdp::timing::RepeatTimes;

#[test]
fn test_parse_repeat_times() {
    let input = "7d 1h 0 25h";
    let repeat_times = RepeatTimes::parse(input).expect("Failed to parse repeat times");

    assert_eq!(repeat_times.repeat_interval(), Duration::days(7));
    assert_eq!(repeat_times.active_duration(), Duration::hours(1));
    assert_eq!(repeat_times.offsets(), &[Duration::seconds(0), Duration::hours(25)]);
}

#[test]
fn test_parse_invalid_repeat_times() {
    let input = "7d 1h";
    assert!(RepeatTimes::parse(input).is_err());

    let input = "7d 1x 0 25h";
    assert!(RepeatTimes::parse(input).is_err());
}
