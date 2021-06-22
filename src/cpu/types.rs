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

impl Interrupt {
    pub fn clear(&mut self) {
        self.bits = 0;
    }
}

pub struct Pin {
    pub delay: u32,
    pub state: bool,
}

impl Pin {
    pub fn default() -> Pin {
        Pin {
            delay: 0,
            state: false,
        }
    }

    pub fn pull(&mut self) {
        self.delay = 0;
        self.state = true;
    }

    pub fn clear(&mut self) {
        self.delay = 0;
        self.state = false;
    }

    pub fn pull_with_delay(&mut self, delay: u32) {
        self.delay = delay;
        self.state = true;
    }

    pub fn is_pulled(&self) -> bool {
        self.delay == 0 && self.state
    }

    pub fn decrease_delay(&mut self) {
        if self.delay > 0 {
            self.delay -= 1;
        }
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
            self.hi = self.hi.wrapping_add(1);
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

pub type ShiftBinaryOperation = fn(u16, u16) -> u16;

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

    // ABX, ABY, IZY
    SetCrossPage,

    // JMP IND
    IndReadLo,
    IndReadHi,
    IndReadActualLo,
    IndReadActualHiAndJump,

    DelayedExecute,
    Execute,

    // ASL & other shift operations
    ShiftA,
    ShiftFetch,
    ShiftWrite,
    ShiftAddAndWrite,

    // BRK
    BrkPushPCHi,
    BrkPushPCLo,
    BrkPushStatus,
    BrkPushReadPCLo,
    BrkPushReadPCHi,
    BrkSetPC,

    // PHA
    PhaPushStack,

    // PLA
    PlaPull,
    PlaPull1,

    // PHP
    PhpPushStack,

    // PLP
    PlpPull,
    PlpPull1,

    // JSR
    JsrSaveHiPrevPc,
    JsrSaveLoPrevPc,
    JsrJump,

    // RTS
    RtsGetPcLo,
    RtsGetPcHi,
    RtsJump,
    RtsWasteOneCycle,

    // RTI
    RtiPopStatus,
    RtiPopLoPC,
    RtiPopHiPC,
    RtiSetPC,

    // Branch
    BranchReadOffsetAndCheck,
    BranchJumpIfTrue,
    BranchJumpIfTrueAndCrossPage,

    // DEC
    DecReadData,
    IncReadData,

    IncDecWriteOld,
    IncDecWriteNew,
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
    /* 00 */ (AddressMode::Imp, Opcode::Brk, 7),
    /* 01 */ (AddressMode::Izx, Opcode::Ora, 6),
    /* 02 */ (AddressMode::Imp, Opcode::Xxx, 2),
    /* 03 */ (AddressMode::Imp, Opcode::Xxx, 8),
    /* 04 */ (AddressMode::Imp, Opcode::Nop, 3),
    /* 05 */ (AddressMode::Zp0, Opcode::Ora, 3),
    /* 06 */ (AddressMode::Zp0, Opcode::Asl, 5),
    /* 07 */ (AddressMode::Imp, Opcode::Xxx, 5),
    /* 08 */ (AddressMode::Imp, Opcode::Php, 3),
    /* 09 */ (AddressMode::Imm, Opcode::Ora, 2),
    /* 0a */ (AddressMode::Acc, Opcode::Asl, 2),
    /* 0b */ (AddressMode::Imp, Opcode::Xxx, 2),
    /* 0c */ (AddressMode::Imp, Opcode::Nop, 4),
    /* 0d */ (AddressMode::Abs, Opcode::Ora, 4),
    /* 0e */ (AddressMode::Abs, Opcode::Asl, 6),
    /* 0f */ (AddressMode::Imp, Opcode::Xxx, 6),
    /* 10 */ (AddressMode::Rel, Opcode::Bpl, 2),
    /* 11 */ (AddressMode::Izy, Opcode::Ora, 5),
    /* 12 */ (AddressMode::Imp, Opcode::Xxx, 2),
    /* 13 */ (AddressMode::Imp, Opcode::Xxx, 8),
    /* 14 */ (AddressMode::Imp, Opcode::Nop, 4),
    /* 15 */ (AddressMode::Zpx, Opcode::Ora, 4),
    /* 16 */ (AddressMode::Zpx, Opcode::Asl, 6),
    /* 17 */ (AddressMode::Imp, Opcode::Xxx, 6),
    /* 18 */ (AddressMode::Imp, Opcode::Clc, 2),
    /* 19 */ (AddressMode::Aby, Opcode::Ora, 4),
    /* 1a */ (AddressMode::Imp, Opcode::Nop, 2),
    /* 1b */ (AddressMode::Imp, Opcode::Xxx, 7),
    /* 1c */ (AddressMode::Abx, Opcode::Nop, 4),
    /* 1d */ (AddressMode::Abx, Opcode::Ora, 4),
    /* 1e */ (AddressMode::Abx, Opcode::Asl, 7),
    /* 1f */ (AddressMode::Imp, Opcode::Xxx, 7),
    /* 20 */ (AddressMode::Abs, Opcode::Jsr, 6),
    /* 21 */ (AddressMode::Izx, Opcode::And, 6),
    /* 22 */ (AddressMode::Imp, Opcode::Xxx, 2),
    /* 23 */ (AddressMode::Imp, Opcode::Xxx, 8),
    /* 24 */ (AddressMode::Zp0, Opcode::Bit, 3),
    /* 25 */ (AddressMode::Zp0, Opcode::And, 3),
    /* 26 */ (AddressMode::Zp0, Opcode::Rol, 5),
    /* 27 */ (AddressMode::Imp, Opcode::Xxx, 5),
    /* 28 */ (AddressMode::Imp, Opcode::Plp, 4),
    /* 29 */ (AddressMode::Imm, Opcode::And, 2),
    /* 2a */ (AddressMode::Acc, Opcode::Rol, 2),
    /* 2b */ (AddressMode::Imp, Opcode::Xxx, 2),
    /* 2c */ (AddressMode::Abs, Opcode::Bit, 4),
    /* 2d */ (AddressMode::Abs, Opcode::And, 4),
    /* 2e */ (AddressMode::Abs, Opcode::Rol, 6),
    /* 2f */ (AddressMode::Imp, Opcode::Xxx, 6),
    /* 30 */ (AddressMode::Rel, Opcode::Bmi, 2),
    /* 31 */ (AddressMode::Izy, Opcode::And, 5),
    /* 32 */ (AddressMode::Imp, Opcode::Xxx, 2),
    /* 33 */ (AddressMode::Imp, Opcode::Xxx, 8),
    /* 34 */ (AddressMode::Imp, Opcode::Nop, 4),
    /* 35 */ (AddressMode::Zpx, Opcode::And, 4),
    /* 36 */ (AddressMode::Zpx, Opcode::Rol, 6),
    /* 37 */ (AddressMode::Imp, Opcode::Xxx, 6),
    /* 38 */ (AddressMode::Imp, Opcode::Sec, 2),
    /* 39 */ (AddressMode::Aby, Opcode::And, 4),
    /* 3a */ (AddressMode::Imp, Opcode::Nop, 2),
    /* 3b */ (AddressMode::Imp, Opcode::Xxx, 7),
    /* 3c */ (AddressMode::Abx, Opcode::Nop, 4),
    /* 3d */ (AddressMode::Abx, Opcode::And, 4),
    /* 3e */ (AddressMode::Abx, Opcode::Rol, 7),
    /* 3f */ (AddressMode::Imp, Opcode::Xxx, 7),
    /* 40 */ (AddressMode::Imp, Opcode::Rti, 6),
    /* 41 */ (AddressMode::Izx, Opcode::Eor, 6),
    /* 42 */ (AddressMode::Imp, Opcode::Xxx, 2),
    /* 43 */ (AddressMode::Imp, Opcode::Xxx, 8),
    /* 44 */ (AddressMode::Imp, Opcode::Nop, 3),
    /* 45 */ (AddressMode::Zp0, Opcode::Eor, 3),
    /* 46 */ (AddressMode::Zp0, Opcode::Lsr, 5),
    /* 47 */ (AddressMode::Imp, Opcode::Xxx, 5),
    /* 48 */ (AddressMode::Imp, Opcode::Pha, 3),
    /* 49 */ (AddressMode::Imm, Opcode::Eor, 2),
    /* 4a */ (AddressMode::Acc, Opcode::Lsr, 2),
    /* 4b */ (AddressMode::Imp, Opcode::Xxx, 2),
    /* 4c */ (AddressMode::Abs, Opcode::Jmp, 3),
    /* 4d */ (AddressMode::Abs, Opcode::Eor, 4),
    /* 4e */ (AddressMode::Abs, Opcode::Lsr, 6),
    /* 4f */ (AddressMode::Imp, Opcode::Xxx, 6),
    /* 50 */ (AddressMode::Rel, Opcode::Bvc, 2),
    /* 51 */ (AddressMode::Izy, Opcode::Eor, 5),
    /* 52 */ (AddressMode::Imp, Opcode::Xxx, 2),
    /* 53 */ (AddressMode::Imp, Opcode::Xxx, 8),
    /* 54 */ (AddressMode::Imp, Opcode::Nop, 4),
    /* 55 */ (AddressMode::Zpx, Opcode::Eor, 4),
    /* 56 */ (AddressMode::Zpx, Opcode::Lsr, 6),
    /* 57 */ (AddressMode::Imp, Opcode::Xxx, 6),
    /* 58 */ (AddressMode::Imp, Opcode::Cli, 2),
    /* 59 */ (AddressMode::Aby, Opcode::Eor, 4),
    /* 5a */ (AddressMode::Imp, Opcode::Nop, 2),
    /* 5b */ (AddressMode::Imp, Opcode::Xxx, 7),
    /* 5c */ (AddressMode::Abx, Opcode::Nop, 4),
    /* 5d */ (AddressMode::Abx, Opcode::Eor, 4),
    /* 5e */ (AddressMode::Abx, Opcode::Lsr, 7),
    /* 5f */ (AddressMode::Imp, Opcode::Xxx, 7),
    /* 60 */ (AddressMode::Imp, Opcode::Rts, 6),
    /* 61 */ (AddressMode::Izx, Opcode::Adc, 6),
    /* 62 */ (AddressMode::Imp, Opcode::Xxx, 2),
    /* 63 */ (AddressMode::Imp, Opcode::Xxx, 8),
    /* 64 */ (AddressMode::Imp, Opcode::Nop, 3),
    /* 65 */ (AddressMode::Zp0, Opcode::Adc, 3),
    /* 66 */ (AddressMode::Zp0, Opcode::Ror, 5),
    /* 67 */ (AddressMode::Imp, Opcode::Xxx, 5),
    /* 68 */ (AddressMode::Imp, Opcode::Pla, 4),
    /* 69 */ (AddressMode::Imm, Opcode::Adc, 2),
    /* 6a */ (AddressMode::Acc, Opcode::Ror, 2),
    /* 6b */ (AddressMode::Imp, Opcode::Xxx, 2),
    /* 6c */ (AddressMode::Ind, Opcode::Jmp, 5),
    /* 6d */ (AddressMode::Abs, Opcode::Adc, 4),
    /* 6e */ (AddressMode::Abs, Opcode::Ror, 6),
    /* 6f */ (AddressMode::Imp, Opcode::Xxx, 6),
    /* 70 */ (AddressMode::Rel, Opcode::Bvs, 2),
    /* 71 */ (AddressMode::Izy, Opcode::Adc, 5),
    /* 72 */ (AddressMode::Imp, Opcode::Xxx, 2),
    /* 73 */ (AddressMode::Imp, Opcode::Xxx, 8),
    /* 74 */ (AddressMode::Imp, Opcode::Nop, 4),
    /* 75 */ (AddressMode::Zpx, Opcode::Adc, 4),
    /* 76 */ (AddressMode::Zpx, Opcode::Ror, 6),
    /* 77 */ (AddressMode::Imp, Opcode::Xxx, 6),
    /* 78 */ (AddressMode::Imp, Opcode::Sei, 2),
    /* 79 */ (AddressMode::Aby, Opcode::Adc, 4),
    /* 7a */ (AddressMode::Imp, Opcode::Nop, 2),
    /* 7b */ (AddressMode::Imp, Opcode::Xxx, 7),
    /* 7c */ (AddressMode::Abx, Opcode::Nop, 4),
    /* 7d */ (AddressMode::Abx, Opcode::Adc, 4),
    /* 7e */ (AddressMode::Abx, Opcode::Ror, 7),
    /* 7f */ (AddressMode::Imp, Opcode::Xxx, 7),
    /* 80 */ (AddressMode::Imp, Opcode::Nop, 2),
    /* 81 */ (AddressMode::Izx, Opcode::Sta, 6),
    /* 82 */ (AddressMode::Imp, Opcode::Nop, 2),
    /* 83 */ (AddressMode::Imp, Opcode::Xxx, 6),
    /* 84 */ (AddressMode::Zp0, Opcode::Sty, 3),
    /* 85 */ (AddressMode::Zp0, Opcode::Sta, 3),
    /* 86 */ (AddressMode::Zp0, Opcode::Stx, 3),
    /* 87 */ (AddressMode::Imp, Opcode::Xxx, 3),
    /* 88 */ (AddressMode::Imp, Opcode::Dey, 2),
    /* 89 */ (AddressMode::Imp, Opcode::Nop, 2),
    /* 8a */ (AddressMode::Imp, Opcode::Txa, 2),
    /* 8b */ (AddressMode::Imp, Opcode::Xxx, 2),
    /* 8c */ (AddressMode::Abs, Opcode::Sty, 4),
    /* 8d */ (AddressMode::Abs, Opcode::Sta, 4),
    /* 8e */ (AddressMode::Abs, Opcode::Stx, 4),
    /* 8f */ (AddressMode::Imp, Opcode::Xxx, 4),
    /* 90 */ (AddressMode::Rel, Opcode::Bcc, 2),
    /* 91 */ (AddressMode::Izy, Opcode::Sta, 6),
    /* 92 */ (AddressMode::Imp, Opcode::Xxx, 2),
    /* 93 */ (AddressMode::Imp, Opcode::Xxx, 6),
    /* 94 */ (AddressMode::Zpx, Opcode::Sty, 4),
    /* 95 */ (AddressMode::Zpx, Opcode::Sta, 4),
    /* 96 */ (AddressMode::Zpy, Opcode::Stx, 4),
    /* 97 */ (AddressMode::Imp, Opcode::Xxx, 4),
    /* 98 */ (AddressMode::Imp, Opcode::Tya, 2),
    /* 99 */ (AddressMode::Aby, Opcode::Sta, 5),
    /* 9a */ (AddressMode::Imp, Opcode::Txs, 2),
    /* 9b */ (AddressMode::Imp, Opcode::Xxx, 5),
    /* 9c */ (AddressMode::Imp, Opcode::Nop, 5),
    /* 9d */ (AddressMode::Abx, Opcode::Sta, 5),
    /* 9e */ (AddressMode::Imp, Opcode::Xxx, 5),
    /* 9f */ (AddressMode::Imp, Opcode::Xxx, 5),
    /* a0 */ (AddressMode::Imm, Opcode::Ldy, 2),
    /* a1 */ (AddressMode::Izx, Opcode::Lda, 6),
    /* a2 */ (AddressMode::Imm, Opcode::Ldx, 2),
    /* a3 */ (AddressMode::Imp, Opcode::Xxx, 6),
    /* a4 */ (AddressMode::Zp0, Opcode::Ldy, 3),
    /* a5 */ (AddressMode::Zp0, Opcode::Lda, 3),
    /* a6 */ (AddressMode::Zp0, Opcode::Ldx, 3),
    /* a7 */ (AddressMode::Imp, Opcode::Xxx, 3),
    /* a8 */ (AddressMode::Imp, Opcode::Tay, 2),
    /* a9 */ (AddressMode::Imm, Opcode::Lda, 2),
    /* aa */ (AddressMode::Imp, Opcode::Tax, 2),
    /* ab */ (AddressMode::Imp, Opcode::Xxx, 2),
    /* ac */ (AddressMode::Abs, Opcode::Ldy, 4),
    /* ad */ (AddressMode::Abs, Opcode::Lda, 4),
    /* ae */ (AddressMode::Abs, Opcode::Ldx, 4),
    /* af */ (AddressMode::Imp, Opcode::Xxx, 4),
    /* b0 */ (AddressMode::Rel, Opcode::Bcs, 2),
    /* b1 */ (AddressMode::Izy, Opcode::Lda, 5),
    /* b2 */ (AddressMode::Imp, Opcode::Xxx, 2),
    /* b3 */ (AddressMode::Imp, Opcode::Xxx, 5),
    /* b4 */ (AddressMode::Zpx, Opcode::Ldy, 4),
    /* b5 */ (AddressMode::Zpx, Opcode::Lda, 4),
    /* b6 */ (AddressMode::Zpy, Opcode::Ldx, 4),
    /* b7 */ (AddressMode::Imp, Opcode::Xxx, 4),
    /* b8 */ (AddressMode::Imp, Opcode::Clv, 2),
    /* b9 */ (AddressMode::Aby, Opcode::Lda, 4),
    /* ba */ (AddressMode::Imp, Opcode::Tsx, 2),
    /* bb */ (AddressMode::Imp, Opcode::Xxx, 4),
    /* bc */ (AddressMode::Abx, Opcode::Ldy, 4),
    /* bd */ (AddressMode::Abx, Opcode::Lda, 4),
    /* be */ (AddressMode::Aby, Opcode::Ldx, 4),
    /* bf */ (AddressMode::Imp, Opcode::Xxx, 4),
    /* c0 */ (AddressMode::Imm, Opcode::Cpy, 2),
    /* c1 */ (AddressMode::Izx, Opcode::Cmp, 6),
    /* c2 */ (AddressMode::Imp, Opcode::Nop, 2),
    /* c3 */ (AddressMode::Imp, Opcode::Xxx, 8),
    /* c4 */ (AddressMode::Zp0, Opcode::Cpy, 3),
    /* c5 */ (AddressMode::Zp0, Opcode::Cmp, 3),
    /* c6 */ (AddressMode::Zp0, Opcode::Dec, 5),
    /* c7 */ (AddressMode::Imp, Opcode::Xxx, 5),
    /* c8 */ (AddressMode::Imp, Opcode::Iny, 2),
    /* c9 */ (AddressMode::Imm, Opcode::Cmp, 2),
    /* ca */ (AddressMode::Imp, Opcode::Dex, 2),
    /* cb */ (AddressMode::Imp, Opcode::Xxx, 2),
    /* cc */ (AddressMode::Abs, Opcode::Cpy, 4),
    /* cd */ (AddressMode::Abs, Opcode::Cmp, 4),
    /* ce */ (AddressMode::Abs, Opcode::Dec, 6),
    /* cf */ (AddressMode::Imp, Opcode::Xxx, 6),
    /* d0 */ (AddressMode::Rel, Opcode::Bne, 2),
    /* d1 */ (AddressMode::Izy, Opcode::Cmp, 5),
    /* d2 */ (AddressMode::Imp, Opcode::Xxx, 2),
    /* d3 */ (AddressMode::Imp, Opcode::Xxx, 8),
    /* d4 */ (AddressMode::Imp, Opcode::Nop, 4),
    /* d5 */ (AddressMode::Zpx, Opcode::Cmp, 4),
    /* d6 */ (AddressMode::Zpx, Opcode::Dec, 6),
    /* d7 */ (AddressMode::Imp, Opcode::Xxx, 6),
    /* d8 */ (AddressMode::Imp, Opcode::Cld, 2),
    /* d9 */ (AddressMode::Aby, Opcode::Cmp, 4),
    /* da */ (AddressMode::Imp, Opcode::Nop, 2),
    /* db */ (AddressMode::Imp, Opcode::Xxx, 7),
    /* dc */ (AddressMode::Abx, Opcode::Nop, 4),
    /* dd */ (AddressMode::Abx, Opcode::Cmp, 4),
    /* de */ (AddressMode::Abx, Opcode::Dec, 7),
    /* df */ (AddressMode::Imp, Opcode::Xxx, 7),
    /* e0 */ (AddressMode::Imm, Opcode::Cpx, 2),
    /* e1 */ (AddressMode::Izx, Opcode::Sbc, 6),
    /* e2 */ (AddressMode::Imp, Opcode::Nop, 2),
    /* e3 */ (AddressMode::Imp, Opcode::Xxx, 8),
    /* e4 */ (AddressMode::Zp0, Opcode::Cpx, 3),
    /* e5 */ (AddressMode::Zp0, Opcode::Sbc, 3),
    /* e6 */ (AddressMode::Zp0, Opcode::Inc, 5),
    /* e7 */ (AddressMode::Imp, Opcode::Xxx, 5),
    /* e8 */ (AddressMode::Imp, Opcode::Inx, 2),
    /* e9 */ (AddressMode::Imm, Opcode::Sbc, 2),
    /* ea */ (AddressMode::Imp, Opcode::Nop, 2),
    /* eb */ (AddressMode::Imp, Opcode::Sbc, 2),
    /* ec */ (AddressMode::Abs, Opcode::Cpx, 4),
    /* ed */ (AddressMode::Abs, Opcode::Sbc, 4),
    /* ee */ (AddressMode::Abs, Opcode::Inc, 6),
    /* ef */ (AddressMode::Imp, Opcode::Xxx, 6),
    /* f0 */ (AddressMode::Rel, Opcode::Beq, 2),
    /* f1 */ (AddressMode::Izy, Opcode::Sbc, 5),
    /* f2 */ (AddressMode::Imp, Opcode::Xxx, 2),
    /* f3 */ (AddressMode::Imp, Opcode::Xxx, 8),
    /* f4 */ (AddressMode::Imp, Opcode::Nop, 4),
    /* f5 */ (AddressMode::Zpx, Opcode::Sbc, 4),
    /* f6 */ (AddressMode::Zpx, Opcode::Inc, 6),
    /* f7 */ (AddressMode::Imp, Opcode::Xxx, 6),
    /* f8 */ (AddressMode::Imp, Opcode::Sed, 2),
    /* f9 */ (AddressMode::Aby, Opcode::Sbc, 4),
    /* fa */ (AddressMode::Imp, Opcode::Nop, 2),
    /* fb */ (AddressMode::Imp, Opcode::Xxx, 7),
    /* fc */ (AddressMode::Abx, Opcode::Nop, 4),
    /* fd */ (AddressMode::Abx, Opcode::Sbc, 4),
    /* fe */ (AddressMode::Abx, Opcode::Inc, 7),
    /* ff */ (AddressMode::Imp, Opcode::Xxx, 7),
];
