#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::cell::RefCell;
use std::rc::Rc;
use mmu::MMU;

// The Nintendo documents describe the CPU & instructions speed in machine
// cycles; while this document will be describing them in clock cycles. Here is
// the conversion between the two:
//                | GB CPU Speed | NOP Instruction
// -------------------------------------------------
// Machine Cycles |  1.05 MHz    |  1 Cycle
// Clock Cycles   |  4.19 MHz    |  4 Cycles


// Register types
// NOTE: C0 is really (0xFF00 + C) == (C)
#[derive(Debug)]
pub enum Register {
    // 8-bit registers
    A, B, D, H, F, C, C0, E, L,
    // 16-bit registers
    AF, BC, DE, HL, SP,
    // 16-bit Value
    BYTE,//Raw8,
    WORD,//Raw16
}


// Instruction List
#[derive(Debug)]
pub enum Instruction {
    ExtInstr, // Extended Instruction Set use byte (0xCB)
    LD   (Register, Register),
    LDD  (Register, Register),
    LDI  (Register, Register),
    LDH  (Register, Register),
    LDHL (Register, Register),
    PUSH (Register),
    POP  (Register),
    ADD  (Register, Register),
    ADC  (Register, Register),
    SUB  (Register, Register),
    SBC  (Register, Register),
    AND  (Register),
    OR   (Register),
    XOR  (Register),
    CP   (Register),
    INC  (Register),
    DEC  (Register),
    DAA,
    CPL,
    CCF,
    SCF,
    NOP,
    HALT,
    STOP,
    DI,
    EI,
    RLCA,
    RLA,
    RRCA,
    RRA,
    JP   (Register),
    JPNZ (Register),
    JPZ  (Register),
    JPNC (Register),
    JPC  (Register),
    JPHL,
    JR   (Register),
    JRNZ (Register),
    JRZ  (Register),
    JRNC (Register),
    JRC  (Register),
    CALL (Register),
    CALLNZ (Register),
    CALLZ  (Register),
    CALLNC (Register),
    CALLC  (Register),
    RST (u8),
    RET,
    RETNZ (Register),
    RETZ  (Register),
    RETNC (Register),
    RETC  (Register),
    RETI,
    // extended instructions
    SWAP (Register),
    RLC  (Register),
    RL   (Register),
    RRC  (Register),
    RR   (Register),
    SLA  (Register),
    SRA  (Register),
    SRL  (Register),
    BIT  (Register, Register),
    SET  (Register, Register),
    RES  (Register, Register),
}

pub struct CPU {
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

impl CPU {
    pub fn new() -> CPU {
        CPU {
            halt:    false, stop:    false,
            clock_m: 0.0,   clock_t: 0.0,
            reg_a:   0,     reg_b:   0,
            reg_c:   0,     reg_d:   0,
            reg_e:   0,     reg_h:   0,
            reg_l:   0,     reg_f:   0,
            reg_pc:  0,     reg_sp:  0,
            reg_m:   0,     reg_t:   0,
            reg_ime: 0,
        }
    }

