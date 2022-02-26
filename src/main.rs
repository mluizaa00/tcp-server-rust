#[macro_use]
extern crate lazy_static;

mod connection_state;
mod handshake;
mod var_int;
mod server;

use std::collections::HashMap;

use server::ServerInfo;
use tokio::{net::{TcpListener, TcpStream}};

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

async fn handle_connection(mut stream: TcpStream) {
    let address = stream.local_addr().unwrap().ip();
    println!("Connection established! {}", address.to_string());

    let packet_id = var_int::read_varint_i32(&mut stream).await.unwrap();
    println!("PACKET ID: {}", packet_id);

    let connection: Connection;
    if REGISTRY.contains_key(&address.to_string()) {
        connection = REGISTRY.get(&address.to_string()).cloned().unwrap();
    } else {
        connection = connection_state::create_default_connection();
    };

    match connection.state {
        handshake => handshake::handle_handshake(stream, connection).await,
    }

    // REGISTRY.insert(address.to_string(), connection);
}