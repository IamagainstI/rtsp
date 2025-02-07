use super::bandwidth::Bandwidth;
use crate::{
    address_type::AddressType, data_transfer_mode::DataTransferMode,
    media_attribute::UnknownMediaAttribute, media_description::MediaDescription, origin::Origin,
    time::timing::Timing, 
    KEY_VALUE_SEPARATOR, RAW_SEPARATOR, TRIM,
};
use abstractions::{
    extensions::{array_extensions::ArrayExt, utf8_array_extensions::U8ArrayExt},
    instancing::default_instance::DefaultInstance,
    parsing::{parsing_error::ParsingError, payload_parser::PayloadParser},
};
use http::Uri;
use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

const VERSION: &[u8] = b"v";
const ORIGIN: &[u8] = b"o";
const SESSION_NAME: &[u8] = b"s";
const MEDIA_TITLE: &[u8] = b"i";
const URI: &[u8] = b"u";
const EMAIL: &[u8] = b"e";
const PHONE: &[u8] = b"p";
const CONNECTION: &[u8] = b"c";
const BANDWIDTH: &[u8] = b"b";
const TIMING: &[u8] = b"t";
const MEDIA_DESC: &[u8] = b"m";
const ATTRIBUTE: &[u8] = b"a";

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
    uri_of_description: Option<Uri>,

    /// e=
    email_address: Option<String>,

    /// p=
    phone_number: Option<String>,

    /// o= [3]
    network_address: IpAddr,

    /// c=
    connection_address: Option<IpAddr>,

    /// k=
    encryption_key: Option<String>,

    /// b=
    bandwidth: Option<Bandwidth>,

    /// t=
    timing: Option<Timing>,

    media_descriptions: Vec<MediaDescription>,

    media_attributes: Vec<UnknownMediaAttribute>,

    data_transfer_mode: Option<DataTransferMode>,
}

impl Default for MediaSession {
    fn default() -> Self {
        Self {
            protocol_version: Default::default(),
            originator_of_session: Default::default(),
            session_name: Default::default(),
            media_title: None,
            uri_of_description: None,
            email_address: None,
            phone_number: None,
            network_address: DefaultInstance::default(),
            connection_address: None,
            encryption_key: None,
            bandwidth: None,
            timing: None,
            media_descriptions: Default::default(),
            media_attributes: Default::default(),
            data_transfer_mode: None,
        }
    }
}

impl PayloadParser for MediaSession {
    fn parse(data: &[u8]) -> Result<Self, ParsingError> {
        let mut media_attributes: Vec<UnknownMediaAttribute> = Vec::default();
        let mut session = MediaSession::default();
        let mut slice = data;

        while let Some((top, bot)) = slice.separate_trimmed(RAW_SEPARATOR, TRIM) {
            if let Some((left, right)) = top.separate_trimmed(KEY_VALUE_SEPARATOR, TRIM) {
                match left {
                    VERSION => session.set_protocol_version(right.utf8_to_number::<i32>()?),
                    ORIGIN => session.set_originator_of_session(Origin::parse(right)?),
                    SESSION_NAME => session.set_session_name(right.utf8_to_str()?.to_string()),
                    MEDIA_TITLE => session.set_media_title(Some(right.utf8_to_str()?.to_string())),
                    URI => session.set_uri_of_description(Some(get_uri(right)?)),
                    EMAIL => session.set_email_address(Some(right.utf8_to_str()?.to_string())),
                    PHONE => session.set_phone_number(Some(right.utf8_to_str()?.to_string())),
                    CONNECTION => session.set_connection_address(Some(get_connection_address(right)?)),
                    BANDWIDTH => session.set_bandwidth(Some(Bandwidth::parse(data)?)),
                    TIMING => session.set_timing(Some(Timing::parse(right)?)),
                    MEDIA_DESC => {
                        session.set_media_attributes(media_attributes);
                        let mut media_descriptions: Vec<MediaDescription> = Vec::default();
                        let separator = [MEDIA_DESC, KEY_VALUE_SEPARATOR].concat();
                        while let Some((top, bot)) = slice[2..].separate_trimmed(separator.as_slice(), TRIM) {
                            let media_desc: MediaDescription = MediaDescription::parse(top)?;
                            media_descriptions.push(media_desc);
                            slice = bot;
                        }
                        if !slice.is_empty() {
                            let media_desc: MediaDescription = MediaDescription::parse(slice.trim(TRIM))?;
                            media_descriptions.push(media_desc);
                        }
                        session.set_media_descriptions(media_descriptions);
                        break;
                    }
                    ATTRIBUTE => {
                        if let Some((key, value)) = right.separate_trimmed(TRIM, TRIM) {
                            let media_attribute = UnknownMediaAttribute::new(
                                key.utf8_to_str()?.to_string(),
                                Some(value.utf8_to_str()?.to_string()),
                            );
                            media_attributes.push(media_attribute);
                        } 
                        else if let Some(data_transfer_mode) = DataTransferMode::from_bytes(right) {
                            session.set_data_transfer_mode(Some(data_transfer_mode));
                        }
                        else {
                            let media_attribute = UnknownMediaAttribute::new(
                                right.utf8_to_str()?.to_string(),
                                None,
                            );
                            media_attributes.push(media_attribute);
                        }
                    }
                    _ => {
                        continue;
                    }
                }
            }
            slice = bot;
        }
        if !session.is_valid() {
            return Err(ParsingError::from_bytes(data));
        }
        Ok(session)
    }
}

