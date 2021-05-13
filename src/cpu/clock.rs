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
    // Clock the CPU
    pub fn clock(&mut self, memory: &mut dyn Memory) {
        match self.state {
            CPUStatus::FetchOpcode => {
                let curr_pc = self.get_pc();
                self.opcode = self.read(memory, curr_pc);
                self.set_instruction();
                self.next_state(CPUStatus::FetchParameters);
            }
            CPUStatus::FetchParameters => {
                match self.address_mode {
                    AddressMode::Imp => {
                        // do nothing
                        self.next_state(CPUStatus::Execute);
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        if let CPUStatus::Execute = self.state {
            match self.opcode_type {
                Opcode::Brk => {
                    // the behavior of BRK is defined
                    // according to this article: https://www.pagetable.com/?p=410

                    let vector = if self.interrupt_type & Interrupt::RESET == Interrupt::RESET {
                        INTERRUPT_RESET
                    } else if self.interrupt_type & Interrupt::NMI == Interrupt::NMI {
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
                            if self.interrupt_type & Interrupt::RESET == Interrupt::RESET {
                                self.push_stack(memory, 0);
                            } else {
                                self.push_stack(memory, hi!(self.regs.pc));
                            }
                        }
                        {
                            if self.interrupt_type & Interrupt::RESET == Interrupt::RESET {
                                self.push_stack(memory, 0);
                            } else {
                                self.push_stack(memory, lo!(self.regs.pc));
                            }
                        }
                        {
                            self.regs.p |= StatusFlag::B;
                            self.regs.p |= StatusFlag::U;

                            if self.interrupt_type & Interrupt::RESET == Interrupt::RESET {
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
