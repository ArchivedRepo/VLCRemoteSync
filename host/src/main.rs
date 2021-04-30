use std::io::prelude::*;
use std::net::TcpStream;
use contract::Identity;

fn main() {
    // Connect to a peer
    let mut stream = TcpStream::connect("127.0.0.1:8848").unwrap();
    let mut buf: [u8; 2] = [Identity::Host.into(); 2];
    stream.write_all(&buf[0..1]).unwrap();
    buf[0] = 1;
    buf[1] = 88;
    stream.write_all(&buf).unwrap();
    loop {
        
    }
}