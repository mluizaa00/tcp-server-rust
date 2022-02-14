pub enum ConnectionState {
    Handshake,
    Status,
    Login,
    Play
}

pub struct Connection {
    pub state: ConnectionState
}

pub fn create_default_connection() -> Connection {
    return Connection {
        state: ConnectionState::Handshake
    };
}