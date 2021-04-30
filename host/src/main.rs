use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let mut stream = TcpStream::connect("127.0.0.1:8848").await?;
    let mut buf: [u8; 2] = [0;2];
    stream.write_all(&buf[0..1]).await?;
    buf[0] = 1;
    buf[1] = 88;
    stream.write_all(&buf).await?;
    loop {
        
    }
}