pub trait IntoBitHash {
    fn get_bit(self) -> bool;
}

impl IntoBitHash for (u32, bool) {
    fn get_bit(self) -> bool {
        self.1
    }
}

impl IntoBitHash for (u32, u8) {
    fn get_bit(self) -> bool {
        ((self.0 >> self.1) & 1) != 0
    }
}