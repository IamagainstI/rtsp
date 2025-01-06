use abstractions::{
    instancing::default_instance::DefaultCustom, 
    parsing::payload_parser::PayloadParser,
    extensions::vector_extensions::VecExt,
};
use chrono::{DateTime, TimeZone, Utc};
use http::Uri;
use std::{
    net::{
        IpAddr, 
        Ipv4Addr
    }, 
    str::FromStr
};
use timespan::DateTimeSpan;

use crate::{
    address_type::AddressType, 
    media_attribute::PropertyMediaAttribute, 
    media_description::MediaDescription, 
    origin::Origin, 
    time::timing::Timing, 
    KEY_VALUE_SEPARATOR, 
    RAW_SEPARATOR, 
    TRIM, 
    TRIM_REF
};

use super::bandwidth::Bandwidth;


pub struct MediaSession {
    /// v=  
    protocol_version: i32,

    ///   
    originator_of_session: Origin,

    /// s=
    session_name: String,

    /// i=
    media_title: Option<String>,

    /// u=    
    uri_of_description: Uri,

    /// e=
    email_address: Option<String>,

    /// p=
    phone_number: Option<String>,

    /// o= [3]
    network_address: IpAddr,

    /// c=
    connection_address: IpAddr,

    /// k=
    encryption_key: Option<String>,

    /// b=
    bandwidth: Bandwidth,

    /// t=
    timing: Option<Timing>,

    media_descriptions: Vec<MediaDescription>,

    media_attributes: Vec<Box<dyn PropertyMediaAttribute>>,
}

impl Default for MediaSession {
    fn default() -> Self {
        Self {
            protocol_version: Default::default(),
            originator_of_session: Default::default(),
            session_name: Default::default(),
            media_title: Default::default(),
            uri_of_description: Default::default(),
            email_address: Default::default(),
            phone_number: Default::default(),
            network_address: DefaultCustom::default(),
            connection_address: DefaultCustom::default(),
            encryption_key: Default::default(),
            bandwidth: Default::default(),
            timing: None,
            media_descriptions: Default::default(),
            media_attributes: Default::default(),
        }
    }
}

impl PayloadParser<MediaSession> for MediaSession {
    fn parse(data: &[u8]) -> Result<MediaSession, std::io::Error> {
        let session = MediaSession::default();
        let mut slice = data;
        let mut is_session_info_filling = true;

        while let Some((top, bot)) = slice.separate_trimmed(RAW_SEPARATOR, TRIM_REF)  {
            if let Some((left, right)) = top.separate_trimmed(KEY_VALUE_SEPARATOR, TRIM_REF) {
                match *left {
                    b"v" => session.set_protocol_version(std::str::from_utf8(right)?.parse()?),
                    b"o" => session.set_originator_of_session(Origin::parse(right)?),
                    b"s" => session.set_session_name(String::from_utf8_lossy(right).into_owned()),
                    b"i" => session.set_media_title(Some(String::from_utf8_lossy(right).into_owned())),
                    b"u" => session.set_uri_of_description(get_uri(right)?),
                    b"e" => session.set_email_address(Some(String::from_utf8_lossy(right).into_owned())),
                    b"p" => session.set_phone_number(Some(String::from_utf8_lossy(right).into_owned())),
                    b"c" => session.set_connection_address(get_connection_address(right)?),
                    b"b" => session.set_bandwidth(Bandwidth::parse(data)?),
                    b"t" => session.set_timing(timing::Timing::parse(right)?),
                    b"m" =>
                    {
                        is_session_info_filling = false;
                        
                    },
                    b"a" =>
                    {
                        
                    },
                    _ => 
                    {

                    }
                }
            }
            slice = bot;
        }
        todo!()
    }
}

impl MediaSession {
    pub fn protocol_version(&self) -> i32 {
        self.protocol_version
    }

    pub fn originator_of_session(&self) -> &Origin {
        &self.originator_of_session
    }

    pub fn session_name(&self) -> &str {
        &self.session_name
    }

    pub fn media_title(&self) -> Option<&String> {
        self.media_title.as_ref()
    }

    pub fn uri_of_description(&self) -> &Uri {
        &self.uri_of_description
    }

    pub fn email_address(&self) -> Option<&String> {
        self.email_address.as_ref()
    }

    pub fn phone_number(&self) -> Option<&String> {
        self.phone_number.as_ref()
    }

    pub fn network_address(&self) -> IpAddr {
        self.network_address
    }

    pub fn connection_address(&self) -> IpAddr {
        self.connection_address
    }

    pub fn encryption_key(&self) -> Option<&String> {
        self.encryption_key.as_ref()
    }

    pub fn bandwidth(&self) -> &Bandwidth {
        &self.bandwidth
    }

    pub fn timing(&self) -> Option<&Timing> {
        self.timing.as_ref()
    }

    fn set_protocol_version(&mut self, protocol_version: i32) {
        self.protocol_version = protocol_version;
    }

    fn set_originator_of_session(&mut self, originator_of_session: Origin) {
        self.originator_of_session = originator_of_session;
    }

    fn set_session_name(&mut self, session_name: String) {
        self.session_name = session_name;
    }

    fn set_media_title(&mut self, media_title: Option<String>) {
        self.media_title = media_title;
    }

    fn set_uri_of_description(&mut self, uri_of_description: Uri) {
        self.uri_of_description = uri_of_description;
    }

    fn set_email_address(&mut self, email_address: Option<String>) {
        self.email_address = email_address;
    }

    fn set_phone_number(&mut self, phone_number: Option<String>) {
        self.phone_number = phone_number;
    }

    fn set_network_address(&mut self, network_address: IpAddr) {
        self.network_address = network_address;
    }

    fn set_connection_address(&mut self, connection_address: IpAddr) {
        self.connection_address = connection_address;
    }

    fn set_encryption_key(&mut self, encryption_key: Option<String>) {
        self.encryption_key = encryption_key;
    }

    fn set_bandwidth(&mut self, bandwidth: Bandwidth) {
        self.bandwidth = bandwidth;
    }

    fn set_timing(&mut self, timing: Option<Timing>) {
        self.timing = timing;
    }
}


fn get_uri(data: &[u8]) -> Result<Uri, std::io::Error> {
    let uri_str = std::str::from_utf8(data)?;
    let uri = Uri::from_str(uri_str)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    Ok(uri)
}

fn get_connection_address(data: &[u8]) -> Result<IpAddr, std::io::Error> {
    let parts: Vec<&[u8]> = data.split(|&b| b == b' ').collect();
    if let Some((_, next)) = data.separate_trimmed(TRIM, TRIM_REF) &&
       let Some((_type, address)) = next.separate_trimmed(TRIM, TRIM_REF) {
        
        let addr_type = AddressType::from_bytes(_type)?;
        let connection_address = std::str::from_utf8(address)?;

        let ip_addr = match addr_type {
            AddressType::Ipv4 => IpAddr::V4(Ipv4Addr::from_str(connection_address)?),
            AddressType::Ipv6 => IpAddr::V4(Ipv4Addr::from_str(connection_address)?),
            _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Incorrect format {}", std::str::from_utf8(data)?))),
        };
        Ok(ip_addr)
    }
    Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Incorrect format {}", std::str::from_utf8(data)?)))
}