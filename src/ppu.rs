use crate::cartridge::*;
use crate::memory::Memory;
use crate::utils::XORShiftRand;

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

type PPUColor = (u8, u8, u8);

pub static PPU_COLORS: [PPUColor; 0x40] = [
    (84, 84, 84),
    (0, 30, 116),
    (8, 16, 144),
    (48, 0, 136),
    (68, 0, 100),
    (92, 0, 48),
    (84, 4, 0),
    (60, 24, 0),
    (32, 42, 0),
    (8, 58, 0),
    (0, 64, 0),
    (0, 60, 0),
    (0, 50, 60),
    (0, 0, 0),
    (0, 0, 0),
    (0, 0, 0),
    (152, 150, 152),
    (8, 76, 196),
    (48, 50, 236),
    (92, 30, 228),
    (136, 20, 176),
    (160, 20, 100),
    (152, 34, 32),
    (120, 60, 0),
    (84, 90, 0),
    (40, 114, 0),
    (8, 124, 0),
    (0, 118, 40),
    (0, 102, 120),
    (0, 0, 0),
    (0, 0, 0),
    (0, 0, 0),
    (236, 238, 236),
    (76, 154, 236),
    (120, 124, 236),
    (176, 98, 236),
    (228, 84, 236),
    (236, 88, 180),
    (236, 106, 100),
    (212, 136, 32),
    (160, 170, 0),
    (116, 196, 0),
    (76, 208, 32),
    (56, 204, 108),
    (56, 180, 204),
    (60, 60, 60),
    (0, 0, 0),
    (0, 0, 0),
    (236, 238, 236),
    (168, 204, 236),
    (188, 188, 236),
    (212, 178, 236),
    (236, 174, 236),
    (236, 174, 212),
    (236, 180, 176),
    (228, 196, 144),
    (204, 210, 120),
    (180, 222, 120),
    (168, 226, 144),
    (152, 226, 180),
    (160, 214, 228),
    (160, 162, 160),
    (0, 0, 0),
    (0, 0, 0),
];

pub fn get_screen_buffer_pointer() -> *const u8 {
    let pointer: *const u8;
    unsafe {
        pointer = NES_SCREEN_BUFFER.as_ptr();
    }

    return pointer;
}

pub fn get_screen_buffer<'a>() -> &'a [u8] {
    unsafe { &NES_SCREEN_BUFFER }
}

pub type PPURef = Rc<RefCell<PPU>>;

pub struct PPU {
    pub cartridge: CartridgeRef,
    pattern_table: [[u8; 0x1000]; 2], // 0x0000 - 0x1fff
    nametable: [[u8; 0x0400]; 2],     // 0x2000 - 0x2fff
    palette_table: [u8; 32],          // 0x3f00 - 0x3fff

    screen: [u8; NES_SCREEN_BUFFER_SIZE],
    cycle: i32,
    scanline: i32,
    pub done_drawing: bool,

