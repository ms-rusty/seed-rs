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

impl From<i32> for VarInt {
    fn from(mut value: i32) -> Self {
        let mut position = 0;

        loop {
            // let mut byte = (value & 0x7F) as u8;
            value >>= 7;

            // if value != 0 {
            // byte |= 0x80;
            // }

            position += 1;

            if value == 0 {
                break;
            }
        }

        Self { value, position }
    }
}
