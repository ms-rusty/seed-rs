use bevy::prelude::Component;

use crate::shared::{Packet, PacketWriter};

use super::ServerStatusPackets;

#[derive(Component)]
pub struct ServerPingResponsePacket {
    pub payload: i64,
}

impl From<&ServerPingResponsePacket> for Packet {
    fn from(packet: &ServerPingResponsePacket) -> Self {
        let mut writer = PacketWriter::new(ServerStatusPackets::PingResponse.into());
        writer.write_i64(packet.payload);

        writer.into()
    }
}
