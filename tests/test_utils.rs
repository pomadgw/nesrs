use nesrs::memory::*;

pub struct RAM {
    pub ram: Vec<u8>,
}

impl RAM {
    pub fn new() -> RAM {
        RAM {
            ram: vec![0; 0x10000],
        }
    }
}

impl Memory for RAM {
    fn read(&self, address: usize, _is_read_only: bool) -> u8 {
        self.ram[address]
    }

    fn write(&mut self, address: usize, value: u8) {
        self.ram[address] = value;
    }
}
