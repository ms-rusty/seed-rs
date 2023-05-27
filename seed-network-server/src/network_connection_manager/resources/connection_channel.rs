use bevy::prelude::Resource;
use crossbeam_channel::{Receiver, Sender};

use crate::network_connection_manager::events::ConnectionEvent;

#[derive(Resource)]
pub struct ConnectionChannel {
    pub sender: Sender<ConnectionEvent>,
    pub receiver: Receiver<ConnectionEvent>,
}

impl Default for ConnectionChannel {
    fn default() -> Self {
        let (sender, receiver) = crossbeam_channel::bounded::<ConnectionEvent>(1);
        Self { sender, receiver }
    }
}
