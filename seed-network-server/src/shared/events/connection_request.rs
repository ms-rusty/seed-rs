pub enum ConnectionRequest {
    Success((TcpStream, SocketAddr)),
    Failure(std::io::Error),
}
