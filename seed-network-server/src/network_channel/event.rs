use seed_network_common::Connection;
use std::net::SocketAddr;
use tokio::net::TcpStream;

pub type NetworkEventChannelSender = crossbeam_channel::Sender<NetworkEventChannel>;
pub type NetworkEventChannelReceiver = crossbeam_channel::Receiver<NetworkEventChannel>;

pub enum NetworkEventChannel {
    ConnectionEvent(ConnectionEvent),
    DisconnectionEvent(Connection),
}

pub enum ConnectionEvent {
    Success((TcpStream, SocketAddr)),
    Failure(std::io::Error),
}

impl From<Result<(TcpStream, SocketAddr), std::io::Error>> for ConnectionEvent {
    fn from(value: Result<(TcpStream, SocketAddr), std::io::Error>) -> Self {
        match value {
            Ok(connection) => ConnectionEvent::Success(connection),
            Err(error) => ConnectionEvent::Failure(error),
        }
    }
}
