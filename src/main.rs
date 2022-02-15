#[macro_use]
extern crate lazy_static;

mod connection_state;
mod handshake;
mod status;

use std::{net::{TcpListener, TcpStream}};
use std::collections::HashMap;
use varuint::{ReadVarint};

use crate::connection_state::Connection;

lazy_static! {
    pub static ref REGISTRY: HashMap<String, connection_state::Connection> = HashMap::new();
}

fn main() {
    println!("Hello, world!");
    start_server(String::from("127.0.0.1:25565"));
}

fn start_server(ip: String) {
    println!("Starting TCP server at {}.", ip);

    let listener = TcpListener::bind(ip);
    match listener {
        Ok(listener) => {
            println!("Connected server.");
            listener.incoming().for_each(|stream| {
                match stream {
                    Ok(stream) => handle_connection(stream),
                    Err(error) => println!("Stream connection failed. {}", error.to_string())
                }
            });
        },
        Err(error) => {
            panic!("Couldn't connect to server. Error thrown: {}", error.to_string());
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let address = stream.local_addr().unwrap().ip();
    println!("Connection established! {}", address.to_string());

    let connection: Connection = REGISTRY.contains_key(&address.to_string())
        .then(|| REGISTRY.get(&address.to_string()).cloned().unwrap_or(connection_state::create_default_connection()))
        .unwrap();

    let packet_id: u8 = stream.read_varint().unwrap();

    match connection.state {
        Handshake => handshake::handle_handshake(stream),
        Status => status::handle_status(stream, packet_id)
    }

    // REGISTRY.insert(address.to_string(), connection);
}