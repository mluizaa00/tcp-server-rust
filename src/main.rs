use std::{net::{TcpListener, TcpStream}};

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
    println!("Connection established! {}", stream.local_addr().unwrap());
    
}