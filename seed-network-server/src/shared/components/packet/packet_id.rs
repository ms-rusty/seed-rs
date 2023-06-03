use bevy::prelude::{Component, Deref, DerefMut};
use seed_network_server_common::VarInt;

#[derive(Component, Debug, Deref, DerefMut)]
pub struct PacketId {
    pub id: VarInt,
}

impl PacketId {
    pub fn new(id: VarInt) -> Self {
        Self { id }
    }
}
