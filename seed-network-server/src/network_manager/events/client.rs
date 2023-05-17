use bevy::utils::Uuid;
use std::sync::Arc;
use tokio::{sync::Mutex, task::JoinHandle};

use super::Connection;
use crate::network_manager::resources::NetworkChannel;

#[derive(Hash, PartialEq, Eq)]
pub struct ClientId(pub Uuid);

pub struct Client {
    pub connection: Arc<Mutex<Connection>>,
    pub client_message_channel: NetworkChannel<()>,
    pub server_message_channel: NetworkChannel<()>,
    pub client_packet_handler: JoinHandle<()>,
}

impl Client {
    pub fn new(connection: Arc<Mutex<Connection>>, client_packet_handler: JoinHandle<()>) -> Self {
        let client_message_channel = NetworkChannel::new(crossbeam_channel::unbounded::<()>());
        let server_message_channel = NetworkChannel::new(crossbeam_channel::unbounded::<()>());

        Self {
            connection,
            client_message_channel,
            server_message_channel,
            client_packet_handler,
        }
    }
}
