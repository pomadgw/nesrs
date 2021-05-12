mod test;
use std::fmt;

/// This constant represents the address of the low byte of 6502's reset vector address
pub const INTERRUPT_RESET: u16 = 0xFFFC;
pub const INTERRUPT_NMI: u16 = 0xFFFA;
pub const INTERRUPT_IRQ: u16 = 0xFFFE;

use crate::memory::Memory;

pub fn hello() {
    println!("Hello");
}

macro_rules! word {
    ($lo:expr, $hi:expr) => {
        (($hi as u16) << 8) | ($lo as u16)
    };
}

macro_rules! hi {
    ($number:expr) => {
        (($number >> 8) & 0xff) as u8
    };
}

macro_rules! lo {
    ($number:expr) => {
        ($number & 0xff) as u8
    };
}

bitflags! {
    pub struct StatusFlag: u8 {
        const C = 1 << 0; // Carry bit
        const Z = 1 << 1; // Zero
        const I = 1 << 2; // Disable Interrupt
        const D = 1 << 3; // BCD mode
        const B = 1 << 4; // Break
        const U = 1 << 5; // UNUSED -_-
        const V = 1 << 6; // Overflow
        const N = 1 << 7; // Negative
    }
}

impl StatusFlag {
    pub fn set_from_byte(&mut self, value: u8) {
        self.bits = value;
    }
}

impl fmt::Display for StatusFlag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Representing 6502's registers
pub struct CPURegisters {
    /// Representing A register (accumulator)
    pub a: u8,
    /// Representing X register
    pub x: u8,
    /// Representing Y register
    pub y: u8,
    /// Representing stack pointer register
    pub sp: u8,
    /// Representing cpu status register
    pub p: StatusFlag,
    /// Representing program counter register
    pub pc: u16,
}

impl Default for CPURegisters {
    fn default() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            sp: 0,
            p: StatusFlag::empty(),
            pc: 0,
        }
    }
}

bitflags! {
    pub struct Interrupt: u8 {
        const IRQ = 1 << 0;
        const NMI = 1 << 1;
        const RESET = 1 << 2;
    }
}

pub enum AddressMode {
    Imp,
    Acc,
    Abs,
    Abx,
    Aby,
    Imm,
    Zp0,
    Zpx,
    Zpy,
    Izx,
    Izy,
    Rel,
    Ind,
}

pub enum Opcode {
    // Xxx is dummy opcode
    Xxx,
    Brk,
    Jsr,
    Rti,
    Rts,
    Ora,
    And,
    Eor,
    Adc,
    Sta,
    Lda,
    Cmp,
    Sbc,
    Asl,
    Rol,
    Lsr,
    Ror,
    Stx,
    Ldx,
    Dec,
    Inc,
    Bit,
    Jmp,
    Sty,
    Ldy,
    Cpy,
    Cpx,
    Bpl,
    Bmi,
    Bvc,
    Bvs,
    Bcc,
    Bcs,
    Bne,
    Beq,
    Php,
    Plp,
    Pha,
    Pla,
    Dey,
    Tay,
    Iny,
    Inx,
    Clc,
    Sec,
    Cli,
    Sei,
    Tya,
    Clv,
    Cld,
    Sed,
    Txa,
    Txs,
    Tax,
    Tsx,
    Dex,
    Nop,
}

/// Emulating 6502 CPU
pub struct CPU {
    pub regs: CPURegisters,
    pub total_cycles: u32,
    cycles: u32,
    opcode: u8,
    interrupt_type: Interrupt,
    is_read_instruction: bool,
    address_mode: AddressMode,
    opcode_type: Opcode,
}

impl CPU {
    /// Create new CPU instance
    pub fn new() -> Self {
        CPU {
            regs: Default::default(),
            total_cycles: 0,
            cycles: 0,
            opcode: 0,
            interrupt_type: Interrupt::empty(),
            is_read_instruction: false,
            address_mode: AddressMode::Imp,
            opcode_type: Opcode::Brk,
        }
    }

    /// Reset the CPU
    pub fn reset(&mut self) {
        self.opcode = 0; // change opcode to BRK
        self.interrupt_type |= Interrupt::RESET; // Set interrupt type to reset
        self.is_read_instruction = false;
        self.set_instruction();
    }

