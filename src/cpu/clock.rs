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
        self.run_next_state(memory);

        self.total_cycles += 1;
    }

    pub fn run_next_state(&mut self, memory: &mut dyn Memory) {
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
                        self.run_next_state(memory);
                    }
                    AddressMode::Imp => {
                        self.register_access = RegisterAccess::None;
                        self.next_state(Microcode::Execute);
                    }
                    AddressMode::Imm => {
                        self.register_access = RegisterAccess::None;
                        self.next_state(Microcode::FetchImm);
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
                    AddressMode::Izx => {
                        self.next_state(Microcode::FetchIZX1);
                    }
                    AddressMode::Izy => {
                        self.next_state(Microcode::FetchIZY1);
                    }
                    AddressMode::Ind => {
                        self.next_state(Microcode::IndReadLo);
                    }
                    _ => {
                        self.next_state(Microcode::FetchOpcode);
                    }
                }
            }
            Microcode::FetchImm => {
                self.absolute_address = self.get_pc();
                self.next_state(Microcode::Execute);
                self.run_next_state(memory);
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
                        self.next_state(Microcode::FetchLoZP1);
                    }
                    RegisterAccess::Y => {
                        self.address.lo += self.regs.y;
                        self.next_state(Microcode::FetchLoZP1);
                    }
                    _ => {
                        self.absolute_address = self.address.to_usize();
                        self.next_state(Microcode::Execute);
                    }
                }
            }
            Microcode::FetchLoZP1 => {
                self.absolute_address = self.address.to_usize();
                self.next_state(Microcode::Execute);
            }
            Microcode::FetchHi => {
                self.address.hi = self.get_next_pc_value(memory);
                self.absolute_address = self.address.to_usize();

                if let Opcode::Jmp = self.opcode_type {
                    // JMP ABS use 3 cycles -_-
                    self.regs.pc = self.absolute_address as u16;
                    self.fetch_opcode();
                } else if let Opcode::Jsr = self.opcode_type {
                    self.next_state(Microcode::JsrSaveHiPrevPc);
                } else {
                    self.next_state(Microcode::Execute);
                }
            }
            Microcode::FetchHiX => {
                self.address.hi = self.get_next_pc_value(memory);
                self.address += self.regs.x;

                if self.address.has_carry() || self.is_write_instruction() {
                    self.next_state(Microcode::SetCrossPage);
                } else {
                    self.absolute_address = self.address.to_usize();
                    self.next_state(Microcode::Execute);
                }
            }
            Microcode::FetchHiY => {
                self.address.hi = self.get_next_pc_value(memory);
                self.address += self.regs.y;

                if self.address.has_carry() || self.is_write_instruction() {
                    self.next_state(Microcode::SetCrossPage);
                } else {
                    self.absolute_address = self.address.to_usize();
                    self.next_state(Microcode::Execute);
                }
            }
            Microcode::FetchIZX1 => {
                self.temp = self.get_next_pc_value(memory);
                self.next_state(Microcode::FetchIZX2);
            }
            Microcode::FetchIZX2 => {
                self.absolute_address = self.read(memory, self.temp as usize) as usize;
                self.next_state(Microcode::FetchIZX3);
            }
            Microcode::FetchIZX3 => {
                self.temp = self.temp.wrapping_add(self.regs.x);
                self.address.lo = self.read(memory, self.temp as usize);
                self.next_state(Microcode::FetchIZX4);
            }
            Microcode::FetchIZX4 => {
                self.temp = self.temp.wrapping_add(1);
                self.address.hi = self.read(memory, self.temp as usize);
                self.absolute_address = self.address.to_usize();
                self.next_state(Microcode::Execute);
            }
            Microcode::FetchIZY1 => {
                self.temp = self.get_next_pc_value(memory);
                self.next_state(Microcode::FetchIZY2);
            }
            Microcode::FetchIZY2 => {
                self.address.lo = self.read(memory, self.temp as usize);
                self.next_state(Microcode::FetchIZY3);
            }
            Microcode::FetchIZY3 => {
                self.address.hi = self.read(memory, self.temp.wrapping_add(1) as usize);
                self.address += self.regs.y;

                if self.address.has_carry() || self.is_write_instruction() {
                    self.next_state(Microcode::SetCrossPage);
                } else {
                    self.absolute_address = self.address.to_usize();
                    self.next_state(Microcode::Execute);
                }
            }
            Microcode::SetCrossPage => {
                self.address.add_hi_from_carry();
                self.absolute_address = self.address.to_usize();
                self.next_state(Microcode::Execute);
            }
            Microcode::IndReadLo => {
                self.tmp_address.lo = self.get_next_pc_value(memory);
                self.next_state(Microcode::IndReadHi);
            }
            Microcode::IndReadHi => {
                self.tmp_address.hi = self.get_next_pc_value(memory);
                self.next_state(Microcode::IndReadActualLo);
            }
            Microcode::IndReadActualLo => {
                self.address.lo = self.read(memory, self.tmp_address.to_usize());
                self.next_state(Microcode::IndReadActualHiAndJump);
            }
            Microcode::IndReadActualHiAndJump => {
                // JMP indirect has a bug:
                // if address is $xxff, the actual jump fetched
                // is in $xxff and $xx00 (instead of $xxff + 1)
                self.tmp_address += 1;
                self.address.hi = self.read(memory, self.tmp_address.to_usize());

                // Jump immediately
                self.regs.pc = self.address.to_u16();
                self.fetch_opcode();
            }
            Microcode::Execute => {
                self.do_instruction(memory);
            }

            // ASL
            Microcode::AslA => {
                let mut fetched = self.regs.a as u16;
                fetched = fetched << 1;
                let result = (fetched & 0xff) as u8;
                self.regs.a = result;
                if fetched > 0xff {
                    self.regs.p |= StatusFlag::C;
                } else {
                    self.regs.p &= !StatusFlag::C;
                }
                self.set_nz(self.regs.a);
                self.next_state(Microcode::FetchOpcode);
            }

            Microcode::AslFetch => {
                self.fetched_data = self.read(memory, self.absolute_address);
                self.next_state(Microcode::AslWrite);
            }
            Microcode::AslWrite => {
                self.write(memory, self.absolute_address, self.fetched_data);
                self.next_state(Microcode::AslAddAndWrite);
            }
            Microcode::AslAddAndWrite => {
                let mut fetched = self.fetched_data as u16;
                fetched = fetched << 1;
                let result = (fetched & 0xff) as u8;
                if fetched > 0xff {
                    self.regs.p |= StatusFlag::C;
                } else {
                    self.regs.p &= !StatusFlag::C;
                }
                self.write(memory, self.absolute_address, result);
                self.set_nz(result);
                self.next_state(Microcode::FetchOpcode);
            }

            // BRK
            Microcode::BrkPushPCHi => {
                // Skip next PC if it is invoked from BRK instruction
                // (not interupted by any means)
                if self.interrupt_type.bits() == 0 {
                    self.get_pc();
                }

                self.address.set_u16(self.regs.pc);

                if self.interrupt_type.contains(Interrupt::RESET) {
                    self.push_stack(memory, 0);
                } else {
                    self.push_stack(memory, self.address.hi);
                }

                self.next_state(Microcode::BrkPushPCLo);
            }
            Microcode::BrkPushPCLo => {
                if self.interrupt_type.contains(Interrupt::RESET) {
                    self.push_stack(memory, 0);
                } else {
                    self.push_stack(memory, self.address.lo);
                }

                self.next_state(Microcode::BrkPushStatus);
            }
            Microcode::BrkPushStatus => {
                self.regs.p |= StatusFlag::B;
                self.regs.p |= StatusFlag::U;

                if self.interrupt_type.contains(Interrupt::RESET) {
                    self.push_stack(memory, 0);
                } else {
                    self.push_stack(memory, self.regs.p.bits());
                    self.regs.p &= !StatusFlag::U;
                }

                self.regs.p &= !StatusFlag::B;
                self.regs.p |= StatusFlag::I;

                self.next_state(Microcode::BrkPushReadPCLo);
            }
            Microcode::BrkPushReadPCLo => {
                self.address.lo = self.read(memory, self.vector_address());

                self.next_state(Microcode::BrkPushReadPCHi);
            }
            Microcode::BrkPushReadPCHi => {
                self.address.hi = self.read(memory, self.vector_address() + 1);

                self.next_state(Microcode::BrkSetPC);
            }
            Microcode::BrkSetPC => {
                self.regs.pc = self.address.to_u16();
                self.fetch_opcode();
            }
            // PHA
            Microcode::PhaPushStack => {
                self.push_stack(memory, self.regs.a);
                self.fetch_opcode();
            }
            // PLA
            Microcode::PlaPull => {
                self.fetched_data = self.pull_stack(memory);
                self.next_state(Microcode::PlaPull1);
            }
            Microcode::PlaPull1 => {
                self.regs.a = self.fetched_data;
                self.set_nz(self.fetched_data);
                self.fetch_opcode();
            }
            // PHP
            Microcode::PhpPushStack => {
                self.push_stack(memory, self.regs.p.bits() | 0b00110000);
                self.fetch_opcode();
            }
            // PLP
            Microcode::PlpPull => {
                self.fetched_data = self.pull_stack(memory);
                self.next_state(Microcode::PlpPull1);
            }
            Microcode::PlpPull1 => {
                self.regs.p.set_from_byte(self.fetched_data);
                self.fetch_opcode();
            }
            // JSR
            Microcode::JsrSaveHiPrevPc => {
                self.tmp_address.set_u16(self.regs.pc - 1);
                self.push_stack(memory, self.tmp_address.hi);
                self.next_state(Microcode::JsrSaveLoPrevPc);
            }
            Microcode::JsrSaveLoPrevPc => {
                self.push_stack(memory, self.tmp_address.lo);
                self.next_state(Microcode::JsrJump);
            }
            Microcode::JsrJump => {
                self.regs.pc = self.absolute_address as u16;
                self.fetch_opcode();
            }
            _ => {
                self.fetch_opcode();
            }
        }
    }
}
