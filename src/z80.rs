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

    pub fn add_b(&mut self) {
        let a = self.reg_a;
        self.reg_a += self.reg_b;
        self.reg_f = if self.reg_a > 255 { 0x10 } else { 0 };
        self.reg_a &= 255;
        if self.reg_a != 0 { self.reg_f |= 0x80; };
        if ((self.reg_a ^ self.reg_b ^ a) & 0x10) != 0 { self.reg_f |= 0x20; };
        self.reg_m = 1;
    }
    pub fn add_c(&mut self) {
        let a = self.reg_a;
        self.reg_a += self.reg_c;
        self.reg_f = if self.reg_a > 255 { 0x10 } else { 0 };
        self.reg_a &= 255; if self.reg_a != 0 { self.reg_f |= 0x80; };
        if ((self.reg_a ^ self.reg_c ^ a) & 0x10) != 0 { self.reg_f |= 0x20; }
        self.reg_m = 1;
    }
    pub fn add_d(&mut self) {
        let a = self.reg_a;
        self.reg_a += self.reg_d;
        self.reg_f = if self.reg_a > 255 { 0x10 } else { 0 };
        self.reg_a &= 255; if self.reg_a != 0 { self.reg_f |= 0x80; };
        if ((self.reg_a ^ self.reg_d ^ a) & 0x10) != 0 { self.reg_f |= 0x20; };
        self.reg_m = 1;
    }
    pub fn add_e(&mut self) {
        let a = self.reg_a;
        self.reg_a += self.reg_e;
        self.reg_f = if self.reg_a > 255 { 0x10 } else { 0 };
        self.reg_a &= 255; if self.reg_a != 0 { self.reg_f |= 0x80; };
        if ((self.reg_a ^ self.reg_e ^ a) & 0x10) != 0 { self.reg_f |= 0x20; };
        self.reg_m = 1;
    }
    pub fn add_h(&mut self) {
        let a = self.reg_a;
        self.reg_a += self.reg_h;
        self.reg_f = if self.reg_a > 255 { 0x10 } else { 0 };
        self.reg_a &= 255; if self.reg_a != 0 { self.reg_f |= 0x80; };
        if ((self.reg_a ^ self.reg_h ^ a) & 0x10) != 0 { self.reg_f |= 0x20; };
        self.reg_m = 1;
    }
    pub fn add_l(&mut self) {
        let a = self.reg_a;
        self.reg_a += self.reg_l;
        self.reg_f = if self.reg_a > 255 { 0x10 } else { 0 };
        self.reg_a &= 255; if self.reg_a != 0 { self.reg_f |= 0x80; };
        if ((self.reg_a ^ self.reg_l ^ a) & 0x10) != 0 { self.reg_f |= 0x20; };
        self.reg_m = 1;
    }
    pub fn add_a(&mut self) {
        let a = self.reg_a;
        self.reg_a += self.reg_a;
        self.reg_f = if self.reg_a > 255 { 0x10 } else { 0 };
        self.reg_a &= 255; if self.reg_a != 0 { self.reg_f |= 0x80; };
        if ((self.reg_a ^ self.reg_a ^ a) & 0x10) != 0 { self.reg_f |= 0x20; };
        self.reg_m = 1;
    }
    pub fn add_hl(&mut self) {
        let a = self.reg_a;
        let m = self.mmu.borrow_mut().rb(&self, (self.reg_h as u16) << 8 + self.reg_l as u16);
        self.reg_a += m;
        self.reg_f = if self.reg_a > 255 { 0x10 } else { 0 };
        self.reg_a &= 255;
        if self.reg_a != 0 {
            self.reg_f |= 0x80;
        };
        if ((self.reg_a ^ a ^ m) & 0x10) != 0 {
            self.reg_f |= 0x20;
        }
        self.reg_m=2;
    }
    pub fn add_n(&mut self) {
        let a = self.reg_a;
        let m = self.mmu.borrow_mut().rb(&self, self.reg_pc);
        self.reg_a += m;
        self.reg_pc += 1;
        self.reg_f = if self.reg_a > 255 { 0x10 } else { 0 };
        self.reg_a &= 255;
        if self.reg_a != 0 {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ a ^ m ) & 0x10 ) != 0 {
            self.reg_f |= 0x20;
        }
        self.reg_m = 2;
    }
    pub fn add_hl_bc(&mut self) {
        let mut hl:u32 = (self.reg_h as u32) << 8 + self.reg_l as u32;
        hl += (self.reg_b as u32) << 8 + self.reg_c as u32;
        // TODO: checked overflow
        if hl > 65535 {
            self.reg_f |= 0x10;
        } else {
            self.reg_f &= 0xEF;
        }
        self.reg_h = (( hl >> 8) & 255) as u8;
        self.reg_l = (hl & 255) as u8;
        self.reg_m = 3;
    }
    pub fn add_hl_de(&mut self) {
        let mut hl:u32 = (self.reg_h as u32) << 8 + self.reg_l as u32;
        hl += (self.reg_d as u32) << 8 + self.reg_e as u32;
        if hl > 65535 {
            self.reg_f |= 0x10;
        } else {
            self.reg_f &= 0xEF;
        }
        self.reg_h = ((hl >> 8) & 255) as u8;
        self.reg_l = (hl & 255) as u8;
        self.reg_m = 3;
    }
    pub fn add_hl_hl(&mut self) {
        let mut hl:u32 = (self.reg_h as u32) << 8 + self.reg_l as u32;
        hl += (self.reg_h as u32) << 8 + self.reg_l as u32;
        if hl > 65535 {
            self.reg_f |= 0x10;
        } else {
            self.reg_f &= 0xEF;
        }
        self.reg_h = ((hl >> 8) & 255) as u8;
        self.reg_l = (hl & 255) as u8;
        self.reg_m = 3;
    }
    pub fn add_hl_sp(&mut self) {
        let mut hl = (self.reg_h as u32) << 8 + self.reg_l as u32;
        hl += self.reg_sp as u32;
        if hl > 65535 {
            self.reg_f |= 0x10;
        } else {
            self.reg_f &= 0xEF;
        }
        self.reg_h = ((hl>>8) & 255) as u8;
        self.reg_l = (hl&255) as u8;
        self.reg_m = 3;
    }
    pub fn add_sp_n (&mut self) {
        let mut n = self.mmu.borrow_mut().rb(&self, self.reg_pc);
        if n > 127 {
            n -= (!n + 1) & 255;
        }
        self.reg_pc += 1;
        self.reg_sp += n as u16;
        self.reg_m = 4;
    }

    pub fn add_c_b(&mut self) {
        let a = self.reg_a;
        self.reg_a += self.reg_b;
        self.reg_a += if (self.reg_f & 0x10) != 0 { 1 } else { 0 };
        self.reg_f = if self.reg_a > 255 { 0x10 } else { 0 };
        self.reg_a &= 255;
        if self.reg_a == 0 {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ self.reg_b ^ a) & 0x10) != 0 {
            self.reg_f|=0x20;
        }
        self.reg_m = 1;
    }
    pub fn add_c_c(&mut self) {
        let a = self.reg_a;
        self.reg_a += self.reg_c;
        self.reg_a += if (self.reg_f & 0x10) != 0 { 1 } else { 0 };
        self.reg_f = if self.reg_a > 255 { 0x10 } else { 0 };
        self.reg_a &= 255;
        if self.reg_a == 0 {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ self.reg_c ^ a) & 0x10) != 0 {
            self.reg_f|=0x20;
        }
        self.reg_m = 1;
    }
    pub fn add_c_d(&mut self) {
        let a = self.reg_a;
        self.reg_a += self.reg_d;
        self.reg_a += if (self.reg_f & 0x10) != 0 { 1 } else { 0 };
        self.reg_f = if self.reg_a > 255 { 0x10 } else { 0 };
        self.reg_a &= 255;
        if self.reg_a == 0 {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ self.reg_d ^ a) & 0x10) != 0 {
            self.reg_f |= 0x20;
        } self.reg_m = 1;
    }
    pub fn add_c_e(&mut self) {
        let a = self.reg_a;
        self.reg_a += self.reg_e;
        self.reg_a += if (self.reg_f & 0x10) != 0 { 1 } else { 0 };
        self.reg_f = if self.reg_a > 255 { 0x10 } else { 0 };
        self.reg_a &= 255;
        if self.reg_a == 0 {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ self.reg_e ^ a) & 0x10) != 0 {
            self.reg_f |= 0x20;
        }
        self.reg_m = 1;
    }
    pub fn add_c_h(&mut self) {
        let a = self.reg_a;
        self.reg_a += self.reg_h;
        self.reg_a += if (self.reg_f & 0x10) != 0 { 1 } else { 0 };
        self.reg_f = if self.reg_a > 255 { 0x10 } else { 0 };
        self.reg_a &= 255;
        if self.reg_a == 0 {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ self.reg_h ^ a) & 0x10) != 0 {
            self.reg_f |= 0x20;
        }
        self.reg_m = 1;
    }
    pub fn add_c_l(&mut self) {
        let a = self.reg_a;
        self.reg_a += self.reg_l;
        self.reg_a += if (self.reg_f & 0x10) != 0 { 1 } else { 0 };
        self.reg_f = if self.reg_a > 255 { 0x10 } else { 0 };
        self.reg_a &= 255;
        if self.reg_a == 0 {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ self.reg_l ^ a) & 0x10) != 0 {
            self.reg_f|=0x20;
        }
        self.reg_m = 1;
    }
    pub fn add_c_a(&mut self) {
        let a = self.reg_a;
        self.reg_a += self.reg_a;
        self.reg_a += if (self.reg_f & 0x10) != 0 { 1 } else { 0 };
        self.reg_f = if self.reg_a > 255 { 0x10 } else { 0 };
        self.reg_a &= 255;
        if self.reg_a == 0 {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ self.reg_a ^ a) & 0x10) != 0 {
            self.reg_f |= 0x20;
        }
            self.reg_m = 1;
    }
    pub fn add_c_n(&mut self) {
        let a = self.reg_a;
        let m = self.mmu.borrow_mut().rb(&self, self.reg_pc);
        self.reg_a += m;
        self.reg_pc += 1;
        self.reg_a += if (self.reg_f & 0x10) != 0 { 1 } else { 0 };
        self.reg_f = if self.reg_a > 255 { 0x10 } else { 0 };
        self.reg_a &= 255;
        if self.reg_a == 0 {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ m ^ a) & 0x10) != 0 {
            self.reg_f |= 0x20;
        }
        self.reg_m=2;
    }
    pub fn add_c_hl(&mut self) {
        let a = self.reg_a;
        let m = self.mmu.borrow_mut().rb(&self, (self.reg_h as u16) << 8 + self.reg_l as u16);
        self.reg_a += m;
        self.reg_a += if (self.reg_f & 0x10) != 0 { 1 } else { 0 };
        self.reg_f = if self.reg_a > 255 { 0x10 } else { 0 };
        self.reg_a &= 255;
        if self.reg_a == 0 {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ m ^ a) & 0x10) != 0 {
            self.reg_f |= 0x20;
        }
        self.reg_m = 2;
    }

    pub fn sub_b(&mut self) {
        let a = self.reg_a;
        self.reg_a -= self.reg_b;
        self.reg_f = if self.reg_a < 0 { 0x50} else { 0x40 };
        self.reg_a &= 255;
        if (self.reg_a == 0) {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ self.reg_b ^ a) & 0x10) != 0 {
            self.reg_f |= 0x20;
        }
        self.reg_m = 1;
    }
    pub fn sub_c(&mut self) {
        let a = self.reg_a;
        self.reg_a -= self.reg_c;
        self.reg_f = if self.reg_a < 0 { 0x50} else { 0x40 };
        self.reg_a &= 255;
        if (self.reg_a == 0) {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ self.reg_c ^ a) & 0x10) != 0 {
            self.reg_f |= 0x20;
        }
        self.reg_m = 1;
    }
    pub fn sub_d(&mut self) {
        let a = self.reg_a;
        self.reg_a -= self.reg_d;
        self.reg_f = if self.reg_a < 0 { 0x50} else { 0x40 };
        self.reg_a &= 255;
        if (self.reg_a == 0) {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ self.reg_d ^ a) & 0x10) != 0 {
            self.reg_f |= 0x20;
        }
        self.reg_m = 1;
    }
    pub fn sub_e(&mut self) {
        let a = self.reg_a;
        self.reg_a -= self.reg_e;
        self.reg_f = if self.reg_a < 0 { 0x50} else { 0x40 };
        self.reg_a &= 255;
        if (self.reg_a == 0) {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ self.reg_e ^ a) & 0x10) != 0 {
            self.reg_f |= 0x20;
        }
        self.reg_m = 1;
    }
    pub fn sub_h(&mut self) {
        let a = self.reg_a;
        self.reg_a -= self.reg_h;
        self.reg_f = if self.reg_a < 0 { 0x50} else { 0x40 };
        self.reg_a &= 255;
        if (self.reg_a == 0) {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ self.reg_h ^ a) & 0x10) != 0 {
            self.reg_f |= 0x20;
        }
        self.reg_m = 1;
    }
    pub fn sub_l(&mut self) {
        let a = self.reg_a;
        self.reg_a -= self.reg_l;
        self.reg_f = if self.reg_a < 0 { 0x50} else { 0x40 };
        self.reg_a &= 255;
        if (self.reg_a == 0) {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ self.reg_l ^ a) & 0x10) != 0 {
            self.reg_f |= 0x20;
        }
        self.reg_m = 1;
    }
    pub fn sub_a(&mut self) {
        let a = self.reg_a;
        self.reg_a -= self.reg_a;
        self.reg_f = if self.reg_a < 0 { 0x50} else { 0x40 };
        self.reg_a &= 255;
        if (self.reg_a == 0) {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ self.reg_a ^ a) & 0x10) != 0 {
            self.reg_f |= 0x20;
        }
        self.reg_m = 1;
    }
    pub fn sub_n(&mut self) {
        let a = self.reg_a;
        let m = self.mmu.borrow_mut().rb(&self, self.reg_pc);
        self.reg_a -= m;
        self.reg_pc += 1;
        self.reg_f = if self.reg_a < 0 { 0x50 } else { 0x40 };
        self.reg_a &= 255;
        if (self.reg_a == 0) {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ m ^ a) & 0x10) != 0 {
            self.reg_f |= 0x20;
        }
        self.reg_m = 2;
    }
    pub fn sub_hl(&mut self) {
        let a = self.reg_a;
        let m = self.mmu.borrow_mut().rb(&self, (self.reg_h as u16) << 8 + self.reg_l as u16);
        self.reg_a -= m;
        self.reg_f = if self.reg_a < 0 { 0x50 } else { 0x40 };
        self.reg_a &= 255;
        if self.reg_a == 0 {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ m ^ a) & 0x10) != 0 {
            self.reg_f|=0x20;
        }
        self.reg_m = 2;
    }

    /// (SBC (B)): Subtract from (B) and Carry flag from (A)
    pub fn sbc_b(&mut self) {
        let a = self.reg_a;
        self.reg_a -= self.reg_b;
        self.reg_a -= if (self.reg_f & 0x10) != 0 { 1 } else { 0 };
        self.reg_f = if (self.reg_a<0) { 0x50 } else { 0x40 };
        self.reg_a &= 255;
        if self.reg_a == 0 {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ self.reg_b ^ a) & 0x10) != 0 {
            self.reg_f |= 0x20;
        }
        self.reg_m = 1;
    }
    pub fn sbc_c(&mut self) {
        let a = self.reg_a;
        self.reg_a -= self.reg_c;
        self.reg_a -= if (self.reg_f & 0x10) != 0 { 1 } else { 0 };
        self.reg_f = if (self.reg_a<0) { 0x50 } else { 0x40 };
        self.reg_a &= 255;
        if self.reg_a == 0 {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ self.reg_c ^ a) & 0x10) != 0 {
            self.reg_f |= 0x20;
        }
        self.reg_m = 1;
    }
    pub fn sbc_d(&mut self) {
        let a = self.reg_a;
        self.reg_a -= self.reg_d;
        self.reg_a -= if (self.reg_f & 0x10) != 0 { 1 } else { 0 };
        self.reg_f = if (self.reg_a<0) { 0x50 } else { 0x40 };
        self.reg_a &= 255;
        if self.reg_a == 0 {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ self.reg_d ^ a) & 0x10) != 0 {
            self.reg_f |= 0x20;
        }
        self.reg_m = 1;
    }
    pub fn sbc_e(&mut self) {
        let a = self.reg_a;
        self.reg_a -= self.reg_e;
        self.reg_a -= if (self.reg_f & 0x10) != 0 { 1 } else { 0 };
        self.reg_f = if (self.reg_a<0) { 0x50 } else { 0x40 };
        self.reg_a &= 255;
        if self.reg_a == 0 {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ self.reg_e ^ a) & 0x10) != 0 {
            self.reg_f |= 0x20;
        }
        self.reg_m = 1;
    }
    pub fn sbc_h(&mut self) {
        let a = self.reg_a;
        self.reg_a -= self.reg_h;
        self.reg_a -= if (self.reg_f & 0x10) != 0 { 1 } else { 0 };
        self.reg_f = if (self.reg_a<0) { 0x50 } else { 0x40 };
        self.reg_a &= 255;
        if self.reg_a == 0 {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ self.reg_h ^ a) & 0x10) != 0 {
            self.reg_f |= 0x20;
        }
        self.reg_m = 1;
    }
    pub fn sbc_l(&mut self) {
        let a = self.reg_a;
        self.reg_a -= self.reg_l;
        self.reg_a -= if (self.reg_f & 0x10) != 0 { 1 } else { 0 };
        self.reg_f = if (self.reg_a<0) { 0x50 } else { 0x40 };
        self.reg_a &= 255;
        if self.reg_a == 0 {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ self.reg_l ^ a) & 0x10) != 0 {
            self.reg_f |= 0x20;
        }
        self.reg_m = 1;
    }
    pub fn sbc_a(&mut self) {
        let a = self.reg_a;
        self.reg_a -= self.reg_a;
        self.reg_a -= if (self.reg_f & 0x10) != 0 { 1 } else { 0 };
        self.reg_f = if (self.reg_a<0) { 0x50 } else { 0x40 };
        self.reg_a &= 255;
        if self.reg_a == 0 {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ self.reg_a ^ a) & 0x10) != 0 {
            self.reg_f |= 0x20;
        }
        self.reg_m = 1;
    }
    pub fn sbc_hl(&mut self) {
        let a = self.reg_a;
        let m = self.mmu.borrow_mut().rb(&self, (self.reg_h as u16) << 8 + self.reg_l as u16);
        self.reg_a -= m;
        self.reg_a -= if (self.reg_f & 0x10) != 0 { 1 } else { 0 };
        self.reg_f = if self.reg_a < 0 { 0x50 } else { 0x40 };
        self.reg_a &= 255;
        if self.reg_a == 0 {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ m ^ a) & 0x10) != 0 {
            self.reg_f |= 0x20;
        }
        self.reg_m = 2;
    }
    pub fn sbc_n(&mut self) {
        let a = self.reg_a;
        let m = self.mmu.borrow_mut().rb(&self, self.reg_pc);
        self.reg_a -= m;
        self.reg_pc += 1;
        self.reg_a -= if (self.reg_f & 0x10) != 0 { 1 } else { 0 };
        self.reg_f = if self.reg_a < 0 { 0x50 } else { 0x40 };
        self.reg_a &= 255;
        if self.reg_a == 0 {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ m ^ a) & 0x10) != 0 {
            self.reg_f |= 0x20;
        }
        self.reg_m = 2;
    }

    pub fn cp_b(&mut self) {
        let mut i = self.reg_a;
        i -= self.reg_b;
        self.reg_f = if i < 0 { 0x50 } else { 0x40 };
        i &= 255;
        if i == 0 {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ self.reg_b ^ i) & 0x10) != 0 {
            self.reg_f |= 0x20;
        }
        self.reg_m = 1;
    }
    pub fn cp_c(&mut self) {
        let mut i = self.reg_a;
        i -= self.reg_c;
        self.reg_f = if i < 0 { 0x50 } else { 0x40 };
        i &= 255;
        if i == 0 {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ self.reg_c ^ i) & 0x10) != 0 {
            self.reg_f |= 0x20;
        }
        self.reg_m = 1;
    }
    pub fn cp_d(&mut self) {
        let mut i = self.reg_a;
        i -= self.reg_d;
        self.reg_f = if i < 0 { 0x50 } else { 0x40 };
        i &= 255;
        if i == 0 {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ self.reg_d ^ i) & 0x10) != 0 {
            self.reg_f |= 0x20;
        }
        self.reg_m = 1;
    }
    pub fn cp_e(&mut self) {
        let mut i = self.reg_a;
        i -= self.reg_e;
        self.reg_f = if i < 0 { 0x50 } else { 0x40 };
        i &= 255;
        if i == 0 {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ self.reg_e ^ i) & 0x10) != 0 {
            self.reg_f |= 0x20;
        }
        self.reg_m = 1;
    }
    pub fn cp_h(&mut self) {
        let mut i = self.reg_a;
        i -= self.reg_h;
        self.reg_f = if i < 0 { 0x50 } else { 0x40 };
        i &= 255;
        if i == 0 {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ self.reg_h ^ i) & 0x10) != 0 {
            self.reg_f |= 0x20;
        }
        self.reg_m = 1;
    }
    pub fn cp_l(&mut self) {
        let mut i = self.reg_a;
        i -= self.reg_l;
        self.reg_f = if i < 0 { 0x50 } else { 0x40 };
        i &= 255;
        if i == 0 {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ self.reg_l ^ i) & 0x10) != 0 {
            self.reg_f |= 0x20;
        }
        self.reg_m = 1;
    }
    pub fn cp_a(&mut self) {
        let mut i = self.reg_a;
        i -= self.reg_a;
        self.reg_f = if i < 0 { 0x50 } else { 0x40 };
        i &= 255;
        if i == 0 {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ self.reg_a ^ i) & 0x10) != 0 {
            self.reg_f |= 0x20;
        }
        self.reg_m = 1;
    }
    pub fn cp_hl(&mut self){
        let mut i = self.reg_a;
        let m = self.mmu.borrow_mut().rb(&self, (self.reg_h as u16) << 8 + self.reg_l as u16);
        i -= m;
        self.reg_f = if i < 0 { 0x50 } else { 0x40 };
        i &= 255;
        if i == 0 {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ i ^ m) & 0x10) != 0 {
            self.reg_f |= 0x20;
        }
        self.reg_m = 2;
    }
    pub fn cp_n(&mut self) {
        let mut i = self.reg_a;
        let m = self.mmu.borrow_mut().rb(&self, self.reg_pc);
        i -= m;
        self.reg_pc += 1;
        self.reg_f = if i < 0 { 0x50 } else { 0x40 };
        i &= 255;
        if i == 0 {
            self.reg_f |= 0x80;
        }
        if ((self.reg_a ^ i ^ m) & 0x10) != 0 {
            self.reg_f |= 0x20;
        }
        self.reg_m = 2;
    }

    pub fn daa(&mut self) {
        let a = self.reg_a;
        if ((self.reg_f & 0x20) != 0 || (self.reg_a & 15) > 9) {
            self.reg_a += 6;
        }
        self.reg_f &= 0xEF;
        if ((self.reg_f & 0x20) != 0 || ( a > 0x99)) {
            self.reg_a += 0x60;
            self.reg_f |= 0x10;
        }
        self.reg_m = 1;
    }

    pub fn and_b(&mut self) {
        self.reg_a &= self.reg_b;
        self.reg_a &= 255;
        self.reg_f = if self.reg_a != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn and_c(&mut self) {
        self.reg_a &= self.reg_c;
        self.reg_a &= 255;
        self.reg_f = if self.reg_a != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn and_d(&mut self) {
        self.reg_a &= self.reg_d;
        self.reg_a &= 255;
        self.reg_f = if self.reg_a != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn and_e(&mut self) {
        self.reg_a &= self.reg_e;
        self.reg_a &= 255;
        self.reg_f = if self.reg_a != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn and_h(&mut self) {
        self.reg_a &= self.reg_h;
        self.reg_a &= 255;
        self.reg_f = if self.reg_a != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn and_l(&mut self) {
        self.reg_a &= self.reg_l;
        self.reg_a &= 255;
        self.reg_f = if self.reg_a != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn and_a(&mut self) {
        self.reg_a &= self.reg_a;
        self.reg_a &= 255;
        self.reg_f = if self.reg_a != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn and_hl(&mut self) {
        self.reg_a &= self.mmu.borrow_mut().rb(&self, (self.reg_h as u16) <<8 + self.reg_l as u16);
        self.reg_a &= 255;
        self.reg_f = if self.reg_a != 0 { 0 } else { 0x80 };
        self.reg_m = 2;
    }
    pub fn and_n(&mut self) {
        self.reg_a &= self.mmu.borrow_mut().rb(&self, self.reg_pc);
        self.reg_pc += 1;
        self.reg_a &= 255;
        self.reg_f = if self.reg_a != 0 { 0 } else { 0x80 };
        self.reg_m = 2;
    }

    pub fn or_b(&mut self) {
        self.reg_a |= self.reg_b;
        self.reg_a &= 255;
        self.reg_f = if self.reg_a != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn or_c(&mut self) {
        self.reg_a |= self.reg_c;
        self.reg_a &= 255;
        self.reg_f = if self.reg_a != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn or_d(&mut self) {
        self.reg_a |= self.reg_d;
        self.reg_a &= 255;
        self.reg_f = if self.reg_a != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn or_e(&mut self) {
        self.reg_a |= self.reg_e;
        self.reg_a &= 255;
        self.reg_f = if self.reg_a != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn or_h(&mut self) {
        self.reg_a |= self.reg_h;
        self.reg_a &= 255;
        self.reg_f = if self.reg_a != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn or_l(&mut self) {
        self.reg_a |= self.reg_l;
        self.reg_a &= 255;
        self.reg_f = if self.reg_a != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn or_a(&mut self) {
        self.reg_a |= self.reg_a;
        self.reg_a &= 255;
        self.reg_f = if self.reg_a != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn or_hl(&mut self) {
        self.reg_a |= self.mmu.borrow_mut().rb(&self, (self.reg_h as u16) << 8 + self.reg_l as u16);
        self.reg_a &= 255;
        self.reg_f = if self.reg_a != 0 { 0 } else { 0x80 };
        self.reg_m = 2;
    }
    pub fn or_n(&mut self) {
        self.reg_a |= self.mmu.borrow_mut().rb(&self, self.reg_pc);
        self.reg_pc += 1;
        self.reg_a &= 255;
        self.reg_f = if self.reg_a != 0 { 0 } else { 0x80 };
        self.reg_m = 2;
    }

    pub fn xor_b(&mut self) {
        self.reg_a ^= self.reg_b;
        self.reg_a &= 255;
        self.reg_f = if self.reg_a != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn xor_c(&mut self) {
        self.reg_a ^= self.reg_c;
        self.reg_a &= 255;
        self.reg_f = if self.reg_a != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn xor_d(&mut self) {
        self.reg_a ^= self.reg_d;
        self.reg_a &= 255;
        self.reg_f = if self.reg_a != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn xor_e(&mut self) {
        self.reg_a ^= self.reg_e;
        self.reg_a &= 255;
        self.reg_f = if self.reg_a != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn xor_h(&mut self) {
        self.reg_a ^= self.reg_h;
        self.reg_a &= 255;
        self.reg_f = if self.reg_a != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn xor_l(&mut self) {
        self.reg_a ^= self.reg_l;
        self.reg_a &= 255;
        self.reg_f = if self.reg_a != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn xor_a(&mut self) {
        self.reg_a ^= self.reg_a;
        self.reg_a &= 255;
        self.reg_f = if self.reg_a != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn xor_hl(&mut self) {
        self.reg_a ^= self.mmu.borrow_mut().rb(&self, (self.reg_h as u16) << 8 + self.reg_l as u16);
        self.reg_a &= 255;
        self.reg_f = if self.reg_a != 0 { 0 } else { 0x80 };
        self.reg_m = 2;
    }
    pub fn xor_n(&mut self) {
        self.reg_a ^= self.mmu.borrow_mut().rb(&self, self.reg_pc);
        self.reg_pc += 1;
        self.reg_a &= 255;
        self.reg_f = if self.reg_a != 0 { 0 } else { 0x80 };
        self.reg_m = 2;
    }
    pub fn inc_b(&mut self) {
        self.reg_b += 1;
        self.reg_b &= 255;
        self.reg_f = if self.reg_b != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn inc_c(&mut self) {
        self.reg_c += 1;
        self.reg_c &= 255;
        self.reg_f = if self.reg_c != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn inc_d(&mut self) {
        self.reg_d += 1;
        self.reg_d &= 255;
        self.reg_f = if self.reg_d != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn inc_e(&mut self) {
        self.reg_e += 1;
        self.reg_e &= 255;
        self.reg_f = if self.reg_e != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn inc_h(&mut self) {
        self.reg_h += 1;
        self.reg_h &= 255;
        self.reg_f = if self.reg_h != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn inc_l(&mut self) {
        self.reg_l += 1;
        self.reg_l &= 255;
        self.reg_f = if self.reg_l != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn inc_a(&mut self) {
        self.reg_a += 1;
        self.reg_a &= 255;
        self.reg_f = if self.reg_a != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn inc_hl(&mut self) {
        let mut i = self.mmu.borrow_mut().rb(&self, (self.reg_h as u16) << 8 + self.reg_l as u16) + 1;
        i &= 255;
        self.mmu.borrow_mut().wb(&self, (self.reg_h as u16) << 8 + self.reg_l as u16,i);
        self.reg_f = if i != 0 { 0 } else { 0x80 };
        self.reg_m = 3;
    }

    pub fn dec_b(&mut self) {
        self.reg_b -= 1;
        self.reg_b &= 255;
        self.reg_f = if self.reg_b != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn dec_c(&mut self) {
        self.reg_c -= 1;
        self.reg_c &= 255;
        self.reg_f = if self.reg_c != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn dec_d(&mut self) {
        self.reg_d -= 1;
        self.reg_d &= 255;
        self.reg_f = if self.reg_d != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn dec_e(&mut self) {
        self.reg_e -= 1;
        self.reg_e &= 255;
        self.reg_f = if self.reg_e != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn dec_h(&mut self) {
        self.reg_h -= 1;
        self.reg_h &= 255;
        self.reg_f = if self.reg_h != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn dec_l(&mut self) {
        self.reg_l -= 1;
        self.reg_l &= 255;
        self.reg_f = if self.reg_l != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn dec_a(&mut self) {
        self.reg_a -= 1;
        self.reg_a &= 255;
        self.reg_f = if self.reg_a != 0 { 0 } else { 0x80 };
        self.reg_m = 1;
    }
    pub fn dec_hl(&mut self) {
        let mut i = self.mmu.borrow_mut().rb(&self, (self.reg_h as u16) << 8 + self.reg_l as u 16) - 1;
        i &= 255;
        self.mmu.borrow_mut().wb(&self, (self.reg_h as u16) << 8 + self.reg_l as u16, i);
        self.reg_f = if i != 0 { 0 } else { 0x80 };
        self.reg_m = 3;
    }

    pub fn inc_bc(&mut self) {
        self.reg_c =(self.reg_c + 1) & 255;
        if(!self.reg_c) == 0 {
            self.reg_b = (self.reg_b + 1) & 255;
        }
        self.reg_m = 1;
    }
    pub fn inc_de(&mut self) {
        self.reg_e =(self.reg_e + 1) & 255;
        if(!self.reg_e) == 0 {
            self.reg_d = (self.reg_d + 1) & 255;
        }
        self.reg_m = 1;
    }
    pub fn inc_hl(&mut self) {
        self.reg_l =(self.reg_l + 1) & 255;
        if(!self.reg_l) == 0 {
            self.reg_h = (self.reg_h + 1) & 255;
        }
        self.reg_m = 1;
    }
    pub fn inc_sp(&mut self) {
        self.reg_sp = (self.reg_sp + 1) & 65535;
        self.reg_m=1;
    }

    pub fn dec_bc(&mut self) {
        self.reg_c = (self.reg_c - 1) & 255;
        if self.reg_c == 255 {
            self.reg_b = (self.reg_b - 1) & 255;
        }
        self.reg_m = 1;
    }
    pub fn dec_de(&mut self) {
        self.reg_e = (self.reg_e - 1) & 255;
        if self.reg_e == 255 {
            self.reg_d = (self.reg_d - 1) & 255;
        }
        self.reg_m = 1;
    }
    pub fn dec_hl(&mut self) {
        self.reg_l = (self.reg_l - 1) & 255;
        if self.reg_l == 255 {
            self.reg_h = (self.reg_h - 1) & 255;
        }
        self.reg_m = 1;
    }
    pub fn dec_sp(&mut self) {
        self.reg_sp = (self.reg_sp - 1) & 65535;
        self.reg_m = 1;
    }

/*
    /*--- Bit manipulation ---*/
    BIT0b: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.b&0x01)?0:0x80; Z80._r.m=2; },
    BIT0c: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.c&0x01)?0:0x80; Z80._r.m=2; },
    BIT0d: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.d&0x01)?0:0x80; Z80._r.m=2; },
    BIT0e: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.e&0x01)?0:0x80; Z80._r.m=2; },
    BIT0h: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.h&0x01)?0:0x80; Z80._r.m=2; },
    BIT0l: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.l&0x01)?0:0x80; Z80._r.m=2; },
    BIT0a: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.a&0x01)?0:0x80; Z80._r.m=2; },
    BIT0m: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(MMU.rb((Z80._r.h<<8)+Z80._r.l)&0x01)?0:0x80; Z80._r.m=3; },

    RES0b: function() { Z80._r.b&=0xFE; Z80._r.m=2; },
    RES0c: function() { Z80._r.c&=0xFE; Z80._r.m=2; },
    RES0d: function() { Z80._r.d&=0xFE; Z80._r.m=2; },
    RES0e: function() { Z80._r.e&=0xFE; Z80._r.m=2; },
    RES0h: function() { Z80._r.h&=0xFE; Z80._r.m=2; },
    RES0l: function() { Z80._r.l&=0xFE; Z80._r.m=2; },
    RES0a: function() { Z80._r.a&=0xFE; Z80._r.m=2; },
    RES0m: function() { var i=MMU.rb((Z80._r.h<<8)+Z80._r.l); i&=0xFE; MMU.wb((Z80._r.h<<8)+Z80._r.l,i); Z80._r.m=4; },

    SET0b: function() { Z80._r.b|=0x01; Z80._r.m=2; },
    SET0c: function() { Z80._r.b|=0x01; Z80._r.m=2; },
    SET0d: function() { Z80._r.b|=0x01; Z80._r.m=2; },
    SET0e: function() { Z80._r.b|=0x01; Z80._r.m=2; },
    SET0h: function() { Z80._r.b|=0x01; Z80._r.m=2; },
    SET0l: function() { Z80._r.b|=0x01; Z80._r.m=2; },
    SET0a: function() { Z80._r.b|=0x01; Z80._r.m=2; },
    SET0m: function() { var i=MMU.rb((Z80._r.h<<8)+Z80._r.l); i|=0x01; MMU.wb((Z80._r.h<<8)+Z80._r.l,i); Z80._r.m=4; },

    BIT1b: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.b&0x02)?0:0x80; Z80._r.m=2; },
    BIT1c: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.c&0x02)?0:0x80; Z80._r.m=2; },
    BIT1d: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.d&0x02)?0:0x80; Z80._r.m=2; },
    BIT1e: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.e&0x02)?0:0x80; Z80._r.m=2; },
    BIT1h: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.h&0x02)?0:0x80; Z80._r.m=2; },
    BIT1l: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.l&0x02)?0:0x80; Z80._r.m=2; },
    BIT1a: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.a&0x02)?0:0x80; Z80._r.m=2; },
    BIT1m: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(MMU.rb((Z80._r.h<<8)+Z80._r.l)&0x02)?0:0x80; Z80._r.m=3; },

    RES1b: function() { Z80._r.b&=0xFD; Z80._r.m=2; },
    RES1c: function() { Z80._r.c&=0xFD; Z80._r.m=2; },
    RES1d: function() { Z80._r.d&=0xFD; Z80._r.m=2; },
    RES1e: function() { Z80._r.e&=0xFD; Z80._r.m=2; },
    RES1h: function() { Z80._r.h&=0xFD; Z80._r.m=2; },
    RES1l: function() { Z80._r.l&=0xFD; Z80._r.m=2; },
    RES1a: function() { Z80._r.a&=0xFD; Z80._r.m=2; },
    RES1m: function() { var i=MMU.rb((Z80._r.h<<8)+Z80._r.l); i&=0xFD; MMU.wb((Z80._r.h<<8)+Z80._r.l,i); Z80._r.m=4; },

    SET1b: function() { Z80._r.b|=0x02; Z80._r.m=2; },
    SET1c: function() { Z80._r.b|=0x02; Z80._r.m=2; },
    SET1d: function() { Z80._r.b|=0x02; Z80._r.m=2; },
    SET1e: function() { Z80._r.b|=0x02; Z80._r.m=2; },
    SET1h: function() { Z80._r.b|=0x02; Z80._r.m=2; },
    SET1l: function() { Z80._r.b|=0x02; Z80._r.m=2; },
    SET1a: function() { Z80._r.b|=0x02; Z80._r.m=2; },
    SET1m: function() { var i=MMU.rb((Z80._r.h<<8)+Z80._r.l); i|=0x02; MMU.wb((Z80._r.h<<8)+Z80._r.l,i); Z80._r.m=4; },

    BIT2b: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.b&0x04)?0:0x80; Z80._r.m=2; },
    BIT2c: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.c&0x04)?0:0x80; Z80._r.m=2; },
    BIT2d: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.d&0x04)?0:0x80; Z80._r.m=2; },
    BIT2e: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.e&0x04)?0:0x80; Z80._r.m=2; },
    BIT2h: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.h&0x04)?0:0x80; Z80._r.m=2; },
    BIT2l: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.l&0x04)?0:0x80; Z80._r.m=2; },
    BIT2a: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.a&0x04)?0:0x80; Z80._r.m=2; },
    BIT2m: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(MMU.rb((Z80._r.h<<8)+Z80._r.l)&0x04)?0:0x80; Z80._r.m=3; },

    RES2b: function() { Z80._r.b&=0xFB; Z80._r.m=2; },
    RES2c: function() { Z80._r.c&=0xFB; Z80._r.m=2; },
    RES2d: function() { Z80._r.d&=0xFB; Z80._r.m=2; },
    RES2e: function() { Z80._r.e&=0xFB; Z80._r.m=2; },
    RES2h: function() { Z80._r.h&=0xFB; Z80._r.m=2; },
    RES2l: function() { Z80._r.l&=0xFB; Z80._r.m=2; },
    RES2a: function() { Z80._r.a&=0xFB; Z80._r.m=2; },
    RES2m: function() { var i=MMU.rb((Z80._r.h<<8)+Z80._r.l); i&=0xFB; MMU.wb((Z80._r.h<<8)+Z80._r.l,i); Z80._r.m=4; },

    SET2b: function() { Z80._r.b|=0x04; Z80._r.m=2; },
    SET2c: function() { Z80._r.b|=0x04; Z80._r.m=2; },
    SET2d: function() { Z80._r.b|=0x04; Z80._r.m=2; },
    SET2e: function() { Z80._r.b|=0x04; Z80._r.m=2; },
    SET2h: function() { Z80._r.b|=0x04; Z80._r.m=2; },
    SET2l: function() { Z80._r.b|=0x04; Z80._r.m=2; },
    SET2a: function() { Z80._r.b|=0x04; Z80._r.m=2; },
    SET2m: function() { var i=MMU.rb((Z80._r.h<<8)+Z80._r.l); i|=0x04; MMU.wb((Z80._r.h<<8)+Z80._r.l,i); Z80._r.m=4; },

    BIT3b: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.b&0x08)?0:0x80; Z80._r.m=2; },
    BIT3c: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.c&0x08)?0:0x80; Z80._r.m=2; },
    BIT3d: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.d&0x08)?0:0x80; Z80._r.m=2; },
    BIT3e: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.e&0x08)?0:0x80; Z80._r.m=2; },
    BIT3h: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.h&0x08)?0:0x80; Z80._r.m=2; },
    BIT3l: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.l&0x08)?0:0x80; Z80._r.m=2; },
    BIT3a: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.a&0x08)?0:0x80; Z80._r.m=2; },
    BIT3m: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(MMU.rb((Z80._r.h<<8)+Z80._r.l)&0x08)?0:0x80; Z80._r.m=3; },

    RES3b: function() { Z80._r.b&=0xF7; Z80._r.m=2; },
    RES3c: function() { Z80._r.c&=0xF7; Z80._r.m=2; },
    RES3d: function() { Z80._r.d&=0xF7; Z80._r.m=2; },
    RES3e: function() { Z80._r.e&=0xF7; Z80._r.m=2; },
    RES3h: function() { Z80._r.h&=0xF7; Z80._r.m=2; },
    RES3l: function() { Z80._r.l&=0xF7; Z80._r.m=2; },
    RES3a: function() { Z80._r.a&=0xF7; Z80._r.m=2; },
    RES3m: function() { var i=MMU.rb((Z80._r.h<<8)+Z80._r.l); i&=0xF7; MMU.wb((Z80._r.h<<8)+Z80._r.l,i); Z80._r.m=4; },

    SET3b: function() { Z80._r.b|=0x08; Z80._r.m=2; },
    SET3c: function() { Z80._r.b|=0x08; Z80._r.m=2; },
    SET3d: function() { Z80._r.b|=0x08; Z80._r.m=2; },
    SET3e: function() { Z80._r.b|=0x08; Z80._r.m=2; },
    SET3h: function() { Z80._r.b|=0x08; Z80._r.m=2; },
    SET3l: function() { Z80._r.b|=0x08; Z80._r.m=2; },
    SET3a: function() { Z80._r.b|=0x08; Z80._r.m=2; },
    SET3m: function() { var i=MMU.rb((Z80._r.h<<8)+Z80._r.l); i|=0x08; MMU.wb((Z80._r.h<<8)+Z80._r.l,i); Z80._r.m=4; },

    BIT4b: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.b&0x10)?0:0x80; Z80._r.m=2; },
    BIT4c: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.c&0x10)?0:0x80; Z80._r.m=2; },
    BIT4d: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.d&0x10)?0:0x80; Z80._r.m=2; },
    BIT4e: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.e&0x10)?0:0x80; Z80._r.m=2; },
    BIT4h: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.h&0x10)?0:0x80; Z80._r.m=2; },
    BIT4l: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.l&0x10)?0:0x80; Z80._r.m=2; },
    BIT4a: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.a&0x10)?0:0x80; Z80._r.m=2; },
    BIT4m: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(MMU.rb((Z80._r.h<<8)+Z80._r.l)&0x10)?0:0x80; Z80._r.m=3; },

    RES4b: function() { Z80._r.b&=0xEF; Z80._r.m=2; },
    RES4c: function() { Z80._r.c&=0xEF; Z80._r.m=2; },
    RES4d: function() { Z80._r.d&=0xEF; Z80._r.m=2; },
    RES4e: function() { Z80._r.e&=0xEF; Z80._r.m=2; },
    RES4h: function() { Z80._r.h&=0xEF; Z80._r.m=2; },
    RES4l: function() { Z80._r.l&=0xEF; Z80._r.m=2; },
    RES4a: function() { Z80._r.a&=0xEF; Z80._r.m=2; },
    RES4m: function() { var i=MMU.rb((Z80._r.h<<8)+Z80._r.l); i&=0xEF; MMU.wb((Z80._r.h<<8)+Z80._r.l,i); Z80._r.m=4; },

    SET4b: function() { Z80._r.b|=0x10; Z80._r.m=2; },
    SET4c: function() { Z80._r.b|=0x10; Z80._r.m=2; },
    SET4d: function() { Z80._r.b|=0x10; Z80._r.m=2; },
    SET4e: function() { Z80._r.b|=0x10; Z80._r.m=2; },
    SET4h: function() { Z80._r.b|=0x10; Z80._r.m=2; },
    SET4l: function() { Z80._r.b|=0x10; Z80._r.m=2; },
    SET4a: function() { Z80._r.b|=0x10; Z80._r.m=2; },
    SET4m: function() { var i=MMU.rb((Z80._r.h<<8)+Z80._r.l); i|=0x10; MMU.wb((Z80._r.h<<8)+Z80._r.l,i); Z80._r.m=4; },

    BIT5b: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.b&0x20)?0:0x80; Z80._r.m=2; },
    BIT5c: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.c&0x20)?0:0x80; Z80._r.m=2; },
    BIT5d: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.d&0x20)?0:0x80; Z80._r.m=2; },
    BIT5e: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.e&0x20)?0:0x80; Z80._r.m=2; },
    BIT5h: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.h&0x20)?0:0x80; Z80._r.m=2; },
    BIT5l: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.l&0x20)?0:0x80; Z80._r.m=2; },
    BIT5a: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.a&0x20)?0:0x80; Z80._r.m=2; },
    BIT5m: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(MMU.rb((Z80._r.h<<8)+Z80._r.l)&0x20)?0:0x80; Z80._r.m=3; },

    RES5b: function() { Z80._r.b&=0xDF; Z80._r.m=2; },
    RES5c: function() { Z80._r.c&=0xDF; Z80._r.m=2; },
    RES5d: function() { Z80._r.d&=0xDF; Z80._r.m=2; },
    RES5e: function() { Z80._r.e&=0xDF; Z80._r.m=2; },
    RES5h: function() { Z80._r.h&=0xDF; Z80._r.m=2; },
    RES5l: function() { Z80._r.l&=0xDF; Z80._r.m=2; },
    RES5a: function() { Z80._r.a&=0xDF; Z80._r.m=2; },
    RES5m: function() { var i=MMU.rb((Z80._r.h<<8)+Z80._r.l); i&=0xDF; MMU.wb((Z80._r.h<<8)+Z80._r.l,i); Z80._r.m=4; },

    SET5b: function() { Z80._r.b|=0x20; Z80._r.m=2; },
    SET5c: function() { Z80._r.b|=0x20; Z80._r.m=2; },
    SET5d: function() { Z80._r.b|=0x20; Z80._r.m=2; },
    SET5e: function() { Z80._r.b|=0x20; Z80._r.m=2; },
    SET5h: function() { Z80._r.b|=0x20; Z80._r.m=2; },
    SET5l: function() { Z80._r.b|=0x20; Z80._r.m=2; },
    SET5a: function() { Z80._r.b|=0x20; Z80._r.m=2; },
    SET5m: function() { var i=MMU.rb((Z80._r.h<<8)+Z80._r.l); i|=0x20; MMU.wb((Z80._r.h<<8)+Z80._r.l,i); Z80._r.m=4; },

    BIT6b: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.b&0x40)?0:0x80; Z80._r.m=2; },
    BIT6c: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.c&0x40)?0:0x80; Z80._r.m=2; },
    BIT6d: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.d&0x40)?0:0x80; Z80._r.m=2; },
    BIT6e: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.e&0x40)?0:0x80; Z80._r.m=2; },
    BIT6h: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.h&0x40)?0:0x80; Z80._r.m=2; },
    BIT6l: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.l&0x40)?0:0x80; Z80._r.m=2; },
    BIT6a: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.a&0x40)?0:0x80; Z80._r.m=2; },
    BIT6m: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(MMU.rb((Z80._r.h<<8)+Z80._r.l)&0x40)?0:0x80; Z80._r.m=3; },

    RES6b: function() { Z80._r.b&=0xBF; Z80._r.m=2; },
    RES6c: function() { Z80._r.c&=0xBF; Z80._r.m=2; },
    RES6d: function() { Z80._r.d&=0xBF; Z80._r.m=2; },
    RES6e: function() { Z80._r.e&=0xBF; Z80._r.m=2; },
    RES6h: function() { Z80._r.h&=0xBF; Z80._r.m=2; },
    RES6l: function() { Z80._r.l&=0xBF; Z80._r.m=2; },
    RES6a: function() { Z80._r.a&=0xBF; Z80._r.m=2; },
    RES6m: function() { var i=MMU.rb((Z80._r.h<<8)+Z80._r.l); i&=0xBF; MMU.wb((Z80._r.h<<8)+Z80._r.l,i); Z80._r.m=4; },

    SET6b: function() { Z80._r.b|=0x40; Z80._r.m=2; },
    SET6c: function() { Z80._r.b|=0x40; Z80._r.m=2; },
    SET6d: function() { Z80._r.b|=0x40; Z80._r.m=2; },
    SET6e: function() { Z80._r.b|=0x40; Z80._r.m=2; },
    SET6h: function() { Z80._r.b|=0x40; Z80._r.m=2; },
    SET6l: function() { Z80._r.b|=0x40; Z80._r.m=2; },
    SET6a: function() { Z80._r.b|=0x40; Z80._r.m=2; },
    SET6m: function() { var i=MMU.rb((Z80._r.h<<8)+Z80._r.l); i|=0x40; MMU.wb((Z80._r.h<<8)+Z80._r.l,i); Z80._r.m=4; },

    BIT7b: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.b&0x80)?0:0x80; Z80._r.m=2; },
    BIT7c: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.c&0x80)?0:0x80; Z80._r.m=2; },
    BIT7d: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.d&0x80)?0:0x80; Z80._r.m=2; },
    BIT7e: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.e&0x80)?0:0x80; Z80._r.m=2; },
    BIT7h: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.h&0x80)?0:0x80; Z80._r.m=2; },
    BIT7l: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.l&0x80)?0:0x80; Z80._r.m=2; },
    BIT7a: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(Z80._r.a&0x80)?0:0x80; Z80._r.m=2; },
    BIT7m: function() { Z80._r.f&=0x1F; Z80._r.f|=0x20; Z80._r.f=(MMU.rb((Z80._r.h<<8)+Z80._r.l)&0x80)?0:0x80; Z80._r.m=3; },

    RES7b: function() { Z80._r.b&=0x7F; Z80._r.m=2; },
    RES7c: function() { Z80._r.c&=0x7F; Z80._r.m=2; },
    RES7d: function() { Z80._r.d&=0x7F; Z80._r.m=2; },
    RES7e: function() { Z80._r.e&=0x7F; Z80._r.m=2; },
    RES7h: function() { Z80._r.h&=0x7F; Z80._r.m=2; },
    RES7l: function() { Z80._r.l&=0x7F; Z80._r.m=2; },
    RES7a: function() { Z80._r.a&=0x7F; Z80._r.m=2; },
    RES7m: function() { var i=MMU.rb((Z80._r.h<<8)+Z80._r.l); i&=0x7F; MMU.wb((Z80._r.h<<8)+Z80._r.l,i); Z80._r.m=4; },

    SET7b: function() { Z80._r.b|=0x80; Z80._r.m=2; },
    SET7c: function() { Z80._r.b|=0x80; Z80._r.m=2; },
    SET7d: function() { Z80._r.b|=0x80; Z80._r.m=2; },
    SET7e: function() { Z80._r.b|=0x80; Z80._r.m=2; },
    SET7h: function() { Z80._r.b|=0x80; Z80._r.m=2; },
    SET7l: function() { Z80._r.b|=0x80; Z80._r.m=2; },
    SET7a: function() { Z80._r.b|=0x80; Z80._r.m=2; },
    SET7m: function() { var i=MMU.rb((Z80._r.h<<8)+Z80._r.l); i|=0x80; MMU.wb((Z80._r.h<<8)+Z80._r.l,i); Z80._r.m=4; },

    RLA: function() { var ci=Z80._r.f&0x10?1:0; var co=Z80._r.a&0x80?0x10:0; Z80._r.a=(Z80._r.a<<1)+ci; Z80._r.a&=255; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=1; },
    RLCA: function() { var ci=Z80._r.a&0x80?1:0; var co=Z80._r.a&0x80?0x10:0; Z80._r.a=(Z80._r.a<<1)+ci; Z80._r.a&=255; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=1; },
    RRA: function() { var ci=Z80._r.f&0x10?0x80:0; var co=Z80._r.a&1?0x10:0; Z80._r.a=(Z80._r.a>>1)+ci; Z80._r.a&=255; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=1; },
    RRCA: function() { var ci=Z80._r.a&1?0x80:0; var co=Z80._r.a&1?0x10:0; Z80._r.a=(Z80._r.a>>1)+ci; Z80._r.a&=255; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=1; },

    RLr_b: function() { var ci=Z80._r.f&0x10?1:0; var co=Z80._r.b&0x80?0x10:0; Z80._r.b=(Z80._r.b<<1)+ci; Z80._r.b&=255; Z80._r.f=(Z80._r.b)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    RLr_c: function() { var ci=Z80._r.f&0x10?1:0; var co=Z80._r.c&0x80?0x10:0; Z80._r.c=(Z80._r.c<<1)+ci; Z80._r.c&=255; Z80._r.f=(Z80._r.c)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    RLr_d: function() { var ci=Z80._r.f&0x10?1:0; var co=Z80._r.d&0x80?0x10:0; Z80._r.d=(Z80._r.d<<1)+ci; Z80._r.d&=255; Z80._r.f=(Z80._r.d)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    RLr_e: function() { var ci=Z80._r.f&0x10?1:0; var co=Z80._r.e&0x80?0x10:0; Z80._r.e=(Z80._r.e<<1)+ci; Z80._r.e&=255; Z80._r.f=(Z80._r.e)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    RLr_h: function() { var ci=Z80._r.f&0x10?1:0; var co=Z80._r.h&0x80?0x10:0; Z80._r.h=(Z80._r.h<<1)+ci; Z80._r.h&=255; Z80._r.f=(Z80._r.h)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    RLr_l: function() { var ci=Z80._r.f&0x10?1:0; var co=Z80._r.l&0x80?0x10:0; Z80._r.l=(Z80._r.l<<1)+ci; Z80._r.l&=255; Z80._r.f=(Z80._r.l)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    RLr_a: function() { var ci=Z80._r.f&0x10?1:0; var co=Z80._r.a&0x80?0x10:0; Z80._r.a=(Z80._r.a<<1)+ci; Z80._r.a&=255; Z80._r.f=(Z80._r.a)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    RLHL: function() { var i=MMU.rb((Z80._r.h<<8)+Z80._r.l); var ci=Z80._r.f&0x10?1:0; var co=i&0x80?0x10:0; i=(i<<1)+ci; i&=255; Z80._r.f=(i)?0:0x80; MMU.wb((Z80._r.h<<8)+Z80._r.l,i); Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=4; },

    RLCr_b: function() { var ci=Z80._r.b&0x80?1:0; var co=Z80._r.b&0x80?0x10:0; Z80._r.b=(Z80._r.b<<1)+ci; Z80._r.b&=255; Z80._r.f=(Z80._r.b)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    RLCr_c: function() { var ci=Z80._r.c&0x80?1:0; var co=Z80._r.c&0x80?0x10:0; Z80._r.c=(Z80._r.c<<1)+ci; Z80._r.c&=255; Z80._r.f=(Z80._r.c)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    RLCr_d: function() { var ci=Z80._r.d&0x80?1:0; var co=Z80._r.d&0x80?0x10:0; Z80._r.d=(Z80._r.d<<1)+ci; Z80._r.d&=255; Z80._r.f=(Z80._r.d)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    RLCr_e: function() { var ci=Z80._r.e&0x80?1:0; var co=Z80._r.e&0x80?0x10:0; Z80._r.e=(Z80._r.e<<1)+ci; Z80._r.e&=255; Z80._r.f=(Z80._r.e)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    RLCr_h: function() { var ci=Z80._r.h&0x80?1:0; var co=Z80._r.h&0x80?0x10:0; Z80._r.h=(Z80._r.h<<1)+ci; Z80._r.h&=255; Z80._r.f=(Z80._r.h)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    RLCr_l: function() { var ci=Z80._r.l&0x80?1:0; var co=Z80._r.l&0x80?0x10:0; Z80._r.l=(Z80._r.l<<1)+ci; Z80._r.l&=255; Z80._r.f=(Z80._r.l)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    RLCr_a: function() { var ci=Z80._r.a&0x80?1:0; var co=Z80._r.a&0x80?0x10:0; Z80._r.a=(Z80._r.a<<1)+ci; Z80._r.a&=255; Z80._r.f=(Z80._r.a)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    RLCHL: function() { var i=MMU.rb((Z80._r.h<<8)+Z80._r.l); var ci=i&0x80?1:0; var co=i&0x80?0x10:0; i=(i<<1)+ci; i&=255; Z80._r.f=(i)?0:0x80; MMU.wb((Z80._r.h<<8)+Z80._r.l,i); Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=4; },

    RRr_b: function() { var ci=Z80._r.f&0x10?0x80:0; var co=Z80._r.b&1?0x10:0; Z80._r.b=(Z80._r.b>>1)+ci; Z80._r.b&=255; Z80._r.f=(Z80._r.b)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    RRr_c: function() { var ci=Z80._r.f&0x10?0x80:0; var co=Z80._r.c&1?0x10:0; Z80._r.c=(Z80._r.c>>1)+ci; Z80._r.c&=255; Z80._r.f=(Z80._r.c)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    RRr_d: function() { var ci=Z80._r.f&0x10?0x80:0; var co=Z80._r.d&1?0x10:0; Z80._r.d=(Z80._r.d>>1)+ci; Z80._r.d&=255; Z80._r.f=(Z80._r.d)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    RRr_e: function() { var ci=Z80._r.f&0x10?0x80:0; var co=Z80._r.e&1?0x10:0; Z80._r.e=(Z80._r.e>>1)+ci; Z80._r.e&=255; Z80._r.f=(Z80._r.e)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    RRr_h: function() { var ci=Z80._r.f&0x10?0x80:0; var co=Z80._r.h&1?0x10:0; Z80._r.h=(Z80._r.h>>1)+ci; Z80._r.h&=255; Z80._r.f=(Z80._r.h)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    RRr_l: function() { var ci=Z80._r.f&0x10?0x80:0; var co=Z80._r.l&1?0x10:0; Z80._r.l=(Z80._r.l>>1)+ci; Z80._r.l&=255; Z80._r.f=(Z80._r.l)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    RRr_a: function() { var ci=Z80._r.f&0x10?0x80:0; var co=Z80._r.a&1?0x10:0; Z80._r.a=(Z80._r.a>>1)+ci; Z80._r.a&=255; Z80._r.f=(Z80._r.a)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    RRHL: function() { var i=MMU.rb((Z80._r.h<<8)+Z80._r.l); var ci=Z80._r.f&0x10?0x80:0; var co=i&1?0x10:0; i=(i>>1)+ci; i&=255; MMU.wb((Z80._r.h<<8)+Z80._r.l,i); Z80._r.f=(i)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=4; },

    RRCr_b: function() { var ci=Z80._r.b&1?0x80:0; var co=Z80._r.b&1?0x10:0; Z80._r.b=(Z80._r.b>>1)+ci; Z80._r.b&=255; Z80._r.f=(Z80._r.b)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    RRCr_c: function() { var ci=Z80._r.c&1?0x80:0; var co=Z80._r.c&1?0x10:0; Z80._r.c=(Z80._r.c>>1)+ci; Z80._r.c&=255; Z80._r.f=(Z80._r.c)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    RRCr_d: function() { var ci=Z80._r.d&1?0x80:0; var co=Z80._r.d&1?0x10:0; Z80._r.d=(Z80._r.d>>1)+ci; Z80._r.d&=255; Z80._r.f=(Z80._r.d)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    RRCr_e: function() { var ci=Z80._r.e&1?0x80:0; var co=Z80._r.e&1?0x10:0; Z80._r.e=(Z80._r.e>>1)+ci; Z80._r.e&=255; Z80._r.f=(Z80._r.e)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    RRCr_h: function() { var ci=Z80._r.h&1?0x80:0; var co=Z80._r.h&1?0x10:0; Z80._r.h=(Z80._r.h>>1)+ci; Z80._r.h&=255; Z80._r.f=(Z80._r.h)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    RRCr_l: function() { var ci=Z80._r.l&1?0x80:0; var co=Z80._r.l&1?0x10:0; Z80._r.l=(Z80._r.l>>1)+ci; Z80._r.l&=255; Z80._r.f=(Z80._r.l)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    RRCr_a: function() { var ci=Z80._r.a&1?0x80:0; var co=Z80._r.a&1?0x10:0; Z80._r.a=(Z80._r.a>>1)+ci; Z80._r.a&=255; Z80._r.f=(Z80._r.a)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    RRCHL: function() { var i=MMU.rb((Z80._r.h<<8)+Z80._r.l); var ci=i&1?0x80:0; var co=i&1?0x10:0; i=(i>>1)+ci; i&=255; MMU.wb((Z80._r.h<<8)+Z80._r.l,i); Z80._r.f=(i)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=4; },

    SLAr_b: function() { var co=Z80._r.b&0x80?0x10:0; Z80._r.b=(Z80._r.b<<1)&255; Z80._r.f=(Z80._r.b)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    SLAr_c: function() { var co=Z80._r.c&0x80?0x10:0; Z80._r.c=(Z80._r.c<<1)&255; Z80._r.f=(Z80._r.c)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    SLAr_d: function() { var co=Z80._r.d&0x80?0x10:0; Z80._r.d=(Z80._r.d<<1)&255; Z80._r.f=(Z80._r.d)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    SLAr_e: function() { var co=Z80._r.e&0x80?0x10:0; Z80._r.e=(Z80._r.e<<1)&255; Z80._r.f=(Z80._r.e)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    SLAr_h: function() { var co=Z80._r.h&0x80?0x10:0; Z80._r.h=(Z80._r.h<<1)&255; Z80._r.f=(Z80._r.h)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    SLAr_l: function() { var co=Z80._r.l&0x80?0x10:0; Z80._r.l=(Z80._r.l<<1)&255; Z80._r.f=(Z80._r.l)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    SLAr_a: function() { var co=Z80._r.a&0x80?0x10:0; Z80._r.a=(Z80._r.a<<1)&255; Z80._r.f=(Z80._r.a)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },

    SLLr_b: function() { var co=Z80._r.b&0x80?0x10:0; Z80._r.b=(Z80._r.b<<1)&255+1; Z80._r.f=(Z80._r.b)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    SLLr_c: function() { var co=Z80._r.c&0x80?0x10:0; Z80._r.c=(Z80._r.c<<1)&255+1; Z80._r.f=(Z80._r.c)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    SLLr_d: function() { var co=Z80._r.d&0x80?0x10:0; Z80._r.d=(Z80._r.d<<1)&255+1; Z80._r.f=(Z80._r.d)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    SLLr_e: function() { var co=Z80._r.e&0x80?0x10:0; Z80._r.e=(Z80._r.e<<1)&255+1; Z80._r.f=(Z80._r.e)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    SLLr_h: function() { var co=Z80._r.h&0x80?0x10:0; Z80._r.h=(Z80._r.h<<1)&255+1; Z80._r.f=(Z80._r.h)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    SLLr_l: function() { var co=Z80._r.l&0x80?0x10:0; Z80._r.l=(Z80._r.l<<1)&255+1; Z80._r.f=(Z80._r.l)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    SLLr_a: function() { var co=Z80._r.a&0x80?0x10:0; Z80._r.a=(Z80._r.a<<1)&255+1; Z80._r.f=(Z80._r.a)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },

    SRAr_b: function() { var ci=Z80._r.b&0x80; var co=Z80._r.b&1?0x10:0; Z80._r.b=((Z80._r.b>>1)+ci)&255; Z80._r.f=(Z80._r.b)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    SRAr_c: function() { var ci=Z80._r.c&0x80; var co=Z80._r.c&1?0x10:0; Z80._r.c=((Z80._r.c>>1)+ci)&255; Z80._r.f=(Z80._r.c)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    SRAr_d: function() { var ci=Z80._r.d&0x80; var co=Z80._r.d&1?0x10:0; Z80._r.d=((Z80._r.d>>1)+ci)&255; Z80._r.f=(Z80._r.d)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    SRAr_e: function() { var ci=Z80._r.e&0x80; var co=Z80._r.e&1?0x10:0; Z80._r.e=((Z80._r.e>>1)+ci)&255; Z80._r.f=(Z80._r.e)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    SRAr_h: function() { var ci=Z80._r.h&0x80; var co=Z80._r.h&1?0x10:0; Z80._r.h=((Z80._r.h>>1)+ci)&255; Z80._r.f=(Z80._r.h)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    SRAr_l: function() { var ci=Z80._r.l&0x80; var co=Z80._r.l&1?0x10:0; Z80._r.l=((Z80._r.l>>1)+ci)&255; Z80._r.f=(Z80._r.l)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    SRAr_a: function() { var ci=Z80._r.a&0x80; var co=Z80._r.a&1?0x10:0; Z80._r.a=((Z80._r.a>>1)+ci)&255; Z80._r.f=(Z80._r.a)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },

    SRLr_b: function() { var co=Z80._r.b&1?0x10:0; Z80._r.b=(Z80._r.b>>1)&255; Z80._r.f=(Z80._r.b)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    SRLr_c: function() { var co=Z80._r.c&1?0x10:0; Z80._r.c=(Z80._r.c>>1)&255; Z80._r.f=(Z80._r.c)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    SRLr_d: function() { var co=Z80._r.d&1?0x10:0; Z80._r.d=(Z80._r.d>>1)&255; Z80._r.f=(Z80._r.d)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    SRLr_e: function() { var co=Z80._r.e&1?0x10:0; Z80._r.e=(Z80._r.e>>1)&255; Z80._r.f=(Z80._r.e)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    SRLr_h: function() { var co=Z80._r.h&1?0x10:0; Z80._r.h=(Z80._r.h>>1)&255; Z80._r.f=(Z80._r.h)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    SRLr_l: function() { var co=Z80._r.l&1?0x10:0; Z80._r.l=(Z80._r.l>>1)&255; Z80._r.f=(Z80._r.l)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },
    SRLr_a: function() { var co=Z80._r.a&1?0x10:0; Z80._r.a=(Z80._r.a>>1)&255; Z80._r.f=(Z80._r.a)?0:0x80; Z80._r.f=(Z80._r.f&0xEF)+co; Z80._r.m=2; },

    CPL: function() { Z80._r.a ^= 255; Z80._r.f=Z80._r.a?0:0x80; Z80._r.m=1; },
    NEG: function() { Z80._r.a=0-Z80._r.a; Z80._r.f=(Z80._r.a<0)?0x10:0; Z80._r.a&=255; if(!Z80._r.a) Z80._r.f|=0x80; Z80._r.m=2; },

    CCF: function() { var ci=Z80._r.f&0x10?0:0x10; Z80._r.f=(Z80._r.f&0xEF)+ci; Z80._r.m=1; },
    SCF: function() { Z80._r.f|=0x10; Z80._r.m=1; },

    /*--- Stack ---*/
    PUSHBC: function() { Z80._r.sp--; MMU.wb(Z80._r.sp,Z80._r.b); Z80._r.sp--; MMU.wb(Z80._r.sp,Z80._r.c); Z80._r.m=3; },
    PUSHDE: function() { Z80._r.sp--; MMU.wb(Z80._r.sp,Z80._r.d); Z80._r.sp--; MMU.wb(Z80._r.sp,Z80._r.e); Z80._r.m=3; },
    PUSHHL: function() { Z80._r.sp--; MMU.wb(Z80._r.sp,Z80._r.h); Z80._r.sp--; MMU.wb(Z80._r.sp,Z80._r.l); Z80._r.m=3; },
    PUSHAF: function() { Z80._r.sp--; MMU.wb(Z80._r.sp,Z80._r.a); Z80._r.sp--; MMU.wb(Z80._r.sp,Z80._r.f); Z80._r.m=3; },

    POPBC: function() { Z80._r.c=MMU.rb(Z80._r.sp); Z80._r.sp++; Z80._r.b=MMU.rb(Z80._r.sp); Z80._r.sp++; Z80._r.m=3; },
    POPDE: function() { Z80._r.e=MMU.rb(Z80._r.sp); Z80._r.sp++; Z80._r.d=MMU.rb(Z80._r.sp); Z80._r.sp++; Z80._r.m=3; },
    POPHL: function() { Z80._r.l=MMU.rb(Z80._r.sp); Z80._r.sp++; Z80._r.h=MMU.rb(Z80._r.sp); Z80._r.sp++; Z80._r.m=3; },
    POPAF: function() { Z80._r.f=MMU.rb(Z80._r.sp); Z80._r.sp++; Z80._r.a=MMU.rb(Z80._r.sp); Z80._r.sp++; Z80._r.m=3; },
*/

    /*--- Jump ---*/
    pub fn jp_nn(&mut self) {
        self.reg_pc = self.mmu.borrow_mut().rw(&self, self.reg_pc);
        self.reg_m = 3;
    }
    pub fn jp_hl(&mut self) {
        self.reg_pc = (self.reg_h as u16) << 8 + self.reg_l as u16;
        self.reg_m = 1;
    }
    pub fn jp_nz_nn(&mut self) {
        self.reg_m = 3;
        if (self.reg_f & 0x80) == 0x00 {
            self.reg_pc = self.mmu.borrow_mut().rw(&self, self.reg_pc);
            self.reg_m += 1;
        } else {
            self.reg_pc += 2;
        }
    }
    pub fn jp_z_nn(&mut self) {
        self.reg_m = 3;
        if (self.reg_f & 0x80) == 0x80 {
            self.reg_pc = self.mmu.borrow_mut().rw(&self, self.reg_pc);
            self.reg_m += 1;
        } else {
            self.reg_pc += 2;
        }
    }
    pub fn jp_nc_nn(&mut self) {
        self.reg_m = 3;
        if (self.reg_f & 0x10) == 0x00 {
            self.reg_pc = self.mmu.borrow_mut().rw(&self, self.reg_pc);
            self.reg_m += 1;
        } else {
            self.reg_pc += 2;
        }
    }
    pub fn jp_c_nn(&mut self) {
        self.reg_m = 3;
        if (self.reg_f & 0x10) == 0x10 {
            self.reg_pc = self.mmu.borrow_mut().rw(&self, self.reg_pc);
            self.reg_m += 1;
        } else {
            self.reg_pc += 2;
        }
    }

    /*
    JRn: function() { var i=MMU.rb(Z80._r.pc); if(i>127) i=-((~i+1)&255); Z80._r.pc++; Z80._r.m=2; Z80._r.pc+=i; Z80._r.m++; },
    JRNZn: function() { var i=MMU.rb(Z80._r.pc); if(i>127) i=-((~i+1)&255); Z80._r.pc++; Z80._r.m=2; if((Z80._r.f&0x80)==0x00) { Z80._r.pc+=i; Z80._r.m++; } },
    JRZn: function()  { var i=MMU.rb(Z80._r.pc); if(i>127) i=-((~i+1)&255); Z80._r.pc++; Z80._r.m=2; if((Z80._r.f&0x80)==0x80) { Z80._r.pc+=i; Z80._r.m++; } },
    JRNCn: function() { var i=MMU.rb(Z80._r.pc); if(i>127) i=-((~i+1)&255); Z80._r.pc++; Z80._r.m=2; if((Z80._r.f&0x10)==0x00) { Z80._r.pc+=i; Z80._r.m++; } },
    JRCn: function()  { var i=MMU.rb(Z80._r.pc); if(i>127) i=-((~i+1)&255); Z80._r.pc++; Z80._r.m=2; if((Z80._r.f&0x10)==0x10) { Z80._r.pc+=i; Z80._r.m++; } },

    DJNZn: function() { var i=MMU.rb(Z80._r.pc); if(i>127) i=-((~i+1)&255); Z80._r.pc++; Z80._r.m=2; Z80._r.b--; if(Z80._r.b) { Z80._r.pc+=i; Z80._r.m++; } },
    */

    pub fn call_nn(&mut self) {
        self.reg_sp -= 2;
        self.mmu.borrow_mut().ww(&self, self.reg_sp, self.reg_pc + 2);
        self.reg_pc = self.mmu.borrow_mut().rw(&self, self.reg_pc);
        self.reg_m = 5;
    }
    pub fn call_NZnn(&mut self) {
        self.reg_m = 3;
        if (self.reg_f & 0x80) == 0x00 {
            self.reg_sp -= 2;
            self.mmu.borrow_mut().ww(&self, self.reg_sp, self.reg_pc + 2);
            self.reg_pc = self.mmu.borrow_mut().rw(&self, self.reg_pc);
            self.reg_m += 2;
        } else {
            self.reg_pc += 2;
        }
    }
    pub fn call_Znn(&mut self) {
        self.reg_m = 3;
        if (self.reg_f & 0x80) == 0x80 {
            self.reg_sp -= 2;
            self.mmu.borrow_mut().ww(&self, self.reg_sp, self.reg_pc + 2);
            self.reg_pc = self.mmu.borrow_mut().rw(&self, self.reg_pc);
            self.reg_m += 2;
        } else {
            self.reg_pc += 2;
        }
    }
    pub fn call_NCnn(&mut self) {
        self.reg_m = 3;
        if (self.reg_f & 0x10) == 0x00 {
            self.reg_sp -= 2;
            self.mmu.borrow_mut().ww(&self, self.reg_sp, self.reg_pc + 2);
            self.reg_pc = self.mmu.borrow_mut().rw(&self, self.reg_pc);
            self.reg_m += 2;
        } else {
            self.reg_pc += 2;
        }
    }
    pub fn call_Cnn(&mut self) {
        self.reg_m = 3;
        if (self.reg_f & 0x10) == 0x10 {
            self.reg_sp -= 2;
            self.mmu.borrow_mut().ww(&self, self.reg_sp, self.reg_pc + 2);
            self.reg_pc = self.mmu.borrow_mut().rw(&self, self.reg_pc);
            self.reg_m += 2;
        } else {
            self.reg_pc += 2;
        }
    }

// Return operations
    pub fn ret(&mut self) {
        self.reg_pc = self.mmu.borrow_mut().rw(&self, self.reg_sp);
        self.reg_sp += 2;
        self.reg_m = 3;
    }
    /*
    pub fn ret_i(&mut self) {
        self.reg_ime = 1;
        Z80._ops.rrs(); //TODO:
        self.reg_pc = self.mmu.borrow_mut().rw(&self, self.reg_sp);
        self.reg_sp += 2;
        self.reg_m = 3;
    }
    */
    pub fn ret_nz(&mut self) {
        self.reg_m = 1;
        if (self.reg_f & 0x80) == 0x00 {
            self.reg_pc = self.mmu.borrow_mut().rw(&self, self.reg_sp);
            self.reg_sp += 2;
            self.reg_m += 2;
        }
    }
    pub fn ret_z(&mut self) {
        self.reg_m = 1;
        if (self.reg_f & 0x80) == 0x80 {
            self.reg_pc = self.mmu.borrow_mut().rw(&self, self.reg_sp);
            self.reg_sp += 2;
            self.reg_m += 2;
        }
    }
    pub fn ret_nc(&mut self) {
        self.reg_m = 1;
        if (self.reg_f & 0x10) == 0x00 {
            self.reg_pc = self.mmu.borrow_mut().rw(&self, self.reg_sp);
            self.reg_sp += 2;
            self.reg_m += 2;
        }
    }
    pub fn ret_c(&mut self) {
        self.reg_m = 1;
        if (self.reg_f & 0x10) == 0x10 {
            self.reg_pc = self.mmu.borrow_mut().rw(&self, self.reg_sp);
            self.reg_sp += 2;
            self.reg_m += 2;
        }
    }

    /*
     * TODO: I dunno what the _ops.rsv() is supposed to do...
    /// (RST 0x00): All routine at 0x0000
    pub fn rst_00(&mut self) { Z80._ops.rsv(); self.reg_sp -= 2; self.mmu.borrow_mut().ww(&self, self.reg_sp, self.reg_pc); self.reg_pc = 0x00; self.reg_m = 3; }
    pub fn rst_08(&mut self) { Z80._ops.rsv(); self.reg_sp -= 2; self.mmu.borrow_mut().ww(&self, self.reg_sp, self.reg_pc); self.reg_pc = 0x08; self.reg_m = 3; }
    pub fn rst_10(&mut self) { Z80._ops.rsv(); self.reg_sp -= 2; self.mmu.borrow_mut().ww(&self, self.reg_sp, self.reg_pc); self.reg_pc = 0x10; self.reg_m = 3; }
    pub fn rst_18(&mut self) { Z80._ops.rsv(); self.reg_sp -= 2; self.mmu.borrow_mut().ww(&self, self.reg_sp, self.reg_pc); self.reg_pc = 0x18; self.reg_m = 3; }
    pub fn rst_20(&mut self) { Z80._ops.rsv(); self.reg_sp -= 2; self.mmu.borrow_mut().ww(&self, self.reg_sp, self.reg_pc); self.reg_pc = 0x20; self.reg_m = 3; }
    pub fn rst_28(&mut self) { Z80._ops.rsv(); self.reg_sp -= 2; self.mmu.borrow_mut().ww(&self, self.reg_sp, self.reg_pc); self.reg_pc = 0x28; self.reg_m = 3; }
    pub fn rst_30(&mut self) { Z80._ops.rsv(); self.reg_sp -= 2; self.mmu.borrow_mut().ww(&self, self.reg_sp, self.reg_pc); self.reg_pc = 0x30; self.reg_m = 3; }
    pub fn rst_38(&mut self) { Z80._ops.rsv(); self.reg_sp -= 2; self.mmu.borrow_mut().ww(&self, self.reg_sp, self.reg_pc); self.reg_pc = 0x38; self.reg_m = 3; }
    pub fn rst_40(&mut self) { Z80._ops.rsv(); self.reg_sp -= 2; self.mmu.borrow_mut().ww(&self, self.reg_sp, self.reg_pc); self.reg_pc = 0x40; self.reg_m = 3; }
    pub fn rst_48(&mut self) { Z80._ops.rsv(); self.reg_sp -= 2; self.mmu.borrow_mut().ww(&self, self.reg_sp, self.reg_pc); self.reg_pc = 0x48; self.reg_m = 3; }
    pub fn rst_50(&mut self) { Z80._ops.rsv(); self.reg_sp -= 2; self.mmu.borrow_mut().ww(&self, self.reg_sp, self.reg_pc); self.reg_pc = 0x50; self.reg_m = 3; }
    pub fn rst_58(&mut self) { Z80._ops.rsv(); self.reg_sp -= 2; self.mmu.borrow_mut().ww(&self, self.reg_sp, self.reg_pc); self.reg_pc = 0x58; self.reg_m = 3; }
    pub fn rst_60(&mut self) { Z80._ops.rsv(); self.reg_sp -= 2; self.mmu.borrow_mut().ww(&self, self.reg_sp, self.reg_pc); self.reg_pc = 0x60; self.reg_m = 3; }
    */

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
    Z80::add_e,
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

