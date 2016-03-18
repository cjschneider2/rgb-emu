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
    /// (LOAD (HL), C)
    pub fn ld_hl_c(&mut self) {
        let addr:u16 = (self.reg_h as u16) << 8 + self.reg_l as u16;
        self.mmu.borrow_mut().wb( &self, addr, self.reg_c);
        self.reg_m=2;
    }
    /// (LOAD (HL), D)
    pub fn ld_hl_d(&mut self) {
        let addr:u16 = (self.reg_h as u16) << 8 + self.reg_l as u16;
        self.mmu.borrow_mut().wb( &self, addr, self.reg_d);
        self.reg_m=2;
    }
    /// (LOAD (HL), E)
    pub fn ld_hl_e(&mut self) {
        let addr:u16 = (self.reg_h as u16) << 8 + self.reg_l as u16;
        self.mmu.borrow_mut().wb( &self, addr, self.reg_e);
        self.reg_m=2;
    }
    /// (LOAD (HL), H)
    pub fn ld_hl_h(&mut self) {
        let addr:u16 = (self.reg_h as u16) << 8 + self.reg_l as u16;
        self.mmu.borrow_mut().wb( &self, addr, self.reg_h);
        self.reg_m=2;
    }
    /// (LOAD (HL), L)
    pub fn ld_hl_l(&mut self) {
        let addr:u16 = (self.reg_h as u16) << 8 + self.reg_l as u16;
        self.mmu.borrow_mut().wb( &self, addr, self.reg_l);
        self.reg_m=2;
    }
    /// (LOAD (HL), A)
    pub fn ld_hl_a(&mut self) {
        let addr:u16 = (self.reg_h as u16) << 8 + self.reg_l as u16;
        self.mmu.borrow_mut().wb( &self, addr, self.reg_a);
        self.reg_m=2;
    }
    /// (LOAD B, n)
    pub fn ld_b_n(&mut self) {
        self.reg_b = self.mmu.borrow_mut().rb( &self, self.reg_pc);
        self.reg_pc += 1;
        self.reg_m = 2;
    }
    /// (LOAD C, n)
    pub fn ld_c_n(&mut self) {
        self.reg_c = self.mmu.borrow_mut().rb( &self, self.reg_pc);
        self.reg_pc += 1;
        self.reg_m = 2;
    }
    /// (LOAD D, n)
    pub fn ld_d_n(&mut self) {
        self.reg_d = self.mmu.borrow_mut().rb( &self, self.reg_pc);
        self.reg_pc += 1;
        self.reg_m = 2;
    }
    /// (LOAD E, n)
    pub fn ld_e_n(&mut self) {
        self.reg_e = self.mmu.borrow_mut().rb( &self, self.reg_pc);
        self.reg_pc += 1;
        self.reg_m = 2;
    }
    /// (LOAD H, n)
    pub fn ld_h_n(&mut self) {
        self.reg_h = self.mmu.borrow_mut().rb( &self, self.reg_pc);
        self.reg_pc += 1;
        self.reg_m = 2;
    }
    /// (LOAD L, n)
    pub fn ld_l_n(&mut self) {
        self.reg_l = self.mmu.borrow_mut().rb( &self, self.reg_pc);
        self.reg_pc += 1;
        self.reg_m = 2;
    }
    /// (LOAD A, n)
    pub fn ld_a_n(&mut self) {
        self.reg_a = self.mmu.borrow_mut().rb( &self, self.reg_pc);
        self.reg_pc += 1;
        self.reg_m = 2;
    }
    /// (LOAD (HL), n)
    pub fn ld_hl_n(&mut self) {
        self.mmu.borrow_mut().wb(
            &self,
            (self.reg_h as u16) << 8 + self.reg_l as u16,
            self.mmu.borrow_mut().rb(&self, self.reg_pc)
            );
        self.reg_pc += 1;
        self.reg_m = 3;
    }
    /// (LOAD (BC), A)
    pub fn ld_bc_a(&mut self) {
        self.mmu.borrow_mut().wb(
            &self,
            (self.reg_b as u16) << 8 + self.reg_c as u16,
            self.reg_a
            );
        self.reg_m=2;
    }
    /// (LOAD (DE), A)
    pub fn ld_de_a(&mut self) {
        self.mmu.borrow_mut().wb(
            &self,
            (self.reg_d as u16) << 8 + self.reg_e as u16,
            self.reg_a
            );
        self.reg_m=2;
    }
    /// (LOAD Addr, A):
    pub fn ld_mm_a(&mut self) {
        self.mmu.borrow_mut().wb(
            &self,
            self.mmu.borrow_mut().rw(&self, self.reg_pc),
            self.reg_a
            );
        self.reg_pc += 2;
        self.reg_m = 4;
    }
    /// (LOAD A, (BC)):
    pub fn ld_a_bc(&mut self) {
        self.reg_a = self.mmu.borrow_mut().rb(
            &self,
            (self.reg_b as u16) << 8 + self.reg_c as u16
            );
        self.reg_m=2;
    }
    /// (LOAD A, (DE)):
    pub fn ld_a_de(&mut self) {
        self.reg_a = self.mmu.borrow_mut().rb(
            &self,
            (self.reg_d as u16) << 8 + self.reg_e as u16
            );
        self.reg_m=2;
    }
    /// (LOAD A, Addr): Read a byte from an absolute address into reg_a
    pub fn ld_a_mm(&mut self) {
        let addr = self.mmu.borrow_mut().rw(&self, self.reg_pc);
        self.reg_pc += 2;
        self.reg_a = self.mmu.borrow_mut().rb(&self, addr);
        self.reg_m = 4; self.reg_t = 16;
    }
    /// (LOAD, (BC), nn)
    pub fn ld_bc_nn(&mut self) {
        self.reg_c = self.mmu.borrow_mut().rb( &self, self.reg_pc );
        self.reg_b = self.mmu.borrow_mut().rb( &self, self.reg_pc + 1 );
        self.reg_pc += 2;
        self.reg_m = 3;
    }
    /// (LOAD, (DE), nn)
    pub fn ld_de_nn(&mut self) {
        self.reg_e = self.mmu.borrow_mut().rb( &self, self.reg_pc );
        self.reg_d = self.mmu.borrow_mut().rb( &self, self.reg_pc + 1 );
        self.reg_pc += 2;
        self.reg_m = 3;
    }
    /// (LOAD, (HL), nn)
    pub fn ld_hl_nn(&mut self) {
        self.reg_l = self.mmu.borrow_mut().rb( &self, self.reg_pc );
        self.reg_h = self.mmu.borrow_mut().rb( &self, self.reg_pc + 1 );
        self.reg_pc += 2;
        self.reg_m = 3;
    }
    /// (LOAD, (SP), nn)
    pub fn ld_sp_nn(&mut self) {
        self.reg_sp = self.mmu.borrow_mut().rw( &self, self.reg_pc );
        self.reg_pc += 2;
        self.reg_m = 3;
    }
    /// (LOAD, (HL), mm)
    pub fn ld_hl_mm(&mut self) {
        let addr = self.mmu.borrow_mut().rw( &self, self.reg_pc );
        self.reg_pc += 2;
        self.reg_l = self.mmu.borrow_mut().rb( &self, addr );
        self.reg_h = self.mmu.borrow_mut().rb( &self, addr + 1);
        self.reg_m = 5;
    }
    /// (LOAD, mm, (HL))
    pub fn ld_mm_hl(&mut self) {
        let addr = self.mmu.borrow_mut().rw( &self, self.reg_pc );
        self.reg_pc+=2;
        self.mmu.borrow_mut().ww(&self, addr,(self.reg_h as u16) << 8 + self.reg_l as u16);
        self.reg_m = 5;
    }
    /// (LOAD, (HL), (ia))
    pub fn ld_hl_ia(&mut self) {
        self.mmu.borrow_mut().wb( &self, (self.reg_h as u16) << 8 + self.reg_l, self.reg_a);
        self.reg_l = (self.reg_l + 1) & 255;
        if self.reg_l != 0 {
            self.reg_h = (self.reg_h + 1) & 255;
        }
        self.reg_m = 2;
    }
    /// (LOAD, (HL), (da))
    pub fn ld_hl_da(&mut self) {
        self.mmu.borrow_mut().wb( &self, (self.reg_h as u16) << 8 + self.reg_l as u16, self.reg_a);
        self.reg_l = (self.reg_l - 1) & 255;
        if self.reg_l == 255 {
            self.reg_h = (self.reg_h - 1) & 255;
        }
        self.reg_m = 2;
    }
    /// (LOAD, (AH), (li))
    pub fn ld_ah_li(&mut self) {
        self.reg_a = self.mmu.borrow_mut().rb(&self, (self.reg_h as u16) << 8 + self.reg_l as u16);
        self.reg_l = (self.reg_l + 1) & 255;
        if self.reg_l != 0 {
            self.reg_h = (self.reg_h + 1) & 255;
        }
        self.reg_m = 2;
    }
    /// (LOAD, (AH), (ld))
    pub fn ld_ah_ld(&mut self) {
        self.reg_a = self.mmu.borrow_mut().rb(&self, (self.reg_h as u16) << 8 + self.reg_l as u16);
        self.reg_l = (self.reg_l - 1) & 255;
        if self.reg_l == 255 {
            self.reg_h = (self.reg_h - 1) & 255;
        }
        self.reg_m = 2;
    }
    /// (LOAD, A, (io))
    pub fn ld_a_io(&mut self) {
        self.reg_a = self.mmu.borrow_mut().rb(&self, 0xFF00 + self.mmu.borrow_mut().rb(&self, self.reg_pc) as u16);
        self.reg_pc += 1;
        self.reg_m = 3;
    }
    /// (LOAD, (io), A)
    pub fn ld_io_a(&mut self) {
        self.mmu.borrow_mut().wb(&self, 0xFF00 + self.mmu.borrow_mut().rb(&self, self.reg_pc) as u16, self.reg_a);
        self.reg_pc += 1;
        self.reg_m = 3;
    }
    /// (LOAD, A, (ioc))
    pub fn ld_a_ioc(&mut self) {
        self.reg_a = self.mmu.borrow_mut().rb(&self, 0xFF00 + self.reg_c as u16);
        self.reg_m = 2;
    }
    /// (LOAD, (ioc), A)
    pub fn ld_ioc_a(&mut self) {
        self.mmu.borrow_mut().wb(&self, 0xFF00 + self.reg_c as u16, self.reg_a);
        self.reg_m = 2;
    }
    /// (LOAD, (HL), (SP))
    pub fn ld_hl_sp(&mut self) {
        let mut addr = self.mmu.borrow_mut().rb(&self, self.reg_pc);
        if addr > 127 {
            addr = addr - ((!addr + 1) & 255);
        }
        self.reg_pc += 1;
        let addr = addr as u16 + self.reg_sp;
        self.reg_h = ((addr >> 8) & 255) as u8;
        self.reg_l = (addr & 255) as u8;
        self.reg_m = 3;
    }
