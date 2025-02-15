use std::net::IpAddr;

use crate::{extensions::{self, ip_addr_extensions::IpAddrExt}, net::{address_type::AddressType, network_type::NetworkType}, parsing};
use extensions::{array_extensions::ArrayExt, utf8_array_extensions::U8ArrayExt};
use parsing::{parsing_error::ParsingError, payload_parser::PayloadParser, SLASH, WHITESPACE};


#[derive(Debug, PartialEq)]
pub struct ConnectionAddresses {
    net_type: NetworkType,
    addr_type: AddressType,
    addresses: Vec<IpAddr>,
    ttl: Option<u8>,
}

impl PayloadParser for ConnectionAddresses {
    fn parse(data: &[u8]) -> Result<Self, ParsingError> where Self: Sized {
        let mut ttl: Option<u8> = None;
        let mut count: u8 = 1;
        let mut addresses: Vec<IpAddr> = Vec::new();

        let (net_type, other) = data
            .separate_trimmed(WHITESPACE, WHITESPACE)
            .ok_or_else(|| ParsingError::from_bytes(data))?;

        let net_type = NetworkType::from_bytes(net_type)
            .ok_or_else(|| ParsingError::from_bytes(net_type))?;

        let (addr_type, other) = other
            .separate_trimmed(WHITESPACE, WHITESPACE)
            .ok_or_else(|| ParsingError::from_bytes(data))?;

        let addr_type = AddressType::from_bytes(addr_type)
            .ok_or_else(|| ParsingError::from_bytes(addr_type))?;
        
        let mut addr = if let Some((addr_slice, other)) = other.separate_trimmed(SLASH, WHITESPACE) {
            if let Some((ttl_slice, count_slice)) = other.separate_trimmed(SLASH, WHITESPACE) {
                ttl = Some(ttl_slice.utf8_to_number::<u8>()?);
                count = count_slice.utf8_to_number::<u8>()?;
            }
            IpAddr::parse(addr_slice, &addr_type)?
        }
        else {
            IpAddr::parse(other, &addr_type)?
        };


        addresses.push(addr);
        for _ in 1..count {
            addr = addr.add_digit(1).ok_or_else(|| ParsingError::from_bytes(data))?;
            addresses.push(addr);
        }

        Ok(ConnectionAddresses {
            net_type,
            addr_type,
            addresses,
            ttl,
        })
    }
}

impl ConnectionAddresses {
    pub fn new(addr_type: AddressType, addresses: Vec<IpAddr>, ttl: Option<u8>) -> Self {
        Self { net_type: NetworkType::Internet, addr_type, addresses, ttl }
    }
    
    pub fn net_type(&self) -> &NetworkType {
        &self.net_type
    }

    pub fn addr_type(&self) -> &AddressType {
        &self.addr_type
    }

    pub fn addresses(&self) -> &[IpAddr] {
        &self.addresses
    }
}