    pub fn done(&self) -> bool {
        self.is_read_instruction
    }

    // Clock the CPU
    pub fn clock(&mut self, memory: &mut dyn Memory) {
        if self.is_read_instruction {
            self.opcode = memory.read(self.get_pc(), false);
            self.is_read_instruction = false;
            self.set_instruction();
        }

        self.cycles -= 1;
        self.total_cycles += 1;

        if self.cycles == 0 {
            match self.address_mode {
                AddressMode::Imp => {
                    // do nothing
                }
                _ => {}
            }

            match self.opcode_type {
                Opcode::Brk => {
                    // the behavior of BRK is defined
                    // according to this article: https://www.pagetable.com/?p=410

                    let vector = if self.interrupt_type & Interrupt::RESET == Interrupt::RESET {
                        INTERRUPT_RESET
                    } else if self.interrupt_type & Interrupt::RESET == Interrupt::NMI {
                        INTERRUPT_NMI
                    } else if self.interrupt_type & Interrupt::RESET == Interrupt::IRQ {
                        INTERRUPT_IRQ
                    } else {
                        INTERRUPT_IRQ
                    } as usize;

                    // Skip next PC if it is invoked from BRK instruction
                    // (not interupted by any means)
                    if self.interrupt_type.bits() == 0 {
                        self.get_pc();
                    }

                    if self.interrupt_type & Interrupt::RESET == Interrupt::RESET {
                        self.push_stack(memory, 0);
                        self.push_stack(memory, 0);
                    } else {
                        self.push_stack(memory, hi!(self.regs.pc));
                        self.push_stack(memory, lo!(self.regs.pc));
                    }

                    self.regs.p |= StatusFlag::B;
                    self.regs.p |= StatusFlag::U;

                    if self.interrupt_type & Interrupt::RESET == Interrupt::RESET {
                        self.push_stack(memory, 0);
                    } else {
                        self.push_stack(memory, self.regs.p.bits());
                        self.regs.p &= !StatusFlag::U;
                    }

                    self.regs.p &= !StatusFlag::B;
                    self.regs.p |= StatusFlag::I;

                    let lo = memory.read(vector, false);
                    let hi = memory.read(vector + 1, false);
                    self.regs.pc = word!(lo, hi);
                }
                _ => {}
            }

            self.is_read_instruction = true;
        }
    }

    // BEGIN PRIVATE
    fn push_stack(&mut self, memory: &mut dyn Memory, value: u8) {
        let address = 0x0100 + self.regs.sp as usize;
        memory.write(address, value);
        self.regs.sp = self.regs.sp.wrapping_sub(1);
    }

