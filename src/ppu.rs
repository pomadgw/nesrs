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
pub const OAMDMA: usize = 0x4014;

pub const NES_WIDTH_SIZE: usize = 256;
pub const NES_HEIGHT_SIZE: usize = 240;

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
const PPUADDRRESS_NAMETABLE_X_SELECT_MASK: usize = 0b000_01_00000_00000;
const PPUADDRRESS_NAMETABLE_Y_SELECT_MASK: usize = 0b000_10_00000_00000;
const PPUADDRRESS_FINE_Y_MASK: usize = 0b111_00_00000_00000;

impl PPUAddress {
    pub fn new() -> PPUAddress {
        PPUAddress { address: 0 }
    }

    pub fn address(&self) -> usize {
        self.address & 0b0111_1111_1111_1111
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

    pub fn nametable_select_x(&self) -> usize {
        (self.address & PPUADDRRESS_NAMETABLE_X_SELECT_MASK) >> 10
    }

    pub fn set_nametable_select_x(&mut self, value: usize) {
        self.address &= !PPUADDRRESS_NAMETABLE_X_SELECT_MASK;
        self.address |= (value << 10) & PPUADDRRESS_NAMETABLE_X_SELECT_MASK;
    }

    pub fn toggle_nametable_select_x(&mut self) {
        self.address ^= PPUADDRRESS_NAMETABLE_X_SELECT_MASK;
    }

    pub fn nametable_select_y(&self) -> usize {
        (self.address & PPUADDRRESS_NAMETABLE_Y_SELECT_MASK) >> 11
    }

    pub fn set_nametable_select_y(&mut self, value: usize) {
        self.address &= !PPUADDRRESS_NAMETABLE_Y_SELECT_MASK;
        self.address |= (value << 11) & PPUADDRRESS_NAMETABLE_Y_SELECT_MASK;
    }

    pub fn toggle_nametable_select_y(&mut self) {
        self.address ^= PPUADDRRESS_NAMETABLE_Y_SELECT_MASK;
    }

    pub fn fine_y(&self) -> usize {
        (self.address & PPUADDRRESS_FINE_Y_MASK) >> 12
    }

    pub fn set_fine_y(&mut self, value: usize) {
        self.address &= !PPUADDRRESS_FINE_Y_MASK;
        self.address |= (value << 12) & PPUADDRRESS_FINE_Y_MASK;
    }

    pub fn increase_coarse_x(&mut self) {
        if self.coarse_x() == 31 {
            self.set_coarse_x(0);
            self.toggle_nametable_select_x();
        } else {
            self.address += 1;
        }
    }

    pub fn increase_coarse_y(&mut self) {
        if self.fine_y() < 7 {
            self.address += 0x1000;
        } else {
            self.set_fine_y(0);
            let coarse_y = self.coarse_y();

            if coarse_y == 29 {
                self.set_coarse_y(0);
                self.toggle_nametable_select_y();
            } else if coarse_y == 31 {
                self.set_coarse_y(0);
            } else {
                self.set_coarse_y(coarse_y + 1);
            }
        }
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

impl PPUMask {
    pub fn is_render_bg(&self) -> bool {
        self.contains(PPUMask::SHOW_BG)
    }

    pub fn is_render_sprite(&self) -> bool {
        self.contains(PPUMask::SHOW_SPRITE)
    }

    pub fn is_render_something(&self) -> bool {
        self.is_render_bg() || self.is_render_sprite()
    }

    pub fn is_render_left(&self) -> bool {
        self.contains(PPUMask::SHOW_BG_LEFT) || self.contains(PPUMask::SHOW_SPRITE_LEFT)
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

#[derive(Debug, Clone)]
struct ShiftRegister16 {
    pub lo: u16,
    pub hi: u16,
}

impl ShiftRegister16 {
    pub fn new() -> ShiftRegister16 {
        ShiftRegister16 { lo: 0, hi: 0 }
    }

    pub fn get(&self, index: usize) -> usize {
        let bitmux = 0x8000 >> index;
        let lo = if (self.lo & bitmux) > 0 { 1 } else { 0 };
        let hi = if (self.hi & bitmux) > 0 { 1 } else { 0 };

        ((hi << 1) | lo) as usize
    }

    pub fn shift(&mut self) {
        self.lo <<= 1;
        self.hi <<= 1;
    }

    pub fn load_lo(&mut self, value: u8) {
        self.lo &= 0xff00;
        self.lo |= value as u16;
    }

    pub fn load_hi(&mut self, value: u8) {
        self.hi &= 0xff00;
        self.hi |= value as u16;
    }
}

#[derive(Debug, Clone)]
pub struct OAM {
    pub y: u8,
    pub id: u8,
    pub attr: u8,
    pub x: u8,
}

impl OAM {
    pub fn new() -> OAM {
        OAM {
            y: 0xff,
            id: 0xff,
            attr: 0xff,
            x: 0xff,
        }
    }

    pub fn reset(&mut self) {
        self.y = 0xff;
        self.id = 0xff;
        self.attr = 0xff;
        self.x = 0xff;
    }
}

pub struct OAMS {
    oams: Vec<OAM>,
}

use std::ops::{Index, IndexMut};

impl OAMS {
    pub fn new(size: usize) -> OAMS {
        OAMS {
            oams: vec![OAM::new(); size],
        }
    }

    pub fn get(&self, index: usize) -> &OAM {
        &self.oams[index]
    }

    pub fn get_mut(&mut self, index: usize) -> &mut OAM {
        &mut self.oams[index]
    }

    pub fn reset(&mut self) {
        for oam in &mut self.oams {
            oam.reset();
        }
    }
}

impl Index<usize> for OAMS {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        match index & 0x03 {
            0 => &self.oams[index >> 2].y,
            1 => &self.oams[index >> 2].id,
            2 => &self.oams[index >> 2].attr,
            3 => &self.oams[index >> 2].x,
            _ => &0,
        }
    }
}

impl IndexMut<usize> for OAMS {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index & 0x03 {
            0 => &mut self.oams[index >> 2].y,
            1 => &mut self.oams[index >> 2].id,
            2 => &mut self.oams[index >> 2].attr,
            3 => &mut self.oams[index >> 2].x,
            _ => panic!("Invalid OAM index"),
        }
    }
}

enum PPUSpriteRead {
    ReadY,
    ReadRest,
    OnSpriteOverflow,
}

pub struct PPU {
    pub cartridge: CartridgeRef,
    pattern_table: [[u8; 0x1000]; 2], // 0x0000 - 0x1fff
    nametable: [[u8; 0x0400]; 2],     // 0x2000 - 0x2fff
    palette_table: [u8; 32],          // 0x3f00 - 0x3fff

    oam_address: usize,
    pub oams: OAMS,
    internal_oams: OAMS,
    pub internal_oams_debug: OAMS,
    next_scanline_oams: OAMS,
    next_scanline_sprite_count: usize,
    internal_oam_address: usize,
    sprite_count: usize,
    sprite_read_mode: PPUSpriteRead,
    curr_oam_data: u8,
    is_sprite0_hit_possible: bool,
    is_sprite0_hit_being_rendered: bool,

    screen: Screen,
    cycle: i32,
    scanline: i32,
    pub done_drawing: bool,
    pub call_nmi: bool,

    status: PPUStatus,
    control: PPUControl,
    mask: PPUMask,
    address_latch: AddressLatch,
    data_buffer: u8,
    fine_x: usize,

    temp_address: PPUAddress,
    vaddress: PPUAddress,
    bg_next_tile_id: u8,
    bg_next_tile_attrib: u8,
    bg_next_tile_lsb: u8,
    bg_next_tile_msb: u8,

    bg_pattern_shifter: ShiftRegister16,
    bg_attrib_shifter: ShiftRegister16,
    sprite_pattern_shifter: Vec<ShiftRegister16>,

    // for debug
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
            OAMADDR => 0xff,
            OAMDATA => self.oams[self.oam_address],
            PPUSCROLL => 0,
            PPUADDR => 0,
            PPUDATA => {
                let read_result = self.ppu_read(self.vaddress.into(), is_read_only);

                // result the buffer data...self.control
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
                self.temp_address.set_nametable_select_x(
                    if self.control.contains(PPUControl::NAMETABLE_X) {
                        1
                    } else {
                        0
                    },
                );
                self.temp_address.set_nametable_select_y(
                    if self.control.contains(PPUControl::NAMETABLE_Y) {
                        1
                    } else {
                        0
                    },
                );
            }
            PPUMASK => {
                self.mask.bits = value;
            }
            PPUSTATUS => {}
            OAMADDR => {
                self.oam_address = value as usize;
            }
            OAMDATA => {
                self.write_oam_address(self.oam_address, value);
                self.oam_address = self.oam_address.wrapping_add(1);
            }
            PPUSCROLL => match self.address_latch {
                AddressLatch::Hi => {
                    self.address_latch = AddressLatch::Lo;
                    self.fine_x = (value & 0x07) as usize;
                    self.temp_address.set_coarse_x((value >> 3) as usize);
                }
                AddressLatch::Lo => {
                    self.address_latch = AddressLatch::Hi;
                    self.temp_address.set_fine_y((value & 0x07) as usize);
                    self.temp_address.set_coarse_y((value >> 3) as usize);
                }
            },
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
                    self.vaddress.set_address(self.temp_address.address());
                }
            },
            PPUDATA => {
                self.ppu_write(self.vaddress.address(), value);
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
            oam_address: 0,
            oams: OAMS::new(64),
            internal_oams: OAMS::new(8),
            internal_oams_debug: OAMS::new(8),
            next_scanline_oams: OAMS::new(8),
            next_scanline_sprite_count: 0,
            sprite_count: 0,
            curr_oam_data: 0,
            internal_oam_address: 0,
            sprite_read_mode: PPUSpriteRead::ReadY,
            is_sprite0_hit_possible: false,
            is_sprite0_hit_being_rendered: false,

            screen: Screen::new(NES_WIDTH_SIZE, NES_HEIGHT_SIZE),
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
            fine_x: 0,
            bg_next_tile_id: 0,
            bg_next_tile_attrib: 0,
            bg_next_tile_lsb: 0,
            bg_next_tile_msb: 0,

            bg_pattern_shifter: ShiftRegister16::new(),
            bg_attrib_shifter: ShiftRegister16::new(),
            sprite_pattern_shifter: vec![ShiftRegister16::new(); 8],

            screen_debug_pattern: [Screen::new(128, 128), Screen::new(128, 128)],
        }
    }

