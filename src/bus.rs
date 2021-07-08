use crate::cartridge::*;
use crate::controller::*;
use crate::cpu::*;
use crate::memory::*;
use crate::ppu::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct NesMemoryMapper {
    ram: Vec<u8>,
    cartridge: CartridgeRef,
    ppu: PPURef,
    controllers: Vec<ControllerRef>,
}

impl NesMemoryMapper {
    pub fn new(
        ppu: PPURef,
        cartridge: CartridgeRef,
        controllers: Vec<ControllerRef>,
    ) -> NesMemoryMapper {
        NesMemoryMapper {
            cartridge,
            ram: vec![0; 0x0800],
            ppu,
            controllers,
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
            self.ppu.borrow_mut().read(address & 0x07, is_read_only)
        } else if address == 0x4014 {
            // TODO: OAMDMA
            0
        } else if address <= 0x4013 || (address == 0x4015) || (address == 0x4017) {
            // TODO: APU here
            0
        } else if address == 0x4016 || address == 0x4017 {
            self.controllers[address & 1].borrow_mut().read()
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
            self.controllers[address & 1].borrow_mut().write(value);
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
    pub controller1: ControllerRef,
    pub controller2: ControllerRef,
}

impl Bus {
    pub fn new(cartridge: Cartridge) -> Self {
        let cartref = Rc::new(RefCell::new(cartridge));
        let ppu = Rc::new(RefCell::new(PPU::new(cartref.clone())));
        let controller1 = Controller::new_ref();
        let controller2 = Controller::new_ref();
        let controllers = vec![controller1.clone(), controller2.clone()];

        Bus {
            memory_mapper: NesMemoryMapper::new(ppu.clone(), cartref, controllers),
            cpu: CPU::new(),
            cycle: 0,
            ppu,
            controller1,
            controller2,
        }
    }

    pub fn new_from_array(array: &Vec<u8>) -> Self {
        Self::new(Cartridge::parse(array))
    }

    pub fn clock(&mut self) {
        self.ppu.borrow_mut().clock();

        if self.ppu.borrow().call_nmi {
            self.ppu.borrow_mut().call_nmi = false;
            self.cpu.nmi();
        }

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

    pub fn clock_until_frame_done(&mut self) {
        while !self.ppu.borrow().done_drawing {
            self.clock();
        }

        self.ppu.borrow_mut().done_drawing = false;
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    pub fn memory(&mut self) -> &mut NesMemoryMapper {
        &mut self.memory_mapper
    }
}
