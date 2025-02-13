use std::{
    net::IpAddr, 
    str::FromStr
};

use abstractions::{
    extensions::{array_extensions::ArrayExt, utf8_array_extensions::U8ArrayExt}, 
    instancing::default_instance::DefaultInstance, 
    parsing::{parsing_error::ParsingError, payload_parser::PayloadParser, WHITESPACE}
};

use crate::{
    address_type::AddressType, 
    network_type::NetworkType, 
};

/// Represents the origin field in an SDP message.
///
/// The `Origin` struct corresponds to the `o=` field in SDP, which specifies
/// the originator of the session (their username and the session ID), the
/// session version, the network type, the address type, and the address of
/// the machine from which the session was created.
///
/// According to RFC 4566, the `o=` field has the following syntax:
///
/// ```text
/// o=<username> <sess-id> <sess-version> <nettype> <addrtype> <unicast-address>
/// ```
///
/// - `<username>`: The user's login on the originating host, or a `-` if the
///   originating host does not have a concept of user IDs.
/// - `<sess-id>`: A numeric string representing the session identifier.
/// - `<sess-version>`: A numeric string representing the version of the session.
/// - `<nettype>`: The type of network. Common values are `IN` (Internet).
/// - `<addrtype>`: The type of address. Common values are `IP4` (IPv4) and `IP6` (IPv6).
/// - `<unicast-address>`: The address of the machine from which the session was created.
///
/// Example:
///
/// ```text
/// o=jdoe 2890844526 2890842807 IN IP4 192.0.2.10
/// ```
#[derive(Debug, PartialEq)]
pub struct Origin {
    user_name: String,
    session_version: String,
    session_id: String,
    network_type: NetworkType,
    address_type: AddressType,
    network_address: IpAddr,
}

impl Origin {
    
    pub fn new(user_name: String, session_version: String, session_id: String, address_type: AddressType, network_address: IpAddr) -> Self {
        Self { user_name, session_version, session_id, network_type: NetworkType::Internet, address_type, network_address }
    }
    
    pub fn user_name(&self) -> &str {
        &self.user_name
    }

    pub fn session_version(&self) -> &str {
        &self.session_version
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub fn network_type(&self) -> &NetworkType {
        &self.network_type
    }

    pub fn address_type(&self) -> &AddressType {
        &self.address_type
    }

    pub fn network_address(&self) -> IpAddr {
        self.network_address
    }
}

impl PayloadParser for Origin {
    fn parse(data: &[u8]) -> Result<Self, ParsingError> {
        let (user_name, other) = data
            .separate_trimmed(WHITESPACE, WHITESPACE)
            .ok_or_else(|| ParsingError::from_bytes(data))?;
        let (session_id, other) = other
            .separate_trimmed(WHITESPACE, WHITESPACE)
            .ok_or_else(|| ParsingError::from_bytes(data))?;
        let (session_version, other) = other
            .separate_trimmed(WHITESPACE, WHITESPACE)
            .ok_or_else(|| ParsingError::from_bytes(data))?;
        let (network_type, other) = other
            .separate_trimmed(WHITESPACE, WHITESPACE)
            .ok_or_else(|| ParsingError::from_bytes(data))?;
        let (address_type, network_address) = other
            .separate_trimmed(WHITESPACE, WHITESPACE)
            .ok_or_else(|| ParsingError::from_bytes(data))?;

        let network_type = NetworkType::from_bytes(network_type)
            .ok_or_else(|| ParsingError::from_bytes(data))?;
        let address_type = AddressType::from_bytes(address_type)
            .ok_or_else(|| ParsingError::from_bytes(data))?;

        let network_address = IpAddr::from_str(network_address.utf8_to_str()?)
            .map_err(|_| ParsingError::from_bytes(data))?;

        Ok(Origin {
            user_name: user_name.utf8_to_str()?.to_string(),
            session_id: session_id.utf8_to_str()?.to_string(),
            session_version: session_version.utf8_to_str()?.to_string(),
            network_type,
            address_type,
            network_address,
        })
    }
}


impl Default for Origin {
    fn default() -> Self {
        Self {
            user_name: Default::default(),
            session_version: Default::default(),
            session_id: Default::default(),
            network_type: Default::default(),
            address_type: Default::default(),
            network_address: DefaultInstance::default(),
        }
    }
}
