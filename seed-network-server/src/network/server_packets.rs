use num_derive::FromPrimitive;

// https://wiki.vg/Protocol#Handshaking
#[derive(FromPrimitive)]
pub enum ServerHandshakingPackets {
    // None
}

// https://wiki.vg/Protocol#Status
#[derive(FromPrimitive)]
pub enum ServerStatusPackets {
    StatusResponse = 0x00,
    PingResponse = 0x01,
}

// https://wiki.vg/Protocol#Login
#[derive(FromPrimitive)]
pub enum ServerLoginPackets {
    Disconnect = 0x00,
    EncryptionRequest = 0x01,
    LoginSuccess = 0x02,
    SetCompression = 0x03,
    LoginPluginRequest = 0x04,
}

// https://wiki.vg/Protocol#Play
#[derive(FromPrimitive)]
pub enum ServerPlayPackets {}
