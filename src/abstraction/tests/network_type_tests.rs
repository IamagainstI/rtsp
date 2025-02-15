use rstest::rstest;
use abstractions::net::network_type::NetworkType;

#[rstest]
#[case("IN", Some(NetworkType::Internet))]
#[case("INVALID", None)]
fn test_from_str(#[case] input: &str, #[case] expected: Option<NetworkType>) {
    let result = NetworkType::from_str(input);
    assert_eq!(result, expected);
}

#[rstest]
#[case(b"IN", Some(NetworkType::Internet))]
#[case(b"INVALID", None)]
fn test_from_bytes(#[case] input: &[u8], #[case] expected: Option<NetworkType>) {
    let result = NetworkType::from_bytes(input);
    assert_eq!(result, expected);
}

#[rstest]
#[case(NetworkType::Internet, "IN")]
fn test_as_str(#[case] network_type: NetworkType, #[case] expected: &str) {
    let result = network_type.as_str();
    assert_eq!(result, expected);
}

#[test]
fn test_default() {
    let default_network_type = NetworkType::default();
    assert_eq!(default_network_type, NetworkType::Internet);
}