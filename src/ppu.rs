use crate::cartridge::CartridgeRef;
use crate::memory::Memory;

const PPUCTRL: usize = 0x00;
const PPUMASK: usize = 0x01;
const PPUSTATUS: usize = 0x02;
const OAMADDR: usize = 0x03;
const OAMDATA: usize = 0x04;
const PPUSCROLL: usize = 0x05;
const PPUADDR: usize = 0x06;
const PPUDATA: usize = 0x07;
const OAMDMA: usize = 0x4014;

pub struct PPU {
    pub cartridge: CartridgeRef,
}

impl Memory for PPU {
    fn read(&mut self, address: usize, _is_read_only: bool) -> u8 {
        match address & 0x07 {
            PPUCTRL => 0,
            PPUMASK => 0,
            PPUSTATUS => 0,
            OAMADDR => 0,
            OAMDATA => 0,
            PPUSCROLL => 0,
            PPUADDR => 0,
            PPUDATA => 0,
            _ => 0
        }
    }

    fn write(&mut self, address: usize, _value: u8) {
        match address & 0x07 {
            PPUCTRL => {}
            PPUMASK => {}
            PPUSTATUS => {}
            OAMADDR => {}
            OAMDATA => {}
            PPUSCROLL => {}
            PPUADDR => {}
            PPUDATA => {}
            _ => {}
        }
    }
}

impl PPU {
    pub fn ppu_read(&mut self, _address: usize) -> u8 {
        0
    }

    pub fn ppu_write(&mut self, _address: usize, _value: u8) {
        // ??
    }
}
