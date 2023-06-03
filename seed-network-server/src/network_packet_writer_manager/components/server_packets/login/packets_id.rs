use num_derive::FromPrimitive;
use seed_network_server_common::VarInt;

// https://wiki.vg/Protocol#Login
#[derive(FromPrimitive, Clone, Copy)]
pub enum ServerLoginPackets {
    Disconnect = 0x00,
    EncryptionRequest = 0x01,
    LoginSuccess = 0x02,
    SetCompression = 0x03,
    LoginPluginRequest = 0x04,
}

impl PartialEq<ServerLoginPackets> for VarInt {
    fn eq(&self, packet: &ServerLoginPackets) -> bool {
        self.value == *packet as i32
    }
}
