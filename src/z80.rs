use std::cell::RefCell;
use std::rc::Rc;
use mmu::MMU;

// NOTE:IMPLEMENTATION: Even though it's an 8 bit micro-processor, I don't
// really want to worry about roll-over operations in rust
// TODO: Actually... Change back to 8 bit and use the language checks; they are
// actually really useful. So change back to u8 eventually.
pub struct Z80 {
    mmu:    Rc<RefCell<MMU>>,
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
    pub fn new(mmu: Rc<RefCell<MMU>>) -> Z80 {
        Z80 {
            mmu:     mmu,
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

    // control functions
    /// (NOP): No-operation
    pub fn nop(&mut self) { self.reg_m = 1; self.reg_t = 4; }
    /// (HALT): halt the processor
    pub fn halt(&mut self) { self.halt = true; self.reg_m = 1; }
    /// (DI): TODO: ???
    pub fn di(&mut self) { self.reg_ime = 0; self.reg_m = 1; }
    /// (EI): TODO: ???
    pub fn ei(&mut self) { self.reg_ime = 1; self.reg_m = 1; }
    /// (UNDEF): Undefined operation
    pub fn undef(&mut self) { unreachable!(); }


    // Load / Store
    pub fn ld_b_b(&mut self) { self.reg_b = self.reg_b; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_b_c(&mut self) { self.reg_b = self.reg_c; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_b_d(&mut self) { self.reg_b = self.reg_d; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_b_e(&mut self) { self.reg_b = self.reg_e; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_b_h(&mut self) { self.reg_b = self.reg_h; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_b_l(&mut self) { self.reg_b = self.reg_l; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_b_a(&mut self) { self.reg_b = self.reg_a; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_c_b(&mut self) { self.reg_c = self.reg_b; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_c_c(&mut self) { self.reg_c = self.reg_c; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_c_d(&mut self) { self.reg_c = self.reg_d; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_c_e(&mut self) { self.reg_c = self.reg_e; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_c_h(&mut self) { self.reg_c = self.reg_h; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_c_l(&mut self) { self.reg_c = self.reg_l; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_c_a(&mut self) { self.reg_c = self.reg_a; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_d_b(&mut self) { self.reg_d = self.reg_b; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_d_c(&mut self) { self.reg_d = self.reg_c; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_d_d(&mut self) { self.reg_d = self.reg_d; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_d_e(&mut self) { self.reg_d = self.reg_e; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_d_h(&mut self) { self.reg_d = self.reg_h; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_d_l(&mut self) { self.reg_d = self.reg_l; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_d_a(&mut self) { self.reg_d = self.reg_a; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_e_b(&mut self) { self.reg_e = self.reg_b; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_e_c(&mut self) { self.reg_e = self.reg_c; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_e_d(&mut self) { self.reg_e = self.reg_d; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_e_e(&mut self) { self.reg_e = self.reg_e; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_e_h(&mut self) { self.reg_e = self.reg_h; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_e_l(&mut self) { self.reg_e = self.reg_l; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_e_a(&mut self) { self.reg_e = self.reg_a; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_h_b(&mut self) { self.reg_h = self.reg_b; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_h_c(&mut self) { self.reg_h = self.reg_c; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_h_d(&mut self) { self.reg_h = self.reg_d; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_h_e(&mut self) { self.reg_h = self.reg_e; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_h_h(&mut self) { self.reg_h = self.reg_h; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_h_l(&mut self) { self.reg_h = self.reg_l; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_h_a(&mut self) { self.reg_h = self.reg_a; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_l_b(&mut self) { self.reg_l = self.reg_b; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_l_c(&mut self) { self.reg_l = self.reg_c; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_l_d(&mut self) { self.reg_l = self.reg_d; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_l_e(&mut self) { self.reg_l = self.reg_e; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_l_h(&mut self) { self.reg_l = self.reg_h; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_l_l(&mut self) { self.reg_l = self.reg_l; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_l_a(&mut self) { self.reg_l = self.reg_a; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_a_b(&mut self) { self.reg_a = self.reg_b; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_a_c(&mut self) { self.reg_a = self.reg_c; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_a_d(&mut self) { self.reg_a = self.reg_d; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_a_e(&mut self) { self.reg_a = self.reg_e; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_a_h(&mut self) { self.reg_a = self.reg_h; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_a_l(&mut self) { self.reg_a = self.reg_l; self.reg_m = 1; self.reg_t = 4;}
    pub fn ld_a_a(&mut self) { self.reg_a = self.reg_a; self.reg_m = 1; self.reg_t = 4;}

    /// (LOAD B, (HL))
    pub fn ld_b_hl(&mut self) {
        let addr:u16 = ((self.reg_h as u16) << 8u16) + self.reg_l as u16;
        self.reg_b = self.mmu.borrow_mut().rb(&self, addr);
        self.reg_m=2; self.reg_t=8;
    }
    /// (LOAD C, (HL))
    pub fn ld_c_hl(&mut self) {
        let addr:u16 = ((self.reg_h as u16) << 8u16) + self.reg_l as u16;
        self.reg_c = self.mmu.borrow_mut().rb(&self, addr);
        self.reg_m=2; self.reg_t=8;
    }
    /// (LOAD D, (HL))
    pub fn ld_d_hl(&mut self) {
        let addr:u16 = ((self.reg_h as u16) << 8u16) + self.reg_l as u16;
        self.reg_d = self.mmu.borrow_mut().rb(&self, addr);
        self.reg_m=2; self.reg_t=8;
    }
    /// (LOAD E, (HL))
    pub fn ld_e_hl(&mut self) {
        let addr:u16 = ((self.reg_h as u16) << 8u16) + self.reg_l as u16;
        self.reg_e = self.mmu.borrow_mut().rb(&self, addr);
        self.reg_m=2; self.reg_t=8;
    }
    /// (LOAD H, (HL))
    pub fn ld_h_hl(&mut self) {
        let addr:u16 = ((self.reg_h as u16) << 8u16) + self.reg_l as u16;
        self.reg_h = self.mmu.borrow_mut().rb(&self, addr);
        self.reg_m=2; self.reg_t=8;
    }
    /// (LOAD L, (HL))
    pub fn ld_l_hl(&mut self) {
        let addr:u16 = ((self.reg_h as u16) << 8u16) + self.reg_l as u16;
        self.reg_l = self.mmu.borrow_mut().rb(&self, addr);
        self.reg_m=2; self.reg_t=8;
    }
    /// (LOAD A, (HL))
    pub fn ld_a_hl(&mut self) {
        let addr:u16 = ((self.reg_h as u16) << 8u16) + self.reg_l as u16;
        self.reg_a = self.mmu.borrow_mut().rb(&self, addr);
        self.reg_m=2; self.reg_t=8;
    }

    /// (LOAD (HL), B)
    pub fn ld_hl_b(&mut self) {
        let addr:u16 = (self.reg_h as u16) << 8 + self.reg_l as u16;
        self.mmu.borrow_mut().wb( &self, addr, self.reg_b);
        self.reg_m=2;
    }
    pub fn ld_hl_c(&mut self) {
        let addr:u16 = (self.reg_h as u16) << 8 + self.reg_l as u16;
        self.mmu.borrow_mut().wb( &self, addr, self.reg_c);
        self.reg_m=2;
    }
    pub fn ld_hl_d(&mut self) {
        let addr:u16 = (self.reg_h as u16) << 8 + self.reg_l as u16;
        self.mmu.borrow_mut().wb( &self, addr, self.reg_d);
        self.reg_m=2;
    }
    pub fn ld_hl_e(&mut self) {
        let addr:u16 = (self.reg_h as u16) << 8 + self.reg_l as u16;
        self.mmu.borrow_mut().wb( &self, addr, self.reg_e);
        self.reg_m=2;
    }
    pub fn ld_hl_h(&mut self) {
        let addr:u16 = (self.reg_h as u16) << 8 + self.reg_l as u16;
        self.mmu.borrow_mut().wb( &self, addr, self.reg_h);
        self.reg_m=2;
    }
    pub fn ld_hl_l(&mut self) {
        let addr:u16 = (self.reg_h as u16) << 8 + self.reg_l as u16;
        self.mmu.borrow_mut().wb( &self, addr, self.reg_l);
        self.reg_m=2;
    }
    pub fn ld_hl_a(&mut self) {
        let addr:u16 = (self.reg_h as u16) << 8 + self.reg_l as u16;
        self.mmu.borrow_mut().wb( &self, addr, self.reg_a);
        self.reg_m=2;
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
        let mut tmp = self.reg_a;
        tmp -= self.reg_b;
        self.reg_f |= 0x40;
        if (tmp & 255) == 0 { self.reg_f |= 0x80; }
        if tmp < 0 { self.reg_f |= 0x10; }
        self.reg_m = 1; self.reg_t = 4;
    }

    // memory handling instructions

    /// (PUSH BC): Push reg_b and reg_c onto the stack
    pub fn push_bc(&mut self) {
        self.reg_sp -= 1;
        self.mmu.borrow_mut().wb(&self, self.reg_sp, self.reg_b);
        self.reg_sp -= 1;
        self.mmu.borrow_mut().wb(&self, self.reg_sp, self.reg_c);
        self.reg_m = 3; self.reg_t = 12;
    }

    /// (POP HL): Pop reg_h and reg_l off of the stack
    pub fn pop_hl(&mut self) {
        self.reg_l = self.mmu.borrow_mut().rb(&self, self.reg_sp);
        self.reg_sp += 1;
        self.reg_h = self.mmu.borrow_mut().rb(&self, self.reg_sp);
        self.reg_sp += 1;
        self.reg_m = 3; self.reg_t = 12;
    }

    /// (LD A, Addr): Read a byte from an absolute address into reg_a
    pub fn ld_amm(&mut self) {
        let addr = self.mmu.borrow_mut().rw(&self, self.reg_pc);
        self.reg_pc += 2;
        self.reg_a = self.mmu.borrow_mut().rb(&self, addr);
        self.reg_m = 4; self.reg_t = 16;
    }
}

/// A defined type for our ops?
type CpuOp = fn(&mut Z80);

static cpu_map: [CpuOp; 256] =
[
    // 0x00
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    // 0x10
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    // 0x20
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    // 0x30
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    // 0x40
    Z80::ld_b_b,
    Z80::ld_b_c,
    Z80::ld_b_d,
    Z80::ld_b_e,
    Z80::ld_b_h,
    Z80::ld_b_l,
    Z80::ld_b_hl,
    Z80::ld_b_a,
    Z80::ld_c_b,
    Z80::ld_c_c,
    Z80::ld_c_d,
    Z80::ld_c_e,
    Z80::ld_c_h,
    Z80::ld_c_l,
    Z80::ld_c_hl,
    Z80::ld_c_a,
    // 0x50
    Z80::ld_d_b,
    Z80::ld_d_c,
    Z80::ld_d_d,
    Z80::ld_d_e,
    Z80::ld_d_h,
    Z80::ld_d_l,
    Z80::ld_d_hl,
    Z80::ld_d_a,
    Z80::ld_e_b,
    Z80::ld_e_c,
    Z80::ld_e_d,
    Z80::ld_e_e,
    Z80::ld_e_h,
    Z80::ld_e_l,
    Z80::ld_e_hl,
    Z80::ld_e_a,
    // 0x60
    Z80::ld_h_b,
    Z80::ld_h_c,
    Z80::ld_h_d,
    Z80::ld_h_e,
    Z80::ld_h_h,
    Z80::ld_h_l,
    Z80::ld_h_hl,
    Z80::ld_h_a,
    Z80::ld_l_b,
    Z80::ld_l_c,
    Z80::ld_l_d,
    Z80::ld_l_e,
    Z80::ld_l_h,
    Z80::ld_l_l,
    Z80::ld_l_hl,
    Z80::ld_l_a,
    // 0x70
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::halt,
    Z80::nop,
    Z80::ld_a_b,
    Z80::ld_a_c,
    Z80::ld_a_d,
    Z80::ld_a_e,
    Z80::ld_a_h,
    Z80::ld_a_l,
    Z80::ld_a_hl,
    Z80::ld_a_a,
    // 0x80
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::addr_e,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    // 0x90
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    // 0xA0
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    // 0xB0
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    // 0xC0
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::push_bc,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    // 0xD0
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    // 0xE0
    Z80::nop,
    Z80::pop_hl,
    Z80::nop,
    Z80::undef,
    Z80::undef,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::undef,
    Z80::undef,
    Z80::undef,
    Z80::nop,
    Z80::nop,
    // 0xF0
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::di,
    Z80::undef,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::nop,
    Z80::undef,
    Z80::ld_amm,
    Z80::ei,
    Z80::undef,
    Z80::undef,
    Z80::nop,
    Z80::nop,
];

