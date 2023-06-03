use bevy::prelude::Bundle;
use bytes::Bytes;
use seed_network_server_common::VarInt;

use super::{PacketData, PacketId};

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
