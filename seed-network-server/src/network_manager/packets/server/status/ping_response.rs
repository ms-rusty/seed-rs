use seed_network_server_common::VarInt;

use crate::network_manager::packets::{
    packet_writer::PacketWriter, server::ServerStatusPackets, Packet,
};

pub struct ServerPingResponsePacket {
    pub payload: i64,
}

impl From<&ServerPingResponsePacket> for Packet {
    fn from(packet: &ServerPingResponsePacket) -> Self {
        let mut writer = PacketWriter::new();
        writer.write_i64(packet.payload);

        Self {
            id: VarInt::from(ServerStatusPackets::PingResponse as i32),
            data: writer.get_data(),
        }
    }
}
