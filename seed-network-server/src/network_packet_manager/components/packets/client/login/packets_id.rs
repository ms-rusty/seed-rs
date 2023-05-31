use num_derive::FromPrimitive;
use seed_network_server_common::VarInt;

// https://wiki.vg/Protocol#Login
#[derive(FromPrimitive, Clone, Copy)]
pub enum ClientLoginPackets {
    LoginStart = 0x00,
    EncryptionResponse = 0x01,
    LoginPluginResponse = 0x02,
}

impl PartialEq<ClientLoginPackets> for VarInt {
    fn eq(&self, packet: &ClientLoginPackets) -> bool {
        self.value == *packet as i32
    }
}
