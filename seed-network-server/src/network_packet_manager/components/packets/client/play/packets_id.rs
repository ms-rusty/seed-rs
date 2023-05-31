use num_derive::FromPrimitive;
use seed_network_server_common::VarInt;

// https://wiki.vg/Protocol#Play
#[derive(FromPrimitive, Clone, Copy)]
pub enum ClientPlayPackets {
    //
}

impl PartialEq<ClientPlayPackets> for VarInt {
    fn eq(&self, packet: &ClientPlayPackets) -> bool {
        self.value == *packet as i32
    }
}
