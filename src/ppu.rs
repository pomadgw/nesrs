use crate::cartridge::*;
use crate::memory::Memory;
use crate::utils::*;
use std::convert::{From, Into};
use std::fmt::Write;
use std::ops::AddAssign;

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

pub type PPURef = Rc<RefCell<PPU>>;

#[derive(Debug, Copy, Clone)]
pub struct PPUAddress {
    address: usize,
}

const PPUADDRRESS_COARSE_X_MASK: usize = 0b000_00_00000_11111;
const PPUADDRRESS_COARSE_Y_MASK: usize = 0b000_00_11111_00000;
const PPUADDRRESS_NAMETABLE_SELECT_MASK: usize = 0b000_11_00000_00000;
const PPUADDRRESS_FINE_Y_MASK: usize = 0b111_00_00000_00000;

impl PPUAddress {
    pub fn new() -> PPUAddress {
        PPUAddress { address: 0 }
    }

    pub fn address(&self) -> usize {
        self.address
    }
    pub fn set_address(&mut self, address: usize) {
        self.address = address & 0xffff
    }

    pub fn coarse_x(&self) -> usize {
        self.address & PPUADDRRESS_COARSE_X_MASK
    }

    pub fn set_coarse_x(&mut self, value: usize) {
        self.address &= !PPUADDRRESS_COARSE_X_MASK;
        self.address |= value & PPUADDRRESS_COARSE_X_MASK;
    }

    pub fn add_coarse_x(&mut self, value: usize) {
        let old_address = self.address & !PPUADDRRESS_COARSE_X_MASK;
        self.address += value;
        self.address &= PPUADDRRESS_COARSE_X_MASK;
        self.address |= old_address;
    }

    pub fn coarse_y(&self) -> usize {
        (self.address & PPUADDRRESS_COARSE_Y_MASK) >> 5
    }

    pub fn set_coarse_y(&mut self, value: usize) {
        self.address &= !PPUADDRRESS_COARSE_Y_MASK;
        self.address |= (value << 5) & PPUADDRRESS_COARSE_Y_MASK;
    }

    pub fn add_coarse_y(&mut self, value: usize) {
        let old_address = self.address & !PPUADDRRESS_COARSE_Y_MASK;
        self.address += value << 5;
        self.address &= PPUADDRRESS_COARSE_Y_MASK;
        self.address |= old_address;
    }

    pub fn nametable_select(&self) -> usize {
        (self.address & PPUADDRRESS_NAMETABLE_SELECT_MASK) >> 10
    }

    pub fn set_nametable_select(&mut self, value: usize) {
        self.address &= !PPUADDRRESS_NAMETABLE_SELECT_MASK;
        self.address |= (value << 10) & PPUADDRRESS_NAMETABLE_SELECT_MASK;
    }

    pub fn fine_y(&self) -> usize {
        (self.address & PPUADDRRESS_FINE_Y_MASK) >> 12
    }

    pub fn set_fine_y(&mut self, value: usize) {
        self.address &= !PPUADDRRESS_FINE_Y_MASK;
        self.address |= (value << 12) & PPUADDRRESS_FINE_Y_MASK;
    }
}

impl From<usize> for PPUAddress {
    fn from(value: usize) -> Self {
        PPUAddress { address: value }
    }
}

impl Into<usize> for PPUAddress {
    fn into(self) -> usize {
        self.address
    }
}

impl AddAssign<usize> for PPUAddress {
    fn add_assign(&mut self, rhs: usize) {
        self.address += rhs;
    }
}

bitflags! {
    pub struct PPUStatus: u8 {
        const VBLANK = 0b1000_0000;
        const SPRITE0_HIT = 0b0100_0000;
        const SPRITE_OVERFLOW = 0b0010_0000;
    }
}

bitflags! {
    pub struct PPUMask: u8 {
        const GREYSCALE        = 1 << 0;
        const SHOW_BG_LEFT     = 1 << 1;
        const SHOW_SPRITE_LEFT = 1 << 2;
        const SHOW_BG          = 1 << 3;
        const SHOW_SPRITE      = 1 << 4;
        const EMPHASIS_RED     = 1 << 5;
        const EMPHASIS_GREEN   = 1 << 6;
        const EMPHASIS_BLUE    = 1 << 7;
    }
}

bitflags! {
    pub struct PPUControl: u8 {
        const ENABLE_NMI                   = 0b1000_0000;
        const MASTER_SLAVE                 = 0b0100_0000;
        const SPRITE_SIZE                  = 0b0010_0000;
        const BG_PATTERN_TABLE_ADDRESS     = 0b0001_0000;
        const SPRITE_PATTERN_TABLE_ADDRESS = 0b0000_1000;
        const VRAM_ADDRESS_INCREMENT_MODE  = 0b0000_0100;
        const NAMETABLE_Y                  = 0b0000_0010;
        const NAMETABLE_X                  = 0b0000_0001;
    }
}

