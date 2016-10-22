#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::fs::File;
use std::io::Read;

use cpu::CPU;

/// Documentation
/// -------------
/// Reserved Memory Locations
/// 0x0000 : Restart $00 Address (RST $00 calls this address.)
/// 0x0008 : Restart $08 Address (RST $08 calls this address.)
/// 0x0010 : Restart $10 Address (RST $10 calls this address.)
/// 0x0018 : Restart $18 Address (RST $18 calls this address.)
/// 0x0020 : Restart $20 Address (RST $20 calls this address.)
/// 0x0028 : Restart $28 Address (RST $28 calls this address.)
/// 0x0030 : Restart $30 Address (RST $30 calls this address.)
/// 0x0038 : Restart $38 Address (RST $38 calls this address.)
/// 0x0040 : Vertical Blank Interrupt Start Address
/// 0x0048 : LCDC Status Interrupt Start Address
/// 0x0050 : Timer Overflow Interrupt Start Address
/// 0x0058 : Serial Transfer Completion Interrupt Start Address
/// 0x0060 : High-to-Low of P10-P13 Interrupt Start Address
/// ------
/// 0x0100 - 0x014f : An internal information area and contains:
/// 0x0100 - 0x0103 : Begin code execution point of a cart. Usually
///                   consists of a NOP & JP instruction but not always.
/// 0x0104 - 0x0133 : Scrolling Nintendo graphic
/// 0x0134 - 0x0142 : Title of the game in UPPER CASE ASCII; If less than 16
///                   chars then the remaining bytes are filled with 0x00.
/// 0x0143 : 0x80 = Color GB; 0x00 = other / not color GB
/// 0x0144 : High nibble of license
/// 0x0145 : Low nibble of license; (usually 0x00 if 0x014B != 0x33)
/// 0x0146 : GB / SGB Indicator; 0x00 GB; 0x03 = SGB
/// 0x0147 : Cartridge type:
///          0  - ROM ONLY                 11 - ROM+MBC3
///          1  - ROM+MBC1                 12 - ROM+MBC3+RAM
///          2  - ROM+MBC1+RAM             13 - ROM+MBC3+RAM+BATT
///          3  - ROM+MBC1+RAM+BATT        19 - ROM+MBC5
///          5  - ROM+MBC2                 1A - ROM+MBC5+RAM
///          6  - ROM+MBC2+BATTERY         1B - ROM+MBC5+RAM+BATT
///          8  - ROM+RAM                  1C - ROM+MBC5+RUMBLE
///          9  - ROM+RAM+BATTERY          1D - ROM+MBC5+RUMBLE+SRAM
///          B  - ROM+MMM01                1E - ROM+MBC5+RUMBLE+SRAM+BATT
///          C  - ROM+MMM01+SRAM           1F - Pocket Camera
///          D  - ROM+MMM01+SRAM+BATT      FD - Bandai TAMA5
///          F  - ROM+MBC3+TIMER+BATT      FE - Hudson HuC-3
///          10 - ROM+MBC3+TIMER+RAM+BATT  FF - Hudson HuC-1
/// 0x0148 : ROM Size:
///          0x00 - 256Kbit =  32KByte =   2 banks
///          0x01 - 512Kbit =  64KByte =   4 banks
///          0x02 -   1Mbit = 128KByte =   8 banks
///          0x03 -   2Mbit = 256KByte =  16 banks
///          0x04 -   4Mbit = 512KByte =  32 banks
///          0x05 -   8Mbit =   1MByte =  64 banks
///          0x06 -  16Mbit =   2MByte = 128 banks
///          0x52 -   9Mbit = 1.1MByte =  72 banks
///          0x53 -  10Mbit = 1.2MByte =  80 banks
///          0x54 -  12Mbit = 1.5MByte =  96 banks
/// 0x0149 : RAM Size:
///          0x00 - None
///          0x01 -  16kBit =  2kB = 1 bank
///          0x02 -  64kBit =  8kB = 1 bank
///          0x03 - 256kBit = 32kB = 4 banks
///          0x04 -   1MBit =128kB =16 banks
/// 0x014A : Destination Code:
///          0x00 - Japanese
///          0x01 - Non-Japanese
/// 0x014B : Licensee code (old):
///          0x33 - Check 0144/0145 for Licensee code.
///          0x79 - Accolade
///          0xA4 - Konami (Super GameBoy function won't work if <> $33.)
/// 0x014C : Mask ROM Version number (Usually $00)
/// 0x014D : Complement Check; Program will not run on GB if this is not correct
///          will work with bad CC however on SGB.
/// 0x014E - 0x014F : Checksum (higher-byte first) produced by adding all the
///                   bytes of a cartridge except for two checksum bytes and
///                   taking two lower bytes of the result.
///                   (The GB ignores this value)

