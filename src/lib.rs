#![allow(dead_code)]

// Module defines
mod cpu;
mod mmu;

mod rgb_error {
    use std::fmt;

    pub enum RgbError {
        BootRomLength,
        BootRomBad,
    }

    impl fmt::Display for RgbError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                RgbError::BootRomBad => write!(f, "Bad byte in boot rom at..."),
                RgbError::BootRomLength => write!(f, "Boot rom length is wrong"),
            }
        }
    }
}


/// Returns the current version number as a `u32`.
pub fn version() -> u32 {
    42
}

pub mod emulator_context {
    use ::cpu;
    use ::mmu;

    /// The Emulator context holds all of pieces to the running state of an
    // emulator.
    pub struct EmulatorContext {
        cycles: u64,
        cpu: cpu::CPU,
        mmu: mmu::MMU,
    }

    pub fn new() -> EmulatorContext {
        EmulatorContext {
            cycles: 0,
            cpu: cpu::CPU::new(),
            mmu: mmu::MMU::new(),
        }
    }

    impl EmulatorContext {

        pub fn load_bytes(&mut self, bytes: &[u8]) {
            self.mmu.load_bytes(bytes);
        }

        pub fn step(&mut self) {
            use cpu::Instruction as I;
            use cpu::Register as R;
            use cpu::Register::BYTE as BYTE;
            use cpu::Register::WORD as WORD;

            // Start instruction fetch
            let inst_off = self.cpu.get_pc();
            let inst = {
                // get the next instruction byte
                let byte = self.cpu.p_fetch(&self.mmu);
                // Decode the instruction
                let inst = match cpu::CPU::decode(byte) {
                    I::ExtInstr => {
                        // load another byte
                        let byte = self.cpu.p_fetch(&self.mmu);
                        // and decode the extended instr
                        let _inst = cpu::CPU::decode_extended(byte);
                        _inst

                    },
                    inst => { inst }
                };
                inst
            };// end instruction fetch

            // Start Decode: find out if we need to load a byte/word for the instr.
            #[derive(Debug)]
            enum RegData {
                None,
                Byte(u8),
                Word(u16),
            }
            let data = match inst {
                I::ADC(_, BYTE) |
                I::AND(BYTE)    |
                I::LD(_, BYTE)  |
                I::LD(BYTE, _)  |
                I::LDH(BYTE, _) |
                I::LDH(_, BYTE) |
                I::JR(BYTE)     |
                I::JRNZ(BYTE)   |
                I::JRZ(BYTE)    |
                I::JRNC(BYTE)   |
                I::JRC(BYTE)    |
                I::CP(BYTE)
                    => {
                        let byte = self.cpu.p_fetch(&self.mmu);
                        RegData::Byte(byte)
                    }
                I::LD(_, WORD) |
                I::LD(WORD, _) |
                I::CP(WORD)    |
                I::CALL(WORD)  |
                I::CALLZ(WORD) |
                I::CALLC(WORD) |
                I::RETC(WORD)
                    => {
                        let l_byte = self.cpu.p_fetch(&self.mmu) as u16;
                        let h_byte = self.cpu.p_fetch(&self.mmu) as u16;
                        RegData::Word((h_byte << 8) + l_byte)
                }
                _ => { RegData::None },
            }; // End Decode

            // Debug: print instruction
            let data: u16 = match data {
                RegData::Byte(data) => {
                    println!("0x{:04x}: {:?}, 0x{:02x}", inst_off, inst, data);
                    data as u16
                },
                RegData::Word(data) => {
                    println!("0x{:04x}: {:?}, 0x{:04x}", inst_off, inst, data);
                    data
                },
                RegData::None => {
                    println!("0x{:04x}: {:?}", inst_off, inst);
                    0
                }
            };

            // Start Execute
            match inst {
                I::LD(reg, WORD) => {
                    self.cpu.ld_r_w(reg, data);
                    self.cycles += 12;
                },
                I::XOR(reg) => {
                    self.cpu.xor_r(&mut self.mmu, reg);
                    self.cycles += 8;
                },
                I::LDD(r_a, r_b) => {
                    self.cpu.ldd_r_r(&mut self.mmu, r_a, r_b);
                    self.cycles += 8;
                },
                I::BIT(bit, reg) => {
                    if reg == R::HL {
                        self.cycles += 16;
                    } else {
                        self.cycles += 8;
                    }
                    self.cpu.bit_b_r(&mut self.mmu, bit, reg);
                },
                I::JRNZ(_) => {
                    self.cpu.jpnz(data as u8);
                    self.cycles += 8;
                },
                _ => {
                    println!("{:#?}", self.cpu);
                    println!("Instruction : {:#?}", inst);
                    unimplemented!()
                }
            }
            // End Execute
        }
    }
}

