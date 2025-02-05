use rstest::rstest;
use sdp::bandwidth::Bandwidth;

#[rstest]
#[case(b"AS:128", Ok(Bandwidth::new("AS".to_string(), 128)))]
#[case(b"CT:256", Ok(Bandwidth::new("CT".to_string(), 256)))]
#[case(b"invalid", Err(true))]
#[case(b"AS:invalid", Err(true))]
fn test_parse_bandwidth(#[case] input: &[u8], #[case] expected: Result<Bandwidth, bool>) {
    use abstractions::parsing::payload_parser::PayloadParser; 

    let result = Bandwidth::parse(input).map_err(|_| true);
    assert_eq!(result, expected);
}

#[rstest]
#[case("AS".to_string(), 128)]
#[case("CT".to_string(), 256)]
fn test_bandwidth_methods(#[case] index: String, #[case] element: u32) {
    let bandwidth = Bandwidth::new(index.clone(), element);
    assert_eq!(bandwidth.index(), index);
    assert_eq!(bandwidth.element(), element);
}