impl PPUControl {
    pub fn base_nametable_address(&self) -> u8 {
        self.bits & 0x03
    }

    pub fn set_base_nametable_address(&mut self, value: u8) {
        self.bits &= !(0x03);
        self.bits |= value & 0x03;
    }
}

enum AddressLatch {
    Lo,
    Hi,
}

pub struct PPU {
    pub cartridge: CartridgeRef,
    pattern_table: [[u8; 0x1000]; 2], // 0x0000 - 0x1fff
    nametable: [[u8; 0x0400]; 2],     // 0x2000 - 0x2fff
    palette_table: [u8; 32],          // 0x3f00 - 0x3fff

    screen: Vec<u8>,
    cycle: i32,
    scanline: i32,
    pub done_drawing: bool,
    pub call_nmi: bool,

    status: PPUStatus,
    control: PPUControl,
    mask: PPUMask,
    address_latch: AddressLatch,
    data_buffer: u8,

    temp_address: PPUAddress,
    vaddress: PPUAddress,

    // for debug
    rand: XORShiftRand,
    pub screen_debug_pattern: [Screen; 2],
}

impl Memory for PPU {
    fn read(&mut self, address: usize, is_read_only: bool) -> u8 {
        match address & 0x07 {
            PPUCTRL => 0,
            PPUMASK => 0,
            PPUSTATUS => {
                let status = (self.status.bits() & 0xe0) | (self.data_buffer & 0x1f);

                if !is_read_only {
                    self.status.set(PPUStatus::VBLANK, false);
                    self.address_latch = AddressLatch::Hi;
                }

                status
            }
            OAMADDR => 0,
            OAMDATA => 0,
            PPUSCROLL => 0,
            PPUADDR => 0,
            PPUDATA => {
                let read_result = self.ppu_read(self.vaddress.into(), is_read_only);

                // result the buffer data...
                let mut result = self.data_buffer;

                if self.vaddress.address() >= 0x3f00 {
                    // ...expect if we read palette
                    result = read_result;
                }

                // set the buffer data
                self.data_buffer = read_result;
                if !is_read_only {
                    self.increase_vaddress();
                }
                result
            }
            _ => 0,
        }
    }

    fn write(&mut self, address: usize, value: u8) {
        match address & 0x07 {
            PPUCTRL => {
                self.control.bits = value;
            }
            PPUMASK => {
                self.mask.bits = value;
            }
            PPUSTATUS => {}
            OAMADDR => {}
            OAMDATA => {}
            PPUSCROLL => {}
            PPUADDR => match self.address_latch {
                AddressLatch::Hi => {
                    self.address_latch = AddressLatch::Lo;
                    self.temp_address.set_address(
                        (((value & 0x3f) as usize) << 8) | (self.temp_address.address() & 0x00ff),
                    );
                }
                AddressLatch::Lo => {
                    self.temp_address
                        .set_address((self.temp_address.address() & 0xff00) | (value as usize));
                    self.address_latch = AddressLatch::Hi;
                    self.vaddress = self.temp_address;
                }
            },
            PPUDATA => {
                self.ppu_write(self.vaddress.into(), value);
                self.increase_vaddress();
            }
            _ => {}
        }
    }
}

impl PPU {
    pub fn new(cartridge: CartridgeRef) -> PPU {
        PPU {
            cartridge,
            palette_table: [0; 32],
            nametable: [[0; 0x0400]; 2],
            pattern_table: [[0; 0x1000]; 2],
            screen: vec![0; NES_SCREEN_BUFFER_SIZE],
            cycle: 0,
            scanline: 0,
            done_drawing: false,
            call_nmi: false,

            status: PPUStatus::empty(),
            control: PPUControl::empty(),
            mask: PPUMask::empty(),
            address_latch: AddressLatch::Hi,
            temp_address: PPUAddress::from(0),
            vaddress: PPUAddress::from(0),
            data_buffer: 0,

            rand: XORShiftRand::new(0xad334da55),
            screen_debug_pattern: [Screen::new(128, 128), Screen::new(128, 128)],
        }
    }

