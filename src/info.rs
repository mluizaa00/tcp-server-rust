use crate::connection_state::Connection;
use std::collections::HashMap;

pub struct Registry {
    pub connections: HashMap<String, Connection>
}

pub fn create_default() -> Registry {
    return Registry {
        connections: HashMap::new()
    };
}