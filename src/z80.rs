use mmu::MMU;

// NOTE:IMPLEMENTATION: Even though it's an 8 bit micro-processor, I don't
// really want to worry about roll-over operations in rust
// TODO: Actually... Change back to 8 bit and use the language checks; they are
// actually really useful. So change back to u8 eventually.
pub struct Z80 {
    halt:    bool,
    stop:    bool,
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
    reg_ime: u16,
}

impl Z80 {
    pub fn new() -> Z80 {
        Z80 {
            halt:    false,
            stop:    false,
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
            reg_ime: 0,
        }
    }

    pub fn get_pc(&self) -> u16 { self.reg_pc }

    /// Resets processor state
    pub fn reset(&mut self) {
        self.halt = false; self.stop = false;
        self.clock_m = 0.0; self.clock_t = 0.0;
        self.reg_a =  0; self.reg_b =  0; self.reg_c =  0;
        self.reg_d =  0; self.reg_e =  0; self.reg_h =  0;
        self.reg_l =  0; self.reg_f =  0; self.reg_pc = 0;
        self.reg_sp = 0; self.reg_m =  0; self.reg_t =  0;
        self.reg_ime = 0;
    }

    // Load / Store
    pub fn ldrr_bb(&mut self) { self.reg_b = self.reg_b; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_bc(&mut self) { self.reg_b = self.reg_c; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_bd(&mut self) { self.reg_b = self.reg_d; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_be(&mut self) { self.reg_b = self.reg_e; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_bh(&mut self) { self.reg_b = self.reg_h; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_bl(&mut self) { self.reg_b = self.reg_l; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_ba(&mut self) { self.reg_b = self.reg_a; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_cb(&mut self) { self.reg_c = self.reg_b; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_cc(&mut self) { self.reg_c = self.reg_c; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_cd(&mut self) { self.reg_c = self.reg_d; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_ce(&mut self) { self.reg_c = self.reg_e; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_ch(&mut self) { self.reg_c = self.reg_h; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_cl(&mut self) { self.reg_c = self.reg_l; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_ca(&mut self) { self.reg_c = self.reg_a; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_db(&mut self) { self.reg_d = self.reg_b; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_dc(&mut self) { self.reg_d = self.reg_c; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_dd(&mut self) { self.reg_d = self.reg_d; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_de(&mut self) { self.reg_d = self.reg_e; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_dh(&mut self) { self.reg_d = self.reg_h; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_dl(&mut self) { self.reg_d = self.reg_l; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_da(&mut self) { self.reg_d = self.reg_a; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_eb(&mut self) { self.reg_e = self.reg_b; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_ec(&mut self) { self.reg_e = self.reg_c; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_ed(&mut self) { self.reg_e = self.reg_d; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_ee(&mut self) { self.reg_e = self.reg_e; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_eh(&mut self) { self.reg_e = self.reg_h; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_el(&mut self) { self.reg_e = self.reg_l; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_ea(&mut self) { self.reg_e = self.reg_a; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_hb(&mut self) { self.reg_h = self.reg_b; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_hc(&mut self) { self.reg_h = self.reg_c; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_hd(&mut self) { self.reg_h = self.reg_d; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_he(&mut self) { self.reg_h = self.reg_e; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_hh(&mut self) { self.reg_h = self.reg_h; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_hl(&mut self) { self.reg_h = self.reg_l; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_ha(&mut self) { self.reg_h = self.reg_a; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_lb(&mut self) { self.reg_l = self.reg_b; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_lc(&mut self) { self.reg_l = self.reg_c; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_ld(&mut self) { self.reg_l = self.reg_d; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_le(&mut self) { self.reg_l = self.reg_e; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_lh(&mut self) { self.reg_l = self.reg_h; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_ll(&mut self) { self.reg_l = self.reg_l; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_la(&mut self) { self.reg_l = self.reg_a; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_ab(&mut self) { self.reg_a = self.reg_b; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_ac(&mut self) { self.reg_a = self.reg_c; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_ad(&mut self) { self.reg_a = self.reg_d; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_ae(&mut self) { self.reg_a = self.reg_e; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_ah(&mut self) { self.reg_a = self.reg_h; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_al(&mut self) { self.reg_a = self.reg_l; self.reg_m = 1; self.reg_t = 4;}
    pub fn ldrr_aa(&mut self) { self.reg_a = self.reg_a; self.reg_m = 1; self.reg_t = 4;}

    pub fn LDrHLm_b(&mut self, mmu: &mut MMU ) {
        let addr:u16 = ((self.reg_h as u16) << 8u16) + self.reg_l as u16;
        self.reg_b = mmu.rb(&self, addr);
        self.reg_m=2; self.reg_t=8;
    }
    pub fn LDrHLm_c(&mut self, mmu: &mut MMU ) {
        let addr:u16 = ((self.reg_h as u16) << 8u16) + self.reg_l as u16;
        self.reg_c = mmu.rb(&self, addr);
        self.reg_m=2; self.reg_t=8;
    }
    pub fn LDrHLm_d(&mut self, mmu: &mut MMU ) {
        let addr:u16 = ((self.reg_h as u16) << 8u16) + self.reg_l as u16;
        self.reg_d = mmu.rb(&self, addr);
        self.reg_m=2; self.reg_t=8;
    }
    pub fn LDrHLm_e(&mut self, mmu: &mut MMU ) {
        let addr:u16 = ((self.reg_h as u16) << 8u16) + self.reg_l as u16;
        self.reg_e = mmu.rb(&self, addr);
        self.reg_m=2; self.reg_t=8;
    }
    pub fn LDrHLm_h(&mut self, mmu: &mut MMU ) {
        let addr:u16 = ((self.reg_h as u16) << 8u16) + self.reg_l as u16;
        self.reg_h = mmu.rb(&self, addr);
        self.reg_m=2; self.reg_t=8;
    }
    pub fn LDrHLm_l(&mut self, mmu: &mut MMU ) {
        let addr:u16 = ((self.reg_h as u16) << 8u16) + self.reg_l as u16;
        self.reg_l = mmu.rb(&self, addr);
        self.reg_m=2; self.reg_t=8;
    }
    pub fn LDrHLm_a(&mut self, mmu: &mut MMU ) {
        let addr:u16 = ((self.reg_h as u16) << 8u16) + self.reg_l as u16;
        self.reg_a = mmu.rb(&self, addr);
        self.reg_m=2; self.reg_t=8;
    }

    /// (ADD A, E): Add reg_e to reg_a, result in reg_a
    pub fn addr_e(&mut self) {
        self.reg_a += self.reg_e; self.reg_f = 0;
        if (self.reg_a & 0xFF) == 0 { self.reg_f |= 0x80; }
        if self.reg_a > 0xFF { self.reg_f |= 0x10; }
        self.reg_a &= 0xFF;
        self.reg_m = 1; self.reg_t = 4;
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
    pub fn push_bc(&mut self, mmu: &mut MMU) {
        self.reg_sp -= 1; // decrement stack pointer
        mmu.wb(&self, self.reg_sp, self.reg_b); // Write reg_b
        self.reg_sp -= 1; // decrement stack pointer
        mmu.wb(&self, self.reg_sp, self.reg_c); // Write reg_c
        self.reg_m = 3; self.reg_t = 12; // 3 M-times taken
    }

    /// (POP HL): Pop reg_h and reg_l off of the stack
    pub fn pop_hl(&mut self, mmu: &mut MMU) {
        self.reg_l = mmu.rb(&self, self.reg_sp); // read reg_l
        self.reg_sp += 1; // increment stack pointer
        self.reg_h = mmu.rb(&self, self.reg_sp); // read reg_h
        self.reg_sp += 1; // increment stack pointer
        self.reg_m = 3; self.reg_t = 12; // 3 M-times taken
    }

    /// (LD A, Addr): Read a byte from an absolute address into reg_a
    pub fn ld_amm(&mut self, mmu: &mut MMU) {
        let addr = mmu.rw(&self, self.reg_pc); // get address from instruction
        self.reg_pc += 2; // increment program counter
        self.reg_a = mmu.rb(&self, addr); // read from the address
        self.reg_m = 4; self.reg_t = 16; // 4 M-times taken
    }
}
