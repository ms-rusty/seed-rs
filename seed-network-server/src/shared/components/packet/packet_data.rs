use bevy::prelude::{Component, Deref, DerefMut};
use bytes::Bytes;

#[derive(Component, Debug)]
pub struct PacketData {
    pub data: Bytes,
}

impl PacketData {
    pub fn new(data: Bytes) -> Self {
        Self { data }
    }
}
