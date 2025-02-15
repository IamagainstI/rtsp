use std::net::IpAddr;
use rstest::rstest;
use sdp::origin::Origin;
use abstractions::net::{address_type::AddressType, network_type::NetworkType};

#[rstest]
#[case(b"jdoe 2890844526 2890842807 IN IP4 192.0.2.10", Ok(Origin::new(
    "jdoe".to_string(),
    "2890842807".to_string(),
    "2890844526".to_string(),
    AddressType::Ipv4,
    "192.0.2.10".parse().unwrap(),
)))]

#[case(b"jdoe 2890844526 2890842807 IN IP6 2001:db8::1", Ok(Origin::new(
    "jdoe".to_string(),
    "2890842807".to_string(),
    "2890844526".to_string(),
    AddressType::Ipv6,
    "2001:db8::1".parse().unwrap(),
)))]
#[case(b"invalid", Err(true))]
fn test_parse_origin(#[case] input: &[u8], #[case] expected: Result<Origin, bool>) {
    use abstractions::parsing::payload_parser::PayloadParser;

    let result = Origin::parse(input).map_err(|_| true);
    assert_eq!(result, expected);
}

#[test]
fn test_origin_methods() {
    let origin = Origin::new(
        "jdoe".to_string(),
        "2890844526".to_string(),
        "2890842807".to_string(),
        AddressType::Ipv4,
        "192.0.2.10".parse().unwrap(),
    );

    assert_eq!(origin.user_name(), "jdoe");
    assert_eq!(origin.session_version(), "2890844526");
    assert_eq!(origin.session_id(), "2890842807");
    assert_eq!(*origin.network_type(), NetworkType::Internet);
    assert_eq!(*origin.address_type(), AddressType::Ipv4);
    assert_eq!(origin.network_address(), "192.0.2.10".parse::<IpAddr>().unwrap());
}