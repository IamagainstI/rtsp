use std::net::{IpAddr, Ipv4Addr};

pub trait DefaultCustom {
    fn default() -> Self;
}

impl DefaultCustom for IpAddr {
    fn default() -> Self {
        IpAddr::V4(Ipv4Addr::LOCALHOST)
    }
}
