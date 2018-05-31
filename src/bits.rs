
pub trait BitOps {
    fn is_bit_set(&self, n: u8) -> bool;
    fn set_bit(&self, n: u8) -> Self;
}

impl BitOps for u8 {
    fn is_bit_set(&self, n: u8) -> bool {
        *self & (1 << n) != 0
    }

    fn set_bit(&self, n: u8) -> u8 {
        *self | (1 << n)
    }
}