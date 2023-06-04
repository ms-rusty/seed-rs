use bevy::prelude::Component;

use crate::{shared::Packet, shared::PacketWriter};

use super::ServerStatusPackets;

#[derive(Component)]
pub struct ServerStatusResponsePacket {
    pub json_response: String,
}

impl From<&ServerStatusResponsePacket> for Packet {
    fn from(packet: &ServerStatusResponsePacket) -> Self {
        let mut writer = PacketWriter::new(ServerStatusPackets::StatusResponse.into());
        writer.write_str(&packet.json_response);

        writer.into()
    }
}
