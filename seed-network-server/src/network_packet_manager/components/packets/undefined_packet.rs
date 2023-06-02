use bevy::prelude::{Component, Deref, DerefMut};
use seed_network_server_common::VarInt;

#[derive(Component, Deref, DerefMut)]
pub struct UndefinedPacket {
    pub id: VarInt,
}

impl UndefinedPacket {
    pub fn new(id: VarInt) -> Self {
        Self { id }
    }
}
