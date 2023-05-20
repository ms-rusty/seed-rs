pub use client_packets::{ClientHandshakingPackets, ClientLoginPackets, ClientStatusPackets};
pub use handshaking::{HandshakeState, PacketClientHandshake};
pub use packet::Packet;
pub use server_packets::{ServerLoginPackets, ServerPlayPackets, ServerStatusPackets};
pub use status::PacketClientStatusRequest;

mod client_packets;
mod handshaking;
mod packet;
mod packet_reader;
mod server_packets;
mod status;

#[derive(Debug)]
pub struct VarInt {
    pub value: i32,
    pub position: usize,
}

impl VarInt {
    pub fn new(value: i32, position: usize) -> Self {
        Self { value, position }
    }
}

#[derive(Debug)]
pub struct VarLong {
    pub value: i64,
    pub position: usize,
}

impl VarLong {
    pub fn new(value: i64, position: usize) -> Self {
        Self { value, position }
    }
}