impl MediaSession {
    pub fn originator_of_session(&self) -> &Origin {
        &self.originator_of_session
    }

    pub fn session_name(&self) -> &str {
        &self.session_name
    }

    pub fn media_title(&self) -> Option<&String> {
        self.media_title.as_ref()
    }

    pub fn uri_of_description(&self) -> Option<&Uri> {
        self.uri_of_description.as_ref()
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

    pub fn connection_address(&self) -> Option<IpAddr> {
        self.connection_address
    }

    pub fn encryption_key(&self) -> Option<&String> {
        self.encryption_key.as_ref()
    }
    
    pub fn bandwidth(&self) -> Option<&Bandwidth> {
        self.bandwidth.as_ref()
    }
    
    pub fn timing(&self) -> Option<&Timing> {
        self.timing.as_ref()
    }
    
    pub fn media_descriptions(&self) -> &[MediaDescription] {
        &self.media_descriptions
    }
    
    pub fn media_attributes(&self) -> &[UnknownMediaAttribute] {
        &self.media_attributes
    }
    
    pub fn data_transfer_mode(&self) -> Option<DataTransferMode> {
        self.data_transfer_mode
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

    fn set_uri_of_description(&mut self, uri_of_description: Option<Uri>) {
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

    fn set_connection_address(&mut self, connection_address: Option<IpAddr>) {
        self.connection_address = connection_address;
    }

    fn set_encryption_key(&mut self, encryption_key: Option<String>) {
        self.encryption_key = encryption_key;
    }

    fn set_bandwidth(&mut self, bandwidth: Option<Bandwidth>) {
        self.bandwidth = bandwidth;
    }

    fn set_timing(&mut self, timing: Option<Timing>) {
        self.timing = timing;
    }

    fn set_media_descriptions(&mut self, media_descriptions: Vec<MediaDescription>) {
        self.media_descriptions = media_descriptions;
    }

    fn set_media_attributes(&mut self, media_attributes: Vec<UnknownMediaAttribute>) {
            self.media_attributes = media_attributes;
        }
    
    pub fn set_data_transfer_mode(&mut self, data_transfer_mode: Option<DataTransferMode>) {
        self.data_transfer_mode = data_transfer_mode;
    }

    fn is_valid(&self) -> bool {
        self.protocol_version != i32::default()
            && self.originator_of_session != Origin::default()
            && self.session_name != String::default()
            && self.network_address != IpAddr::default()
            && !self.media_attributes.is_empty()
            && !self.media_descriptions.is_empty()
    }
    
    
}

fn get_uri(data: &[u8]) -> Result<Uri, ParsingError> {
    let uri_str = data.utf8_to_str()?;
    let uri = Uri::from_str(uri_str).map_err(|_| ParsingError::from_bytes(data))?;
    Ok(uri)
}

fn get_connection_address(data: &[u8]) -> Result<IpAddr, ParsingError> {
    if let Some((_, next)) = data.separate_trimmed(TRIM, TRIM) {
        if let Some((_type, address)) = next.separate_trimmed(TRIM, TRIM) {
            let addr_type = AddressType::from_bytes(_type).ok_or(ParsingError::from_bytes(data))?;
            let connection_address = address.utf8_to_str()?;

            let ip_addr = match addr_type {
                AddressType::Ipv4 => IpAddr::V4(
                    Ipv4Addr::from_str(connection_address)
                        .map_err(|_| ParsingError::from_bytes(data))?,
                ),
                AddressType::Ipv6 => IpAddr::V6(
                    Ipv6Addr::from_str(connection_address)
                        .map_err(|_| ParsingError::from_bytes(data))?,
                ),
            };
            return Ok(ip_addr);
        }
    }
    Err(ParsingError::from_bytes(data))
}
