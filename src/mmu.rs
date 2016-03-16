
struct MMU {
    foo: f32
}

impl MMU {
    pub fn rb(addr: u16) {
        // read 8-bit byte from memory
        unimplemented!();
    }
    pub fn rw(addr: u16) {
        // read 16-bit word from memory
        unimplemented!();
    }
    pub fn wb(addr: u16, val: u8) {
        // write 8-bit byte from memory
        unimplemented!();
    }
    pub fn ww(addr: u16, val: u16) {
        // write 16-bit word from memory
        unimplemented!();
    }
}
