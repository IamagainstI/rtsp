use std::{net::{IpAddr, Ipv4Addr, Ipv6Addr}, str::FromStr};

use abstractions::{extensions::vector_extensions::VecExt, instancing::default_instance::DefaultCustom, parsing::payload_parser::PayloadParser};

use crate::{address_type::{self, AddressType}, network_type::NetworkType, TRIM_REF};

pub struct Origin {
    user_name: String,
    session_version: String,
    session_id: String,
    network_type: NetworkType,
    address_type: AddressType,
    network_address: IpAddr,
}

impl PayloadParser<Origin> for Origin {
    fn parse(data: &[u8]) -> Result<Origin, std::io::Error> {
        if let Some((user_name, other)) = data.separate_trimmed(b" ", TRIM_REF) && 
           let Some((id, other)) = other.separate_trimmed(b" ", TRIM_REF) && 
           let Some((session_id, other)) = other.separate_trimmed(b" ", TRIM_REF) && 
           let Some((session_version, other)) = other.separate_trimmed(b" ", TRIM_REF) &&
           let Some((network_type, other)) = other.separate_trimmed(b" ", TRIM_REF) && 
           let Some((address_type, network_address)) = other.separate_trimmed(b" ", TRIM_REF) {

            let str_network_address = std::str::from_utf8(network_address)?;
            let address_type_res = AddressType::from_bytes(address_type)?;
            
            let network_address_res = match address_type_res {
                AddressType::Ipv4 => IpAddr::V4(Ipv4Addr::from_str(str_network_address)?),
                AddressType::Ipv6 => IpAddr::V6(Ipv6Addr::from_str(str_network_address)?),
                _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid address type")),
            };
            return Ok(Origin::new(
                std::str::from_utf8(user_name)?.to_string(),
                std::str::from_utf8(session_version)?.to_string(),
                std::str::from_utf8(session_id)?.to_string(),
                NetworkType::from_bytes(network_type).ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid network type"))?,
                address_type_res,
                network_address_res,
            ));
        }
        Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Invalid data at parsing Origin: {:?}", data)))
    }
}

impl Origin {
    fn new(
        user_name: String,
        session_version: String,
        session_id: String,
        network_type: NetworkType,
        address_type: AddressType,
        network_address: IpAddr,
    ) -> Self {
        Self { 
            user_name, 
            session_version, 
            session_id, 
            network_type, 
            address_type, 
            network_address 
        }
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
            network_address: DefaultCustom::default(),
        }
    }
}
