use crate::utils::Memory;

pub struct Bus {
    pub ram: Vec<u8>,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            ram: vec![0; 0x0800],
        }
    }
}

impl Memory for Bus {
    fn read(&self, addressing: u16, _is_read_only: bool) -> u8 {
        self.ram[(addressing & 0x07ff) as usize]
    }

    fn write(&mut self, addressing: u16, value: u8) {
        self.ram[(addressing & 0x07ff) as usize] = value;
    }
}
