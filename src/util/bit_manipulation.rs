pub trait BitManipulation {
    fn set_bits(&mut self, shift: Self, length: Self, val: Self);
    fn set_bit(&mut self, shift: Self, enable: bool);

    fn get_bits(&self, shift: Self, length: Self) -> Self;
}

// Implement the trait for u64
impl BitManipulation for u64 {
    fn set_bits(&mut self, shift: u64, length: u64, val: u64) {
        let mut mask = (1 << length) - 1;
        mask <<= shift;

        *self &= !mask; // Clear the bits in the specified range
        *self |= (val << shift) & mask; // Set the bits to the new value
    }

    fn set_bit(&mut self, shift: u64, enable: bool) {
        self.set_bits(shift, 1, enable as u64);
    }

    fn get_bits(&self, shift: u64, length: u64) -> u64 {
        let mask = (1 << length) - 1;
        (*self >> shift) & mask
    }
}

impl BitManipulation for u8 {
    fn set_bits(&mut self, shift: u8, length: u8, val: u8) {
        let mut mask = (1 << length) - 1;
        mask <<= shift;

        *self &= !mask; // Clear the bits in the specified range
        *self |= (val << shift) & mask; // Set the bits to the new value
    }

    fn set_bit(&mut self, shift: u8, enable: bool) {
        self.set_bits(shift, 1, enable as u8);
    }

    fn get_bits(&self, shift: u8, length: u8) -> u8 {
        let mask = (1 << length) - 1;
        (*self >> shift) & mask
    }
}
