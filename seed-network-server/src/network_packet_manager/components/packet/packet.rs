use bevy::prelude::{Component, Entity};
use bytes::{Bytes, BytesMut};
use seed_network_server_common::VarInt;

// https://wiki.vg/Protocol#Packet_format
#[derive(Component, Debug)]
pub struct Packet {
    pub id: VarInt,
    pub data: Bytes,
}

#[derive(Component)]
pub struct PacketConnectionEntity {
    entity: Entity,
}

impl PacketConnectionEntity {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

impl TryFrom<BytesMut> for Packet {
    type Error = anyhow::Error;

    fn try_from(mut buffer: BytesMut) -> Result<Self, Self::Error> {
        let mut reader = PacketReader::new(&buffer);
        let packet_length = reader.read_var_int()?;
        let packet_id = reader.read_var_int()?;

        let buffer = buffer
            .split_off(packet_length.position + packet_id.position)
            .freeze();

        // info!(target: "packets", "RECV [{}] [{:03X}] {:02x?}", packet_length.value, packet_id.value, &buffer[..]);

        Ok(Packet {
            id: packet_id,
            data: buffer,
        })
    }
}
