use crate::cpu::types::*;
use crate::cpu::*;

// macro_rules! word {
//     ($lo:expr, $hi:expr) => {
//         (($hi as u16) << 8) | ($lo as u16)
//     };
// }

macro_rules! hi {
    ($number:expr) => {
        (($number >> 8) & 0xff) as u8
    };
}

macro_rules! lo {
    ($number:expr) => {
        ($number & 0xff) as u8
    };
}

macro_rules! step {
    ($self:ident ; $n:expr; ) => {};
    ($self:ident ; $n:expr; $block:block $(, $rest:block)*) => {
        if ($self.cycles == $n) {
            $block
        } else {
            step!($self; $n + 1; $($rest),*);
        }
    };
    ($self:ident, $($blocks:block)+) => { step!($self; 0; $($blocks),*); };
}

impl CPU {
    fn set_nz(&mut self, value: u8) {
        if value == 0 {
            self.regs.p |= StatusFlag::Z;
        }

        if (value & 0x80) > 0 {
            self.regs.p |= StatusFlag::N;
        }
    }

    fn get_next_pc_value(&mut self, memory: &mut dyn Memory) -> u8 {
        let curr_pc = self.get_pc();
        self.read(memory, curr_pc)
    }

    // Clock the CPU
    pub fn clock(&mut self, memory: &mut dyn Memory) {
        match self.state {
            CPUStatus::FetchOpcode => {
                self.opcode = self.get_next_pc_value(memory);
                self.set_instruction();
                self.next_state(CPUStatus::FetchParameters);
                println!("{}", self.address_mode);
                println!("x {}", self.cycles);
            }
            CPUStatus::FetchParameters => {
                match self.address_mode {
                    AddressMode::Imp | AddressMode::Acc => {
                        // do nothing
                        self.next_state(CPUStatus::Execute);
                    }
                    AddressMode::Imm => {
                        // the parameter right next to the opcode
                        self.absolute_address = self.get_pc();
                        self.next_state(CPUStatus::Execute);
                    }
                    AddressMode::Abs => {
                        step!(self,
                        {
                            self.lo = self.get_next_pc_value(memory);
                        }
                        {
                            self.hi = self.get_next_pc_value(memory);
                            self.absolute_address = self.get_curr_word() as usize;
                            self.next_state(CPUStatus::DelayedExecute);
                        });
                    }
                    AddressMode::Abx | AddressMode::Aby => {
                        let offset = match self.address_mode {
                            AddressMode::Abx => self.regs.x,
                            AddressMode::Aby => self.regs.y,
                            _ => 0,
                        } as usize;

                        step!(self,
                        {
                            self.lo = self.get_next_pc_value(memory);
                        }
                        {
                            self.hi = self.get_next_pc_value(memory);
                            self.absolute_address = self.get_curr_word() as usize;

                            let new_lo = (self.lo as usize) + offset;

                            let do_inplace_writing = match self.opcode_type {
                                Opcode::Asl => true,
                                _ => false
                            };

                            if new_lo < 0x0100 && !do_inplace_writing {
                                self.absolute_address += offset;
                                self.next_state(CPUStatus::DelayedExecute);
                            }
                        }
                        {
                            self.absolute_address += offset;
                            self.next_state(CPUStatus::DelayedExecute);
                        });
                    }
                    AddressMode::Zp0 => {
                        step!(self, {
                            self.absolute_address = self.get_next_pc_value(memory) as usize;
                            self.next_state(CPUStatus::DelayedExecute);
                        });
                    }
                    AddressMode::Zpx | AddressMode::Zpy => {
                        let offset = match self.address_mode {
                            AddressMode::Zpx => self.regs.x,
                            AddressMode::Zpy => self.regs.y,
                            _ => 0,
                        } as u8;

                        step!(self,
                        {
                            self.lo = self.get_next_pc_value(memory);
                        }
                        {
                            self.lo = self.lo.wrapping_add(offset);
                            self.absolute_address = self.lo as usize;
                            self.next_state(CPUStatus::DelayedExecute);
                        });
                    }
                    _ => {
                        self.next_state(CPUStatus::FetchOpcode);
                    }
                }
            }
            _ => {}
        }

        if let CPUStatus::Execute = self.state {
            match self.opcode_type {
                Opcode::Brk => {
                    // the behavior of BRK is defined
                    // according to this article: https://www.pagetable.com/?p=410

                    let vector = if self.interrupt_type.contains(Interrupt::RESET) {
                        INTERRUPT_RESET
                    } else if self.interrupt_type.contains(Interrupt::NMI) {
                        INTERRUPT_NMI
                    } else {
                        INTERRUPT_IRQ
                    } as usize;

                    step!(self,
                        {
                            // Skip next PC if it is invoked from BRK instruction
                            // (not interupted by any means)
                            if self.interrupt_type.bits() == 0 {
                                self.get_pc();
                            }
                            if self.interrupt_type.contains(Interrupt::RESET) {
                                self.push_stack(memory, 0);
                            } else {
                                self.push_stack(memory, hi!(self.regs.pc));
                            }
                        }
                        {
                            if self.interrupt_type.contains(Interrupt::RESET) {
                                self.push_stack(memory, 0);
                            } else {
                                self.push_stack(memory, lo!(self.regs.pc));
                            }
                        }
                        {
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
                        }
                        {
                            self.lo = self.read(memory, vector);
                        }
                        {
                            self.hi = self.read(memory, vector + 1);
                        }
                        {
                            self.regs.pc = self.get_curr_word();
                            self.fetch_opcode();
                        }
                    );
                }
                Opcode::Lda => {
                    step!(self, {
                        self.regs.a = self.read(memory, self.absolute_address);
                        self.set_nz(self.regs.a);
                        self.next_state(CPUStatus::FetchOpcode);
                    });
                }
                Opcode::Ldx => {
                    step!(self, {
                        self.regs.x = self.read(memory, self.absolute_address);
                        self.set_nz(self.regs.x);
                        self.next_state(CPUStatus::FetchOpcode);
                    });
                }
                Opcode::Asl => {
                    match self.address_mode {
                        AddressMode::Acc => {
                            step!(self, {
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
                                self.next_state(CPUStatus::FetchOpcode);
                            });
                        },
                        _ => {
                            step!(self,
                            {
                                self.fetched_data = self.read(memory, self.absolute_address);
                            }
                            {
                                self.write(memory, self.absolute_address, self.fetched_data);
                            }
                            {
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
                                self.next_state(CPUStatus::FetchOpcode);
                            });
                        }
                    };
                }
                _ => {}
            }
        }

        if let CPUStatus::DelayedExecute = self.state {
            println!("EXECUTE AFTER THIS");
            self.next_state(CPUStatus::Execute);
        }

        self.total_cycles += 1;
    }
}
