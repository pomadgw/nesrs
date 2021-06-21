use crate::cpu::types::*;
use crate::cpu::*;
use std::fmt::Write;

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
                self.instruction_debug.clear();
                self.prev_pc = self.regs.pc;

                // println!("{:08b}", self.interrupt_type.bits());

                if self.interrupt_type.contains(Interrupt::NMI)
                    || self.interrupt_type.contains(Interrupt::RESET)
                {
                    self.opcode = 0;
                } else if self.interrupt_type.contains(Interrupt::IRQ)
                    && !self.regs.p.contains(StatusFlag::I)
                {
                    self.opcode = 0;
                } else {
                    self.opcode = self.get_next_pc_value(memory);
                }

                self.set_instruction();
                self.address.lo = 0;
                self.address.hi = 0;
                self.address.clear_carry();

                if self.debug {
                    self.instruction_debug.push(self.opcode);
                    self.formatted_params.clear();
                    self.prev_cycles = self.total_cycles;
                }

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
                    AddressMode::Rel => {
                        match self.opcode_type {
                            Opcode::Bpl => {
                                self.branch_status_to_test = StatusFlag::N;
                                self.branch_when = false;
                            }
                            Opcode::Bmi => {
                                self.branch_status_to_test = StatusFlag::N;
                                self.branch_when = true;
                            }
                            Opcode::Bvc => {
                                self.branch_status_to_test = StatusFlag::V;
                                self.branch_when = false;
                            }
                            Opcode::Bvs => {
                                self.branch_status_to_test = StatusFlag::V;
                                self.branch_when = true;
                            }
                            Opcode::Bcc => {
                                self.branch_status_to_test = StatusFlag::C;
                                self.branch_when = false;
                            }
                            Opcode::Bcs => {
                                self.branch_status_to_test = StatusFlag::C;
                                self.branch_when = true;
                            }
                            Opcode::Bne => {
                                self.branch_status_to_test = StatusFlag::Z;
                                self.branch_when = false;
                            }
                            Opcode::Beq => {
                                self.branch_status_to_test = StatusFlag::Z;
                                self.branch_when = true;
                            }
                            _ => {
                                // impossible
                            }
                        }

                        self.next_state(Microcode::BranchReadOffsetAndCheck);
                    }
                }
            }
            Microcode::FetchImm => {
                self.absolute_address = self.get_pc();
                if self.debug {
                    self.instruction_debug
                        .push(memory.read(self.absolute_address, true));
                    write!(self.formatted_params, "#${:02X}", self.instruction_debug[1]).unwrap();
                }
                self.next_state(Microcode::Execute);
                self.run_next_state(memory);
            }
            Microcode::FetchLo => {
                self.address.lo = self.get_next_pc_value(memory);
                if self.debug {
                    self.instruction_debug.push(self.address.lo);
                }
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
                if self.debug {
                    self.instruction_debug.push(self.address.lo);
                }
                match self.register_access {
                    RegisterAccess::X => {
                        if self.debug {
                            write!(self.formatted_params, "${:02X},X", self.address.lo).unwrap();
                        }
                        self.address.lo = self.address.lo.wrapping_add(self.regs.x);
                        self.next_state(Microcode::FetchLoZP1);
                    }
                    RegisterAccess::Y => {
                        if self.debug {
                            write!(self.formatted_params, "${:02X},Y", self.address.lo).unwrap();
                        }
                        self.address.lo = self.address.lo.wrapping_add(self.regs.y);
                        self.next_state(Microcode::FetchLoZP1);
                    }
                    _ => {
                        if self.debug {
                            write!(self.formatted_params, "${:02X}", self.address.lo).unwrap();
                        }
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

                if self.debug {
                    self.instruction_debug.push(self.address.hi);

                    match self.opcode_type {
                        Opcode::Jmp | Opcode::Jsr => {
                            write!(self.formatted_params, "${:04X}", self.absolute_address)
                                .unwrap();
                        }
                        _ => {
                            let value = memory.read(self.absolute_address, true);
                            write!(
                                self.formatted_params,
                                "${:04X} = {:02X}",
                                self.absolute_address, value
                            )
                            .unwrap();
                        }
                    }
                }

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

                if self.debug {
                    self.instruction_debug.push(self.address.hi);
                    write!(self.formatted_params, "${:04X},X", self.absolute_address).unwrap();
                }

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
                if self.debug {
                    self.instruction_debug.push(self.address.hi);
                    write!(self.formatted_params, "${:04X},Y", self.absolute_address).unwrap();
                }
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
                if self.debug {
                    self.instruction_debug.push(self.temp);
                    write!(self.formatted_params, "(${:02X},X)", self.temp).unwrap();
                }
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
                if self.debug {
                    write!(self.formatted_params, "(${:02X}),Y", self.temp).unwrap();
                    self.instruction_debug.push(self.temp);
                }
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
                if self.debug {
                    self.instruction_debug.push(self.tmp_address.lo);
                }
                self.next_state(Microcode::IndReadHi);
            }
            Microcode::IndReadHi => {
                self.tmp_address.hi = self.get_next_pc_value(memory);
                if self.debug {
                    self.instruction_debug.push(self.tmp_address.hi);
                    write!(
                        self.formatted_params,
                        "(${:04X})",
                        self.tmp_address.to_u16()
                    )
                    .unwrap();
                }
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
                self.interrupt_type.clear();
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
                self.fetched_data = self.pop_stack(memory);
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
                self.fetched_data = self.pop_stack(memory);
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
            // RTS
            Microcode::RtsGetPcLo => {
                self.tmp_address.lo = self.pop_stack(memory);
                self.next_state(Microcode::RtsGetPcHi);
            }
            Microcode::RtsGetPcHi => {
                self.tmp_address.hi = self.pop_stack(memory);
                self.next_state(Microcode::RtsWasteOneCycle);
            }
            Microcode::RtsWasteOneCycle => {
                self.next_state(Microcode::RtsJump);
            }
            Microcode::RtsJump => {
                self.regs.pc = self.tmp_address.to_u16() + 1;
                self.fetch_opcode();
            }
            // RTI
            Microcode::RtiPopStatus => {
                let status = self.pop_stack(memory);
                self.regs.p.set_from_byte(status);
                self.next_state(Microcode::RtiPopLoPC);
            }
            Microcode::RtiPopLoPC => {
                let lo = self.pop_stack(memory);
                self.address.lo = lo;
                self.next_state(Microcode::RtiPopHiPC);
            }
            Microcode::RtiPopHiPC => {
                let hi = self.pop_stack(memory);
                self.address.hi = hi;
                self.next_state(Microcode::RtiSetPC);
            }
            Microcode::RtiSetPC => {
                self.regs.pc = self.address.to_u16();
                self.regs.p |= StatusFlag::U;
                self.regs.p &= !StatusFlag::B;
                self.fetch_opcode();
            }
            Microcode::BranchReadOffsetAndCheck => {
                let next_pc = self.get_next_pc_value(memory);
                self.relative_address = next_pc as i8;
                if self.debug {
                    self.instruction_debug.push(next_pc);

                    let next_pc = (self.regs.pc as i32) + (self.relative_address as i32);
                    write!(
                        self.formatted_params,
                        "{:02X} = ${:04X}",
                        self.relative_address, next_pc
                    )
                    .unwrap();
                }

                if self.regs.p.contains(self.branch_status_to_test) == self.branch_when {
                    self.next_state(Microcode::BranchJumpIfTrue);
                } else {
                    self.fetch_opcode();
                }
            }
            Microcode::BranchJumpIfTrue => {
                let next_pc = (self.regs.pc as i32) + (self.relative_address as i32);
                if (self.regs.pc & 0xff00) == (next_pc as u16) & 0xff00 {
                    self.regs.pc = next_pc as u16;
                    self.fetch_opcode();
                } else {
                    self.next_state(Microcode::BranchJumpIfTrueAndCrossPage);
                }
            }
            Microcode::BranchJumpIfTrueAndCrossPage => {
                let next_pc = (self.regs.pc as i32) + (self.relative_address as i32);
                self.regs.pc = next_pc as u16;
                self.fetch_opcode();
            }
            _ => {
                if self.cycles > 0 {
                    self.cycles -= 1;
                } else {
                    self.fetch_opcode();
                }
            }
        }
    }
}
