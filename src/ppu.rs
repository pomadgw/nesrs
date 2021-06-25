use crate::cartridge::CartridgeRef;
use crate::memory::Memory;

pub struct PPU {
    pub cartridge: CartridgeRef,
}

impl Memory for PPU {
    fn read(&mut self, _address: usize, _is_read_only: bool) -> u8 {
        0
    }

    fn write(&mut self, _address: usize, _value: u8) {
        // ?
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
