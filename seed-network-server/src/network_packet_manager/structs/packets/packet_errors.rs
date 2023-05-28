use thiserror::Error;

#[derive(Debug, Error)]
pub enum PacketError {
    #[error("Invalid packet.")]
    InvalidPacket,

    #[error("Read error. {0}")]
    ReadError(PacketReaderError),

    #[error("Write error. {0}")]
    WriteError(PacketWriterError),
}

#[derive(Debug, Error)]
pub enum PacketReaderError {
    #[error("Not enough remaining data.")]
    NotEnoughRemainingData,

    #[error("Invalid next state.")]
    InvalidNextState,

    #[error("VarInt is too big.")]
    VarIntTooBig,

    #[error("VarLong is too big.")]
    VarLongTooBig,
}

impl From<PacketReaderError> for PacketError {
    fn from(err: PacketReaderError) -> Self {
        Self::ReadError(err)
    }
}

#[derive(Debug, Error)]
pub enum PacketWriterError {}

impl From<PacketWriterError> for PacketError {
    fn from(err: PacketWriterError) -> Self {
        Self::WriteError(err)
    }
}