    /// Dispatches an instruction
    pub fn decode(byte: u8) -> Instruction {
        use self::Instruction as I;
        use self::Register::BYTE as BYTE;
        use self::Register::WORD as WORD;
        use self::Register as Reg;

        match byte {
            // 8-Bit loads ----------------------------------------------------
            // LD nn, n : put value of `nn` into `n` (8 Bit val)
            0x06 => I::LD(Reg::B, BYTE),
            0x0E => I::LD(Reg::C, BYTE),
            0x16 => I::LD(Reg::D, BYTE),
            0x1E => I::LD(Reg::E, BYTE),
            0x26 => I::LD(Reg::H, BYTE),
            0x2E => I::LD(Reg::L, BYTE),
            // LD r1, r1 : put value r2 into r1
            // -- (A -> n)
            0x78 => I::LD(Reg::A, Reg::B),
            0x79 => I::LD(Reg::A, Reg::C),
            0x7A => I::LD(Reg::A, Reg::D),
            0x7B => I::LD(Reg::A, Reg::E),
            0x7C => I::LD(Reg::A, Reg::H),
            0x7D => I::LD(Reg::A, Reg::L),
            0x7E => I::LD(Reg::A, Reg::HL),
            // -- (B -> n)
            0x40 => I::LD(Reg::B, Reg::B),
            0x41 => I::LD(Reg::B, Reg::C),
            0x42 => I::LD(Reg::B, Reg::D),
            0x43 => I::LD(Reg::B, Reg::E),
            0x44 => I::LD(Reg::B, Reg::H),
            0x45 => I::LD(Reg::B, Reg::L),
            0x46 => I::LD(Reg::B, Reg::HL),
            // -- (C -> n)
            0x48 => I::LD(Reg::C, Reg::B),
            0x49 => I::LD(Reg::C, Reg::C),
            0x4A => I::LD(Reg::C, Reg::D),
            0x4B => I::LD(Reg::C, Reg::E),
            0x4C => I::LD(Reg::C, Reg::H),
            0x4D => I::LD(Reg::C, Reg::L),
            0x4E => I::LD(Reg::C, Reg::HL),
            // -- (D -> n)
            0x50 => I::LD(Reg::D, Reg::B),
            0x51 => I::LD(Reg::D, Reg::C),
            0x52 => I::LD(Reg::D, Reg::D),
            0x53 => I::LD(Reg::D, Reg::E),
            0x54 => I::LD(Reg::D, Reg::H),
            0x55 => I::LD(Reg::D, Reg::L),
            0x56 => I::LD(Reg::D, Reg::HL),
            // -- (E -> n)
            0x58 => I::LD(Reg::E, Reg::B),
            0x59 => I::LD(Reg::E, Reg::C),
            0x5A => I::LD(Reg::E, Reg::D),
            0x5B => I::LD(Reg::E, Reg::E),
            0x5C => I::LD(Reg::E, Reg::H),
            0x5D => I::LD(Reg::E, Reg::L),
            0x5E => I::LD(Reg::E, Reg::HL),
            // -- (H -> n)
            0x60 => I::LD(Reg::D, Reg::B),
            0x61 => I::LD(Reg::D, Reg::C),
            0x62 => I::LD(Reg::D, Reg::D),
            0x63 => I::LD(Reg::D, Reg::E),
            0x64 => I::LD(Reg::D, Reg::H),
            0x65 => I::LD(Reg::D, Reg::L),
            0x66 => I::LD(Reg::D, Reg::HL),
            // -- (L -> n)
            0x68 => I::LD(Reg::L, Reg::B),
            0x69 => I::LD(Reg::L, Reg::C),
            0x6A => I::LD(Reg::L, Reg::D),
            0x6B => I::LD(Reg::L, Reg::E),
            0x6C => I::LD(Reg::L, Reg::H),
            0x6D => I::LD(Reg::L, Reg::L),
            0x6E => I::LD(Reg::L, Reg::HL),
            // -- (HL -> n)
            0x70 => I::LD(Reg::HL, Reg::B),
            0x71 => I::LD(Reg::HL, Reg::C),
            0x72 => I::LD(Reg::HL, Reg::D),
            0x73 => I::LD(Reg::HL, Reg::E),
            0x74 => I::LD(Reg::HL, Reg::H),
            0x75 => I::LD(Reg::HL, Reg::L),
            0x36 => I::LD(Reg::HL, BYTE),
            // LD A, n : put value of `n` into `A`
            0x7F => I::LD(Reg::A, Reg::A),
            0x0A => I::LD(Reg::A, Reg::BC),
            0x1A => I::LD(Reg::A, Reg::DE),
            0xFA => I::LD(Reg::A, WORD), // LS byte first
            0x3E => I::LD(Reg::A, BYTE),
            // LD n, A : put value of `A` into `n`
            0x47 => I::LD(Reg::B,  Reg::A),
            0x4F => I::LD(Reg::C,  Reg::A),
            0x57 => I::LD(Reg::D,  Reg::A),
            0x5F => I::LD(Reg::E,  Reg::A),
            0x67 => I::LD(Reg::H,  Reg::A),
            0x6F => I::LD(Reg::L,  Reg::A),
            0x02 => I::LD(Reg::BC, Reg::A),
            0x12 => I::LD(Reg::DE, Reg::A),
            0x77 => I::LD(Reg::HL, Reg::A),
            0xEA => I::LD(BYTE, Reg::A),
            // LD A, (C) : put value at 0xFF00 + Reg::C into A
            //             same as LD A, (0xFF00 + C)
            0xF2 => I::LD(Reg::A, Reg::C0),
            // LD A, (C) : put value at 0xFF00 + Reg::C into A
            // Same as: (0xFF00 + C), A
            0xE2 => I::LD(Reg::C0, Reg::A),
            // LDD A, (HL) : Put value at addr of HL into A, decrement HL
            // Same as : LD A, (HLD) ; LD A, (HL-)
            0x3A => I::LDD(Reg::A, Reg::HL),
            // LDD (HL), A : Put A into memory at HL, decrement HL
            // Same as : LD (HLD), A ; LD (HL-), A
            0x32 => I::LDD(Reg::HL, Reg::A),
            // LDI A, (HL) : Put value at addr of HL into A, increment HL
            // Same as : LD A, (HLI) ; LD A, (HL+)
            0x2A => I::LDI(Reg::A, Reg::HL),
            // LDI (HL), A : Put A into memory at HL, increment HL
            // Same as : LD (HLI), A ; LD (HL+), A
            0x22 => I::LDI(Reg::HL, Reg::A),
            // LDH (n), A :
            0xE0 => I::LDH(BYTE, Reg::A),
            // LDH A, (n) : but memory address (0xFF00 + n) into A
            0xF0 => I::LDH(Reg::A, BYTE),
            // 16-Bit loads ---------------------------------------------------
            // lD n, nn : Put value `nn` into `n`
            0x01 => I::LD(Reg::BC, WORD),
            0x11 => I::LD(Reg::DE, WORD),
            0x21 => I::LD(Reg::HL, WORD),
            0x31 => I::LD(Reg::SP, WORD),
            // LD SP, HL : Put HL into Stack Pointer (SP)
            0xF9 => I::LD(Reg::SP, Reg::HL),
            // LDHL SP, n : Put (SP + n) effective address into HL
            // NOTE: n is one byte signed value
            0xF8 => I::LDHL(Reg::SP, BYTE),
            // LD (nn), SP : Put stack pointer to address `n`
            0x08 => I::LD(WORD, Reg::SP),
            // PUSH nn : Push register pair `nn` onto stack.
            //         : Decrement stack pointer twice.
            0xF5 => I::PUSH(Reg::AF),
            0xC5 => I::PUSH(Reg::BC),
            0xD5 => I::PUSH(Reg::DE),
            0xE5 => I::PUSH(Reg::HL),
            // POP nn : Pop two bytes from the stack onto register pair `nn`
            //        : Increment stack pointer twice.
            0xF1 => I::POP(Reg::AF),
            0xC1 => I::POP(Reg::BC),
            0xD1 => I::POP(Reg::DE),
            0xE1 => I::POP(Reg::HL),
            // 8-Bit ALU ------------------------------------------------------
            // ADD A, n : Add n to A
            0x87 => I::ADD(Reg::A, Reg::A),
            0x80 => I::ADD(Reg::A, Reg::B),
            0x81 => I::ADD(Reg::A, Reg::C),
            0x82 => I::ADD(Reg::A, Reg::D),
            0x83 => I::ADD(Reg::A, Reg::E),
            0x84 => I::ADD(Reg::A, Reg::H),
            0x85 => I::ADD(Reg::A, Reg::L),
            0x86 => I::ADD(Reg::A, Reg::HL),
            0xC6 => I::ADD(Reg::A, BYTE),
            // ADC A, n : Add n + carry flag to A
            0x8F => I::ADC(Reg::A, Reg::A),
            0x88 => I::ADC(Reg::A, Reg::B),
            0x89 => I::ADC(Reg::A, Reg::C),
            0x8A => I::ADC(Reg::A, Reg::D),
            0x8B => I::ADC(Reg::A, Reg::E),
            0x8C => I::ADC(Reg::A, Reg::H),
            0x8D => I::ADC(Reg::A, Reg::L),
            0x8E => I::ADC(Reg::A, Reg::HL),
            0xCE => I::ADC(Reg::A, BYTE),
            // SUB A, n : Subtract n from A
            0x97 => I::SUB(Reg::A, Reg::A),
            0x90 => I::SUB(Reg::A, Reg::B),
            0x91 => I::SUB(Reg::A, Reg::C),
            0x92 => I::SUB(Reg::A, Reg::D),
            0x93 => I::SUB(Reg::A, Reg::E),
            0x94 => I::SUB(Reg::A, Reg::H),
            0x95 => I::SUB(Reg::A, Reg::L),
            0x96 => I::SUB(Reg::A, Reg::HL),
            0xD6 => I::SUB(Reg::A, BYTE),
            // SBC A, n : Subtract n + carry flag from A
            0x9F => I::SBC(Reg::A, Reg::A),
            0x98 => I::SBC(Reg::A, Reg::B),
            0x99 => I::SBC(Reg::A, Reg::C),
            0x9A => I::SBC(Reg::A, Reg::D),
            0x9B => I::SBC(Reg::A, Reg::E),
            0x9C => I::SBC(Reg::A, Reg::H),
            0x9D => I::SBC(Reg::A, Reg::L),
            0x9E => I::SBC(Reg::A, Reg::HL),
            //0x__ => I::ADC(Reg::A, BYTE), // XXX: does this opcode exist?
            // AND n : Logically AND `n` with A, Result in A
            0xA7 => I::AND(Reg::A),
            0xA0 => I::AND(Reg::B),
            0xA1 => I::AND(Reg::C),
            0xA2 => I::AND(Reg::D),
            0xA3 => I::AND(Reg::E),
            0xA4 => I::AND(Reg::H),
            0xA5 => I::AND(Reg::L),
            0xA6 => I::AND(Reg::HL),
            0xE6 => I::AND(BYTE),
            // OR n : Logically OR `n` with A, Result in A
            0xB7 => I::OR(Reg::A),
            0xB0 => I::OR(Reg::B),
            0xB1 => I::OR(Reg::C),
            0xB2 => I::OR(Reg::D),
            0xB3 => I::OR(Reg::E),
            0xB4 => I::OR(Reg::H),
            0xB5 => I::OR(Reg::L),
            0xB6 => I::OR(Reg::HL),
            0xF6 => I::OR(BYTE),
            // XOR n : Logically XOR `n` with A, Result in A
            0xAF => I::XOR(Reg::A),
            0xA8 => I::XOR(Reg::B),
            0xA9 => I::XOR(Reg::C),
            0xAA => I::XOR(Reg::D),
            0xAB => I::XOR(Reg::E),
            0xAC => I::XOR(Reg::H),
            0xAD => I::XOR(Reg::L),
            0xAE => I::XOR(Reg::HL),
            0xEE => I::XOR(BYTE),
            // CP n : Compare `A` with `n`.
            0xBF => I::CP(Reg::A),
            0xB8 => I::CP(Reg::B),
            0xB9 => I::CP(Reg::C),
            0xBA => I::CP(Reg::D),
            0xBB => I::CP(Reg::E),
            0xBC => I::CP(Reg::H),
            0xBD => I::CP(Reg::L),
            0xBE => I::CP(Reg::HL),
            0xFE => I::CP(BYTE),
            // INC n : Increment Register n
            0x3C => I::CP(Reg::A),
            0x04 => I::CP(Reg::B),
            0x0C => I::CP(Reg::C),
            0x14 => I::CP(Reg::D),
            0x1C => I::CP(Reg::E),
            0x24 => I::CP(Reg::H),
            0x2C => I::CP(Reg::L),
            0x34 => I::CP(Reg::HL),
            // DEC n : Decrement Register n
            0x3D => I::CP(Reg::A),
            0x05 => I::CP(Reg::B),
            0x0D => I::CP(Reg::C),
            0x15 => I::CP(Reg::D),
            0x1D => I::CP(Reg::E),
            0x25 => I::CP(Reg::H),
            0x2D => I::CP(Reg::L),
            0x35 => I::CP(Reg::HL),
            // 16-Bit ALU -----------------------------------------------------
            // ADD HL, n : Add n to HL
            0x09 => I::ADD(Reg::HL, Reg::BC),
            0x19 => I::ADD(Reg::HL, Reg::DE),
            0x29 => I::ADD(Reg::HL, Reg::HL),
            0x39 => I::ADD(Reg::HL, Reg::SP),
            // ADD SP, n : Add n to SP
            0xE8 => I::ADD(Reg::SP, WORD),
            // INC nn : Increment register nn
            0x03 => I::INC(Reg::BC),
            0x13 => I::INC(Reg::DE),
            0x23 => I::INC(Reg::HL),
            0x33 => I::INC(Reg::SP),
            // DEC nn : decrement register nn
            0x0B => I::DEC(Reg::BC),
            0x1B => I::DEC(Reg::DE),
            0x2B => I::DEC(Reg::HL),
            0x3B => I::DEC(Reg::SP),
            // Misc. ----------------------------------------------------------
            // DAA : Decimal adjust register A
            //     : Adjusts value in register A so that the correct Binary
            //     : Coded Decimal (BCD) is obtained.
            0x27 => I::DAA,
            // CPL : Complement Register `A` (flip all bits)
            0x2F => I::CPL,
            // CCF : Complement Carry Flag (flip carry flag)
            0x3F => I::CCF,
            // SCF : Set Carry Flag ( turn on )
            0x37 => I::SCF,
            // NOP : No Operation
            0x00 => I::NOP,
            // HALT : Power down CPU until an interrupt occurs
            0x76 => I::HALT,
            // STOP : Halt CPU & LCD display until button is pressed
            //      : This is a two byte opcode : 0x10 0x00 or a STOP HALT pair
            0x10 => I::STOP,
            // DI : Disables Interrupts after the instruction is executed.
            0xF3 => I::DI,
            // EI : Enables Interrupts after the instruction is executed.
            0xFB => I::EI,
            // RCLA : Rotate A left; old bit 7 to carry flag
            0x07 => I::RLCA,
            // RLA : Rotate A left through carry flag
            0x17 => I::RLA,
            // RRCA : Rotate A right; Old bit 0 to carry flag
            0x0F => I::RRCA,
            // RCA : Rotate A right through carry flag
            0x1F => I::RRA,
            // Jumps ----------------------------------------------------------
            // JP nn : Jump to specified address, nn.
            0xC3 => I::JP(WORD),
            // JPCC nn : Jump to address if specified condition is true:
            0xC2 => I::JPNZ(WORD), // if Z flag is low
            0xCA => I::JPZ(WORD),  // if Z flag is high
            0xD2 => I::JPNC(WORD), // if C flag is low
            0xDA => I::JPC(WORD),  // if C flag is high
            // JPHL : Jump to address in HL
            0xE9 => I::JPHL,
            // JR nn : Add n to current address and jump to it
            0x18 => I::JR(BYTE),
            // JPCC nn : add n to value and jump to address if specified
            //         : condition is true:
            0x20 => I::JRNZ(WORD), // if Z flag is low
            0x28 => I::JRZ(WORD),  // if Z flag is high
            0x30 => I::JRNC(WORD), // if C flag is low
            0x38 => I::JRC(WORD),  // if C flag is high
            // Calls ----------------------------------------------------------
            // CALL nn : Push address of next address onto stack and then jump
            //         : to that address.
            0xCD => I::CALL(WORD),
            // CALL nn : Call address if specified condition is true:
            0xC4 => I::CALLNZ(WORD), // if Z flag is low
            0xCC => I::CALLZ(WORD),  // if Z flag is high
            0xD4 => I::CALLNC(WORD), // if C flag is low
            0xDC => I::CALLC(WORD),  // if C flag is high
            // Resets ---------------------------------------------------------
            // RST n : Push present address onto stack,
            //       : jump to address 0x0000 + n
            0xC7 => I::RST(0x00),
            0xCF => I::RST(0x08),
            0xD7 => I::RST(0x10),
            0xDF => I::RST(0x18),
            0xE7 => I::RST(0x20),
            0xEF => I::RST(0x28),
            0xF7 => I::RST(0x30),
            0xFF => I::RST(0x38),
            // Returns --------------------------------------------------------
            // RET : pop two bytes from the stack and jump to that address
            0xC9 => I::RET,
            // RET cc : return if following condition is true
            0xC0 => I::RETNZ(WORD), // if Z flag is low
            0xC8 => I::RETZ(WORD),  // if Z flag is high
            0xD0 => I::RETNC(WORD), // if C flag is low
            0xD8 => I::RETC(WORD),  // if C flag is high
            // RETI : pop two bytes from stack and jump to that address
            //      : while also enabling interrupts.
            0xD9 => I::RETI,
            // ExtInstr : Marker that the next byte is an extended instruction
            0xCB => I::ExtInstr,
            _ => {
                println!("Decoded invalid instruction: 0x{:X}", byte);
                I::NOP
            },
        }
    }

