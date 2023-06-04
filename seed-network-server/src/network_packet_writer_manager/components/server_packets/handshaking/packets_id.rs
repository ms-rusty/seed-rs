use num_derive::FromPrimitive;
use seed_network_server_common::VarInt;

// https://wiki.vg/Protocol#Handshaking
#[derive(FromPrimitive, Clone, Copy)]
pub enum ServerHandshakingPackets {
    // None
}

impl PartialEq<ServerHandshakingPackets> for VarInt {
    fn eq(&self, packet: &ServerHandshakingPackets) -> bool {
        self.value == *packet as i32
    }
}

impl Into<VarInt> for ServerHandshakingPackets {
    fn into(self) -> VarInt {
        VarInt::from(self as i32)
    }
}
