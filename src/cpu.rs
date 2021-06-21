mod clock;
mod opcodes;
pub mod types;

use crate::cpu::types::*;

/// This constant represents the address of the low byte of 6502's reset vector address
pub const INTERRUPT_RESET: u16 = 0xFFFC;
pub const INTERRUPT_NMI: u16 = 0xFFFA;
pub const INTERRUPT_IRQ: u16 = 0xFFFE;

use crate::memory::Memory;

/// Emulating 6502 CPU
pub struct CPU {
    pub regs: CPURegisters,
    pub total_cycles: u32,
    cycles: u32,
    opcode: u8,
    pub interrupt_type: Interrupt,
    address_mode: AddressMode,
    opcode_type: Opcode,
    is_read: bool,
    address: Int16,
    tmp_address: Int16,
    temp: u8,
    state: Microcode,
    absolute_address: usize,
    relative_address: i8,
    fetched_data: u8,
    register_access: RegisterAccess,
    branch_status_to_test: StatusFlag,
    branch_when: bool,

    pub irq_pin: Pin,
    pub nmi_pin: Pin,
    stop_irq_pin: Pin,

    // for debug
    instruction_debug: Vec<u8>,
    prev_pc: u16,
    prev_cycles: u32,
    formatted_params: String,
    pub debug: bool,
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
            address_mode: AddressMode::Imp,
            opcode_type: Opcode::Brk,
            is_read: true,
            temp: 0,
            state: Microcode::FetchOpcode,
            absolute_address: 0,
            relative_address: 0,
            fetched_data: 0,
            address: Int16::new_from_16(0),
            tmp_address: Int16::new_from_16(0),
            register_access: RegisterAccess::None,
            irq_pin: Pin::default(),
            nmi_pin: Pin::default(),
            stop_irq_pin: Pin::default(),
            branch_status_to_test: StatusFlag::empty(),
            branch_when: false,

            instruction_debug: Vec::new(),
            prev_pc: 0,
            prev_cycles: 0,
            formatted_params: String::new(),
            debug: false,
        }
    }

    /// Reset the CPU
    pub fn reset(&mut self) {
        self.interrupt_type |= Interrupt::RESET; // Set interrupt type to reset
        self.next_state(Microcode::FetchOpcode);
    }

    pub fn irq(&mut self) {
        if !self.regs.p.contains(StatusFlag::I) || !self.stop_irq_pin.is_pulled() {
            self.interrupt_type |= Interrupt::IRQ;
        }
    }

    pub fn nmi(&mut self) {
        self.interrupt_type |= Interrupt::NMI;
    }

    pub fn done(&self) -> bool {
        match self.state {
            Microcode::FetchOpcode => true,
            _ => false,
        }
    }

    pub fn is_read(&self) -> bool {
        self.is_read
    }

    fn see_prev_pc(&self) -> String {
        format!("{:04X}", self.prev_pc)
    }

    fn get_registry_status(&self) -> String {
        format!(
            "A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
            self.regs.a, self.regs.x, self.regs.y, self.regs.p, self.regs.sp
        )
    }

    fn see_prev_instruction_bytes(&self) -> String {
        match self.instruction_debug.len() {
            1 => format!("{:02X}", self.instruction_debug[0]),
            2 => format!(
                "{:02X} {:02X}",
                self.instruction_debug[0], self.instruction_debug[1]
            ),
            3 => format!(
                "{:02X} {:02X} {:02X}",
                self.instruction_debug[0], self.instruction_debug[1], self.instruction_debug[2]
            ),
            _ => format!(""),
        }
    }

    fn see_prev_instruction(&self) -> String {
        format!(
            "{} {}",
            self.opcode_type.to_string().to_uppercase(),
            self.formatted_params
        )
    }

    pub fn print_debug(&self) {
        if self.debug {
            println!(
                "{}  {:9} {:31} {} CYC:{:<5}",
                self.see_prev_pc(),
                self.see_prev_instruction_bytes(),
                self.see_prev_instruction(),
                self.get_registry_status(),
                self.prev_cycles
            );
        }
    }

    // BEGIN PRIVATE
    fn read(&mut self, memory: &mut dyn Memory, address: usize) -> u8 {
        self.is_read = true;
        self.advance_cycle();
        memory.read(address, false)
    }

    fn write(&mut self, memory: &mut dyn Memory, address: usize, value: u8) {
        self.is_read = false;
        self.advance_cycle();
        memory.write(address, value);
    }

    fn push_stack(&mut self, memory: &mut dyn Memory, value: u8) {
        let address = 0x0100 + self.regs.sp as usize;
        self.write(memory, address, value);
        self.regs.sp = self.regs.sp.wrapping_sub(1);
    }

    fn pop_stack(&mut self, memory: &mut dyn Memory) -> u8 {
        self.regs.sp = self.regs.sp.wrapping_add(1);
        let address = 0x0100 + self.regs.sp as usize;
        self.read(memory, address)
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

    fn next_state(&mut self, state: Microcode) {
        self.cycles = 0;
        self.state = state;
    }

    fn advance_cycle(&mut self) {
        self.cycles += 1;
    }

    fn fetch_opcode(&mut self) {
        self.next_state(Microcode::FetchOpcode);
    }

    fn is_write_instruction(&self) -> bool {
        match self.opcode_type {
            Opcode::Asl | Opcode::Sta | Opcode::Stx | Opcode::Sty => true,
            _ => false,
        }
    }

    fn vector_address(&self) -> usize {
        if self.interrupt_type.contains(Interrupt::RESET) {
            INTERRUPT_RESET as usize
        } else if self.interrupt_type.contains(Interrupt::NMI) {
            INTERRUPT_NMI as usize
        } else {
            INTERRUPT_IRQ as usize
        }
    }

    fn set_nz(&mut self, value: u8) {
        self.regs.p.set(StatusFlag::Z, value == 0);
        self.regs.p.set(StatusFlag::N, (value & 0x80) > 0);
    }
    // END PRIVATE
}
