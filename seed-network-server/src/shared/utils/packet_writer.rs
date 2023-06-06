use bytes::{BufMut, Bytes, BytesMut};
use seed_network_server_common::VarInt;

use crate::shared::Packet;

#[derive(Debug)]
pub struct PacketWriter {
    packet_id: VarInt,
    data: BytesMut,
}

#[allow(dead_code)]
impl PacketWriter {
    pub fn new(packet_id: VarInt) -> Self {
        Self {
            packet_id,
            data: BytesMut::new(),
        }
    }

    pub fn get_data(self) -> Bytes {
        self.data.freeze()
    }

    pub fn write_bytes(&mut self, value: &[u8]) {
        self.data.put(value);
    }

    pub fn write_i8(&mut self, value: i8) {
        self.data.put_i8(value);
    }

    pub fn write_u8(&mut self, value: u8) {
        self.data.put_u8(value);
    }

    pub fn write_i16(&mut self, value: i16) {
        self.data.put_i16(value);
    }

    pub fn write_u16(&mut self, value: u16) {
        self.data.put_u16(value);
    }

    pub fn write_i16_le(&mut self, value: i16) {
        self.data.put_i16_le(value);
    }

    pub fn write_u16_le(&mut self, value: u16) {
        self.data.put_u16_le(value);
    }

    pub fn write_i32(&mut self, value: i32) {
        self.data.put_i32(value);
    }

    pub fn write_u32(&mut self, value: u32) {
        self.data.put_u32(value);
    }

    pub fn write_i32_le(&mut self, value: i32) {
        self.data.put_i32_le(value);
    }

    pub fn write_u32_le(&mut self, value: u32) {
        self.data.put_u32_le(value);
    }

    pub fn write_i64(&mut self, value: i64) {
        self.data.put_i64(value);
    }

    pub fn write_u64(&mut self, value: u64) {
        self.data.put_u64(value);
    }

    pub fn write_i64_le(&mut self, value: i64) {
        self.data.put_i64_le(value);
    }

    pub fn write_u64_le(&mut self, value: u64) {
        self.data.put_u64_le(value);
    }

    pub fn write_i128(&mut self, value: i128) {
        self.data.put_i128(value);
    }

    pub fn write_u128(&mut self, value: u128) {
        self.data.put_u128(value);
    }

    pub fn write_i128_le(&mut self, value: i128) {
        self.data.put_i128_le(value);
    }

    pub fn write_u128_le(&mut self, value: u128) {
        self.data.put_u128_le(value);
    }

    pub fn write_bool(&mut self, value: bool) {
        self.data.put_u8(value as u8);
    }

    pub fn write_var_int(&mut self, value: i32) -> usize {
        let mut value = value as u32;
        let mut position = 0;

        loop {
            let mut byte = (value & 0x7F) as u8;
            value >>= 7;

            if value != 0 {
                byte |= 0x80;
            }

            self.write_u8(byte);

            position += 1;

            if value == 0 {
                break;
            }
        }

        position
    }

    pub fn write_var_long(&mut self, mut value: i64) -> usize {
        let mut position = 0;

        loop {
            let mut byte = (value & 0x7F) as u8;
            value >>= 7;

            if value != 0 {
                byte |= 0x80;
            }

            self.write_u8(byte);

            position += 1;

            if value == 0 {
                break;
            }
        }

        position
    }

    pub fn write_str(&mut self, value: &str) {
        self.write_var_int(value.len() as i32);
        self.write_bytes(value.as_bytes());
    }
}

impl Into<Packet> for PacketWriter {
    fn into(self) -> Packet {
        Packet::new(self.packet_id, self.get_data())
    }
}
