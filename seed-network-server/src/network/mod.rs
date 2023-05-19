pub use client_packets::{ClientHandshakingPackets, ClientLoginPackets, ClientStatusPackets};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
pub use packet::{Packet, PacketReader};
pub use server_packets::{ServerLoginPackets, ServerPlayPackets, ServerStatusPackets};

mod client_packets;
mod packet;
mod server_packets;

pub struct VarInt {
    pub value: i32,
    pub length: usize,
}

impl VarInt {
    pub fn new(value: i32, length: usize) -> Self {
        Self { value, length }
    }
}

pub struct PacketClientHandshaking<'packet> {
    pub protocol_version: VarInt,
    pub server_address: &'packet str,
    pub server_port: u16,
    pub next_state: HandshakeState,
}

impl<'a> TryFrom<&'a Packet> for PacketClientHandshaking<'a> {
    type Error = std::io::Error;

    fn try_from(packet: &'a Packet) -> Result<Self, std::io::Error> {
        if packet.command != ClientHandshakingPackets::Handshake as i32 {
            return Err(std::io::ErrorKind::InvalidData.into());
        }

        let mut reader = PacketReader::from(packet);
        let protocol_version = reader.read_var_int();
        let server_address = reader.read_str();
        let server_port = reader.read_u16();
        let next_state = HandshakeState::new(reader.read_var_int()).unwrap();
        Ok(Self {
            protocol_version,
            server_address,
            server_port,
            next_state,
        })
    }
}

#[derive(FromPrimitive, PartialEq, Eq, Clone)]
pub enum HandshakeState {
    Status = 1,
    Login = 2,
}

impl HandshakeState {
    pub fn new(var_int: VarInt) -> Option<Self> {
        FromPrimitive::from_i32(var_int.value)
    }
}
