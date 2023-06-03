use bevy::prelude::Component;

use crate::shared::Packet;

#[derive(Component)]
pub struct ConnectionClientPacketsChannel {
    pub sender: crossbeam_channel::Sender<Result<Packet, anyhow::Error>>,
    pub receiver: crossbeam_channel::Receiver<Result<Packet, anyhow::Error>>,
}

impl Default for ConnectionClientPacketsChannel {
    fn default() -> Self {
        let (sender, receiver) = crossbeam_channel::unbounded::<Result<Packet, anyhow::Error>>();
        Self { sender, receiver }
    }
}
