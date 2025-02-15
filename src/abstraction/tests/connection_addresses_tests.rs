use rstest::rstest;
use abstractions::net::connection_addresses::ConnectionAddresses;
use abstractions::net::address_type::AddressType;
use abstractions::parsing::payload_parser::PayloadParser;

#[rstest]
#[case(
    b"IN IP4 192.168.1.1/1/2",
    Ok(ConnectionAddresses::new(
        AddressType::Ipv4,
        vec![
            "192.168.1.1".parse().unwrap(),
            "192.168.1.2".parse().unwrap(),
        ],
        Some(1),
    ))
)]
#[case(
    b"IN IP4 192.168.1.1",
    Ok(ConnectionAddresses::new(
        AddressType::Ipv4,
        vec![
            "192.168.1.1".parse().unwrap(),
        ],
        None,
    ))
)]
#[case(
    b"IN IP6 fd00::1/1/2",
    Ok(ConnectionAddresses::new(
        AddressType::Ipv6,
        vec![
            "fd00::1".parse().unwrap(),
            "fd00::2".parse().unwrap(),
        ],
        Some(1),
    ))
)]
#[case(
    b"invalid data",
    Err(())
)]
fn test_parse_connection_addresses(
    #[case] data: &[u8],
    #[case] expected: Result<ConnectionAddresses, ()>,
) {

    let result = ConnectionAddresses::parse(data).map_err(|_| ());
    assert_eq!(result, expected);
}