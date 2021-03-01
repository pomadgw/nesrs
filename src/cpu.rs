#[macro_use]
mod instructions;

use crate::utils::Memory;

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
            0xad => {
                set_instruction!(self, 4, {
                    abs!(self, memory);
                    lda!(self, memory);
                });
            }
            0xb9 => {
                set_instruction!(self, 4, {
                    aby!(self, memory, false);
                    lda!(self, memory);
                });
            }
            0xbd => {
                set_instruction!(self, 4, {
                    abx!(self, memory, false);
                    lda!(self, memory);
                });
            }
            _ => {}
        }

        self.steps += 1;
        self.cycles += 1;
    }

    fn next_pc(&mut self) -> u16 {
        let current_pc = self.pc;
        self.pc += 1;

        current_pc
    }
}
