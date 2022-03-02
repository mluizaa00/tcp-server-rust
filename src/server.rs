use std::collections::HashMap;
use uuid::Uuid;
use tokio::net::TcpStream;
use async_trait::async_trait;
pub struct ServerInfo {
    pub version: String,
    pub protocol_version: i32,
    pub max_players: i32,
    pub online_players: i32,
    pub players: HashMap<Uuid, PlayerConnection>,
    pub description: String,
    pub favicon: String
}

impl ServerInfo {
    pub fn create_default() -> ServerInfo {
        return ServerInfo {
            version: String::from("1.8.9"),
            protocol_version: 47,
            max_players: 20,
            online_players: 0,
            players: HashMap::new(),
            description: String::from("This is a test"),
            favicon: String::from("")
        };
    }
}

#[async_trait]
pub trait Packet<T> {
    async fn read(buffer: TcpStream) -> T;
    async fn write(buffer: TcpStream);
}

pub struct PlayerConnection {
    id: Uuid,
    name: String,
    address: String
}