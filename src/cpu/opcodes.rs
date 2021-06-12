use crate::cpu::types::*;
use crate::cpu::*;

impl CPU {
    pub fn do_instruction(&mut self, memory: &mut dyn Memory) {
        self.next_state(Microcode::FetchOpcode);
    }
}