    fn set_instruction(&mut self) {
        // set default addressing mode
        self.address_mode = AddressMode::Imp;

        // decode instructions based on
        // https://llx.com/Neil/a2/opcodes.html
        match self.opcode {
            0 => {
                self.opcode_type = Opcode::Brk;
            }
            0x20 => {
                self.opcode_type = Opcode::Jsr;
                self.address_mode = AddressMode::Abs;
            }
            0x40 => {
                self.opcode_type = Opcode::Rti;
            }
            0x60 => {
                self.opcode_type = Opcode::Rts;
            }
            0x08 => {
                self.opcode_type = Opcode::Php;
            }
            0x28 => {
                self.opcode_type = Opcode::Plp;
            }
            0x48 => {
                self.opcode_type = Opcode::Pha;
            }
            0x68 => {
                self.opcode_type = Opcode::Pla;
            }
            0x88 => {
                self.opcode_type = Opcode::Dey;
            }
            0xA8 => {
                self.opcode_type = Opcode::Tay;
            }
            0xC8 => {
                self.opcode_type = Opcode::Iny;
            }
            0xE8 => {
                self.opcode_type = Opcode::Inx;
            }
            0x18 => {
                self.opcode_type = Opcode::Clc;
            }
            0x38 => {
                self.opcode_type = Opcode::Sec;
            }
            0x58 => {
                self.opcode_type = Opcode::Cli;
            }
            0x78 => {
                self.opcode_type = Opcode::Sei;
            }
            0x98 => {
                self.opcode_type = Opcode::Tya;
            }
            0xB8 => {
                self.opcode_type = Opcode::Clv;
            }
            0xD8 => {
                self.opcode_type = Opcode::Cld;
            }
            0xF8 => {
                self.opcode_type = Opcode::Sed;
            }
            0x8A => {
                self.opcode_type = Opcode::Txa;
            }
            0x9A => {
                self.opcode_type = Opcode::Txs;
            }
            0xAA => {
                self.opcode_type = Opcode::Tax;
            }
            0xBA => {
                self.opcode_type = Opcode::Tsx;
            }
            0xCA => {
                self.opcode_type = Opcode::Dex;
            }
            0xEA => {
                self.opcode_type = Opcode::Nop;
            }
            _ => {
                let a = (self.opcode >> 5) & 0x07;
                let b = (self.opcode >> 2) >> 0x07;
                let c = self.opcode & 0x03;
                let is_branching_opcode = (self.opcode & 0x1f) == 0x10;

                if is_branching_opcode {
                    let x = (self.opcode >> 6) & 0x03;
                    let y = (self.opcode >> 5) & 0x01;
                    self.address_mode = AddressMode::Rel;

                    // opcode: status flag is clear
                    if y == 0 {
                        self.opcode_type = match x {
                            0 => Opcode::Bpl,
                            1 => Opcode::Bvc,
                            2 => Opcode::Bcc,
                            3 => Opcode::Bne,
                            _ => Opcode::Xxx,
                        };
                    } else {
                        self.opcode_type = match x {
                            0 => Opcode::Bmi,
                            1 => Opcode::Bvs,
                            2 => Opcode::Bcs,
                            3 => Opcode::Beq,
                            _ => Opcode::Xxx,
                        };
                    }
                } else {
                    if c == 0 {
                        self.opcode_type = match a {
                            1 => Opcode::Bit,
                            2 => Opcode::Jmp,
                            3 => Opcode::Jmp,
                            4 => Opcode::Sty,
                            5 => Opcode::Ldy,
                            6 => Opcode::Cpy,
                            7 => Opcode::Cpx,
                            _ => Opcode::Xxx,
                        };

                        self.address_mode = match b {
                            0 => AddressMode::Imm,
                            1 => AddressMode::Zp0,
                            3 => AddressMode::Abs,
                            5 => AddressMode::Zpx,
                            7 => AddressMode::Abx,
                            _ => AddressMode::Imp,
                        };

                        if a == 3 {
                            self.address_mode = AddressMode::Ind;
                        }
                    } else if c == 1 {
                        self.opcode_type = match a {
                            0 => Opcode::Ora,
                            1 => Opcode::And,
                            2 => Opcode::Eor,
                            3 => Opcode::Adc,
                            4 => Opcode::Sta,
                            5 => Opcode::Lda,
                            6 => Opcode::Cmp,
                            7 => Opcode::Sbc,
                            _ => Opcode::Xxx,
                        };

                        self.address_mode = match b {
                            0 => AddressMode::Izx,
                            1 => AddressMode::Zp0,
                            2 => AddressMode::Imm,
                            3 => AddressMode::Abs,
                            4 => AddressMode::Izy,
                            5 => AddressMode::Zpx,
                            6 => AddressMode::Abx,
                            7 => AddressMode::Aby,
                            _ => AddressMode::Imp,
                        }
                    } else if c == 2 {
                        self.opcode_type = match a {
                            0 => Opcode::Asl,
                            1 => Opcode::Rol,
                            2 => Opcode::Lsr,
                            3 => Opcode::Ror,
                            4 => Opcode::Stx,
                            5 => Opcode::Ldx,
                            6 => Opcode::Dec,
                            7 => Opcode::Inc,
                            _ => Opcode::Xxx,
                        };

                        self.address_mode = match b {
                            0 => AddressMode::Imm,
                            1 => AddressMode::Zp0,
                            2 => AddressMode::Acc,
                            3 => AddressMode::Abs,
                            5 => AddressMode::Zpx,
                            7 => AddressMode::Abx,
                            _ => AddressMode::Imp,
                        }
                    }
                }
            }
        }
    }

    fn get_pc(&mut self) -> usize {
        let pc = self.regs.pc as usize;

        self.regs.pc = self.regs.pc.wrapping_add(1);

        pc
    }
    // END PRIVATE
}
