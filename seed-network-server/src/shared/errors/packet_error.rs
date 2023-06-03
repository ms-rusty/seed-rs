use thiserror::Error;

use super::{packet_reader_error::PacketReaderError, packet_writer_error::PacketWriterError};

#[derive(Debug, Error)]
pub enum PacketError {
    #[error("Invalid packet.")]
    InvalidPacket,

    #[error("Read error. {0}")]
    ReadError(PacketReaderError),

    #[error("Write error. {0}")]
    WriteError(PacketWriterError),
}
