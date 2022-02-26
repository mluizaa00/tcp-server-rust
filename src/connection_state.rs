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