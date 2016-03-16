
pub struct MMU {
    foo: f32
}

impl MMU {
    pub fn rb(&self, addr: u16) -> u8 {
        // read 8-bit byte from memory
        unimplemented!();
    }
    pub fn rw(&self, addr: u16) -> u16 {
        // read 16-bit word from memory
        unimplemented!();
    }
    pub fn wb(&self, addr: u16, val: u8) {
        // write 8-bit byte from memory
        unimplemented!();
    }
    pub fn ww(&self, addr: u16, val: u16) {
        // write 16-bit word from memory
        unimplemented!();
    }
}
