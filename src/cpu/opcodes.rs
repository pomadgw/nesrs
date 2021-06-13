use crate::cpu::types::*;
use crate::cpu::*;

impl CPU {
    pub fn do_instruction(&mut self, memory: &mut dyn Memory) {
        match self.opcode_type {
            Opcode::Brk => {
                self.next_state(Microcode::BrkPushPCHi);
                self.run_next_state(memory);
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
            Opcode::Sta => {
                self.write(memory, self.absolute_address, self.regs.a);
                self.fetch_opcode();
            }
            Opcode::Stx => {
                self.write(memory, self.absolute_address, self.regs.x);
                self.fetch_opcode();
            }
            Opcode::Sty => {
                self.write(memory, self.absolute_address, self.regs.y);
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
            Opcode::Tax => {
                self.regs.x = self.regs.a;
                self.set_nz(self.regs.x);
                self.fetch_opcode();
            }
            Opcode::Tay => {
                self.regs.y = self.regs.a;
                self.set_nz(self.regs.y);
                self.fetch_opcode();
            }
            Opcode::Txa => {
                self.regs.a = self.regs.x;
                self.set_nz(self.regs.a);
                self.fetch_opcode();
            }
            Opcode::Tya => {
                self.regs.a = self.regs.y;
                self.set_nz(self.regs.a);
                self.fetch_opcode();
            }
            Opcode::Inx => {
                self.regs.x = self.regs.x.wrapping_add(1);
                self.set_nz(self.regs.x);
                self.fetch_opcode();
            }
            Opcode::Iny => {
                self.regs.y = self.regs.y.wrapping_add(1);
                self.set_nz(self.regs.y);
                self.fetch_opcode();
            }
            Opcode::Dex => {
                self.regs.x = self.regs.x.wrapping_sub(1);
                self.set_nz(self.regs.x);
                self.fetch_opcode();
            }
            Opcode::Dey => {
                self.regs.y = self.regs.y.wrapping_sub(1);
                self.set_nz(self.regs.y);
                self.fetch_opcode();
            }
            // stack-related
            Opcode::Txs => {
                self.regs.sp = self.regs.x;
                self.fetch_opcode();
            }
            Opcode::Tsx => {
                self.regs.x = self.regs.sp;
                self.fetch_opcode();
            }
            Opcode::Pha => {
                self.next_state(Microcode::PhaReadA);
            }
            Opcode::Pla => {
                self.next_state(Microcode::PlaPull);
            }
            _ => {
                self.fetch_opcode();
            }
        }
    }
}
