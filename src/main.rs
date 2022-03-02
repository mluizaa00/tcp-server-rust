#[macro_use]
extern crate lazy_static;

mod connection_state;
mod handshake;
mod var_int;
mod server;
mod decoder;

use std::{collections::HashMap};
use tokio::{net::{TcpListener, TcpStream}};

use server::ServerInfo;
use crate::{connection_state::Connection};

lazy_static! {
    pub static ref REGISTRY: HashMap<String, connection_state::Connection> = HashMap::new();
    pub static ref SERVER_INFO: ServerInfo = ServerInfo::create_default();
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    start_server(String::from("127.0.0.1:25565")).await;
}

async fn start_server(ip: String) {
    println!("Starting TCP server at {}.", ip);

    let result = TcpListener::bind(ip).await;
    match result {
        Ok(listener) => {
            println!("Connected server.");
            loop {
                let socket = listener.accept().await;
                match socket {
                    Ok(stream) => handle_connection(stream.0).await,
                    Err(error) => println!("Stream connection failed. {}", error.to_string())
                };
            }
        },
        Err(error) => {
            panic!("Couldn't connect to server. Check your ports. Error thrown: {}", error.to_string());
        }
    }
}

async fn handle_connection(stream: TcpStream) {
    let address: String = stream.local_addr().unwrap().ip().to_string();
    println!("Connection established! {}", address);

    decoder::decode(stream, address).await;
}