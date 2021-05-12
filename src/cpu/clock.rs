use crate::cpu::types::*;
use crate::cpu::*;

macro_rules! word {
    ($lo:expr, $hi:expr) => {
        (($hi as u16) << 8) | ($lo as u16)
    };
}

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
    // Clock the CPU
    pub fn clock(&mut self, memory: &mut dyn Memory) {
        if self.is_read_instruction {
            self.opcode = memory.read(self.get_pc(), false);
            self.is_read_instruction = false;
            self.set_instruction();
        }

        self.cycles -= 1;
        self.total_cycles += 1;

        if self.cycles == 0 {
            match self.address_mode {
                AddressMode::Imp => {
                    // do nothing
                }
                _ => {}
            }

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

                    // Skip next PC if it is invoked from BRK instruction
                    // (not interupted by any means)
                    if self.interrupt_type.bits() == 0 {
                        self.get_pc();
                    }

                    if self.interrupt_type & Interrupt::RESET == Interrupt::RESET {
                        self.push_stack(memory, 0);
                        self.push_stack(memory, 0);
                    } else {
                        self.push_stack(memory, hi!(self.regs.pc));
                        self.push_stack(memory, lo!(self.regs.pc));
                    }

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

                    let lo = memory.read(vector, false);
                    let hi = memory.read(vector + 1, false);
                    self.regs.pc = word!(lo, hi);
                }
                _ => {}
            }

            self.is_read_instruction = true;
        }
    }
}
