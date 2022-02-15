#[macro_use]
extern crate lazy_static;

mod connection_state;
mod handshake;
mod status;

use std::collections::HashMap;

use tokio::net::{TcpListener, TcpStream};

use crate::connection_state::Connection;

lazy_static! {
    pub static ref REGISTRY: HashMap<String, connection_state::Connection> = HashMap::new();
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
            let socket = listener.accept().await;

            match socket {
                Ok(stream) => handle_connection(stream.0),
                Err(error) => println!("Stream connection failed. {}", error.to_string())
            };
        },
        Err(error) => {
            panic!("Couldn't connect to server. Check your ports. Error thrown: {}", error.to_string());
        }
    }
}

fn handle_connection(stream: TcpStream) {
    let address = stream.local_addr().unwrap().ip();
    println!("Connection established! {}", address.to_string());

    let connection: Connection = REGISTRY.contains_key(&address.to_string())
        .then(|| REGISTRY.get(&address.to_string()).cloned().unwrap_or(connection_state::create_default_connection()))
        .unwrap();

    match connection.state {
        Handshake => handshake::handle_handshake(stream),
    }

    // REGISTRY.insert(address.to_string(), connection);
}