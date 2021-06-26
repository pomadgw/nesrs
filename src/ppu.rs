use crate::cartridge::CartridgeRef;
use crate::memory::Memory;

use std::cell::RefCell;
use std::rc::Rc;

const PPUCTRL: usize = 0x00;
const PPUMASK: usize = 0x01;
const PPUSTATUS: usize = 0x02;
const OAMADDR: usize = 0x03;
const OAMDATA: usize = 0x04;
const PPUSCROLL: usize = 0x05;
const PPUADDR: usize = 0x06;
const PPUDATA: usize = 0x07;
const OAMDMA: usize = 0x4014;

pub const NES_WIDTH_SIZE: usize = 256;
pub const NES_HEIGHT_SIZE: usize = 240;
const NES_SCREEN_BUFFER_SIZE: usize = NES_WIDTH_SIZE * NES_HEIGHT_SIZE * 4;

pub static mut NES_SCREEN_BUFFER: [u8; NES_SCREEN_BUFFER_SIZE] = [0; NES_SCREEN_BUFFER_SIZE];

pub fn get_screen_buffer_pointer() -> *const u8 {
    let pointer: *const u8;
    unsafe {
        pointer = NES_SCREEN_BUFFER.as_ptr();
    }

    return pointer;
}

pub type PPURef = Rc<RefCell<PPU>>;

pub struct PPU {
    pub cartridge: CartridgeRef,
    pattern_table: [[u8; 0x1000]; 2],
    nametable: [[u8; 0x0400]; 2],
    palette_table: [u8; 32],

    screen: [u8; NES_SCREEN_BUFFER_SIZE],
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
            _ => 0,
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
    pub fn new(cartridge: CartridgeRef) -> Self {
        Self {
            cartridge,
            palette_table: [0; 32],
            nametable: [[0; 0x0400]; 2],
            pattern_table: [[0; 0x1000]; 2],
            screen: [0; NES_SCREEN_BUFFER_SIZE],
        }
    }

    pub fn ppu_read(&mut self, address: usize, is_read_only: bool) -> u8 {
        let data = self.cartridge.borrow_mut().ppu_read(address, is_read_only);

        if self.cartridge.borrow().use_cartridge_data() {
            return data;
        }

        0
    }

    pub fn ppu_write(&mut self, address: usize, value: u8) {
        self.cartridge.borrow_mut().ppu_write(address, value);

        if self.cartridge.borrow().use_cartridge_data() {
            // ??
        }
    }

    pub fn set_buffer(address: usize, value: u8) {
        unsafe {
            NES_SCREEN_BUFFER[address] = value;
        }
    }
}
