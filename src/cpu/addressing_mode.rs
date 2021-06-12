use crate::cpu::types::*;
use crate::cpu::*;

impl CPU {
    pub fn do_addressing_mode(&mut self, memory: &mut dyn Memory) {
        self.next_state(Microcode::FetchOpcode);
    }
}
