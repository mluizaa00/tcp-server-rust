use std::{net::TcpStream, io::{Read, BufReader}};
use varuint::ReadVarint;
struct HandshakePacket {
    protocol_version: i32,
    address: String,
    port: u16,
    next_state: NextState
}

impl HandshakePacket {
    pub fn read(mut buffer: BufReader<TcpStream>) -> HandshakePacket {
        let protocol_version = buffer.read_varint().unwrap();

        let mut address: String = String::new();
        buffer.read_to_string(&mut address)
            .expect("Unable to read Handshake Packet address field.");
    
        let mut port_bytes = [0; 5];
        buffer.read_exact(&mut port_bytes)
            .expect("Unable to read Handshake Packet port field.");

        let next_state: u8 = 1;
        buffer.read_exact(&mut [next_state])
            .expect("Unable to read Handshake Packet next_state field.");
    
        let handshake_packet = HandshakePacket {
                protocol_version: protocol_version,
                address: address,
                port: 25565,
                next_state: NextState::id(next_state)
        };

        return handshake_packet;
    } 
}
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

pub fn handle_handshake(stream: TcpStream) {
    let buffer: BufReader<TcpStream> = BufReader::new(stream.try_clone().unwrap());

    let handshake_packet = HandshakePacket::read(buffer);
    // TODO: Handle handshake packet
}