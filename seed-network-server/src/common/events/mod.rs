pub type ConnectionSuccess = (tokio::net::TcpStream, std::net::SocketAddr);
pub type ConnectionFailure = std::io::Error;
pub type ConnectionResult = Result<ConnectionSuccess, ConnectionFailure>;

#[derive(Debug)]
pub enum NetworkConnectionEvent {
    Success(ConnectionSuccess),
    Failure(ConnectionFailure),
}

impl From<ConnectionResult> for NetworkConnectionEvent {
    fn from(result: ConnectionResult) -> Self {
        match result {
            Ok(success) => Self::Success(success),
            Err(failure) => Self::Failure(failure),
        }
    }
}
