use num_derive::FromPrimitive;
use seed_network_server_common::VarInt;

// https://wiki.vg/Protocol#Handshaking
#[derive(FromPrimitive, Clone, Copy)]
pub enum ClientHandshakingPackets {
    Handshake = 0x00,
}

// https://wiki.vg/Protocol#Status
#[derive(FromPrimitive, Clone, Copy)]
pub enum ClientStatusPackets {
    StatusRequest = 0x00,
    PingRequest = 0x01,
}

// https://wiki.vg/Protocol#Login
#[derive(FromPrimitive, Clone, Copy)]
pub enum ClientLoginPackets {
    LoginStart = 0x00,
    EncryptionResponse = 0x01,
    LoginPluginResponse = 0x02,
}

// https://wiki.vg/Protocol#Play
#[derive(FromPrimitive, Clone, Copy)]
pub enum ServerPlayPackets {
    //
}

impl PartialEq<ClientHandshakingPackets> for VarInt {
    fn eq(&self, packet: &ClientHandshakingPackets) -> bool {
        self.value == *packet as i32
    }
}

impl PartialEq<ClientStatusPackets> for VarInt {
    fn eq(&self, packet: &ClientStatusPackets) -> bool {
        self.value == *packet as i32
    }
}

impl PartialEq<ClientLoginPackets> for VarInt {
    fn eq(&self, packet: &ClientLoginPackets) -> bool {
        self.value == *packet as i32
    }
}

impl PartialEq<ServerPlayPackets> for VarInt {
    fn eq(&self, packet: &ServerPlayPackets) -> bool {
        self.value == *packet as i32
    }
}
