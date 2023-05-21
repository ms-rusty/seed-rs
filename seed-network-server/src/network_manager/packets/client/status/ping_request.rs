use crate::network_manager::packets::{
    packet_errors::PacketError, packet_reader::PacketReader, ClientStatusPackets, Packet,
};

#[derive(Debug)]
pub struct ClientPingRequestPacket {
    pub payload: i64,
}

impl TryFrom<&Packet> for ClientPingRequestPacket {
    type Error = PacketError;

    fn try_from(packet: &Packet) -> Result<Self, Self::Error> {
        if packet.id != ClientStatusPackets::PingRequest {
            return Err(Self::Error::InvalidPacket);
        }

        let mut reader = PacketReader::from(packet);
        let payload = reader.read_int(reader.remaining())?;

        Ok(Self { payload })
    }
}
