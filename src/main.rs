#[macro_use]
extern crate lazy_static;

mod connection_state;
mod handshake;
mod status;
mod info;

use std::{net::{TcpListener, TcpStream}};

lazy_static! {
    pub static ref REGISTRY: info::Registry = info::create_default();
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

fn handle_connection(stream: TcpStream) {
    let address = stream.local_addr().unwrap().ip();
    println!("Connection established! {}", address.to_string());

    // TODO: verify if stream is valid and it is a packet

    let connection: connection_state::Connection = connection_state::create_default_connection();
    match connection.state {
        Handshake => handshake::handle_handshake(stream),
        Status => status::handle_status(stream)
    }

    // REGISTRY.connections.insert(address.to_string(), connection);
}