use std::fmt;
use std::ops::{Add, AddAssign};

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

pub struct Int16 {
    pub lo: u8,
    pub hi: u8,
    is_carry: bool,
}

impl Int16 {
    pub fn new(lo: u8, hi: u8) -> Self {
        Self {
            lo,
            hi,
            is_carry: false,
        }
    }

    pub fn new_from_16(number: u16) -> Self {
        Self::new((number & 0xff) as u8, ((number >> 8) & 0xff) as u8)
    }

    pub fn set_u16(&mut self, number: u16) {
        self.lo = (number & 0xff) as u8;
        self.hi = ((number >> 8) & 0xff) as u8;
    }

    pub fn to_u16(&self) -> u16 {
        let hi = self.hi as u16;
        let lo = self.lo as u16;

        (hi << 8) | lo
    }

    pub fn to_usize(&self) -> usize {
        self.to_u16() as usize
    }

    pub fn add_hi_from_carry(&mut self) {
        if self.is_carry {
            self.hi += 1;
            self.clear_carry();
        }
    }

    pub fn clear_carry(&mut self) {
        self.is_carry = false;
    }

    pub fn has_carry(&self) -> bool {
        self.is_carry
    }
}

impl Add<u8> for Int16 {
    type Output = Int16;

    fn add(self, number: u8) -> Self {
        let hi = self.hi as u16;
        let lo = self.lo as u16;
        let result = lo.wrapping_add(number as u16);

        Self {
            lo: (result & 0xff) as u8,
            hi: hi as u8,
            is_carry: result >= 0x100,
        }
    }
}

impl AddAssign<u8> for Int16 {
    fn add_assign(&mut self, number: u8) {
        let lo = self.lo as u16;
        let result = lo.wrapping_add(number as u16);

        self.is_carry = result >= 0x100;
        self.lo = (result & 0xff) as u8;
    }
}

pub enum RegisterAccess {
    A,
    X,
    Y,
    None,
}

pub enum Microcode {
    FetchOpcode,
    FetchParameters,

    // IMM
    FetchImm,

    // ABS
    FetchLo,
    FetchHi,

    // ABX
    // FetchLoX,
    FetchHiX,

    // ABY
    // FetchLoY,
    FetchHiY,

    // ZP0
    FetchLoZP,

    // ZPX, ZPY
    FetchLoZP1,

    // IZX
    FetchIZX1,
    FetchIZX2,
    FetchIZX3,
    FetchIZX4,

    // IZY
    FetchIZY1,
    FetchIZY2,
    FetchIZY3,
    FetchIZY4,

    // ABX, ABY, IZY
    SetCrossPage,

    DelayedExecute,
    Execute,

    // ASL
    AslA,
    AslFetch,
    AslWrite,
    AslAddAndWrite,

