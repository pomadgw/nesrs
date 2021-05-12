mod test;
use std::fmt;

/// This constant represents the address of the low byte of 6502's reset vector address
pub const INTERRUPT_RESET: u16 = 0xFFFC;

use crate::memory::Memory;

pub fn hello() {
    println!("Hello");
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

/// Emulating 6502 CPU
pub struct CPU {
    pub regs: CPURegisters,
}

impl CPU {
    /// Create new CPU instance
    pub fn new() -> Self {
        CPU {
            regs: Default::default(),
        }
    }

    /// Reset the CPU
    pub fn reset(&mut self, memory: &mut dyn Memory) {
        //
    }

    // Clock the CPU
    pub fn clock(&mut self, memory: &mut dyn Memory) {
        let pc = self.get_pc();
        println!("{}", memory.read(pc, false));
    }

    // BEGIN PRIVATE
    fn get_pc(&mut self) -> usize {
        let pc = self.regs.pc as usize;

        self.regs.pc = self.regs.pc.wrapping_add(1);

        pc
    }
    // END PRIVATE
}
