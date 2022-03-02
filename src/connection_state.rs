use tokio::net::TcpStream;

use crate::{handshake::HandshakePacket, server::Packet};

#[derive(Clone, Copy)]
pub enum ConnectionState {
    Handshake,
    Status,
    Login,
    Play
}

#[derive(Clone)]
pub struct Connection {
    pub state: ConnectionState
}

pub fn create_default_connection() -> Connection {
    return Connection {
        state: ConnectionState::Handshake
    };
}

pub enum HandshakePackets {
    Handshake
}

impl HandshakePackets {
    async fn get<T>(id: u8, stream: TcpStream) -> HandshakePacket {
        match id {
            0x00 => HandshakePacket::read(stream).await,
            _ => panic!("A invalid packet has been received in the Handshake end point.")
        }
    }
}

pub enum StatusPackets {
    Request,
    Response,
    Ping,
    Pong
}

pub enum LoginPackets {
    Start,
    Success
}