    // BRK
    BrkPushPCHi,
    BrkPushPCLo,
    BrkPushStatus,
    BrkPushReadPCLo,
    BrkPushReadPCHi,
    BrkSetPC,
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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

impl fmt::Display for AddressMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub const OPCODE_TABLE: [(AddressMode, Opcode, u32); 256] = [
    (AddressMode::Imp, Opcode::Brk, 7),
    (AddressMode::Izx, Opcode::Ora, 6),
    (AddressMode::Imp, Opcode::Xxx, 2),
    (AddressMode::Imp, Opcode::Xxx, 8),
    (AddressMode::Imp, Opcode::Nop, 3),
    (AddressMode::Zp0, Opcode::Ora, 3),
    (AddressMode::Zp0, Opcode::Asl, 5),
    (AddressMode::Imp, Opcode::Xxx, 5),
    (AddressMode::Imp, Opcode::Php, 3),
    (AddressMode::Imm, Opcode::Ora, 2),
    (AddressMode::Acc, Opcode::Asl, 2),
    (AddressMode::Imp, Opcode::Xxx, 2),
    (AddressMode::Imp, Opcode::Nop, 4),
    (AddressMode::Abs, Opcode::Ora, 4),
    (AddressMode::Abs, Opcode::Asl, 6),
    (AddressMode::Imp, Opcode::Xxx, 6),
    (AddressMode::Rel, Opcode::Bpl, 2),
    (AddressMode::Izy, Opcode::Ora, 5),
    (AddressMode::Imp, Opcode::Xxx, 2),
    (AddressMode::Imp, Opcode::Xxx, 8),
    (AddressMode::Imp, Opcode::Nop, 4),
    (AddressMode::Zpx, Opcode::Ora, 4),
    (AddressMode::Zpx, Opcode::Asl, 6),
    (AddressMode::Imp, Opcode::Xxx, 6),
    (AddressMode::Imp, Opcode::Clc, 2),
    (AddressMode::Aby, Opcode::Ora, 4),
    (AddressMode::Imp, Opcode::Nop, 2),
    (AddressMode::Imp, Opcode::Xxx, 7),
    (AddressMode::Imp, Opcode::Nop, 4),
    (AddressMode::Abx, Opcode::Ora, 4),
    (AddressMode::Abx, Opcode::Asl, 7),
    (AddressMode::Imp, Opcode::Xxx, 7),
    (AddressMode::Abs, Opcode::Jsr, 6),
    (AddressMode::Izx, Opcode::And, 6),
    (AddressMode::Imp, Opcode::Xxx, 2),
    (AddressMode::Imp, Opcode::Xxx, 8),
    (AddressMode::Zp0, Opcode::Bit, 3),
    (AddressMode::Zp0, Opcode::And, 3),
    (AddressMode::Zp0, Opcode::Rol, 5),
    (AddressMode::Imp, Opcode::Xxx, 5),
    (AddressMode::Imp, Opcode::Plp, 4),
    (AddressMode::Imm, Opcode::And, 2),
    (AddressMode::Acc, Opcode::Rol, 2),
    (AddressMode::Imp, Opcode::Xxx, 2),
    (AddressMode::Abs, Opcode::Bit, 4),
    (AddressMode::Abs, Opcode::And, 4),
    (AddressMode::Abs, Opcode::Rol, 6),
    (AddressMode::Imp, Opcode::Xxx, 6),
    (AddressMode::Rel, Opcode::Bmi, 2),
    (AddressMode::Izy, Opcode::And, 5),
    (AddressMode::Imp, Opcode::Xxx, 2),
    (AddressMode::Imp, Opcode::Xxx, 8),
    (AddressMode::Imp, Opcode::Nop, 4),
    (AddressMode::Zpx, Opcode::And, 4),
    (AddressMode::Zpx, Opcode::Rol, 6),
    (AddressMode::Imp, Opcode::Xxx, 6),
    (AddressMode::Imp, Opcode::Sec, 2),
    (AddressMode::Aby, Opcode::And, 4),
    (AddressMode::Imp, Opcode::Nop, 2),
    (AddressMode::Imp, Opcode::Xxx, 7),
    (AddressMode::Imp, Opcode::Nop, 4),
    (AddressMode::Abx, Opcode::And, 4),
    (AddressMode::Abx, Opcode::Rol, 7),
    (AddressMode::Imp, Opcode::Xxx, 7),
    (AddressMode::Imp, Opcode::Rti, 6),
    (AddressMode::Izx, Opcode::Eor, 6),
    (AddressMode::Imp, Opcode::Xxx, 2),
    (AddressMode::Imp, Opcode::Xxx, 8),
    (AddressMode::Imp, Opcode::Nop, 3),
    (AddressMode::Zp0, Opcode::Eor, 3),
    (AddressMode::Zp0, Opcode::Lsr, 5),
    (AddressMode::Imp, Opcode::Xxx, 5),
    (AddressMode::Imp, Opcode::Pha, 3),
    (AddressMode::Imm, Opcode::Eor, 2),
    (AddressMode::Acc, Opcode::Lsr, 2),
    (AddressMode::Imp, Opcode::Xxx, 2),
    (AddressMode::Abs, Opcode::Jmp, 3),
    (AddressMode::Abs, Opcode::Eor, 4),
    (AddressMode::Abs, Opcode::Lsr, 6),
    (AddressMode::Imp, Opcode::Xxx, 6),
    (AddressMode::Rel, Opcode::Bvc, 2),
    (AddressMode::Izy, Opcode::Eor, 5),
    (AddressMode::Imp, Opcode::Xxx, 2),
    (AddressMode::Imp, Opcode::Xxx, 8),
    (AddressMode::Imp, Opcode::Nop, 4),
    (AddressMode::Zpx, Opcode::Eor, 4),
    (AddressMode::Zpx, Opcode::Lsr, 6),
    (AddressMode::Imp, Opcode::Xxx, 6),
    (AddressMode::Imp, Opcode::Cli, 2),
    (AddressMode::Aby, Opcode::Eor, 4),
    (AddressMode::Imp, Opcode::Nop, 2),
    (AddressMode::Imp, Opcode::Xxx, 7),
    (AddressMode::Imp, Opcode::Nop, 4),
    (AddressMode::Abx, Opcode::Eor, 4),
    (AddressMode::Abx, Opcode::Lsr, 7),
    (AddressMode::Imp, Opcode::Xxx, 7),
    (AddressMode::Imp, Opcode::Rts, 6),
    (AddressMode::Izx, Opcode::Adc, 6),
    (AddressMode::Imp, Opcode::Xxx, 2),
    (AddressMode::Imp, Opcode::Xxx, 8),
    (AddressMode::Imp, Opcode::Nop, 3),
    (AddressMode::Zp0, Opcode::Adc, 3),
    (AddressMode::Zp0, Opcode::Ror, 5),
    (AddressMode::Imp, Opcode::Xxx, 5),
    (AddressMode::Imp, Opcode::Pla, 4),
    (AddressMode::Imm, Opcode::Adc, 2),
    (AddressMode::Acc, Opcode::Ror, 2),
    (AddressMode::Imp, Opcode::Xxx, 2),
    (AddressMode::Ind, Opcode::Jmp, 5),
    (AddressMode::Abs, Opcode::Adc, 4),
    (AddressMode::Abs, Opcode::Ror, 6),
    (AddressMode::Imp, Opcode::Xxx, 6),
    (AddressMode::Rel, Opcode::Bvs, 2),
    (AddressMode::Izy, Opcode::Adc, 5),
    (AddressMode::Imp, Opcode::Xxx, 2),
    (AddressMode::Imp, Opcode::Xxx, 8),
    (AddressMode::Imp, Opcode::Nop, 4),
    (AddressMode::Zpx, Opcode::Adc, 4),
    (AddressMode::Zpx, Opcode::Ror, 6),
    (AddressMode::Imp, Opcode::Xxx, 6),
    (AddressMode::Imp, Opcode::Sei, 2),
    (AddressMode::Aby, Opcode::Adc, 4),
    (AddressMode::Imp, Opcode::Nop, 2),
    (AddressMode::Imp, Opcode::Xxx, 7),
    (AddressMode::Imp, Opcode::Nop, 4),
    (AddressMode::Abx, Opcode::Adc, 4),
    (AddressMode::Abx, Opcode::Ror, 7),
    (AddressMode::Imp, Opcode::Xxx, 7),
    (AddressMode::Imp, Opcode::Nop, 2),
    (AddressMode::Izx, Opcode::Sta, 6),
    (AddressMode::Imp, Opcode::Nop, 2),
    (AddressMode::Imp, Opcode::Xxx, 6),
    (AddressMode::Zp0, Opcode::Sty, 3),
    (AddressMode::Zp0, Opcode::Sta, 3),
    (AddressMode::Zp0, Opcode::Stx, 3),
    (AddressMode::Imp, Opcode::Xxx, 3),
    (AddressMode::Imp, Opcode::Dey, 2),
    (AddressMode::Imp, Opcode::Nop, 2),
    (AddressMode::Imp, Opcode::Txa, 2),
    (AddressMode::Imp, Opcode::Xxx, 2),
    (AddressMode::Abs, Opcode::Sty, 4),
    (AddressMode::Abs, Opcode::Sta, 4),
    (AddressMode::Abs, Opcode::Stx, 4),
    (AddressMode::Imp, Opcode::Xxx, 4),
    (AddressMode::Rel, Opcode::Bcc, 2),
    (AddressMode::Izy, Opcode::Sta, 6),
    (AddressMode::Imp, Opcode::Xxx, 2),
    (AddressMode::Imp, Opcode::Xxx, 6),
    (AddressMode::Zpx, Opcode::Sty, 4),
    (AddressMode::Zpx, Opcode::Sta, 4),
    (AddressMode::Zpy, Opcode::Stx, 4),
    (AddressMode::Imp, Opcode::Xxx, 4),
    (AddressMode::Imp, Opcode::Tya, 2),
    (AddressMode::Aby, Opcode::Sta, 5),
    (AddressMode::Imp, Opcode::Txs, 2),
    (AddressMode::Imp, Opcode::Xxx, 5),
    (AddressMode::Imp, Opcode::Nop, 5),
    (AddressMode::Abx, Opcode::Sta, 5),
    (AddressMode::Imp, Opcode::Xxx, 5),
    (AddressMode::Imp, Opcode::Xxx, 5),
    (AddressMode::Imm, Opcode::Ldy, 2),
    (AddressMode::Izx, Opcode::Lda, 6),
    (AddressMode::Imm, Opcode::Ldx, 2),
    (AddressMode::Imp, Opcode::Xxx, 6),
    (AddressMode::Zp0, Opcode::Ldy, 3),
    (AddressMode::Zp0, Opcode::Lda, 3),
    (AddressMode::Zp0, Opcode::Ldx, 3),
    (AddressMode::Imp, Opcode::Xxx, 3),
    (AddressMode::Imp, Opcode::Tay, 2),
    (AddressMode::Imm, Opcode::Lda, 2),
    (AddressMode::Imp, Opcode::Tax, 2),
    (AddressMode::Imp, Opcode::Xxx, 2),
    (AddressMode::Abs, Opcode::Ldy, 4),
    (AddressMode::Abs, Opcode::Lda, 4),
    (AddressMode::Abs, Opcode::Ldx, 4),
    (AddressMode::Imp, Opcode::Xxx, 4),
    (AddressMode::Rel, Opcode::Bcs, 2),
    (AddressMode::Izy, Opcode::Lda, 5),
    (AddressMode::Imp, Opcode::Xxx, 2),
    (AddressMode::Imp, Opcode::Xxx, 5),
    (AddressMode::Zpx, Opcode::Ldy, 4),
    (AddressMode::Zpx, Opcode::Lda, 4),
    (AddressMode::Zpy, Opcode::Ldx, 4),
    (AddressMode::Imp, Opcode::Xxx, 4),
    (AddressMode::Imp, Opcode::Clv, 2),
    (AddressMode::Aby, Opcode::Lda, 4),
    (AddressMode::Imp, Opcode::Tsx, 2),
    (AddressMode::Imp, Opcode::Xxx, 4),
    (AddressMode::Abx, Opcode::Ldy, 4),
    (AddressMode::Abx, Opcode::Lda, 4),
    (AddressMode::Aby, Opcode::Ldx, 4),
    (AddressMode::Imp, Opcode::Xxx, 4),
    (AddressMode::Imm, Opcode::Cpy, 2),
    (AddressMode::Izx, Opcode::Cmp, 6),
    (AddressMode::Imp, Opcode::Nop, 2),
    (AddressMode::Imp, Opcode::Xxx, 8),
    (AddressMode::Zp0, Opcode::Cpy, 3),
    (AddressMode::Zp0, Opcode::Cmp, 3),
    (AddressMode::Zp0, Opcode::Dec, 5),
    (AddressMode::Imp, Opcode::Xxx, 5),
    (AddressMode::Imp, Opcode::Iny, 2),
    (AddressMode::Imm, Opcode::Cmp, 2),
    (AddressMode::Imp, Opcode::Dex, 2),
    (AddressMode::Imp, Opcode::Xxx, 2),
    (AddressMode::Abs, Opcode::Cpy, 4),
    (AddressMode::Abs, Opcode::Cmp, 4),
    (AddressMode::Abs, Opcode::Dec, 6),
    (AddressMode::Imp, Opcode::Xxx, 6),
    (AddressMode::Rel, Opcode::Bne, 2),
    (AddressMode::Izy, Opcode::Cmp, 5),
    (AddressMode::Imp, Opcode::Xxx, 2),
    (AddressMode::Imp, Opcode::Xxx, 8),
    (AddressMode::Imp, Opcode::Nop, 4),
    (AddressMode::Zpx, Opcode::Cmp, 4),
    (AddressMode::Zpx, Opcode::Dec, 6),
    (AddressMode::Imp, Opcode::Xxx, 6),
    (AddressMode::Imp, Opcode::Cld, 2),
    (AddressMode::Aby, Opcode::Cmp, 4),
    (AddressMode::Imp, Opcode::Nop, 2),
    (AddressMode::Imp, Opcode::Xxx, 7),
    (AddressMode::Imp, Opcode::Nop, 4),
    (AddressMode::Abx, Opcode::Cmp, 4),
    (AddressMode::Abx, Opcode::Dec, 7),
    (AddressMode::Imp, Opcode::Xxx, 7),
    (AddressMode::Imm, Opcode::Cpx, 2),
    (AddressMode::Izx, Opcode::Sbc, 6),
    (AddressMode::Imp, Opcode::Nop, 2),
    (AddressMode::Imp, Opcode::Xxx, 8),
    (AddressMode::Zp0, Opcode::Cpx, 3),
    (AddressMode::Zp0, Opcode::Sbc, 3),
    (AddressMode::Zp0, Opcode::Inc, 5),
    (AddressMode::Imp, Opcode::Xxx, 5),
    (AddressMode::Imp, Opcode::Inx, 2),
    (AddressMode::Imm, Opcode::Sbc, 2),
    (AddressMode::Imp, Opcode::Nop, 2),
    (AddressMode::Imp, Opcode::Sbc, 2),
    (AddressMode::Abs, Opcode::Cpx, 4),
    (AddressMode::Abs, Opcode::Sbc, 4),
    (AddressMode::Abs, Opcode::Inc, 6),
    (AddressMode::Imp, Opcode::Xxx, 6),
    (AddressMode::Rel, Opcode::Beq, 2),
    (AddressMode::Izy, Opcode::Sbc, 5),
    (AddressMode::Imp, Opcode::Xxx, 2),
    (AddressMode::Imp, Opcode::Xxx, 8),
    (AddressMode::Imp, Opcode::Nop, 4),
    (AddressMode::Zpx, Opcode::Sbc, 4),
    (AddressMode::Zpx, Opcode::Inc, 6),
    (AddressMode::Imp, Opcode::Xxx, 6),
    (AddressMode::Imp, Opcode::Sed, 2),
    (AddressMode::Aby, Opcode::Sbc, 4),
    (AddressMode::Imp, Opcode::Nop, 2),
    (AddressMode::Imp, Opcode::Xxx, 7),
    (AddressMode::Imp, Opcode::Nop, 4),
    (AddressMode::Abx, Opcode::Sbc, 4),
    (AddressMode::Abx, Opcode::Inc, 7),
    (AddressMode::Imp, Opcode::Xxx, 7),
];
