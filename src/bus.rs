use crate::Cartridge;
use crate::Memory;
use crate::CPU;

pub struct Bus {
    pub ram: Vec<u8>,
    pub cartridge: Option<Box<Cartridge>>,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            ram: vec![0; 0x0800],
            cartridge: None,
        }
    }

    pub fn insert_cartridge(&mut self, cart: Cartridge) {
        self.cartridge = Some(Box::new(cart));
    }
}

impl Memory for Bus {
    fn read(&self, address: u16, _is_read_only: bool) -> u8 {
        let mut result: u8 = 0;
        if self
            .cartridge
            .as_ref()
            .unwrap()
            .cpu_read(address, &mut result)
        {
            // skip
        } else if address < 0x2000 {
            result = self.ram[(address & 0x07ff) as usize];
        } else if address < 0x4000 {
            // TODO: PPU here
        } else if (address >= 0x4000 && address < 0x4014) || address == 0x4015 || address == 0x4017
        {
            // TODO: APU here
        } else if address == 0x4016 || address == 0x4017 {
            // TODO: controllers here
        }

        result
    }

    fn write(&mut self, address: u16, value: u8) {
        if self.cartridge.as_mut().unwrap().cpu_write(address, value) {
            // skip
        } else if address < 0x2000 {
            self.ram[(address & 0x07ff) as usize] = value;
        } else if address < 0x4000 {
            // TODO: PPU here
        } else if (address >= 0x4000 && address < 0x4014) || address == 0x4015 || address == 0x4017
        {
            // TODO: APU here
        } else if address == 0x4016 || address == 0x4017 {
            // TODO: controllers here
        }
    }
}

pub struct NES {
    pub bus: Box<Bus>,
    pub cpu: Box<CPU>,
    pub cycles: u32,
    cpuclock: u8,
}

impl NES {
    pub fn new() -> NES {
        NES {
            bus: Box::new(Bus::new()),
            cpu: Box::new(CPU::new()),
            cycles: 0,
            cpuclock: 0,
        }
    }

    pub fn clock(&mut self) {
        if self.cpuclock == 0 {
            self.cpu.as_mut().clock(self.bus.as_mut());
            self.cpuclock = 3;
        }
        self.cycles += 1;
        self.cpuclock -= 1;
    }

    pub fn reset(&mut self) {
        self.cycles = 0;
    }
}
