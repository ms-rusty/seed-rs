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
