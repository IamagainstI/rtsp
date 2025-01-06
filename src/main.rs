use std::io::Read;
use std::net::TcpStream;
use std::net::{IpAddr, Ipv4Addr};

use byteorder::ReadBytesExt;

fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);

    // `c` is destroyed and the memory freed
}

const B: i32 = 1;

fn main() {
    let a = &[1];
    let b = 1;

    match a {
        [B] => print!("sdfsadf"),
        _ => print!("s")
    }
}
