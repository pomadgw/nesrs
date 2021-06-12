use crate::cpu::types::*;
use crate::cpu::*;

// macro_rules! word {
//     ($lo:expr, $hi:expr) => {
//         (($hi as u16) << 8) | ($lo as u16)
//     };
// }

impl CPU {
    pub fn get_next_pc_value(&mut self, memory: &mut dyn Memory) -> u8 {
        let curr_pc = self.get_pc();
        self.read(memory, curr_pc)
    }

    // Clock the CPU
    pub fn clock(&mut self, memory: &mut dyn Memory) {
        match self.state {
            Microcode::FetchOpcode => {
                self.opcode = self.get_next_pc_value(memory);
                self.set_instruction();
                self.address.lo = 0;
                self.address.hi = 0;
                self.address.clear_carry();
                // self.next_state(Microcode::FetchParameters);

                match self.address_mode {
                    AddressMode::Imp => {
                        self.next_state(Microcode::Execute);
                    }
                    AddressMode::Abs => {
                        self.next_state(Microcode::FetchLo);
                    }
                    AddressMode::Abx => {
                        self.next_state(Microcode::FetchLoX);
                    }
                    _ => {
                        self.next_state(Microcode::FetchOpcode);
                    }
                }
            }
            Microcode::FetchParameters => {
                self.do_addressing_mode(memory);
            }
            Microcode::FetchLo => {
                self.address.lo = self.get_next_pc_value(memory);
                self.next_state(Microcode::FetchHi);
            }
            Microcode::FetchHi => {
                self.address.hi = self.get_next_pc_value(memory);
                self.absolute_address = self.address.to_usize();
                self.next_state(Microcode::Execute);
            }
            Microcode::FetchLoX => {
                self.address.lo = self.get_next_pc_value(memory);
                self.next_state(Microcode::FetchHiX);
            }
            Microcode::FetchHiX => {
                self.address.hi = self.get_next_pc_value(memory);
                self.address += self.regs.x;

                if self.address.has_carry() {
                    self.next_state(Microcode::FetchHiX2);
                } else {
                    self.absolute_address = self.address.to_usize();
                    self.next_state(Microcode::Execute);
                }
            }
            Microcode::FetchHiX2 => {
                self.address.add_hi_from_carry();
                self.absolute_address = self.address.to_usize();
                self.next_state(Microcode::Execute);
            }
            _ => {}
        }

        if let Microcode::Execute = self.state {
            self.do_instruction(memory);
        }

        if let Microcode::DelayedExecute = self.state {
            self.next_state(Microcode::Execute);
        }

        self.total_cycles += 1;
    }
}
