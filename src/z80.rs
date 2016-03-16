
// NOTE:IMPLEMENTATION: Even though it's an 8 bit micro-processor, I don't
// really want to worry about roll-over operations in rust
// TODO: Actually... Change back to 8 bit and use the language checks; they are
// actually really useful. So change back to u8 eventually.
struct Z80 {
    clock_m: f32,
    clock_t: f32,
    reg_a:   i16,
    reg_b:   i16,
    reg_c:   i16,
    reg_d:   i16,
    reg_e:   i16,
    reg_h:   i16,
    reg_l:   i16,
    reg_f:   i16,
    reg_pc:  i16,
    reg_sp:  i16,
    reg_m:   i16,
    reg_t:   i16,
}

impl Z80 {
    pub fn new() -> Z80 {
        Z80 {
            clock_m: 0.0,
            clock_t: 0.0,
            reg_a:   0,
            reg_b:   0,
            reg_c:   0,
            reg_d:   0,
            reg_e:   0,
            reg_h:   0,
            reg_l:   0,
            reg_f:   0,
            reg_pc:  0,
            reg_sp:  0,
            reg_m:   0,
            reg_t:   0,
        }
    }

    /// (ADD A, E): Add reg_e to reg_a, result in reg_a
    pub fn addr_e(&mut self) {
        self.reg_a += self.reg_e; // add operation
        self.reg_f = 0; // clear flags
        if (self.reg_a & 0xFF) == 0 { self.reg_f |= 0x80; } // check for zero
        if self.reg_a > 0xFF { self.reg_f |= 0x10; } // check for carry
        self.reg_a &= 0xFF; // mask to 8-bit
        self.reg_m = 1; self.reg_t = 4;// 1 M-time taken
    }

    /// (CP A, B): Compare reg_b to reg_a, setting flags
    pub fn cpr_b(&mut self) {
        let mut tmp = self.reg_a; // temp copy of reg_a
        tmp -= self.reg_b; // subtract reg_b
        self.reg_f |= 0x40; // set subtract flag
        if (tmp & 255) == 0 { self.reg_f |= 0x80; }// check for zero
        if tmp < 0 { self.reg_f |= 0x10; } // check for underflow
        self.reg_m = 1; self.reg_t = 4;// 1 M-time taken
    }

    /// (NOP): No-operation
    pub fn nop(&mut self) {
        self.reg_m = 1; self.reg_t = 4;// 1 M-time taken
    }
}
