#[macro_use]
mod instructions;
#[macro_use]
mod addressings;

use crate::utils::Memory;

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

pub struct CPU {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub p: u8,
    pub pc: u16,

    pub cycles: u32,

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
            is_crossing_page: false,
        }
    }

    pub fn is_clocking_done(&self) -> bool {
        self.sync
    }

    pub fn clock(&mut self, memory: &mut dyn Memory) {
        if self.sync {
            self.sync = false;
            self.is_crossing_page = false;
            self.current_opcode = memory.read(self.next_pc(), false);
        }

        match self.current_opcode {
            0xa9 => {
                set_instruction!(self, 2, {
                    imm!(self, memory);
                    lda!(self, memory);
                });
            }
            0xad => {
                set_instruction!(self, 4, {
                    abs!(self, memory);
                    lda!(self, memory);
                });
            }
            0xb9 => {
                set_instruction!(self, 4, {
                    aby!(self, memory);
                    lda!(self, memory);
                });
            }
            0xbd => {
                set_instruction!(self, 4, {
                    abx!(self, memory);
                    lda!(self, memory);
                });
            }
            0xee => {
                set_instruction!(self, 6, {
                    abs!(self, memory);
                    inc!(self, memory);
                });
            }
            0xfe => {
                set_instruction!(self, 7, {
                    abx!(self, memory);
                    inc!(self, memory);
                });
            }
            _ => {
                self.sync = true;
            }
        }

        self.steps += 1;
        self.cycles += 1;
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
}
