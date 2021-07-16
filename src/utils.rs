use crate::bus::NesMemoryMapper;
use crate::memory::Memory;
use crate::cpu::types::*;
use std::fmt::Write;

pub struct XORShiftRand {
    state: u64,
}

impl XORShiftRand {
    pub fn new(init: u64) -> XORShiftRand {
        XORShiftRand { state: init }
    }

    pub fn rand(&mut self) -> u64 {
        let mut x = self.state;

        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;

        self.state = x;

        x
    }
}

pub type PPUColor = (u8, u8, u8);

pub struct Screen {
    width: usize,
    height: usize,
    image: Vec<u8>,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Screen {
        Screen {
            width,
            height,
            image: vec![0; width * height * 4],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: PPUColor) {
        let pos = self.width * y + x;
        let pos = pos * 4;

        assert!(pos + 3 < self.width * self.height * 4);

        self.image[pos + 0] = color.0;
        self.image[pos + 1] = color.1;
        self.image[pos + 2] = color.2;
        self.image[pos + 3] = 255;
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn image(&self) -> &Vec<u8> {
        &self.image
    }

    pub fn copy_to(&self, buffer: &mut [u8]) {
        assert_eq!(self.image.len(), buffer.len());

        buffer.copy_from_slice(&self.image);
    }
}

pub fn read_cpu_instructions(nes_memory: &mut NesMemoryMapper, start_address: usize, len: usize) -> Vec<String> {
    let mut instructions = Vec::new();

    let mut current_address = start_address;
    let mut total = 0;

    while current_address < 0xffff && total < len {
        let mut formatted_instruction = String::new();
        let opcode = nes_memory.read(current_address, true);
        current_address += 1;

        let (adressing_mode, opcode, _) = OPCODE_TABLE[opcode as usize];
        write!(formatted_instruction, " {:04X}: {}", (current_address - 1), opcode.to_string().to_uppercase()).unwrap();

        match adressing_mode {
            AddressMode::Imp => {
                // do nothing
            }
            AddressMode::Acc => {
                write!(formatted_instruction, " A").unwrap();
            }
            AddressMode::Imm => {
                let param = nes_memory.read(current_address, true);
                write!(formatted_instruction, " #${:02X}", param).unwrap();
                current_address += 1;
            }
            AddressMode::Zp0 => {
                let param = nes_memory.read(current_address, true);
                write!(formatted_instruction, " ${:02X}", param).unwrap();
                current_address += 1;
            }
            AddressMode::Zpx => {
                let param = nes_memory.read(current_address, true);
                write!(formatted_instruction, " ${:02X},X", param).unwrap();
                current_address += 1;
            }
            AddressMode::Zpy => {
                let param = nes_memory.read(current_address, true);
                write!(formatted_instruction, " ${:02X},Y", param).unwrap();
                current_address += 1;
            }
            AddressMode::Abs => {
                let lo = nes_memory.read(current_address, true) as u16;
                current_address += 1;
                let hi = nes_memory.read(current_address, true) as u16;
                current_address += 1;
                let param = (hi << 8) | lo;
                write!(formatted_instruction, " ${:04X}", param).unwrap();
            }
            AddressMode::Abx => {
                let lo = nes_memory.read(current_address, true) as u16;
                current_address += 1;
                let hi = nes_memory.read(current_address, true) as u16;
                current_address += 1;
                let param = (hi << 8) | lo;
                write!(formatted_instruction, " ${:04X},X", param).unwrap();
            }
            AddressMode::Aby => {
                let lo = nes_memory.read(current_address, true) as u16;
                current_address += 1;
                let hi = nes_memory.read(current_address, true) as u16;
                current_address += 1;
                let param = (hi << 8) | lo;
                write!(formatted_instruction, " ${:04X},Y", param).unwrap();
            }
            AddressMode::Izx => {
                let param = nes_memory.read(current_address, true);
                write!(formatted_instruction, " (${:02X},X)", param).unwrap();
                current_address += 1;
            }
            AddressMode::Izy => {
                let param = nes_memory.read(current_address, true);
                write!(formatted_instruction, " (${:02X}),Y", param).unwrap();
                current_address += 1;
            }
            AddressMode::Ind => {
                let lo = nes_memory.read(current_address, true) as u16;
                current_address += 1;
                let hi = nes_memory.read(current_address, true) as u16;
                current_address += 1;
                let param = (hi << 8) | lo;
                write!(formatted_instruction, " (${:04X})", param).unwrap();
            }
            AddressMode::Rel => {
                let offset = nes_memory.read(current_address, true) as i8;
                current_address += 1;
                let param = (current_address as isize) + (offset as isize);
                write!(formatted_instruction, " ${:04X}", param).unwrap();
            }
        }

        instructions.push(formatted_instruction);
        total += 1;
    }

    instructions
}
