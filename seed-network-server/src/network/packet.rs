use bytes::{Buf, Bytes, BytesMut};
use std::io::Cursor;

use super::VarInt;

// https://wiki.vg/Protocol#Packet_format
pub struct Packet {
    pub length: i32,
    pub command: i32,
    data: Bytes,
}

impl Packet {
    pub fn new(mut data: BytesMut) -> Self {
        let mut reader = PacketReader::new(&data);
        let packet_length_var_int = reader.read_var_int();
        let packet_command_var_int = reader.read_var_int();

        let data: Bytes = data
            .split_off(packet_length_var_int.length + packet_command_var_int.length)
            .freeze();

        Self {
            length: packet_length_var_int.value,
            command: packet_command_var_int.value,
            data,
        }
    }
}

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

    pub fn read_u8(&mut self) -> u8 {
        self.cursor.get_u8()
    }

    pub fn read_u16(&mut self) -> u16 {
        self.cursor.get_u16()
    }

    pub fn read_var_int(&mut self) -> VarInt {
        let mut length = 0;
        let mut value = 0;

        loop {
            let byte = self.cursor.get_u8();
            let byte_value = i32::from(byte & 0b0111_1111);
            value |= byte_value.overflowing_shl(7 * length).0;

            length += 1;

            if length > 5 {
                panic!("VarInt too long (max length: 5)");
            }
            if byte & 0b1000_0000 == 0 {
                break;
            }
        }

        VarInt::new(value, length as usize)
    }

    pub fn read_str(&mut self) -> &'packet str {
        let var_int = self.read_var_int();

        let start = self.cursor.position() as usize;
        let end = start + var_int.value as usize;
        self.cursor.set_position(end as u64);
        let value = &self.cursor.get_ref()[start..end];

        std::str::from_utf8(value).unwrap()
    }
}
