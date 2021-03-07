#[macro_use]
mod instructions;
#[macro_use]
mod addressings;
#[macro_use]
mod macros;
#[macro_use]
mod clock;

use crate::Memory;

pub enum CPUStatus {
    C = 0x01,
    Z = 0x02,
    I = 0x04,
    D = 0x08,
    B = 0x10,
    U = 0x20,
    V = 0x40,
    N = 0x80,
}

pub enum IRQStatus {
    Reset = 0x01,
    NMI = 0x02,
    External = 0x04,
    Frame = 0x08,
    DPCM = 0x10,
}

#[allow(dead_code)]
pub struct CPU {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub p: u8,
    pub pc: u16,

    pub cycles: u32,

    pub irq_triggers: u8,

    sync: bool,
    current_opcode: u8,

    absolute_address: u16,
    relative_address: i16,
    steps: i32,
    is_crossing_page: bool,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            a: 0,
            x: 0,
            y: 0,
            sp: 0,
            p: 0,
            pc: 0,

            sync: true,
            current_opcode: 0,
            absolute_address: 0,
            relative_address: 0,
            steps: 0,
            cycles: 0,
            irq_triggers: 0,
            is_crossing_page: false,
        }
    }

    pub fn is_clocking_done(&self) -> bool {
        self.sync
    }

    pub fn reset(&mut self) {
        self.sync = true;
        self.set_trigger(IRQStatus::Reset);
        self.cycles = 0;
        self.steps = 0;
    }

    fn next_pc(&mut self) -> u16 {
        let current_pc = self.pc;
        self.pc += 1;

        current_pc
    }

    fn set_nz(&mut self, value: u8) {
        let is_zero = value == 0;
        let is_neg = (value & 0x80) > 0;

        self.set_status(CPUStatus::Z, is_zero);
        self.set_status(CPUStatus::N, is_neg);
    }

    pub fn get_status(&mut self, flag: CPUStatus) -> bool {
        (self.p & (flag as u8)) > 0
    }

    fn set_status(&mut self, flag: CPUStatus, value: bool) {
        if value {
            self.p |= flag as u8;
        } else {
            self.p &= !(flag as u8);
        }
    }

    fn init_opcode(&mut self, memory: &mut dyn Memory) {
        if self.sync {
            self.sync = false;
            self.is_crossing_page = false;

            if self.check_trigger(IRQStatus::Reset) || self.check_trigger(IRQStatus::NMI) {
                self.current_opcode = 0x00;
            } else {
                self.current_opcode = memory.read(self.next_pc(), false);
            }
        }
    }

    fn check_trigger(&self, trigger: IRQStatus) -> bool {
        (self.irq_triggers) & (trigger as u8) != 0
    }

    pub fn set_trigger(&mut self, trigger: IRQStatus) {
        self.irq_triggers |= trigger as u8;
    }

    pub fn clear_trigger(&mut self, trigger: IRQStatus) {
        self.irq_triggers &= !(trigger as u8);
    }

    fn push_stack(&mut self, memory: &mut dyn Memory, value: u8) {
        memory.write((self.sp as u16) + 0x0100, value);
        self.sp = self.sp.wrapping_sub(1);
    }

    #[allow(dead_code)]
    fn pop_stack(&mut self, memory: &mut dyn Memory) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        memory.read((self.sp as u16) + 0x0100, false)
    }
}
