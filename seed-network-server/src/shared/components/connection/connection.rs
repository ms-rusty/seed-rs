use bevy::prelude::Component;
use std::net::SocketAddr;

#[derive(Component)]
pub struct Connection {
    pub socket_address: SocketAddr,
}

impl Connection {
    pub fn new(socket_address: SocketAddr) -> Self {
        Self { socket_address }
    }
}
