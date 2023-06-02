use bevy::prelude::{Bundle, Component, Deref, DerefMut};
use bytes::{Bytes, BytesMut};
use seed_network_server_common::VarInt;

use super::packet_reader::PacketReader;

// https://wiki.vg/Protocol#Packet_format
#[derive(Bundle, Debug)]
pub struct Packet {
    pub id: PacketId,
    pub data: PacketData,
}

impl Packet {
    pub fn new(id: VarInt, data: Bytes) -> Self {
        Self {
            id: PacketId::new(id),
            data: PacketData::new(data),
        }
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
            id: PacketId::new(packet_id),
            data: PacketData::new(buffer),
        })
    }
}

#[derive(Component, Debug, Deref, DerefMut)]
pub struct PacketId {
    pub id: VarInt,
}

impl PacketId {
    pub fn new(id: VarInt) -> Self {
        Self { id }
    }
}

#[derive(Component, Debug, Deref, DerefMut)]
pub struct PacketData {
    pub data: Bytes,
}

impl PacketData {
    pub fn new(data: Bytes) -> Self {
        Self { data }
    }
}
