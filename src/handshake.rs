struct HandshakePacket {
    protocol_version: i32,
    address: String,
    port: u8,
    next_state: NextState
}

enum NextState {
    STATUS(u8),
    LOGIN(u8)
}

impl NextState {
    fn id(&self) -> u8 {
        match *self {
            NextState::STATUS => 1,
            NextState::LOGIN => 2,
        }
    }
}