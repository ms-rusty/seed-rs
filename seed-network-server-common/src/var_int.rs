#[derive(Debug, Clone, Copy)]
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
    fn from(value: i32) -> Self {
        let mut var_int_value = value;
        let mut position = 0;

        loop {
            // let mut byte = (var_int_value & 0x7F) as u8;
            var_int_value >>= 7;

            // if var_int_value != 0 {
            // byte |= 0x80;
            // }

            position += 1;

            if var_int_value == 0 {
                break;
            }
        }

        Self { value, position }
    }
}
