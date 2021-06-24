use crate::cartridge::*;
use crate::cpu::*;
use crate::memory::*;
use crate::ppu::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct NesMemoryMapper {
    ram: Vec<u8>,
    cartridge: CartridgeRef,
    ppu: PPU,
}

impl NesMemoryMapper {
    pub fn new(cartridge: Cartridge) -> NesMemoryMapper {
        let cart_ref = Rc::new(RefCell::new(cartridge));
        NesMemoryMapper {
            cartridge: cart_ref.clone(),
            ram: vec![0; 0x0800],
            ppu: PPU {
                cartridge: cart_ref.clone(),
            },
        }
    }
}

impl Memory for NesMemoryMapper {
    fn read(&mut self, address: usize, is_read_only: bool) -> u8 {
        let data = self.cartridge.borrow_mut().read(address, is_read_only);
        if self.cartridge.borrow().use_cartridge_data() {
            return data;
        } else {
            self.ram[address & 0x07FF]
        }
    }

    fn write(&mut self, address: usize, value: u8) {
        if address < 0x8000 {
            self.ram[address] = value;
        }
    }
}

pub struct Bus {
    memory_mapper: NesMemoryMapper,
    pub cpu: CPU,
    pub cycle: u32,
}

impl Bus {
    pub fn new(cartridge: Cartridge) -> Self {
        Bus {
            memory_mapper: NesMemoryMapper::new(cartridge),
            cpu: CPU::new(),
            cycle: 0,
        }
    }

    pub fn clock(&mut self) {
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
}