pub struct MMU {
    bios: [u8; 256],
    rom: Vec<u8>,
    cartridge_type: Option<u8>,
    mbc: u32, // TODO: ???
    rom_offset: u16,
    ram_offset: u16,
    eram: Vec<u8>,
    wram: Vec<u8>,
    zram: Vec<u8>,
    in_bios: bool,
    in_e: bool, // TODO: ???
    int_flag: bool,
}

impl MMU {
    pub fn new() -> MMU {
        MMU {
        bios: [
                0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32,
                0xCB, 0x7C, 0x20, 0xFB, 0x21, 0x26, 0xFF, 0x0E,
                0x11, 0x3E, 0x80, 0x32, 0xE2, 0x0C, 0x3E, 0xF3,
                0xE2, 0x32, 0x3E, 0x77, 0x77, 0x3E, 0xFC, 0xE0,
                0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1A,
                0xCD, 0x95, 0x00, 0xCD, 0x96, 0x00, 0x13, 0x7B,
                0xFE, 0x34, 0x20, 0xF3, 0x11, 0xD8, 0x00, 0x06,
                0x08, 0x1A, 0x13, 0x22, 0x23, 0x05, 0x20, 0xF9,
                0x3E, 0x19, 0xEA, 0x10, 0x99, 0x21, 0x2F, 0x99,
                0x0E, 0x0C, 0x3D, 0x28, 0x08, 0x32, 0x0D, 0x20,
                0xF9, 0x2E, 0x0F, 0x18, 0xF3, 0x67, 0x3E, 0x64,
                0x57, 0xE0, 0x42, 0x3E, 0x91, 0xE0, 0x40, 0x04,
                0x1E, 0x02, 0x0E, 0x0C, 0xF0, 0x44, 0xFE, 0x90,
                0x20, 0xFA, 0x0D, 0x20, 0xF7, 0x1D, 0x20, 0xF2,
                0x0E, 0x13, 0x24, 0x7C, 0x1E, 0x83, 0xFE, 0x62,
                0x28, 0x06, 0x1E, 0xC1, 0xFE, 0x64, 0x20, 0x06,
                0x7B, 0xE2, 0x0C, 0x3E, 0x87, 0xF2, 0xF0, 0x42,
                0x90, 0xE0, 0x42, 0x15, 0x20, 0xD2, 0x05, 0x20,
                0x4F, 0x16, 0x20, 0x18, 0xCB, 0x4F, 0x06, 0x04,
                0xC5, 0xCB, 0x11, 0x17, 0xC1, 0xCB, 0x11, 0x17,
                0x05, 0x20, 0xF5, 0x22, 0x23, 0x22, 0x23, 0xC9,
            // start: Scrolling Nintendo graphic
                0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B,
                0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
                0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E,
                0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
                0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC,
                0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
            // end: Scrolling Nintendo graphic
                0x3c, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x4C,
                0x21, 0x04, 0x01, 0x11, 0xA8, 0x00, 0x1A, 0x13,
                0xBE, 0x20, 0xFE, 0x23, 0x7D, 0xFE, 0x34, 0x20,
                0xF5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20,
                0xFB, 0x86, 0x20, 0xFE, 0x3E, 0x01, 0xE0, 0x50
              ],
            rom: Vec::<u8>::new(),
            cartridge_type: None,
            mbc: 0, // TODO: ???
            rom_offset: 0x0, // 0x4000,
            ram_offset: 0x0,
            eram: Vec::<u8>::new(),
            wram: Vec::<u8>::new(),
            zram: Vec::<u8>::new(),
            in_bios: true,
            in_e: false,
            int_flag: false,
        }
    }

