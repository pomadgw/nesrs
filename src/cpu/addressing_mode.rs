use crate::cpu::types::*;
use crate::cpu::*;

impl CPU {
    pub fn do_addressing_mode(&mut self, memory: &mut dyn Memory) {
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
}
