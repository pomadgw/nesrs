use crate::cpu::types::*;
use crate::cpu::*;

fn asl(value: u16, _other: u16) -> (u16, bool) {
    let result = value << 1;
    (result, result > 0xff)
}

fn lsr(value: u16, _other: u16) -> (u16, bool) {
    let result = value >> 1;
    (result, (value & 0x01) > 0)
}

fn rol(value: u16, carry: u16) -> (u16, bool) {
    let result = value << 1 | carry;
    (result, (value & 0x80) > 0)
}

fn ror(value: u16, carry: u16) -> (u16, bool) {
    let result = value >> 1 | (carry << 7);
    (result, (value & 0x01) > 0)
}

impl CPU {
    pub fn do_instruction(&mut self, memory: &mut dyn Memory) {
        match self.opcode_type {
            Opcode::Brk => {
                self.next_state(Microcode::BrkPushPCHi);
                self.run_next_state(memory);
            }
            Opcode::Rti => {
                self.next_state(Microcode::RtiPopStatus);
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
            Opcode::Asl | Opcode::Lsr | Opcode::Rol | Opcode::Ror => {
                self.shift_op = match self.opcode_type {
                    Opcode::Asl => Some(asl),
                    Opcode::Lsr => Some(lsr),
                    Opcode::Rol => Some(rol),
                    Opcode::Ror => Some(ror),
                    _ => None,
                };

                match self.register_access {
                    RegisterAccess::A => {
                        self.next_state(Microcode::ShiftA);
                    }
                    _ => {
                        self.next_state(Microcode::ShiftFetch);
                        self.run_next_state(memory);
                    }
                }
            }
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
                self.set_nz(self.regs.x);
                self.fetch_opcode();
            }
            Opcode::Pha => {
                self.next_state(Microcode::PhaPushStack);
            }
            Opcode::Pla => {
                self.next_state(Microcode::PlaPull);
            }
            Opcode::Php => {
                self.next_state(Microcode::PhpPushStack);
            }
            Opcode::Plp => {
                self.next_state(Microcode::PlpPull);
            }
            // For documentation only
            //
            // Copy of the code in clock fn is the one that
            // will be executed
            Opcode::Jmp => {
                self.regs.pc = self.absolute_address as u16;
                self.fetch_opcode();
            }
            Opcode::Rts => {
                self.next_state(Microcode::RtsGetPcLo);
            }
            Opcode::Sei => {
                self.regs.p |= StatusFlag::I;
                self.fetch_opcode();
            }
            Opcode::Cli => {
                self.regs.p &= !StatusFlag::I;
                self.fetch_opcode();
            }
            Opcode::Sec => {
                self.regs.p |= StatusFlag::C;
                self.fetch_opcode();
            }
            Opcode::Clc => {
                self.regs.p &= !StatusFlag::C;
                self.fetch_opcode();
            }
            Opcode::Sed => {
                self.regs.p |= StatusFlag::D;
                self.fetch_opcode();
            }
            Opcode::Cld => {
                self.regs.p &= !StatusFlag::D;
                self.fetch_opcode();
            }
            Opcode::Clv => {
                self.regs.p &= !StatusFlag::V;
                self.fetch_opcode();
            }
            Opcode::And => {
                self.regs.a &= self.read(memory, self.absolute_address);
                self.set_nz(self.regs.a);
                self.fetch_opcode();
            }
            Opcode::Ora => {
                self.regs.a |= self.read(memory, self.absolute_address);
                self.set_nz(self.regs.a);
                self.fetch_opcode();
            }
            Opcode::Eor => {
                self.regs.a ^= self.read(memory, self.absolute_address);
                self.set_nz(self.regs.a);
                self.fetch_opcode();
            }
            Opcode::Bit => {
                self.fetched_data = self.read(memory, self.absolute_address);
                if (self.fetched_data & self.regs.a) == 0 {
                    self.regs.p |= StatusFlag::Z;
                } else {
                    self.regs.p &= !StatusFlag::Z;
                }

                let bit = (self.regs.p.bits() & 0b0011_1111) | (self.fetched_data & 0b1100_0000);
                self.regs.p.set_from_byte(bit);
                self.fetch_opcode();
            }
            Opcode::Adc => {
                let fetched_data = self.read(memory, self.absolute_address) as u16;
                let carry = (self.regs.p.bits() & 0x01) as u16;
                let a = self.regs.a as u16;
                let result = a + fetched_data + carry;
                self.regs.p.set(StatusFlag::C, result > 0xff);
                self.regs.p.set(
                    StatusFlag::V,
                    (!(a ^ fetched_data) & (a ^ (result & 0xff)) & 0x0080) > 0,
                );

                self.regs.a = (result & 0xff) as u8;
                self.set_nz(self.regs.a);
                self.fetch_opcode();
            }
            Opcode::Sbc => {
                let fetched_data = self.read(memory, self.absolute_address);
                let temp = (fetched_data ^ 0xff) as u16;
                let carry = (self.regs.p.bits() & 0x01) as u16;
                let a = self.regs.a as u16;

                let result = a + temp + carry;
                self.regs.a = (result & 0xff) as u8;

                self.regs.p.set(StatusFlag::C, result > 0xff);
                self.regs.p.set(
                    StatusFlag::V,
                    (!(a ^ temp) & (a ^ (result & 0xff)) & 0x0080) > 0,
                );
                self.set_nz(self.regs.a);
                self.fetch_opcode();
            }
            Opcode::Cmp => {
                let fetched_data = self.read(memory, self.absolute_address);
                let result = ((self.regs.a as i16) - (fetched_data as i16)) as i8;
                self.regs.p.set(StatusFlag::C, self.regs.a >= fetched_data);
                self.regs.p.set(StatusFlag::Z, result == 0x00);
                self.regs.p.set(StatusFlag::N, result < 0x00);
                self.fetch_opcode();
            }
            Opcode::Cpx => {
                let fetched_data = self.read(memory, self.absolute_address);
                let result = ((self.regs.x as i16) - (fetched_data as i16)) as i8;
                self.regs.p.set(StatusFlag::C, self.regs.x >= fetched_data);
                self.regs.p.set(StatusFlag::Z, result == 0x00);
                self.regs.p.set(StatusFlag::N, result < 0x00);
                self.fetch_opcode();
            }
            Opcode::Cpy => {
                let fetched_data = self.read(memory, self.absolute_address);
                let result = ((self.regs.y as i16) - (fetched_data as i16)) as i8;
                self.regs.p.set(StatusFlag::C, self.regs.y >= fetched_data);
                self.regs.p.set(StatusFlag::Z, result == 0x00);
                self.regs.p.set(StatusFlag::N, result < 0x00);
                self.fetch_opcode();
            }
            Opcode::Dec => {
                self.next_state(Microcode::DecReadData);
                self.run_next_state(memory);
            }
            Opcode::Inc => {
                self.next_state(Microcode::IncReadData);
                self.run_next_state(memory);
            }
            _ => {
                if self.cycles > 0 {
                    self.cycles -= 1;
                } else {
                    self.fetch_opcode();
                }
            }
        }
    }
}
