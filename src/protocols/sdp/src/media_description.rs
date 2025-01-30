use abstractions::media::codec::Codec;

use crate::{bandwidth::Bandwidth, data_transfer_mode::DataTransferMode, payload_type::PayloadType};

pub struct MediaDescription {
    bandwidth: Bandwidth,
    codecs: Vec<Box<dyn Codec>>,
    data_transfer_mode: DataTransferMode,
    name: String,
    number_of_ports: Option<i32>,
    payload_type: PayloadType,
    port: i32,
    transport: String,
}

impl MediaDescription {
    pub fn set_bandwidth(&mut self, bandwidth: Bandwidth) {
        self.bandwidth = bandwidth;
    }

    pub fn set_codecs(&mut self, codecs: Vec<Box<dyn Codec>>) {
        self.codecs = codecs;
    }

    pub fn set_data_transfer_mode(&mut self, data_transfer_mode: DataTransferMode) {
        self.data_transfer_mode = data_transfer_mode;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_number_of_ports(&mut self, number_of_ports: Option<i32>) {
        self.number_of_ports = number_of_ports;
    }

    pub fn set_payload_type(&mut self, payload_type: PayloadType) {
        self.payload_type = payload_type;
    }

    pub fn set_port(&mut self, port: i32) {
        self.port = port;
    }

    pub fn set_transport(&mut self, transport: String) {
        self.transport = transport;
    }
    
    pub fn bandwidth(&self) -> &Bandwidth {
        &self.bandwidth
    }
    
    pub fn codecs(&self) -> &Vec<Box<dyn Codec>> {
        &self.codecs
    }
    
    pub fn data_transfer_mode(&self) -> &DataTransferMode {
        &self.data_transfer_mode
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn number_of_ports(&self) -> Option<i32> {
        self.number_of_ports
    }
    
    pub fn payload_type(&self) -> &PayloadType {
        &self.payload_type
    }
    
    pub fn port(&self) -> i32 {
        self.port
    }
    
    pub fn transport(&self) -> &str {
        &self.transport
    }
}