#[derive(Debug)]
pub struct VarLong {
    pub value: i64,
    pub position: usize,
}

impl VarLong {
    pub fn new(value: i64, position: usize) -> Self {
        Self { value, position }
    }
}
