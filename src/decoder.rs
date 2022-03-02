use tokio::net::TcpStream;
use crate::{var_int, Connection, REGISTRY, connection_state, handshake};

pub async fn decode(mut stream: TcpStream, address: String) {
    let packet_id = var_int::read_i32(&mut stream).await.unwrap();
    println!("PACKET ID: {}", packet_id);

    let connection: Connection;
    if REGISTRY.contains_key(&address) {
        connection = REGISTRY.get(&address).cloned().unwrap();
    } else {
        connection = connection_state::create_default_connection();
    };

    match connection.state {
        handshake => handshake::handle_handshake(stream, connection).await,
    }

    // REGISTRY.insert(address.to_string(), connection);
}