use bevy::{
    prelude::{Bundle, Component, Deref, DerefMut, Entity},
    utils::Uuid,
};

use tokio::task::JoinHandle;

use crate::network_manager::resources::NetworkChannel;

use super::{Connection, ConnectionHandshakingState};

#[derive(Component, Clone, Copy, Hash, PartialEq, Eq, Deref, DerefMut)]
pub struct ClientId(pub Uuid);

#[derive(Component)]
pub struct Client {
    pub client_message_channel: NetworkChannel<()>,
    pub server_message_channel: NetworkChannel<()>,
    pub client_packet_handler: JoinHandle<()>,
}

impl Client {
    pub fn new(client_packet_handler: JoinHandle<()>) -> Self {
        let client_message_channel = NetworkChannel::new(crossbeam_channel::unbounded::<()>());
        let server_message_channel = NetworkChannel::new(crossbeam_channel::unbounded::<()>());

        Self {
            client_message_channel,
            server_message_channel,
            client_packet_handler,
        }
    }
}
