mod clock;
pub mod types;

use crate::cpu::types::*;

/// This constant represents the address of the low byte of 6502's reset vector address
pub const INTERRUPT_RESET: u16 = 0xFFFC;
pub const INTERRUPT_NMI: u16 = 0xFFFA;
pub const INTERRUPT_IRQ: u16 = 0xFFFE;

use crate::memory::Memory;

pub fn hello() {
    println!("Hello");
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
    is_read: bool,
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
            is_read: true,
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

    // BEGIN PRIVATE
    fn read(&mut self, memory: &mut dyn Memory, address: usize) -> u8 {
        self.is_read = true;
        memory.read(address, false)
    }

    fn write(&mut self, memory: &mut dyn Memory, address: usize, value: u8) {
        self.is_read = false;
        memory.write(address, value);
    }

    fn push_stack(&mut self, memory: &mut dyn Memory, value: u8) {
        let address = 0x0100 + self.regs.sp as usize;
        memory.write(address, value);
        self.regs.sp = self.regs.sp.wrapping_sub(1);
    }

    fn set_instruction(&mut self) {
        let opcode_num = self.opcode as usize;
        match &OPCODE_TABLE[opcode_num] {
            (address_mode, opcode, cycle) => {
                self.address_mode = *address_mode;
                self.opcode_type = *opcode;
                self.cycles = *cycle;
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
