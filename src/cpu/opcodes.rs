use crate::cpu::types::*;
use crate::cpu::*;

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
        self.next_state(Microcode::FetchOpcode);
    }
}
