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
    use ::rgb_error;

    /// The Emulator context holds all of pieces to the running state of an
    // emulator.
    pub struct EmulatorContext {
        cpu: cpu::CPU,
        mmu: mmu::MMU,
    }

    pub fn new() -> EmulatorContext {
        EmulatorContext {
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
            // Start instruction fetch
            {
                // get the next instruction byte
                let byte = self.mmu.get_byte_at_offset();
                self.mmu.incr_rom();
                // Decode the instruction
                let inst = match cpu::CPU::decode(byte) {
                    I::ExtInstr => {
                        // load another byte
                        let byte = self.mmu.get_byte_at_offset();
                        self.mmu.incr_rom();
                        // and decode the extended instr
                        let _inst = cpu::CPU::decode_extended(byte);
                        println!("0x{:02x}\t{:?}\t\t(extended)", byte, _inst);
                        _inst

                    },
                    inst => {
                        println!("0x{:02x}\t{:?}", byte, inst);
                        inst
                    }
                };
            } // end instruction fetch
            // find out if we need to load a byte/word for the instr.
        }
    }
}

