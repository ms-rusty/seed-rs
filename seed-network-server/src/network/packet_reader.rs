use bytes::Buf;
use std::io::Cursor;
use thiserror::Error;

use super::{packet::PacketError, Packet, VarInt, VarLong};

pub struct PacketReader<'packet> {
    pub cursor: Cursor<&'packet [u8]>,
}

impl<'packet> From<&'packet Packet> for PacketReader<'packet> {
    fn from(packet: &'packet Packet) -> Self {
        Self {
            cursor: Cursor::new(&packet.data[..]),
        }
    }
}

impl<'packet> PacketReader<'packet> {
    pub fn new(data: &'packet [u8]) -> Self {
        Self {
            cursor: Cursor::new(data),
        }
    }

    pub fn read_u8(&mut self) -> Result<u8, PacketReaderError> {
        if self.cursor.remaining() < 1 {
            Err(PacketReaderError::NotEnoughRemainingData)
        } else {
            Ok(self.cursor.get_u8())
        }
    }

    pub fn read_i8(&mut self) -> Result<i8, PacketReaderError> {
        if self.cursor.remaining() < 1 {
            Err(PacketReaderError::NotEnoughRemainingData)
        } else {
            Ok(self.cursor.get_i8())
        }
    }

    pub fn read_u16(&mut self) -> Result<u16, PacketReaderError> {
        if self.cursor.remaining() < 2 {
            Err(PacketReaderError::NotEnoughRemainingData)
        } else {
            Ok(self.cursor.get_u16())
        }
    }

    pub fn read_i16(&mut self) -> Result<i16, PacketReaderError> {
        if self.cursor.remaining() < 2 {
            Err(PacketReaderError::NotEnoughRemainingData)
        } else {
            Ok(self.cursor.get_i16())
        }
    }

    pub fn read_u16_le(&mut self) -> Result<u16, PacketReaderError> {
        if self.cursor.remaining() < 2 {
            Err(PacketReaderError::NotEnoughRemainingData)
        } else {
            Ok(self.cursor.get_u16_le())
        }
    }

    pub fn read_i16_le(&mut self) -> Result<i16, PacketReaderError> {
        if self.cursor.remaining() < 2 {
            Err(PacketReaderError::NotEnoughRemainingData)
        } else {
            Ok(self.cursor.get_i16_le())
        }
    }

    pub fn read_u32(&mut self) -> Result<u32, PacketReaderError> {
        if self.cursor.remaining() < 4 {
            Err(PacketReaderError::NotEnoughRemainingData)
        } else {
            Ok(self.cursor.get_u32())
        }
    }

    pub fn read_i32(&mut self) -> Result<i32, PacketReaderError> {
        if self.cursor.remaining() < 4 {
            Err(PacketReaderError::NotEnoughRemainingData)
        } else {
            Ok(self.cursor.get_i32())
        }
    }

    pub fn read_u32_le(&mut self) -> Result<u32, PacketReaderError> {
        if self.cursor.remaining() < 4 {
            Err(PacketReaderError::NotEnoughRemainingData)
        } else {
            Ok(self.cursor.get_u32_le())
        }
    }

    pub fn read_i32_le(&mut self) -> Result<i32, PacketReaderError> {
        if self.cursor.remaining() < 4 {
            Err(PacketReaderError::NotEnoughRemainingData)
        } else {
            Ok(self.cursor.get_i32_le())
        }
    }

    pub fn read_u64(&mut self) -> Result<u64, PacketReaderError> {
        if self.cursor.remaining() < 8 {
            Err(PacketReaderError::NotEnoughRemainingData)
        } else {
            Ok(self.cursor.get_u64())
        }
    }

    pub fn read_i64(&mut self) -> Result<i64, PacketReaderError> {
        if self.cursor.remaining() < 8 {
            Err(PacketReaderError::NotEnoughRemainingData)
        } else {
            Ok(self.cursor.get_i64())
        }
    }

    pub fn read_u64_le(&mut self) -> Result<u64, PacketReaderError> {
        if self.cursor.remaining() < 8 {
            Err(PacketReaderError::NotEnoughRemainingData)
        } else {
            Ok(self.cursor.get_u64_le())
        }
    }

    pub fn read_i64_le(&mut self) -> Result<i64, PacketReaderError> {
        if self.cursor.remaining() < 8 {
            Err(PacketReaderError::NotEnoughRemainingData)
        } else {
            Ok(self.cursor.get_i64_le())
        }
    }

