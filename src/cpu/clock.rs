use crate::cpu::*;
use crate::Memory;

impl CPU {
    pub fn clock(&mut self, memory: &mut dyn Memory) {
        self.init_opcode(memory);

        match self.current_opcode {
            0x00 => {
                set_instruction!(self, 7, {
                    imp!(self, memory);
                    brk!(self, memory);
                });
            }
            0x02 => {
                set_instruction!(self, 2, {});
            }
            0x03 => {
                set_instruction!(self, 8, {});
            }
            0x04 => {
                set_instruction!(self, 3, {});
            }
            0x07 => {
                set_instruction!(self, 5, {});
            }
            0x0b => {
                set_instruction!(self, 2, {});
            }
            0x0c => {
                set_instruction!(self, 4, {});
            }
            0x0f => {
                set_instruction!(self, 6, {});
            }
            0x12 => {
                set_instruction!(self, 2, {});
            }
            0x13 => {
                set_instruction!(self, 8, {});
            }
            0x14 => {
                set_instruction!(self, 4, {});
            }
            0x17 => {
                set_instruction!(self, 6, {});
            }
            0x1a => {
                set_instruction!(self, 2, {});
            }
            0x1b => {
                set_instruction!(self, 7, {});
            }
            0x1c => {
                set_instruction!(self, 4, {});
            }
            0x1f => {
                set_instruction!(self, 7, {});
            }
            0x22 => {
                set_instruction!(self, 2, {});
            }
            0x23 => {
                set_instruction!(self, 8, {});
            }
            0x27 => {
                set_instruction!(self, 5, {});
            }
            0x2b => {
                set_instruction!(self, 2, {});
            }
            0x2f => {
                set_instruction!(self, 6, {});
            }
            0x32 => {
                set_instruction!(self, 2, {});
            }
            0x33 => {
                set_instruction!(self, 8, {});
            }
            0x34 => {
                set_instruction!(self, 4, {});
            }
            0x37 => {
                set_instruction!(self, 6, {});
            }
            0x3a => {
                set_instruction!(self, 2, {});
            }
            0x3b => {
                set_instruction!(self, 7, {});
            }
            0x3c => {
                set_instruction!(self, 4, {});
            }
            0x3f => {
                set_instruction!(self, 7, {});
            }
            0x42 => {
                set_instruction!(self, 2, {});
            }
            0x43 => {
                set_instruction!(self, 8, {});
            }
            0x44 => {
                set_instruction!(self, 3, {});
            }
            0x47 => {
                set_instruction!(self, 5, {});
            }
            0x4b => {
                set_instruction!(self, 2, {});
            }
            0x4c => {
                set_instruction!(self, 3, {
                    abs!(self, memory);
                    jmp!(self, memory);
                });
            }
            0x4f => {
                set_instruction!(self, 6, {});
            }
            0x52 => {
                set_instruction!(self, 2, {});
            }
            0x53 => {
                set_instruction!(self, 8, {});
            }
            0x54 => {
                set_instruction!(self, 4, {});
            }
            0x57 => {
                set_instruction!(self, 6, {});
            }
            0x5a => {
                set_instruction!(self, 2, {});
            }
            0x5b => {
                set_instruction!(self, 7, {});
            }
            0x5c => {
                set_instruction!(self, 4, {});
            }
            0x5f => {
                set_instruction!(self, 7, {});
            }
            0x62 => {
                set_instruction!(self, 2, {});
            }
            0x63 => {
                set_instruction!(self, 8, {});
            }
            0x64 => {
                set_instruction!(self, 3, {});
            }
            0x67 => {
                set_instruction!(self, 5, {});
            }
            0x6b => {
                set_instruction!(self, 2, {});
            }
            0x6c => {
                set_instruction!(self, 5, {
                    ind!(self, memory);
                    jmp!(self, memory);
                });
            }
            0x6f => {
                set_instruction!(self, 6, {});
            }
            0x72 => {
                set_instruction!(self, 2, {});
            }
            0x73 => {
                set_instruction!(self, 8, {});
            }
            0x74 => {
                set_instruction!(self, 4, {});
            }
            0x77 => {
                set_instruction!(self, 6, {});
            }
            0x7a => {
                set_instruction!(self, 2, {});
            }
            0x7b => {
                set_instruction!(self, 7, {});
            }
            0x7c => {
                set_instruction!(self, 4, {});
            }
            0x7f => {
                set_instruction!(self, 7, {});
            }
            0x80 => {
                set_instruction!(self, 2, {});
            }
            0x82 => {
                set_instruction!(self, 2, {});
            }
            0x83 => {
                set_instruction!(self, 6, {});
            }
            0x87 => {
                set_instruction!(self, 3, {});
            }
            0x89 => {
                set_instruction!(self, 2, {});
            }
            0x8b => {
                set_instruction!(self, 2, {});
            }
            0x8f => {
                set_instruction!(self, 4, {});
            }
            0x92 => {
                set_instruction!(self, 2, {});
            }
            0x93 => {
                set_instruction!(self, 6, {});
            }
            0x97 => {
                set_instruction!(self, 4, {});
            }
            0x9b => {
                set_instruction!(self, 5, {});
            }
            0x9c => {
                set_instruction!(self, 5, {});
            }
            0x9e => {
                set_instruction!(self, 5, {});
            }
            0x9f => {
                set_instruction!(self, 5, {});
            }
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
            0xa3 => {
                set_instruction!(self, 6, {});
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
            0xa7 => {
                set_instruction!(self, 3, {});
            }
            0xa9 => {
                set_instruction!(self, 2, {
                    imm!(self, memory);
                    lda!(self, memory);
                });
            }
            0xab => {
                set_instruction!(self, 2, {});
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
            0xaf => {
                set_instruction!(self, 4, {});
            }
            0xb1 => {
                set_instruction!(self, 5, {
                    izy!(self, memory);
                    lda!(self, memory);
                });
            }
            0xb2 => {
                set_instruction!(self, 2, {});
            }
            0xb3 => {
                set_instruction!(self, 5, {});
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
                    ldx!(self, memory);
                });
            }
            0xb7 => {
                set_instruction!(self, 4, {});
            }
            0xb9 => {
                set_instruction!(self, 4, {
                    aby!(self, memory);
                    lda!(self, memory);
                });
            }
            0xbb => {
                set_instruction!(self, 4, {});
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
                    ldx!(self, memory);
                });
            }
            0xbf => {
                set_instruction!(self, 4, {});
            }
            0xc2 => {
                set_instruction!(self, 2, {});
            }
            0xc3 => {
                set_instruction!(self, 8, {});
            }
            0xc7 => {
                set_instruction!(self, 5, {});
            }
            0xcb => {
                set_instruction!(self, 2, {});
            }
            0xcf => {
                set_instruction!(self, 6, {});
            }
            0xd2 => {
                set_instruction!(self, 2, {});
            }
            0xd3 => {
                set_instruction!(self, 8, {});
            }
            0xd4 => {
                set_instruction!(self, 4, {});
            }
            0xd7 => {
                set_instruction!(self, 6, {});
            }
            0xda => {
                set_instruction!(self, 2, {});
            }
            0xdb => {
                set_instruction!(self, 7, {});
            }
            0xdc => {
                set_instruction!(self, 4, {});
            }
            0xdf => {
                set_instruction!(self, 7, {});
            }
            0xe2 => {
                set_instruction!(self, 2, {});
            }
            0xe3 => {
                set_instruction!(self, 8, {});
            }
            0xe6 => {
                set_instruction!(self, 5, {
                    zp0!(self, memory);
                    inc!(self, memory);
                });
                on_step!(self, 1, {
                    self.is_writing = true;
                });
                on_step!(self, 0, {
                    self.is_writing = true;
                });
            }
            0xe7 => {
                set_instruction!(self, 5, {});
            }
            0xea => {
                set_instruction!(self, 2, {});
            }
            0xee => {
                set_instruction!(self, 6, {
                    abs!(self, memory);
                    inc!(self, memory);
                });
                on_step!(self, 1, {
                    self.is_writing = true;
                });
                on_step!(self, 0, {
                    self.is_writing = true;
                });
            }
            0xef => {
                set_instruction!(self, 6, {});
            }
            0xf2 => {
                set_instruction!(self, 2, {});
            }
            0xf3 => {
                set_instruction!(self, 8, {});
            }
            0xf4 => {
                set_instruction!(self, 4, {});
            }
            0xf6 => {
                set_instruction!(self, 6, {
                    zpx!(self, memory);
                    inc!(self, memory);
                });
                on_step!(self, 1, {
                    self.is_writing = true;
                });
                on_step!(self, 0, {
                    self.is_writing = true;
                });
            }
            0xf7 => {
                set_instruction!(self, 6, {});
            }
            0xfa => {
                set_instruction!(self, 2, {});
            }
            0xfb => {
                set_instruction!(self, 7, {});
            }
            0xfc => {
                set_instruction!(self, 4, {});
            }
            0xfe => {
                set_instruction!(self, 7, {
                    abx!(self, memory);
                    inc!(self, memory);
                });
                on_step!(self, 1, {
                    self.is_writing = true;
                });
                on_step!(self, 0, {
                    self.is_writing = true;
                });
            }
            0xff => {
                set_instruction!(self, 7, {});
            }
            _ => {
                self.steps = 1;
            }
        }

        self.steps -= 1;
        self.cycles += 1;
    }
}
