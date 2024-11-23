use std::net::IpAddr;
use abstractions::parsing::payload_parser::PayloadParser;
use http::Uri;
use timespan::DateTimeSpan;
use chrono::Utc;

use crate::origin::Origin;

use super::bandwidth:: Bandwidth;

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
    
    /// t= [0]
    start_time: Option<DateTimeSpan<Utc>>,
    
    /// t= [1]
    stop_time: Option<DateTimeSpan<Utc>>
}

impl PayloadParser<MediaSession> for MediaSession {
    fn parse() -> Result<MediaSession, std::io::Error> {
        todo!()
    }
}

impl MediaSession {
    /// Creates a new [`MediaSession`].
    pub fn new(
        protocol_version: i32, 
        originator_of_session: Origin, 
        session_name: String, 
        media_title: Option<String>, 
        uri_of_description: Uri, 
        email_address: Option<String>, 
        phone_number: Option<String>, 
        network_address: IpAddr, 
        connection_address: IpAddr, 
        encryption_key: Option<String>, 
        bandwidth: Bandwidth, 
        start_time: Option<DateTimeSpan<Utc>>, 
        stop_time: Option<DateTimeSpan<Utc>>
    ) -> Self {
        Self { 
            protocol_version, 
            originator_of_session, 
            session_name, 
            media_title, 
            uri_of_description, 
            email_address, 
            phone_number, 
            network_address, 
            connection_address, 
            encryption_key, 
            bandwidth, 
            start_time, 
            stop_time 
        }
    }
    
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
    
    pub fn start_time(&self) -> Option<&DateTimeSpan<Utc>> {
        self.start_time.as_ref()
    }
    
    pub fn stop_time(&self) -> Option<&DateTimeSpan<Utc>> {
        self.stop_time.as_ref()
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
    
    fn set_start_time(&mut self, start_time: Option<DateTimeSpan<Utc>>) {
        self.start_time = start_time;
    }
    
    fn set_stop_time(&mut self, stop_time: Option<DateTimeSpan<Utc>>) {
        self.stop_time = stop_time;
    }
    
}