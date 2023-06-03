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