    /// The CPU features some extended instructions which are signaled by the
    /// 0xCB instruction which was unused on the 8080.
    pub fn decode_extended(byte: u8) -> Instruction {
        use self::Instruction as I;
        use self::Register::BYTE as BYTE;
        use self::Register::WORD as WORD;
        use self::Register as Reg;

        match byte {
            // SWAP n : Swap upper and lower nibbles of n
            0x37 => I::SWAP(Reg::A),
            0x30 => I::SWAP(Reg::B),
            0x31 => I::SWAP(Reg::C),
            0x32 => I::SWAP(Reg::D),
            0x33 => I::SWAP(Reg::E),
            0x34 => I::SWAP(Reg::H),
            0x35 => I::SWAP(Reg::L),
            0x36 => I::SWAP(Reg::HL),
            // rotate and shifts ----------------------------------------------
            // RL n : Rotate n left; old bit 7 to carry flag
            0x07 => I::RLC(Reg::A),
            0x00 => I::RLC(Reg::B),
            0x01 => I::RLC(Reg::C),
            0x02 => I::RLC(Reg::D),
            0x03 => I::RLC(Reg::E),
            0x04 => I::RLC(Reg::H),
            0x05 => I::RLC(Reg::L),
            0x06 => I::RLC(Reg::HL),
            // RL n : Rotate n left through carry flag
            0x17 => I::RL(Reg::A),
            0x10 => I::RL(Reg::B),
            0x11 => I::RL(Reg::C),
            0x12 => I::RL(Reg::D),
            0x13 => I::RL(Reg::E),
            0x14 => I::RL(Reg::H),
            0x15 => I::RL(Reg::L),
            0x16 => I::RL(Reg::HL),
            // RRC n : Rotate n right; old bit 0 to carry flag
            0x0F => I::RRC(Reg::A),
            0x08 => I::RRC(Reg::B),
            0x09 => I::RRC(Reg::C),
            0x0A => I::RRC(Reg::D),
            0x0B => I::RRC(Reg::E),
            0x0C => I::RRC(Reg::H),
            0x0D => I::RRC(Reg::L),
            0x0E => I::RRC(Reg::HL),
            // RR n : Rotate n right through carry flag
            0x1F => I::RR(Reg::A),
            0x18 => I::RR(Reg::B),
            0x19 => I::RR(Reg::C),
            0x1A => I::RR(Reg::D),
            0x1B => I::RR(Reg::E),
            0x1C => I::RR(Reg::H),
            0x1D => I::RR(Reg::L),
            0x1E => I::RR(Reg::HL),
            // SLA n : Shift n left into carry; LSB of n set to 0
            0x27 => I::SLA(Reg::A),
            0x20 => I::SLA(Reg::B),
            0x21 => I::SLA(Reg::C),
            0x22 => I::SLA(Reg::D),
            0x23 => I::SLA(Reg::E),
            0x24 => I::SLA(Reg::H),
            0x25 => I::SLA(Reg::L),
            0x26 => I::SLA(Reg::HL),
            // SRA n : Shift n right into carry; MSB of n set doesn't change
            0x2F => I::SRA(Reg::A),
            0x28 => I::SRA(Reg::B),
            0x29 => I::SRA(Reg::C),
            0x2A => I::SRA(Reg::D),
            0x2B => I::SRA(Reg::E),
            0x2C => I::SRA(Reg::H),
            0x2D => I::SRA(Reg::L),
            0x2E => I::SRA(Reg::HL),
            // SRL n : Shift n right into carry; MSB of n set to 0
            0x3F => I::SRL(Reg::A),
            0x38 => I::SRL(Reg::B),
            0x39 => I::SRL(Reg::C),
            0x3A => I::SRL(Reg::D),
            0x3B => I::SRL(Reg::E),
            0x3C => I::SRL(Reg::H),
            0x3D => I::SRL(Reg::L),
            0x3E => I::SRL(Reg::HL),
            // bit opcodes ----------------------------------------------------
            // BIT b, r : test bit `b` in register r
            0x47 => I::BIT(BYTE, Reg::A),
            0x40 => I::BIT(BYTE, Reg::B),
            0x41 => I::BIT(BYTE, Reg::C),
            0x42 => I::BIT(BYTE, Reg::D),
            0x43 => I::BIT(BYTE, Reg::E),
            0x44 => I::BIT(BYTE, Reg::H),
            0x45 => I::BIT(BYTE, Reg::L),
            0x46 => I::BIT(BYTE, Reg::HL),
            // SET b, r : Set bit `b` in register r
            0xC7 => I::SET(BYTE, Reg::A),
            0xC0 => I::SET(BYTE, Reg::B),
            0xC1 => I::SET(BYTE, Reg::C),
            0xC2 => I::SET(BYTE, Reg::D),
            0xC3 => I::SET(BYTE, Reg::E),
            0xC4 => I::SET(BYTE, Reg::H),
            0xC5 => I::SET(BYTE, Reg::L),
            0xC6 => I::SET(BYTE, Reg::HL),
            // RES b, r : Reset bit `b` in register r
            0x87 => I::RES(BYTE, Reg::A),
            0x80 => I::RES(BYTE, Reg::B),
            0x81 => I::RES(BYTE, Reg::C),
            0x82 => I::RES(BYTE, Reg::D),
            0x83 => I::RES(BYTE, Reg::E),
            0x84 => I::RES(BYTE, Reg::H),
            0x85 => I::RES(BYTE, Reg::L),
            0x86 => I::RES(BYTE, Reg::HL),
            _ => {
                println!("Decoded invalid extended instruction: 0x{:X}", byte);
                I::NOP
            },
        }
    }

    /// Returns the value of the CPU's program counter
    pub fn get_pc(&self) -> u16 { self.reg_pc }

    /// Resets processor state
    fn reset(&mut self) {
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
    fn nop(&mut self) { self.reg_m = 1; self.reg_t = 4; }
    /// (HALT): halt the processor
    fn halt(&mut self) { self.halt = true; self.reg_m = 1; }
    /// (DI): TODO: ???
    fn di(&mut self) { self.reg_ime = 0; self.reg_m = 1; }
    /// (EI): TODO: ???
    fn ei(&mut self) { self.reg_ime = 1; self.reg_m = 1; }
    /// (UNDEF): Undefined operation
    fn undef(&mut self) { unreachable!(); }

}
