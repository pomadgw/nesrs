use crate::cartridge::*;
use crate::cpu::*;
use crate::memory::*;
use crate::ppu::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct NesMemoryMapper {
    ram: Vec<u8>,
    cartridge: CartridgeRef,
    ppu: PPURef,
}

impl NesMemoryMapper {
    pub fn new(ppu: PPURef, cartridge: CartridgeRef) -> NesMemoryMapper {
        NesMemoryMapper {
            cartridge: cartridge.clone(),
            ram: vec![0; 0x0800],
            ppu: ppu.clone(),
        }
    }
}

impl Memory for NesMemoryMapper {
    fn read(&mut self, address: usize, is_read_only: bool) -> u8 {
        let data = self.cartridge.borrow_mut().read(address, is_read_only);
        if self.cartridge.borrow().use_cartridge_data() {
            return data;
        } else if address < 0x2000 {
            self.ram[address & 0x07FF]
        } else if address < 0x4000 {
            // TODO: PPU here
            self.ppu.borrow_mut().read(address & 0x07, is_read_only)
        } else if address == 0x4014 {
            // TODO: OAMDMA
            0
        } else if address <= 0x4013 || (address == 0x4015) || (address == 0x4017) {
            // TODO: APU here
            0
        } else if address == 0x4016 || address == 0x4017 {
            // TODO: controller
            0
        } else {
            self.ram[address & 0x07FF]
        }
    }

    fn write(&mut self, address: usize, value: u8) {
        self.cartridge.borrow_mut().write(address, value);

        if self.cartridge.borrow().use_cartridge_data() {
        } else if address < 0x2000 {
            self.ram[address & 0x07FF] = value;
        } else if address < 0x4000 {
            // TODO: PPU here
            self.ppu.borrow_mut().write(address & 0x07, value)
        } else if address == 0x4014 {
            // TODO: OAMDMA
        } else if address <= 0x4013 || (address == 0x4015) || (address == 0x4017) {
            // TODO: APU here
        } else if address == 0x4016 || address == 0x4017 {
            // TODO: controller
        } else {
            self.ram[address & 0x07FF] = value;
        }
    }
}

pub struct Bus {
    memory_mapper: NesMemoryMapper,
    pub cpu: CPU,
    pub cycle: u32,
    pub ppu: PPURef,
}

impl Bus {
    pub fn new(cartridge: Cartridge) -> Self {
        let cartref = Rc::new(RefCell::new(cartridge));
        let ppu = Rc::new(RefCell::new(PPU::new(cartref.clone())));

        Bus {
            memory_mapper: NesMemoryMapper::new(ppu.clone(), cartref),
            cpu: CPU::new(),
            cycle: 0,
            ppu,
        }
    }

    pub fn new_from_array(array: &Vec<u8>) -> Self {
        Self::new(Cartridge::parse(array))
    }

    pub fn clock(&mut self) {
        self.ppu.borrow_mut().clock();
        match self.cycle {
            0 | 3 => {
                self.cpu.clock(&mut self.memory_mapper);
            }
            _ => {}
        }
        if self.cycle == 0 {
            self.cycle = 6;
        }

        self.cycle -= 1;
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    pub fn memory(&mut self) -> &mut NesMemoryMapper {
        &mut self.memory_mapper
    }
}
