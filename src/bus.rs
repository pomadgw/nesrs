use crate::cartridge::*;
use crate::controller::*;
use crate::cpu::*;
use crate::memory::*;
use crate::ppu::*;
use std::sync::{Arc, Mutex};

pub struct NesMemoryMapper {
    ram: Vec<u8>,
    cartridge: CartridgeRef,
    ppu: PPURef,
    pub controllers: Vec<ControllerRef>,

    oam_dma_page: u8,
    oam_dma_address: u8,
    dma_data: u8,
    pub do_oam_dma: bool,
    pub oam_dma_cycle: i8,
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
            oam_dma_page: 0,
            oam_dma_address: 0,
            dma_data: 0,
            oam_dma_cycle: 1,
            do_oam_dma: false,
        }
    }

    pub fn transfer_oam(&mut self, cycle: u32) {
        if self.oam_dma_cycle > 0 {
            if cycle & 1 == 1 {
                self.oam_dma_cycle += 1;
            }

            self.oam_dma_cycle -= 1;
        } else {
            if cycle & 1 == 1 {
                let address = (self.oam_dma_page as usize) << 8 | (self.oam_dma_address as usize);
                self.dma_data = self.read(address, false);
            } else {
                let mut ppu = self.ppu.lock().unwrap();
                let address = ppu.oam_address;
                ppu.write_oam_address(address as usize, self.dma_data);
                self.oam_dma_address = self.oam_dma_address.wrapping_add(1);
                ppu.oam_address = address.wrapping_add(1);

                if self.oam_dma_address == 0 {
                    self.oam_dma_cycle = 1;
                    self.do_oam_dma = false;
                }
            }
        }
    }
}

impl Memory for NesMemoryMapper {
    fn read(&mut self, address: usize, is_read_only: bool) -> u8 {
        let (cartridge_data, use_cartridge_data) = {
            let mut cartridge = self.cartridge.lock().unwrap();
            let data = cartridge.read(address, is_read_only);
            let use_cartridge_data = cartridge.use_cartridge_data();
            (data, use_cartridge_data)
        };

        if use_cartridge_data {
            return cartridge_data;
        } else if address < 0x2000 {
            self.ram[address & 0x07FF]
        } else if address < 0x4000 {
            let mut ppu = self.ppu.lock().unwrap();
            ppu.read(address & 0x07, is_read_only)
        } else if address <= 0x4013 || (address == 0x4015) || (address == 0x4017) {
            0
        } else if address == 0x4016 || address == 0x4017 {
            let mut controller = self.controllers[address & 1].lock().unwrap();
            controller.read()
        } else {
            self.ram[address & 0x07FF]
        }
    }

    fn write(&mut self, address: usize, value: u8) {
        let use_cartridge_data = {
            let mut cartridge = self.cartridge.lock().unwrap();
            cartridge.write(address, value);
            let use_cartridge_data = cartridge.use_cartridge_data();
            use_cartridge_data
        };

        if use_cartridge_data {
        } else if address < 0x2000 {
            self.ram[address & 0x07FF] = value;
        } else if address < 0x4000 {
            let mut ppu = self.ppu.lock().unwrap();
            ppu.write(address & 0x07, value)
        } else if address == OAMDMA {
            self.oam_dma_page = value;
            self.oam_dma_address = 0;
            self.do_oam_dma = true;
        } else if address <= 0x4013 || (address == 0x4015) || (address == 0x4017) {
            // TODO: APU here
        } else if address == 0x4016 || address == 0x4017 {
            let mut controller = self.controllers[address & 1].lock().unwrap();
            controller.write(value);
        } else {
            self.ram[address & 0x07FF] = value;
        }
    }
}

pub struct Bus {
    memory_mapper: NesMemoryMapper,
    pub cpu: CPU,
    pub cycle: u32,
    pub total_cycles: u32,
    pub ppu: PPURef,
}

impl Bus {
    pub fn new(cartridge: Cartridge) -> Self {
        let cartref = Arc::new(Mutex::new(cartridge));
        let ppu = Arc::new(Mutex::new(PPU::new(cartref.clone())));
        let controller1 = Controller::new_ref();
        let controller2 = Controller::new_ref();
        let controllers = vec![controller1, controller2];

        Bus {
            memory_mapper: NesMemoryMapper::new(ppu.clone(), cartref, controllers),
            cpu: CPU::new(),
            cycle: 0,
            total_cycles: 0,
            ppu,
        }
    }

    pub fn new_from_array(array: &Vec<u8>) -> Result<Self, String> {
        let cart = Cartridge::parse(array);

        match cart {
            Ok(cart) => Ok(Self::new(cart)),
            Err(s) => Err(s),
        }
    }

    pub fn clock(&mut self) {
        {
            let mut ppu = self.ppu.lock().unwrap();

            ppu.clock();

            if ppu.call_nmi {
                ppu.call_nmi = false;
                self.cpu.nmi();
            }
        }

        match self.cycle {
            0 | 3 => {
                if self.memory_mapper.do_oam_dma {
                    self.memory_mapper.transfer_oam(self.total_cycles);
                } else {
                    self.cpu.clock(&mut self.memory_mapper);
                }
            }
            _ => {}
        }
        if self.cycle == 0 {
            self.cycle = 6;
        }

        self.cycle -= 1;
        self.total_cycles += 1;
    }

    pub fn clock_until_frame_done(&mut self) {
        loop {
            let done_drawing = {
                let mut ppu = self.ppu.lock().unwrap();
                ppu.done_drawing
            };

            if done_drawing {
                let mut ppu = self.ppu.lock().unwrap();
                ppu.done_drawing = false;
                break;
            }

            self.clock();
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    pub fn memory(&mut self) -> &mut NesMemoryMapper {
        &mut self.memory_mapper
    }

    pub fn press_controller_button(
        &mut self,
        controller_id: usize,
        button: ButtonStatus,
        state: bool,
    ) {
        let mut controller = self.memory_mapper.controllers[controller_id]
            .lock()
            .unwrap();
        controller.set_button_status(button, state);
    }

    pub fn cpu_total_cycles(&self) -> u32 {
        self.cpu.total_cycles
    }

    pub fn ppu_total_cycles(&self) -> u32 {
        self.total_cycles
    }
}
