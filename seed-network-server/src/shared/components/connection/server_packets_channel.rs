use bevy::prelude::Component;

use crate::shared::Packet;

#[derive(Component)]
pub struct ConnectionServerPacketsChannel {
    pub sender: crossbeam_channel::Sender<Packet>,
    pub receiver: crossbeam_channel::Receiver<Packet>,
}

impl Default for ConnectionServerPacketsChannel {
    fn default() -> Self {
        let (sender, receiver) = crossbeam_channel::unbounded::<Packet>();
        Self { sender, receiver }
    }
}