    // for debug
    rand: XORShiftRand,
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
            cycle: 0,
            scanline: 0,
            done_drawing: false,
            rand: XORShiftRand::new(0xad334da55),
        }
    }

    pub fn clock(&mut self) {
        // TODO: implement clock

        if self.cycle < 256 && (0 <= self.scanline && self.scanline < 240) {
            let pos = NES_WIDTH_SIZE * (self.scanline as usize) + (self.cycle as usize);
            let pos = pos * 4;

            let color = if self.rand.rand() & 0x01 == 0 {
                PPU_COLORS[0x3f]
            } else {
                PPU_COLORS[0x30]
            };

            Self::set_buffer(pos + 0, color.0);
            Self::set_buffer(pos + 1, color.1);
            Self::set_buffer(pos + 2, color.2);
            Self::set_buffer(pos + 3, 255);
        }

        self.cycle += 1;

        if self.cycle >= 341 {
            self.cycle = 0;
            self.scanline += 1;

            if self.scanline >= 261 {
                self.scanline = -1;
                self.done_drawing = true;
            }
        }
    }

    pub fn ppu_read(&mut self, address: usize, is_read_only: bool) -> u8 {
        let data = self.cartridge.borrow_mut().ppu_read(address, is_read_only);

        if self.cartridge.borrow().use_cartridge_data() {
            return data;
        }

        match address {
            0..=0x0fff => self.pattern_table[0][address & 0x0fff],
            0x1000..=0x1fff => self.pattern_table[1][address & 0x0fff],
            0x2000..=0x3eff => {
                let nametable_address = address & 0x0fff;
                match self.cartridge.borrow().mirroring() {
                    MirroringMode::Horizontal => match nametable_address {
                        0x0000..=0x07ff => self.nametable[0][nametable_address & 0x03ff],
                        0x0800..=0x0fff => self.nametable[1][nametable_address & 0x03ff],
                        _ => 0,
                    },
                    MirroringMode::Vertical => match nametable_address {
                        0x0000..=0x03ff | 0x0800..=0x0bff => {
                            self.nametable[0][nametable_address & 0x03ff]
                        }
                        0x0400..=0x07ff | 0x0c00..=0x0fff => {
                            self.nametable[1][nametable_address & 0x03ff]
                        }
                        _ => 0,
                    },
                    MirroringMode::SingleScreen => self.nametable[0][nametable_address & 0x03ff],
                    MirroringMode::Hardware => {
                        // should not be set as hardware
                        0
                    }
                }
            }
            0x3f00..=0x3fff => {
                let mut palette_index = address & 0x001f;

                if palette_index == 0x10 {
                    palette_index = 0x00;
                } else if palette_index == 0x14 {
                    palette_index = 0x04;
                } else if palette_index == 0x18 {
                    palette_index = 0x08;
                } else if palette_index == 0x1c {
                    palette_index = 0x0c;
                }

                self.palette_table[palette_index]
            }
            _ => 0,
        }
    }

    pub fn ppu_write(&mut self, address: usize, value: u8) {
        self.cartridge.borrow_mut().ppu_write(address, value);

        if self.cartridge.borrow().use_cartridge_data() {
            return;
        }

        match address {
            0..=0x0fff => {
                self.pattern_table[0][address & 0x0fff] = value;
            }
            0x1000..=0x1fff => {
                self.pattern_table[1][address & 0x0fff] = value;
            }
            0x2000..=0x3eff => {
                let nametable_address = address & 0x0fff;
                match self.cartridge.borrow().mirroring() {
                    MirroringMode::Horizontal => match nametable_address {
                        0x0000..=0x07ff => {
                            self.nametable[0][nametable_address & 0x03ff] = value;
                        }
                        0x0800..=0x0fff => {
                            self.nametable[1][nametable_address & 0x03ff] = value;
                        }
                        _ => {}
                    },
                    MirroringMode::Vertical => match nametable_address {
                        0x0000..=0x03ff | 0x0800..=0x0bff => {
                            self.nametable[0][nametable_address & 0x03ff] = value;
                        }
                        0x0400..=0x07ff | 0x0c00..=0x0fff => {
                            self.nametable[1][nametable_address & 0x03ff] = value;
                        }
                        _ => {}
                    },
                    MirroringMode::SingleScreen => {
                        self.nametable[0][nametable_address & 0x03ff] = value;
                    }
                    MirroringMode::Hardware => {
                        // should not be set as hardware
                    }
                }
            }
            0x3f00..=0x3fff => {
                let mut palette_index = address & 0x001f;

                if palette_index == 0x10 {
                    palette_index = 0x00;
                } else if palette_index == 0x14 {
                    palette_index = 0x04;
                } else if palette_index == 0x18 {
                    palette_index = 0x08;
                } else if palette_index == 0x1c {
                    palette_index = 0x0c;
                }

                self.palette_table[palette_index] = value;
            }
            _ => {}
        }
    }

    pub fn set_buffer(address: usize, value: u8) {
        unsafe {
            NES_SCREEN_BUFFER[address] = value;
        }
    }
}