    pub fn load_file(&mut self, file: File) {
        self.rom = file.bytes()
                       .map(|r: Result<u8, _>| r.unwrap())
                       .collect();
        self.cartridge_type = match self.rom.get(0x0147) {
            Some(val) => Some(*val),
            None => None,
        }
    }

    pub fn load_bytes(&mut self, rom: &[u8]) {
        self.rom = rom.to_owned();
    }

    pub fn get_byte_at_offset(&self) -> u8 {
        *self.rom.get(self.rom_offset as usize).unwrap()
    }

    pub fn incr_rom(&mut self) {
        self.rom_offset += 1;
    }

    /// Read byte
    pub fn rb(&mut self, cpu: &CPU, addr: u16) -> u8 {
        match addr & 0xF000 {
            // Rom bank 0
            0x0000 => {
                if self.in_bios {
                    if addr < 0x0100 {
                        return self.bios[addr as usize];
                    } else if cpu.get_pc() == 0x0100 {
                        self.in_bios = false;
                    }
                } else {
                    return *self.rom.get(addr as usize).unwrap();
                }
            },
            0x1000 ... 0x3000 => {
                return *self.rom.get(addr as usize).unwrap();
            },
            // Rom bank 1
            0x4000 ... 0x7000 => {
                let offset = self.rom_offset as usize + (addr & 0x3FFF) as usize;
                return *self.rom.get(offset).unwrap();
            },
            // Video RAM
            0x8000 ... 0x9000 => {
                //return *GPU.vram.get((addr & 0x1FFF) as usize).unwrap();
                unimplemented!();
            },
            // External RAM
            0xA000 ... 0xB000 => {
                let offset = self.ram_offset as usize + (addr & 0x1FFF) as usize;
                return *self.eram.get(offset).unwrap();
            },
            // Work RAM & Echo
            0xC000 ... 0xE000 => {
                return *self.wram.get((addr & 0x1FFF) as usize).unwrap();
            },
            // Everything Else
            0xF000 => {
                match addr & 0x0F00 {
                    // Echo RAM
                    0x000 ... 0xD00 => {
                        return *self.wram.get((addr & 0x1FFF) as usize).unwrap();
                    },
                    // OAM
                    0xE00 => {
                        unimplemented!();
                    },
                    // Zero-page RAM, I/O, Interrupts
                    0xF00 => {
                        if addr == 0xFFFF {
                            return self.in_e as u8;
                        } else if addr > 0xFF7F {
                            return *self.zram.get((addr & 0x7F) as usize).unwrap();
                        }
                        match addr & 0xF0 {
                            0x00 => {
                                match addr & 0xF {
                                    // Directional Pad
                                    0x0 => { unimplemented!() },
                                    // TODO: Timer flags?
                                    0x4 ... 0x7 => { unimplemented!() },
                                    0x15 => { return self.int_flag as u8; },
                                    _ => { return 0u8; },
                                }
                            }
                            0x10 => { return 0u8; },
                            // GPU read-byte
                            0x40 ... 0x70 => { unreachable!()},
                            _ => {unreachable!()},
                        }
                    },
                    _ => {unreachable!()},
                }
            },
            _ => {unreachable!()},
        }
        // read 8-bit byte from memory
        unimplemented!();
    }

    /// Reads a word (2-Bytes) from the given address
    pub fn rw(&mut self, cpu: &CPU, addr: u16) -> u16 {
        // read 16-bit word from memory
        let mut val = 0u16;
        val += self.rb(cpu, addr) as u16;
        val += (self.rb(cpu, addr+1) as u16) << 8;
        val
    }

    /// Writes a byte to the given address
    pub fn wb(&self, cpu: &CPU, addr: u16, val: u8) {
        // write 8-bit byte from memory
        unimplemented!();
    }

    /// Writes a word (2-Bytes) to the given address
    pub fn ww(&self, cpu: &CPU, addr: u16, val: u16) {
        // write 16-bit word from memory
        unimplemented!();
    }
}
