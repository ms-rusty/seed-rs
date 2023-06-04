use num_derive::FromPrimitive;
use seed_network_server_common::VarInt;

// https://wiki.vg/Protocol#Status
#[derive(FromPrimitive, Clone, Copy)]
pub enum ServerStatusPackets {
    StatusResponse = 0x00,
    PingResponse = 0x01,
}

impl PartialEq<ServerStatusPackets> for VarInt {
    fn eq(&self, packet: &ServerStatusPackets) -> bool {
        self.value == *packet as i32
    }
}

impl Into<VarInt> for ServerStatusPackets {
    fn into(self) -> VarInt {
        VarInt::from(self as i32)
    }
}
