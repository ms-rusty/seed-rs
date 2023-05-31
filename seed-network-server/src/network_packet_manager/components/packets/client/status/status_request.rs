use bevy::prelude::Component;

use crate::network_packet_manager::components::packets::{packet_errors::PacketError, Packet};

use super::packets_id::ClientStatusPackets;

#[derive(Component, Debug)]
pub struct ClientStatusRequestPacket;

impl TryFrom<&Packet> for ClientStatusRequestPacket {
    type Error = PacketError;

    fn try_from(packet: &Packet) -> Result<Self, Self::Error> {
        if packet.id != ClientStatusPackets::StatusRequest {
            return Err(Self::Error::InvalidPacket);
        }

        Ok(Self)
    }
}
