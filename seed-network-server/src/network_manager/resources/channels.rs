use bevy::prelude::Resource;

use crate::network_manager::events::{Connection, ConnectionEvent};

#[derive(Resource)]
pub struct NetworkChannels {
    pub pending_connection_channel: NetworkChannel<ConnectionEvent>,
    pub release_connection_channel: NetworkChannel<Connection>,
    pub pending_client_packet_channel: NetworkChannel<i32>,
}

impl Default for NetworkChannels {
    fn default() -> Self {
        let pending_connection_channel =
            NetworkChannel::new(crossbeam_channel::unbounded::<ConnectionEvent>());

        let release_connection_channel =
            NetworkChannel::new(crossbeam_channel::unbounded::<Connection>());

        let pending_client_packet_channel =
            NetworkChannel::new(crossbeam_channel::unbounded::<i32>());

        Self {
            pending_connection_channel,
            release_connection_channel,
            pending_client_packet_channel,
        }
    }
}

type Sender<T> = crossbeam_channel::Sender<T>;
type Receiver<T> = crossbeam_channel::Receiver<T>;
type Channel<T> = (Sender<T>, Receiver<T>);

pub struct NetworkChannel<T> {
    pub sender: Sender<T>,
    pub receiver: Receiver<T>,
}

impl<T> NetworkChannel<T> {
    pub fn new((sender, receiver): Channel<T>) -> Self {
        Self { sender, receiver }
    }
}
