use bevy::prelude::Resource;
use crossbeam_channel::{Receiver, Sender};

use crate::network_packet_manager::events::ReadPacketEvent;

#[derive(Resource)]
pub struct ReadPacketChannel {
    pub sender: Sender<ReadPacketEvent>,
    pub receiver: Receiver<ReadPacketEvent>,
}

impl Default for ReadPacketChannel {
    fn default() -> Self {
        let (sender, receiver) = crossbeam_channel::unbounded::<ReadPacketEvent>();
        Self { sender, receiver }
    }
}
