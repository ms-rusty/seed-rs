use num_derive::FromPrimitive;
use seed_network_server_common::VarInt;

// https://wiki.vg/Protocol#Play
#[derive(FromPrimitive, Clone, Copy)]
pub enum ServerPlayPackets {}

impl PartialEq<ServerPlayPackets> for VarInt {
    fn eq(&self, packet: &ServerPlayPackets) -> bool {
        self.value == *packet as i32
    }
}

impl Into<VarInt> for ServerPlayPackets {
    fn into(self) -> VarInt {
        VarInt::from(self as i32)
    }
}
