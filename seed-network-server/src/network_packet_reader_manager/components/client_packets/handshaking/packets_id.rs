use num_derive::FromPrimitive;
use seed_network_server_common::VarInt;

// https://wiki.vg/Protocol#Handshaking
#[derive(FromPrimitive, Clone, Copy)]
pub enum ClientHandshakingPackets {
    Handshake = 0x00,
}

impl PartialEq<ClientHandshakingPackets> for VarInt {
    fn eq(&self, packet: &ClientHandshakingPackets) -> bool {
        self.value == *packet as i32
    }
}
