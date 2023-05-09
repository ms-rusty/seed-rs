use std::net::SocketAddr;

type ConnectionSuccess = (tokio::net::TcpStream, SocketAddr);
type ConnectionFailure = std::io::Error;
type ConnectionResult = Result<ConnectionSuccess, ConnectionFailure>;

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