    fn load_background_shifters(&mut self) {
        self.bg_pattern_shifter.load_lo(self.bg_next_tile_lsb);
        self.bg_pattern_shifter.load_hi(self.bg_next_tile_msb);

        self.bg_attrib_shifter
            .load_lo(if self.bg_next_tile_attrib & 0b01 > 0 {
                0xff
            } else {
                0
            });
        self.bg_attrib_shifter
            .load_hi(if self.bg_next_tile_attrib & 0b10 > 0 {
                0xff
            } else {
                0
            });
    }

    fn reset_sprite_pattern_shifter(&mut self) {
        for i in 0..7 {
            self.sprite_pattern_shifter[i].load_lo(0);
            self.sprite_pattern_shifter[i].load_hi(0);
        }
    }

    fn update_shitfers(&mut self) {
        if self.mask.is_render_bg() {
            self.bg_pattern_shifter.shift();
            self.bg_attrib_shifter.shift();
        }

        if self.mask.is_render_sprite() && 1 <= self.cycle && self.cycle <= 258 {
            for sprite_index in 0..self.next_scanline_sprite_count {
                let mut sprite = self.next_scanline_oams.get_mut(sprite_index);

                if sprite.x > 0 {
                    sprite.x -= 1;
                } else {
                    self.sprite_pattern_shifter[sprite_index].shift();
                    // println!("SHIFT SPRITE L: {:016b}", self.sprite_pattern_shifter[sprite_index].lo);
                    // println!("SHIFT SPRITE H: {:016b}", self.sprite_pattern_shifter[sprite_index].hi);
                }
            }
        }
    }

