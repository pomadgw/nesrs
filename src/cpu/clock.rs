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
                    AddressMode::Acc => {
                        self.register_access = RegisterAccess::A;
                        self.next_state(Microcode::Execute);
                    }
                    AddressMode::Imp => {
                        self.next_state(Microcode::Execute);
                    }
                    AddressMode::Abs => {
                        self.next_state(Microcode::FetchLo);
                        self.register_access = RegisterAccess::None;
                    }
                    AddressMode::Abx => {
                        self.next_state(Microcode::FetchLo);
                        self.register_access = RegisterAccess::X;
                    }
                    AddressMode::Aby => {
                        self.next_state(Microcode::FetchLo);
                        self.register_access = RegisterAccess::Y;
                    }
                    AddressMode::Zp0 => {
                        self.next_state(Microcode::FetchLoZP);
                        self.register_access = RegisterAccess::None;
                    }
                    AddressMode::Zpx => {
                        self.next_state(Microcode::FetchLoZP);
                        self.register_access = RegisterAccess::X;
                    }
                    AddressMode::Zpy => {
                        self.next_state(Microcode::FetchLoZP);
                        self.register_access = RegisterAccess::Y;
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
                match self.register_access {
                    RegisterAccess::X => {
                        self.next_state(Microcode::FetchHiX);
                    }
                    RegisterAccess::Y => {
                        self.next_state(Microcode::FetchHiY);
                    }
                    _ => {
                        self.next_state(Microcode::FetchHi);
                    }
                }
            }
            Microcode::FetchLoZP => {
                self.address.lo = self.get_next_pc_value(memory);
                match self.register_access {
                    RegisterAccess::X => {
                        self.address.lo += self.regs.x;
                    }
                    RegisterAccess::Y => {
                        self.address.lo += self.regs.y;
                    }
                    _ => {}
                }
                self.next_state(Microcode::Execute);
            }
            Microcode::FetchHi => {
                self.address.hi = self.get_next_pc_value(memory);
                self.absolute_address = self.address.to_usize();
                self.next_state(Microcode::Execute);
            }
            Microcode::FetchHiX => {
                self.address.hi = self.get_next_pc_value(memory);
                self.address += self.regs.x;

                if self.address.has_carry() || self.is_write_instruction() {
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
            Microcode::FetchHiY => {
                self.address.hi = self.get_next_pc_value(memory);
                self.address += self.regs.y;

                if self.address.has_carry() || self.is_write_instruction() {
                    self.next_state(Microcode::FetchHiY2);
                } else {
                    self.absolute_address = self.address.to_usize();
                    self.next_state(Microcode::Execute);
                }
            }
            Microcode::FetchHiY2 => {
                self.address.add_hi_from_carry();
                self.absolute_address = self.address.to_usize();
                self.next_state(Microcode::Execute);
            }
            Microcode::Execute => {
                self.do_instruction(memory);
            }
            _ => {}
        }

        self.total_cycles += 1;
    }
}
