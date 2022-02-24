use tokio::{net::{TcpStream}, io::{AsyncReadExt}};

struct HandshakePacket {
    protocol_version: i32,
    address: String,
    port: u16,
    next_state: NextState
}

impl HandshakePacket {
    pub async fn read(mut buffer: TcpStream) -> HandshakePacket {
        let protocol_version: i32 = buffer.read_i32().await.unwrap();
        let port: u16 = buffer.read_u16().await.unwrap();
        let next_state: u8 = buffer.read_u8().await.unwrap();

        let mut address: String = String::new();
        buffer.read_to_string(&mut address).await
            .expect("Invalid String on Handshake Packet address field.");
    
        let handshake_packet = HandshakePacket {
                protocol_version: protocol_version,
                address: address,
                port: port,
                next_state: NextState::id(next_state)
        };

        return handshake_packet;
    } 
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
            _ => {
                panic!("Invalid NextState ID, {}",  id)
            }
        }
    }
}

pub async fn handle_handshake(stream: TcpStream) {
    let packet = HandshakePacket::read(stream).await;

    println!("HANDSHAKE | PROTOCOL: {}, ADDRESS {}, PORT {} |", packet.protocol_version, packet.address, packet.port)
    // TODO: Handle handshake packet
}