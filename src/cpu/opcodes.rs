use crate::cpu::types::*;
use crate::cpu::*;

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

impl CPU {
    fn set_nz(&mut self, value: u8) {
        if value == 0 {
            self.regs.p |= StatusFlag::Z;
        }

        if (value & 0x80) > 0 {
            self.regs.p |= StatusFlag::N;
        }
    }

    pub fn do_instruction(&mut self, memory: &mut dyn Memory) {
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
            Opcode::Ldy => {
                step!(self, {
                    self.regs.y = self.read(memory, self.absolute_address);
                    self.set_nz(self.regs.y);
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
                    }
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
}
