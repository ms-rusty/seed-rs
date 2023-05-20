use num_derive::FromPrimitive;
use seed_network_common::VarInt;

// https://wiki.vg/Protocol#Handshaking
#[derive(FromPrimitive, Clone, Copy)]
pub enum ServerHandshakingPackets {
    // None
}

// https://wiki.vg/Protocol#Status
#[derive(FromPrimitive, Clone, Copy)]
pub enum ServerStatusPackets {
    StatusResponse = 0x00,
    PingResponse = 0x01,
}

// https://wiki.vg/Protocol#Login
#[derive(FromPrimitive, Clone, Copy)]
pub enum ServerLoginPackets {
    Disconnect = 0x00,
    EncryptionRequest = 0x01,
    LoginSuccess = 0x02,
    SetCompression = 0x03,
    LoginPluginRequest = 0x04,
}

// https://wiki.vg/Protocol#Play
#[derive(FromPrimitive, Clone, Copy)]
pub enum ServerPlayPackets {}

// VarInt Partial Eq

impl PartialEq<ServerLoginPackets> for VarInt {
    fn eq(&self, packet: &ServerLoginPackets) -> bool {
        self.value == *packet as i32
    }
}

impl PartialEq<ServerHandshakingPackets> for VarInt {
    fn eq(&self, packet: &ServerHandshakingPackets) -> bool {
        self.value == *packet as i32
    }
}

impl PartialEq<ServerStatusPackets> for VarInt {
    fn eq(&self, packet: &ServerStatusPackets) -> bool {
        self.value == *packet as i32
    }
}

impl PartialEq<ServerPlayPackets> for VarInt {
    fn eq(&self, packet: &ServerPlayPackets) -> bool {
        self.value == *packet as i32
    }
}
