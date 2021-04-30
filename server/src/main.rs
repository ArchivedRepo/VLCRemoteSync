use tokio::{io::AsyncReadExt, net::{TcpListener, TcpStream}};
use core::time;
use std::{io::{self}, ops::Deref};
use std::error::Error;
use std::collections::HashMap;
use std::sync::{RwLock, Arc};
use contract::Identity;

#[derive(Debug, Clone)]
pub struct Server {
    id: Arc<RwLock<u32>>,
    clients: Arc<RwLock<HashMap<u32, TcpStream>>>,
    host: Arc<RwLock<bool>>,
    url: String,
}

impl Server {
    pub fn new(url: String) -> Server {
        Server {
            id: Arc::new(RwLock::new(0)),
            clients: Arc::new(RwLock::new(HashMap::new())),
            host: Arc::new(RwLock::new(false)),
            url,
        }
    }
}

async fn on_clients_connect(server: Arc<Server>, socket: TcpStream) -> Result<(), Box<dyn Error>> {
    socket.readable().await?;
    let mut buf:[u8; 1] = [0; 1];
    // TODO: Try turn into std socket here to do blocking IO!
    match socket.try_read(&mut buf) {
        Ok(0) => Ok(()),
        Ok(_n) => {
            let identity = buf[0];
            print!("Got identity {}", identity);
            if identity == Identity::Client.into() {
                let mut id = server.id.write().unwrap();
                server.clients.write().unwrap().insert(*id, socket);
                *id += 1;
            } else if identity == Identity::Host.into() {
                let this_val;
                {
                    let mut val = server.host.write().unwrap();
                    this_val = *val;
                    if !this_val {
                        *val = true;
                    }
                }
                loop {
                    if this_val {
                        break;
                    }
                    socket.readable().await?;
                    let mut buf: [u8; 2]= [0; 2];
                    match socket.try_read(&mut buf) {
                        Ok(0) => continue,
                        Ok(_n) => {
                            let command = buf[0];
                            let time_stamp = buf[1];
                            println!("Command {}, time_stamp {}", command, time_stamp);
                        },
                        _ => println!("ERR"),
                    }
                }
            } else {
                println!("A host is already connected");
            }
            Ok(())
        },
        Err(e) => Err(e.into())
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