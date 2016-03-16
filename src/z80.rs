use mmu::MMU;

// NOTE:IMPLEMENTATION: Even though it's an 8 bit micro-processor, I don't
// really want to worry about roll-over operations in rust
// TODO: Actually... Change back to 8 bit and use the language checks; they are
// actually really useful. So change back to u8 eventually.
struct Z80 {
    clock_m: f32,
    clock_t: f32,
    reg_a:   u8,
    reg_b:   u8,
    reg_c:   u8,
    reg_d:   u8,
    reg_e:   u8,
    reg_h:   u8,
    reg_l:   u8,
    reg_f:   u8,
    reg_pc:  u16,
    reg_sp:  u16,
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

    // memory handling instructions

    /// (PUSH BC): Push reg_b and reg_c onto the stack
    pub fn push_bc(&mut self, mmu: MMU) {
        self.reg_sp -= 1; // decrement stack pointer
        mmu.wb(self.reg_sp, self.reg_b); // Write reg_b
        self.reg_sp -= 1; // decrement stack pointer
        mmu.wb(self.reg_sp, self.reg_c); // Write reg_c
        self.reg_m = 3; self.reg_t = 12; // 3 M-times taken
    }

    /// (POP HL): Pop reg_h and reg_l off of the stack
    pub fn pop_hl(&mut self, mmu: MMU) {
        self.reg_l = mmu.rb(self.reg_sp); // read reg_l
        self.reg_sp += 1; // increment stack pointer
        self.reg_h = mmu.rb(self.reg_sp); // read reg_h
        self.reg_sp += 1; // increment stack pointer
        self.reg_m = 3; self.reg_t = 12; // 3 M-times taken
    }

    /// (LD A, Addr): Read a byte from an absolute address into reg_a
    pub fn ld_amm(&mut self, mmu: MMU) {
        let addr = mmu.rw(self.reg_pc); // get address from instruction
        self.reg_pc += 2; // increment program counter
        self.reg_a = mmu.rb(addr); // read from the address
        self.reg_m = 4; self.reg_t = 16; // 4 M-times taken
    }
}
