use thiserror::Error;

use super::packet_error::PacketError;

#[derive(Debug, Error)]
pub enum PacketWriterError {}

impl From<PacketWriterError> for PacketError {
    fn from(err: PacketWriterError) -> Self {
        Self::WriteError(err)
    }
}