    pub fn clock(&mut self) {
        // TODO: implement clock
        if self.cycle == 1 && self.scanline == -1 {
            self.status.set(PPUStatus::VBLANK, false);
            self.status.set(PPUStatus::SPRITE_OVERFLOW, false);
            self.status.set(PPUStatus::SPRITE0_HIT, false);
            self.reset_sprite_pattern_shifter();

            self.is_sprite0_hit_possible = false;
            self.is_sprite0_hit_being_rendered = false;
        }

        if self.cycle == 1 && self.scanline == 241 {
            self.status.set(PPUStatus::VBLANK, true);

            if self.control.contains(PPUControl::ENABLE_NMI) {
                self.call_nmi = true;
            }
        }

        // visible scanline...
        if -1 <= self.scanline && self.scanline < 240 {
            if (1 <= self.cycle && self.cycle <= 256) || (321 <= self.cycle && self.cycle < 338) {
                self.update_shitfers();

                match (self.cycle - 1) & 0x07 {
                    0 => {
                        self.load_background_shifters();

                        self.bg_next_tile_id =
                            self.ppu_read(0x2000 | (self.vaddress.address() & 0x0fff), false);
                    }
                    2 => {
                        /*
                        The low 12 bits of the attribute address are composed in the following way:
                        NN 1111 YYY XXX
                        || |||| ||| +++-- high 3 bits of coarse X (x/4)
                        || |||| +++------ high 3 bits of coarse Y (y/4)
                        || ++++---------- attribute offset (960 bytes)
                        ++--------------- nametable select
                        */
                        let chosen_nametable = self.vaddress.address()
                            & (PPUADDRRESS_NAMETABLE_X_SELECT_MASK
                                | PPUADDRRESS_NAMETABLE_Y_SELECT_MASK);
                        let chosen_coarse_y = (self.vaddress.address() >> 4) & 0x38;
                        let chosen_coarse_x = (self.vaddress.address() >> 2) & 0x07;
                        self.bg_next_tile_attrib = self.ppu_read(
                            0x23c0 | chosen_nametable | chosen_coarse_y | chosen_coarse_x,
                            false,
                        );

                        if self.vaddress.coarse_y() & 0x02 > 0 {
                            self.bg_next_tile_attrib >>= 4;
                        }
                        if self.vaddress.coarse_x() & 0x02 > 0 {
                            self.bg_next_tile_attrib >>= 2;
                        }

                        self.bg_next_tile_attrib &= 0x03;
                    }
                    4 => {
                        let mut address =
                            if self.control.contains(PPUControl::BG_PATTERN_TABLE_ADDRESS) {
                                1 << 12
                            } else {
                                0
                            };
                        address += (self.bg_next_tile_id as usize) << 4;
                        address += self.vaddress.fine_y();
                        self.bg_next_tile_lsb = self.ppu_read(address, false);
                    }
                    6 => {
                        let mut address =
                            if self.control.contains(PPUControl::BG_PATTERN_TABLE_ADDRESS) {
                                1 << 12
                            } else {
                                0
                            };
                        address += (self.bg_next_tile_id as usize) << 4;
                        address += self.vaddress.fine_y() + 8;
                        self.bg_next_tile_msb = self.ppu_read(address, false);
                    }
                    7 => {
                        if self.mask.is_render_something() {
                            self.vaddress.increase_coarse_x();
                        }
                    }
                    _ => {}
                }
            }

            // sprite evaluation
            let sprint_size = if self.control.contains(PPUControl::SPRITE_SIZE) {
                16
            } else {
                8
            };

            match self.cycle {
                0 => {
                    self.sprite_count = 0;
                    self.internal_oam_address = 0;
                    self.sprite_read_mode = PPUSpriteRead::ReadY;
                }
                1..=64 => {
                    self.internal_oams[((self.cycle as usize) - 1) >> 1] = 0xff;
                }
                65..=256 => {
                    if self.oam_address < 256 && self.sprite_count < 9 {
                        if self.cycle & 0x01 == 1 {
                            // read OAM entry
                            self.curr_oam_data = self.oams[self.oam_address];
                            // println!("{:3} self.oam_n({:2}).y = {:02X}", self.scanline, self.oam_n, self.curr_oam_data);
                        } else {
                            match self.sprite_read_mode {
                                PPUSpriteRead::ReadY => {
                                    assert!(self.oam_address & 0x03 == 0);

                                    self.internal_oams[self.internal_oam_address] =
                                        self.curr_oam_data;

                                    self.internal_oams_debug[self.internal_oam_address] =
                                        self.curr_oam_data;

                                    let diff = self.scanline - (self.curr_oam_data as i32);

                                    if diff >= 0 && diff < sprint_size {
                                        if self.oam_address == 0 {
                                            self.is_sprite0_hit_possible = true;
                                        }

                                        self.sprite_read_mode = PPUSpriteRead::ReadRest;
                                        self.oam_address += 1;
                                        self.internal_oam_address += 1;
                                    } else {
                                        self.oam_address += 4;
                                    }
                                }
                                PPUSpriteRead::ReadRest => {
                                    self.internal_oams[self.internal_oam_address] =
                                        self.curr_oam_data;
                                    self.internal_oams_debug[self.internal_oam_address] =
                                        self.curr_oam_data;

                                    self.oam_address += 1;
                                    self.internal_oam_address += 1;

                                    if self.internal_oam_address == 32 {
                                        self.sprite_read_mode = PPUSpriteRead::OnSpriteOverflow;
                                    } else if self.internal_oam_address & 0x03 == 0 {
                                        self.sprite_read_mode = PPUSpriteRead::ReadY;
                                        self.sprite_count += 1;
                                        // println!("SPRINT COUNT: {}", self.sprite_count);
                                    }
                                }
                                PPUSpriteRead::OnSpriteOverflow => {
                                    self.status.set(PPUStatus::SPRITE_OVERFLOW, true);
                                }
                            }
                        }
                    }
                }
                257 => {
                    // todo: does not conform with the actual sprint evaluation

                    for oam_index in 0..32 {
                        self.next_scanline_oams[oam_index] = self.internal_oams[oam_index];
                    }

                    self.next_scanline_sprite_count = self.sprite_count;

                    let is_sprite16_mode = self.control.contains(PPUControl::SPRITE_SIZE);
                    let pattern_sprite = if self
                        .control
                        .contains(PPUControl::SPRITE_PATTERN_TABLE_ADDRESS)
                    {
                        1
                    } else {
                        0
                    };

                    for sprite_index in 0..self.next_scanline_sprite_count {
                        let sprite = self.next_scanline_oams.get(sprite_index);
                        let id = sprite.id as usize;
                        let attr = sprite.attr;
                        let y = sprite.y as i32;
                        // println!("ID: {:02X}, ATTR: {:02X}", id, attr);
                        let sprite_pattern_address_lo: usize;

                        if is_sprite16_mode {
                            // 8x16 sprite mode
                            if !((attr & 0x80) > 0) {
                                // Sprite is normal, not flipped
                                if self.scanline - y < 8 {
                                    // reading top half
                                    sprite_pattern_address_lo = ((id & 0x01) << 12)
                                        | ((id & 0xfe) << 4)
                                        | ((self.scanline - y) & 0x07) as usize;
                                } else {
                                    // reading bottom half
                                    sprite_pattern_address_lo = ((id & 0x01) << 12)
                                        | (((id & 0xfe) + 1) << 4)
                                        | ((self.scanline - y) & 0x07) as usize;
                                }
                            } else {
                                // The sprite is flipped vertically
                                if self.scanline - y < 8 {
                                    // reading top half
                                    sprite_pattern_address_lo = ((id & 0x01) << 12)
                                        | (((id & 0xfe) + 1) << 4)
                                        | (7 - ((self.scanline - y) & 0x07)) as usize;
                                } else {
                                    // reading bottom half
                                    sprite_pattern_address_lo = ((id & 0x01) << 12)
                                        | ((id & 0xfe) << 4)
                                        | (7 - ((self.scanline - y) & 0x07)) as usize;
                                }
                            }
                        } else {
                            // 8x8 sprite mode
                            if !((attr & 0x80) > 0) {
                                // Sprite is normal, not flipped
                                sprite_pattern_address_lo = (pattern_sprite << 12) | (id << 4) | (self.scanline - y) as usize;
                                // println!("8x8 sprite not flipped: {:3} - {:3}", self.scanline, y);
                            } else {
                                // println!("8x8 sprite flipped");
                                // The sprite is flipped vertically
                                sprite_pattern_address_lo = (pattern_sprite << 12) | (id << 4) | (7 - (self.scanline - y)) as usize;
                            }
                        }

                        let sprite_pattern_address_hi = sprite_pattern_address_lo + 8;

                        let mut sprite_pattern_bits_lo = self.ppu_read(sprite_pattern_address_lo, false);
                        let mut sprite_pattern_bits_hi = self.ppu_read(sprite_pattern_address_hi, false);
                        // println!("L {:04X} => {:08b}", sprite_pattern_address_lo, sprite_pattern_bits_lo);
                        // println!("H {:04X} => {:08b}", sprite_pattern_address_hi, sprite_pattern_bits_hi);

                        let is_flipped_horizontally = (attr & 0x40) > 0;

                        if is_flipped_horizontally {
                            sprite_pattern_bits_lo = ((sprite_pattern_bits_lo & 0xf0) >> 4) | ((sprite_pattern_bits_lo & 0x0f) << 4);
                            sprite_pattern_bits_lo = ((sprite_pattern_bits_lo & 0xcc) >> 2) | ((sprite_pattern_bits_lo & 0x33) << 2);
                            sprite_pattern_bits_lo = ((sprite_pattern_bits_lo & 0xaa) >> 1) | ((sprite_pattern_bits_lo & 0x55) << 1);

                            sprite_pattern_bits_hi = ((sprite_pattern_bits_hi & 0xf0) >> 4) | ((sprite_pattern_bits_hi & 0x0f) << 4);
                            sprite_pattern_bits_hi = ((sprite_pattern_bits_hi & 0xcc) >> 2) | ((sprite_pattern_bits_hi & 0x33) << 2);
                            sprite_pattern_bits_hi = ((sprite_pattern_bits_hi & 0xaa) >> 1) | ((sprite_pattern_bits_hi & 0x55) << 1);
                        }

                        self.sprite_pattern_shifter[sprite_index].load_lo(sprite_pattern_bits_lo);
                        self.sprite_pattern_shifter[sprite_index].load_hi(sprite_pattern_bits_hi);
                    }
                }
                _ => {}
            }

            if self.cycle == 256 {
                if self.mask.is_render_something() {
                    self.vaddress.increase_coarse_y();
                }
            } else if self.cycle == 257 {
                if self.mask.is_render_something() {
                    self.vaddress
                        .set_nametable_select_x(self.temp_address.nametable_select_x());
                    self.vaddress.set_coarse_x(self.temp_address.coarse_x());
                }

                self.oam_address = 0;
            }

            if self.scanline == -1 && self.cycle >= 280 && self.cycle < 305 {
                // End of vertical blank period so reset the Y address ready for rendering
                if self.mask.is_render_something() {
                    self.vaddress
                        .set_nametable_select_y(self.temp_address.nametable_select_y());
                    self.vaddress.set_coarse_y(self.temp_address.coarse_y());
                    self.vaddress.set_fine_y(self.temp_address.fine_y());
                }
            }
        }

        if self.cycle < 256 && (0 <= self.scanline && self.scanline < 240) {
            let mut palette = 0;
            let mut pixel = 0;

            let mut bg_palette = 0;
            let mut bg_pixel = 0;

            if self.mask.is_render_bg() {
                bg_pixel = self.bg_pattern_shifter.get(self.fine_x);
                bg_palette = self.bg_attrib_shifter.get(self.fine_x);
            }

            let mut fg_palette = 0;
            let mut fg_pixel = 0;
            let mut fg_priority = false;

            if self.mask.is_render_sprite() {
                if self.next_scanline_sprite_count > 0 {
                    // println!("RENDER SPRITE! {}", self.next_scanline_sprite_count);
                }

                self.is_sprite0_hit_being_rendered = false;

                for sprite_index in 0..self.next_scanline_sprite_count {
                    // println!("{} X = {:3}", sprite_index, self.next_scanline_oams.get(sprite_index).x);

                    if self.next_scanline_oams.get(sprite_index).x == 0 {
                        fg_pixel = self.sprite_pattern_shifter[sprite_index].get(8);
                        // if fg_pixel > 0 { println!("fg_pixel: {}", fg_pixel); }
                        // println!("{}", self.sprite_pattern_shifter[sprite_index].lo);
                        // println!("{}", self.sprite_pattern_shifter[sprite_index].hi);
                        fg_palette = (self.next_scanline_oams.get(sprite_index).attr & 0x03) + 0x04;
                        fg_priority = self.next_scanline_oams.get(sprite_index).attr & 0x20 == 0;

                        if fg_pixel != 0 {
                            if sprite_index == 0 {
                                self.is_sprite0_hit_being_rendered = true;
                            }
                            break;
                        }
                    }
                }
            }

            match (bg_pixel, fg_pixel) {
                (0, 0) => {
                    // skip, use default
                }
                (0, _) => {
                    pixel = fg_pixel as usize;
                    palette = fg_palette as usize;
                }
                (_, 0) => {
                    pixel = bg_pixel;
                    palette = bg_palette;
                }
                (_, _) => {
                    if fg_priority {
                        pixel = fg_pixel as usize;
                        palette = fg_palette as usize;
                    } else {
                        pixel = bg_pixel;
                        palette = bg_palette;
                    }

                    if self.is_sprite0_hit_possible && self.is_sprite0_hit_being_rendered {
                        if self.mask.is_render_something() {
                            if self.mask.is_render_left() {
                                if 1 <= self.cycle && self.cycle < 258 {
                                    self.status.set(PPUStatus::SPRITE0_HIT, true);
                                }
                            } else {
                                if 1 <= self.cycle && self.cycle < 258 {
                                    self.status.set(PPUStatus::SPRITE0_HIT, true);
                                }
                            }
                        }
                    }
                }
            }

            let color = self.get_color(palette, pixel);

            self.screen
                .set_pixel(self.cycle as usize, self.scanline as usize, color);
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

    pub fn write_oam_address(&mut self, address: usize, value: u8) {
        self.oams[address] = value;
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

    pub fn screen(&self) -> &Screen {
        &self.screen
    }

    pub fn get_screen_buffer_pointer(&self) -> *const u8 {
        let pointer: *const u8;
        pointer = self.screen.image().as_ptr();

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
                )
                .unwrap();
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
