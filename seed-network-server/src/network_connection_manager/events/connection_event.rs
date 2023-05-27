use bevy::prelude::Component;
use std::net::SocketAddr;
use tokio::net::TcpStream;

#[derive(Component)]
pub enum ConnectionEvent {
    Success((TcpStream, SocketAddr)),
    Failure(std::io::Error),
}