    pub fn read_u128(&mut self) -> Result<u128, PacketReaderError> {
        if self.cursor.remaining() < 16 {
            Err(PacketReaderError::NotEnoughRemainingData)
        } else {
            Ok(self.cursor.get_u128())
        }
    }

    pub fn read_i128(&mut self) -> Result<i128, PacketReaderError> {
        if self.cursor.remaining() < 16 {
            Err(PacketReaderError::NotEnoughRemainingData)
        } else {
            Ok(self.cursor.get_i128())
        }
    }

    pub fn read_u128_le(&mut self) -> Result<u128, PacketReaderError> {
        if self.cursor.remaining() < 16 {
            Err(PacketReaderError::NotEnoughRemainingData)
        } else {
            Ok(self.cursor.get_u128_le())
        }
    }

    pub fn read_i128_le(&mut self) -> Result<i128, PacketReaderError> {
        if self.cursor.remaining() < 16 {
            Err(PacketReaderError::NotEnoughRemainingData)
        } else {
            Ok(self.cursor.get_i128_le())
        }
    }

    pub fn read_f32(&mut self) -> Result<f32, PacketReaderError> {
        if self.cursor.remaining() < 8 {
            Err(PacketReaderError::NotEnoughRemainingData)
        } else {
            Ok(self.cursor.get_f32())
        }
    }

    pub fn read_f32_le(&mut self) -> Result<f32, PacketReaderError> {
        if self.cursor.remaining() < 8 {
            Err(PacketReaderError::NotEnoughRemainingData)
        } else {
            Ok(self.cursor.get_f32_le())
        }
    }

    pub fn read_f64(&mut self) -> Result<f64, PacketReaderError> {
        if self.cursor.remaining() < 8 {
            Err(PacketReaderError::NotEnoughRemainingData)
        } else {
            Ok(self.cursor.get_f64())
        }
    }

    pub fn read_f64_le(&mut self) -> Result<f64, PacketReaderError> {
        if self.cursor.remaining() < 8 {
            Err(PacketReaderError::NotEnoughRemainingData)
        } else {
            Ok(self.cursor.get_f64_le())
        }
    }

    pub fn read_var_int(&mut self) -> Result<VarInt, PacketReaderError> {
        if self.cursor.remaining() < 1 {
            return Err(PacketReaderError::NotEnoughRemainingData);
        }

        let mut value = 0;
        let mut position = 0;

        loop {
            let byte = self.cursor.get_u8();
            let byte_value = i32::from(byte & 0x7F);
            value |= byte_value.overflowing_shl(7 * position).0;

            position += 1;
            if position > 5 {
                return Err(PacketReaderError::VarIntTooBig);
            }

            if byte & 0x80 == 0 {
                break;
            }
        }

        Ok(VarInt::new(value, position as usize))
    }

    pub fn read_var_long(&mut self) -> Result<VarLong, PacketReaderError> {
        if self.cursor.remaining() < 1 {
            return Err(PacketReaderError::NotEnoughRemainingData);
        }

        let mut value = 0;
        let mut position = 0;

        loop {
            let byte = self.cursor.get_u8();
            let byte_value = i64::from(byte & 0x7F);
            value |= byte_value.overflowing_shl(7 * position).0;

            position += 1;
            if position > 10 {
                return Err(PacketReaderError::VarLongTooBig);
            }

            if byte & 0x80 == 0 {
                break;
            }
        }

        Ok(VarLong::new(value, position as usize))
    }

    pub fn read_str(&mut self) -> Result<&'packet str, PacketReaderError> {
        let var_int = self.read_var_int()?;

        let start_position = self.cursor.position() as usize;
        let end_position = start_position + var_int.value as usize;
        self.cursor.set_position(end_position as u64);

        let slice_reference = &self.cursor.get_ref()[start_position..end_position];

        match std::str::from_utf8(slice_reference) {
            Ok(reference) => Ok(reference),
            Err(_) => Err(PacketReaderError::NotEnoughRemainingData),
        }
    }
}

#[derive(Debug, Error)]
pub enum PacketReaderError {
    #[error("Not enough remaining data.")]
    NotEnoughRemainingData,

    #[error("VarInt is too big.")]
    VarIntTooBig,

    #[error("VarLong is too big.")]
    VarLongTooBig,
}
