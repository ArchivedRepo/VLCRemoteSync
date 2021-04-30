use tokio::{io::AsyncReadExt, net::{TcpListener, TcpStream}};
use core::time;
use std::{io::{self, Read}, ops::Deref};
use std::error::Error;
use std::collections::HashMap;
use std::sync::{RwLock, Arc};
use contract::Identity;

#[derive(Debug, Clone)]
pub struct Server {
    id: Arc<RwLock<u32>>,
    clients: Arc<RwLock<HashMap<u32, std::net::TcpStream>>>,
    url: String,
}

impl Server {
    pub fn new(url: String) -> Server {
        Server {
            id: Arc::new(RwLock::new(0)),
            clients: Arc::new(RwLock::new(HashMap::new())),
            url,
        }
    }
}

async fn on_clients_connect(server: Arc<Server>, socket: TcpStream) -> Result<(), Box<dyn Error>> {
    socket.readable().await?;
    let mut identity_buf:[u8; 1] = [0; 1];
    let mut std_socket = socket.into_std().unwrap();
    std_socket.set_nonblocking(false).unwrap();
    match std_socket.read_exact(&mut identity_buf) {
        Ok(()) => {
            let identity = identity_buf[0];
            println!("Got identity {}", identity);
            if identity == Identity::Client.into() {
                let mut id = server.id.write().unwrap();
                server.clients.write().unwrap().insert(*id, std_socket);
                *id += 1;
            } else if identity == Identity::Host.into() {
                loop {
                    let mut buf: [u8; 2]= [0; 2];
                    match std_socket.read_exact(&mut buf) {
                        Ok(()) => {
                            let command = buf[0];
                            let time_stamp = buf[1];
                            println!("Command {}, time_stamp {}", command, time_stamp);
                        },
                        Err(e) => panic!("{:?}", e),
                    }
                }
            } else {
                println!("Unrecognized Identity");
            }
            Ok(())
        },
        Err(e) => panic!("{:?}", e),
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let server = Arc::new(Server::new("127.0.0.1:8848".into()));
    let listener = TcpListener::bind("127.0.0.1:8848").await?;
    
    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                println!("Accepting connection from {:?}", addr);
                let this_server = server.clone();
                tokio::spawn(async move {
                    on_clients_connect(this_server, socket).await.unwrap();
                });
                
            },
            Err(e) => println!("Error accepting connecting client {:?}", e),
        }
    }
}