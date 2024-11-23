use std::net::{IpAddr, Ipv4Addr};

fn main() {
    // Statements here are executed when the compiled binary is called.

    // Print text to the console.
    println!("Hello World!");
    IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));
}