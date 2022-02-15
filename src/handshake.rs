use std::net::TcpStream;

struct HandshakePacket {
    protocol_version: i32,
    address: String,
    port: u8,
    next_state: NextState
}

enum NextState {
    Status(u8),
    Login(u8)
}

impl NextState {
    fn id(&self) -> u8 {
        match *self {
            NextState::Status(_) => 1,
            NextState::Login(_) => 2,
        }
    }
}

pub fn handle_handshake(stream: TcpStream, packet_id: u8) {
   
}