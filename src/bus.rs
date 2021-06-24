use crate::cartridge::*;
use crate::memory::*;

pub struct NesMemoryMapper {
    ram: Vec<u8>,
    cartridge: Cartridge,
}

impl NesMemoryMapper {
    pub fn new(cartridge: Cartridge) -> NesMemoryMapper {
        NesMemoryMapper {
            cartridge,
            ram: vec![0; 0x0800],
        }
    }
}

impl Memory for NesMemoryMapper {
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
