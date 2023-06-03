use thiserror::Error;

use super::packet_error::PacketError;

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
