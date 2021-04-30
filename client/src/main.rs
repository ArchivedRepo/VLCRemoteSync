use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8848").unwrap();

    let mut buf: [u8; 2] = [1; 2];
    stream.write_all(&buf[0..1]).unwrap();
    loop {
        match stream.read_exact(&mut buf) {
            Ok(()) => {
                println!("Get command {}, with timestamp {}", buf[0], buf[1]);
            },
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }

}