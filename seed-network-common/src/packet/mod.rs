use bytes::Bytes;

pub struct PacketTypeId(u32);

pub struct Packet {
    pub id: PacketId,
    pub data: Bytes,
}

pub struct PacketId {
    pub id: PacketTypeId,
    pub bound: PacketBound,
}

pub enum PacketBound {
    Client(ClientPacketStage),
    Server(ServerPacketStage),
}

pub enum ClientPacketStage {
    Handshaking,
    Status,
    Login,
    Play,
}

pub enum ServerPacketStage {
    Status,
    Login,
    Play,
}

pub enum PacketError {
    InvalidPacket,
}

fn test() {
    let a = PacketBound::Client(ClientPacketStage::Handshaking);
    match a {
        PacketBound::Client(stage) => match stage {
            ClientPacketStage::Handshaking => todo!(),
            ClientPacketStage::Status => todo!(),
            ClientPacketStage::Login => todo!(),
            ClientPacketStage::Play => todo!(),
        },
        PacketBound::Server(stage) => match stage {
            ServerPacketStage::Status => todo!(),
            ServerPacketStage::Login => todo!(),
            ServerPacketStage::Play => todo!(),
        },
    }
}

pub struct RequestStatusPacket;

impl TryFrom<Packet> for RequestStatusPacket {
    type Error = PacketError;

    fn try_from(value: Packet) -> Result<Self, Self::Error> {}
}

impl From<RequestStatusPacket> for Packet {
    fn from(value: RequestStatusPacket) -> Self {}
}
