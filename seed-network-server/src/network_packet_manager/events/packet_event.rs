use bevy::prelude::Entity;

use crate::network_packet_manager::components::Packet;

#[derive(Debug)]
pub enum ReadPacketEvent {
    Success((Entity, Packet)),
    Failure((Entity, anyhow::Error)),
}
