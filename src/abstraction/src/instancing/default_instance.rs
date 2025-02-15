use std::net::{IpAddr, Ipv4Addr};

pub trait DefaultInstance {
    fn default() -> Self;
}

impl DefaultInstance for IpAddr {
    fn default() -> Self {
        IpAddr::V4(Ipv4Addr::LOCALHOST)
    }
}