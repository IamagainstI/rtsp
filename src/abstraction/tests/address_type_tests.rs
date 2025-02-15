use rstest::rstest;
use abstractions::net::address_type::AddressType;

#[rstest]
#[case("IP4", Some(AddressType::Ipv4))]
#[case("IP6", Some(AddressType::Ipv6))]
#[case("INVALID", None)]
fn test_from_str(#[case] input: &str, #[case] expected: Option<AddressType>) {
    let result = AddressType::from_str(input);
    assert_eq!(result, expected);
}

#[rstest]
#[case(b"IP4", Some(AddressType::Ipv4))]
#[case(b"IP6", Some(AddressType::Ipv6))]
#[case(b"INVALID", None)]
fn test_from_bytes(#[case] input: &[u8], #[case] expected: Option<AddressType>) {
    let result = AddressType::from_bytes(input);
    assert_eq!(result, expected);
}

#[test]
fn test_default() {
    let default_address_type = AddressType::default();
    assert_eq!(default_address_type, AddressType::Ipv4);
}