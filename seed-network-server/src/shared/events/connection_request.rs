use std::net::SocketAddr;

use tokio::net::TcpStream;

pub enum ConnectionRequest {
    Success((TcpStream, SocketAddr)),
    Failure(std::io::Error),
}
