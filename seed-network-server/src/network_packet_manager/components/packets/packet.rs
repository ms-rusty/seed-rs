use bevy::prelude::Component;
use bytes::{Bytes, BytesMut};
use seed_network_server_common::VarInt;

use super::packet_reader::PacketReader;

// https://wiki.vg/Protocol#Packet_format
#[derive(Component, Debug)]
pub struct Packet {
    pub id: VarInt,
    pub data: Bytes,
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
