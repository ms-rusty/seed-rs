use num_derive::FromPrimitive;
use seed_network_server_common::VarInt;

// https://wiki.vg/Protocol#Status
#[derive(FromPrimitive, Clone, Copy)]
pub enum ClientStatusPackets {
    StatusRequest = 0x00,
    PingRequest = 0x01,
}

impl PartialEq<ClientStatusPackets> for VarInt {
    fn eq(&self, packet: &ClientStatusPackets) -> bool {
        self.value == *packet as i32
    }
}