// memory swap operations
    /// (SWAP B): Swaps higher and lower bits in register B
    pub fn swap_b(&mut self) {
        let mut tr = self.reg_b;
        self.reg_b = (( tr & 0xF) << 4) | ((tr & 0xF0) >> 4);
        self.reg_f = if self.reg_b != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    /// (SWAP C):
    pub fn swap_c(&mut self) {
        let mut tr = self.reg_c;
        self.reg_c = (( tr & 0xF) << 4) | ((tr & 0xF0) >> 4);
        self.reg_f = if self.reg_c != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    /// (SWAP D):
    pub fn swap_d(&mut self) {
        let mut tr = self.reg_d;
        self.reg_d = (( tr & 0xF) << 4) | ((tr & 0xF0) >> 4);
        self.reg_f = if self.reg_d != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    /// (SWAP E):
    pub fn swap_e(&mut self) {
        let mut tr = self.reg_e;
        self.reg_e = (( tr & 0xF) << 4) | ((tr & 0xF0) >> 4);
        self.reg_f = if self.reg_e != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    /// (SWAP H):
    pub fn swap_h(&mut self) {
        let mut tr = self.reg_h;
        self.reg_h = (( tr & 0xF) << 4) | ((tr & 0xF0) >> 4);
        self.reg_f = if self.reg_h != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    /// (SWAP L):
    pub fn swap_l(&mut self) {
        let mut tr = self.reg_l;
        self.reg_l = (( tr & 0xF) << 4) | ((tr & 0xF0) >> 4);
        self.reg_f = if self.reg_l != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    /// (SWAP A):
    pub fn swap_a(&mut self) {
        let mut tr = self.reg_a;
        self.reg_a = (( tr & 0xF) << 4) | ((tr & 0xF0) >> 4);
        self.reg_f = if self.reg_a != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }

// Data processing operations
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
    Z80::ld_a_mm,
    Z80::ei,
    Z80::undef,
    Z80::undef,
    Z80::nop,
    Z80::nop,
];

