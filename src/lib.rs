#![allow(dead_code)]

// Module defines
mod cpu;
mod mmu;

/// The Emulator context holds all of pieces to the running state of an
// emulator.
pub struct EmulatorContext {
    cpu: cpu::CPU,
    mmu: mmu::MMU,
}

/// Returns the current version number as a `u32`.
pub fn version() -> u32 {
    42
}

pub fn new() -> EmulatorContext {
    EmulatorContext {
        cpu: cpu::CPU::new(),
        mmu: mmu::MMU::new(),
    }
}
