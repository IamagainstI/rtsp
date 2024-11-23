use std::net::IpAddr;

use abstractions::parsing::payload_parser::PayloadParser;

pub struct Origin {
    user_name: String,
    session_version: String,
    session_id: String,
    network_type: String,
    address_type: String,
    network_address: IpAddr 
}

impl PayloadParser<Origin> for Origin {
    fn parse() -> Result<Origin, std::io::Error> {
        todo!()
    }
}