    pub fn clock(&mut self) {
        // TODO: implement clock
        if self.cycle == 1 && self.scanline == -1 {
            self.status.set(PPUStatus::VBLANK, false);
        }

        if self.cycle == 1 && self.scanline == 241 {
            self.status.set(PPUStatus::VBLANK, true);

            if self.control.contains(PPUControl::ENABLE_NMI) {
                self.call_nmi = true;
            }
        }

        if self.cycle < 256 && (0 <= self.scanline && self.scanline < 240) {
            let pos = NES_WIDTH_SIZE * (self.scanline as usize) + (self.cycle as usize);
            let pos = pos * 4;

            let color = if self.rand.rand() & 0x01 == 0 {
                PPU_COLORS[0x3f]
            } else {
                PPU_COLORS[0x30]
            };

            self.set_buffer(pos + 0, color.0);
            self.set_buffer(pos + 1, color.1);
            self.set_buffer(pos + 2, color.2);
            self.set_buffer(pos + 3, 255);
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

    pub fn cycle(&self) -> i32 {
        self.cycle
    }

    pub fn scanline(&self) -> i32 {
        self.scanline
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

    pub fn screen(&self) -> &Vec<u8> {
        &self.screen
    }

    pub fn set_buffer(&mut self, address: usize, value: u8) {
        self.screen[address] = value;
    }

    pub fn get_screen_buffer_pointer(&self) -> *const u8 {
        let pointer: *const u8;
        pointer = self.screen.as_ptr();

        return pointer;
    }

    fn increase_vaddress(&mut self) {
        let factor = if self
            .control
            .contains(PPUControl::VRAM_ADDRESS_INCREMENT_MODE)
        {
            32
        } else {
            1
        };
        self.vaddress += factor;
    }

    pub fn get_color(&mut self, palette: usize, index: usize) -> PPUColor {
        let address = 0x3f00 + (palette << 2) + index;
        let index = self.ppu_read(address, true) as usize;
        PPU_COLORS[index & 0x3f]
    }

    pub fn debug_nametable(&mut self, base: usize) -> Vec<String> {
        let mut result = Vec::new();
        let base_address = 0x2000 + (base * 0x0400);

        for row in 0..30 {
            let mut string = String::new();
            for col in 0..32 {
                write!(
                    string,
                    "{:02X}",
                    self.ppu_read(base_address + row * 32 + col, true)
                );
            }

            result.push(string);
        }

        result
    }

    pub fn debug_pattern(&mut self, base: usize, x: usize, y: usize) -> Vec<PPUColor> {
        let mut pattern = Vec::new();

        /*

            Pattern is here:

            Bit Planes            Pixel Pattern
            $0yyx + 0=$41  01000001
            $0yyx + 1=$C2  11000010
            $0yyx + 2=$44  01000100
            $0yyx + 3=$48  01001000
            $0yyx + 4=$10  00010000
            $0yyx + 5=$20  00100000         .1.....3
            $0yyx + 6=$40  01000000         11....3.
            $0yyx + 7=$80  10000000  =====  .1...3..
                                            .1..3...
            $0yyx + 8=$01  00000001  =====  ...3.22.
            $0yyx + 9=$02  00000010         ..3....2
            $0yyx + A=$04  00000100         .3....2.
            $0yyx + B=$08  00001000         3....222
            $0yyx + C=$16  00010110
            $0yyx + D=$21  00100001
            $0yyx + E=$42  01000010
            $0yyx + F=$87  10000111

        */

        // let offset = 256 * y + 16 * x;
        let offset = (y << 8) | (x << 4);
        let mut temp_row = Vec::new();

        for row in 0..8 {
            let real_base = (0x1000 * base) + offset + row;
            let mut lsb = self.ppu_read(real_base, true);
            let mut msb = self.ppu_read(real_base + 8, true);

            temp_row.clear();

            for _shift in 0..8 {
                let pixel_id = ((msb & 1) << 1) | (lsb & 0x01);
                temp_row.push(self.get_color(0, pixel_id as usize));
                lsb >>= 1;
                msb >>= 1;
            }
            temp_row.reverse();

            pattern.append(&mut temp_row);
        }

        pattern
    }

    pub fn set_debug_pattern_screen(&mut self, index: usize, palette: usize) {
        for tile_y in 0..16 {
            for tile_x in 0..16 {
                let offset = (tile_y << 8) | (tile_x << 4);

                for row in 0..8 {
                    let real_base = (0x1000 * index) + offset + row;
                    let mut lsb = self.ppu_read(real_base, true);
                    let mut msb = self.ppu_read(real_base + 8, true);

                    for col in 0..8 {
                        let pixel_id = ((msb & 1) << 1) | (lsb & 0x01);
                        // temp_row.push();
                        lsb >>= 1;
                        msb >>= 1;

                        let x = tile_x * 8 + (7 - col);
                        let y = tile_y * 8 + row;
                        let color = self.get_color(palette, pixel_id as usize);

                        self.screen_debug_pattern[index].set_pixel(x, y, color);
                    }
                }
            }
        }
    }
}
