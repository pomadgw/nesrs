use crate::CPU;
use crate::Memory;

impl CPU {
    pub fn clock(&mut self, memory: &mut dyn Memory) {
        self.init_opcode(memory);

        match self.current_opcode {
            0xa1 => {
                set_instruction!(self, 6, {
                    izx!(self, memory);
                    lda!(self, memory);
                });
            }
            0xa2 => {
                set_instruction!(self, 2, {
                    imm!(self, memory);
                    ldx!(self, memory);
                });
            }
            0xa5 => {
                set_instruction!(self, 3, {
                    zp0!(self, memory);
                    lda!(self, memory);
                });
            }
            0xa6 => {
                set_instruction!(self, 3, {
                    zp0!(self, memory);
                    ldx!(self, memory);
                });
            }
            0xa9 => {
                set_instruction!(self, 2, {
                    imm!(self, memory);
                    lda!(self, memory);
                });
            }
            0xad => {
                set_instruction!(self, 4, {
                    abs!(self, memory);
                    lda!(self, memory);
                });
            }
            0xae => {
                set_instruction!(self, 4, {
                    abs!(self, memory);
                    ldx!(self, memory);
                });
            }
            0xb1 => {
                set_instruction!(self, 5, {
                    izy!(self, memory);
                    lda!(self, memory);
                });
            }
            0xb5 => {
                set_instruction!(self, 4, {
                    zpx!(self, memory);
                    lda!(self, memory);
                });
            }
            0xb6 => {
                set_instruction!(self, 4, {
                    zpy!(self, memory);
                    lda!(self, memory);
                });
            }
            0xb9 => {
                set_instruction!(self, 4, {
                    aby!(self, memory);
                    lda!(self, memory);
                });
            }
            0xbd => {
                set_instruction!(self, 4, {
                    abx!(self, memory);
                    lda!(self, memory);
                });
            }
            0xbe => {
                set_instruction!(self, 4, {
                    aby!(self, memory);
                    lda!(self, memory);
                });
            }
            0xee => {
                set_instruction!(self, 6, {
                    abs!(self, memory);
                    inc!(self, memory);
                });
            }
            0xfe => {
                set_instruction!(self, 7, {
                    abx!(self, memory);
                    inc!(self, memory);
                });
            }
            _ => {
                self.sync = true;
            }
        }

        self.steps += 1;
        self.cycles += 1;
    }
}
