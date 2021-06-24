use crate::cartridge::*;
use crate::cpu::*;
use crate::memory::*;

pub struct NesMemoryMapper<'a> {
    ram: Vec<u8>,
    cartridge: &'a mut Cartridge,
}

impl<'a> NesMemoryMapper<'a> {
    pub fn new(cartridge: &'a mut Cartridge) -> NesMemoryMapper {
        NesMemoryMapper {
            cartridge,
            ram: vec![0; 0x0800],
        }
    }
}

impl Memory for NesMemoryMapper<'_> {
    fn read(&mut self, address: usize, is_read_only: bool) -> u8 {
        let data = self.cartridge.read(address, is_read_only);
        if self.cartridge.use_cartridge_data() {
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

pub struct Bus<'a> {
    memory_mapper: NesMemoryMapper<'a>,
    pub cpu: CPU,
    pub cycle: u32,
}

impl<'a> Bus<'a> {
    pub fn new(cartridge: &'a mut Cartridge) -> Self {
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
