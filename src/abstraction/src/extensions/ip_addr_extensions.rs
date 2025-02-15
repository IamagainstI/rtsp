use std::{net::{IpAddr, Ipv4Addr, Ipv6Addr}, str::FromStr};

use crate::{net::address_type::AddressType, parsing::parsing_error::ParsingError};

use super::utf8_array_extensions::U8ArrayExt;

pub trait IpAddrExt {
    fn parse(data: &[u8], address_type: &AddressType) -> Result<IpAddr, ParsingError>;
    fn add_digit(&self, digit: u8) -> Option<IpAddr>;
}

impl IpAddrExt for IpAddr {
    fn parse(data: &[u8], address_type: &AddressType) -> Result<IpAddr, ParsingError> {
        let str_data = data.utf8_to_str()?;

        let ip_addr = match address_type {
            AddressType::Ipv4 => IpAddr::V4(
                Ipv4Addr::from_str(str_data)
                    .map_err(|_| ParsingError::from_bytes(data))?,
            ),
            AddressType::Ipv6 => IpAddr::V6(
                Ipv6Addr::from_str(str_data)
                    .map_err(|_| ParsingError::from_bytes(data))?,
            ),
        };
        return Ok(ip_addr);
    }
    
    fn add_digit(&self, digit: u8) -> Option<IpAddr> {
        match self {
            IpAddr::V4(addr) => {
                let mut octets = addr.octets();
                if let Some(new_octet) = octets[3].checked_add(digit) {
                    octets[3] = new_octet;
                    Some(IpAddr::V4(octets.into()))
                } else {
                    None
                }
            }
            IpAddr::V6(addr) => {
                let mut segments = addr.segments();
                if let Some(new_segment) = segments[7].checked_add(digit as u16) {
                    segments[7] = new_segment;
                    Some(IpAddr::V6(segments.into()))
                } else {
                    None
                }
            }
        }
    }
}