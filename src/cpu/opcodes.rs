use crate::cpu::types::*;
use crate::cpu::*;

impl CPU {
    pub fn do_instruction(&mut self, memory: &mut dyn Memory) {
        match self.opcode_type {
            Opcode::Brk => {
                self.next_state(Microcode::BrkPushPCHi);
            }
            Opcode::Lda => {
                self.regs.a = self.read(memory, self.absolute_address);
                self.set_nz(self.regs.a);
                self.fetch_opcode();
            }
            Opcode::Ldx => {
                self.regs.x = self.read(memory, self.absolute_address);
                self.set_nz(self.regs.x);
                self.fetch_opcode();
            }
            Opcode::Ldy => {
                self.regs.y = self.read(memory, self.absolute_address);
                self.set_nz(self.regs.y);
                self.fetch_opcode();
            }
            Opcode::Asl => match self.register_access {
                RegisterAccess::A => {
                    self.next_state(Microcode::AslA);
                }
                _ => {
                    self.next_state(Microcode::AslFetch);
                    self.run_next_state(memory);
                }
            },
            _ => {}
        }
    }
}
