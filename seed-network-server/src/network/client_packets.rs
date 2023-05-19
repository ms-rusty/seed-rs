use num_derive::FromPrimitive;

// https://wiki.vg/Protocol#Handshaking
#[derive(FromPrimitive)]
pub enum ClientHandshakingPackets {
    Handshake = 0x00,
}

// https://wiki.vg/Protocol#Status
#[derive(FromPrimitive)]
pub enum ClientStatusPackets {
    StatusRequest = 0x00,
    PingRequest = 0x01,
}

// https://wiki.vg/Protocol#Login
#[derive(FromPrimitive)]
pub enum ClientLoginPackets {
    LoginStart = 0x00,
    EncryptionResponse = 0x01,
    LoginPluginResponse = 0x02,
}

// https://wiki.vg/Protocol#Play
#[derive(FromPrimitive)]
pub enum ServerPlayPackets {}
