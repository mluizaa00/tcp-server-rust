use tokio::{net::{TcpStream}, io::{AsyncReadExt, BufReader, AsyncBufReadExt}};

use crate::{connection_state::Connection, SERVER_INFO};
use crate::server::Packet;
use async_trait::async_trait;

pub struct HandshakePacket {
    protocol_version: i32,
    address: String,
    port: u16,
    next_state: NextState
}

#[async_trait]
impl Packet<HandshakePacket> for HandshakePacket {
    async fn read(mut buffer: TcpStream) -> HandshakePacket {
        let protocol_version: i32 = buffer.read_i32().await.unwrap();
        let port: u16 = buffer.read_u16().await.unwrap();
        let next_state: u8 = buffer.read_u8().await.unwrap();

        let mut reader = BufReader::new(buffer);
        
        let mut address = String::with_capacity(255);
        reader.read_line(&mut address).await.unwrap();
    
        let handshake_packet = HandshakePacket {
            protocol_version: protocol_version,
            address: address,
            port: port,
            next_state: NextState::id(next_state)
        };

        return handshake_packet;
    }

    async fn write(_buffer: TcpStream) {}
}

#[derive(Debug)]
enum NextState {
    Status,
    Login
}

impl NextState {
    fn id(id: u8) -> NextState {
        match id {
            1 => NextState::Status,
            2 => NextState::Login,
            _ => panic!("Invalid NextState ID, {}",  id)
        }
    }
}

pub async fn handle_handshake(stream: TcpStream, connection: Connection) {
    let packet = HandshakePacket::read(stream).await;
    println!("HANDSHAKE | PROTOCOL: {}, ADDRESS {}, PORT {} |", packet.protocol_version, packet.address, packet.port);

    if packet.protocol_version != SERVER_INFO.protocol_version {
        println!("The protocol version is not compatible with the server version.");
        return;
    }

    // TODO: Handle handshake packet
}