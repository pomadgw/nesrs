use crate::cpu::types::*;
use crate::cpu::*;

// macro_rules! word {
//     ($lo:expr, $hi:expr) => {
//         (($hi as u16) << 8) | ($lo as u16)
//     };
// }

impl CPU {
    pub fn get_next_pc_value(&mut self, memory: &mut dyn Memory) -> u8 {
        let curr_pc = self.get_pc();
        self.read(memory, curr_pc)
    }

    // Clock the CPU
    pub fn clock(&mut self, memory: &mut dyn Memory) {
        match self.state {
            Microcode::FetchOpcode => {
                self.opcode = self.get_next_pc_value(memory);
                self.set_instruction();
                self.next_state(Microcode::FetchParameters);
            }
            Microcode::FetchParameters => {
                self.do_addressing_mode(memory);
            }
            _ => {}
        }

        if let Microcode::Execute = self.state {
            self.do_instruction(memory);
        }

        if let Microcode::DelayedExecute = self.state {
            self.next_state(Microcode::Execute);
        }

        self.total_cycles += 1;
    }
}
