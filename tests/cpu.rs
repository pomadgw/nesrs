mod test_utils;

#[macro_use]
#[path = "../src/macros.rs"]
mod macros;

#[cfg(test)]
mod cpu_tests {
    use crate::test_utils::RAM;
    use nesrs::cpu::types::*;
    use nesrs::cpu::*;
    use nesrs::memory::*;

    #[test]
    fn it_can_reset() {
        let mut cpu = CPU::new();

        let mut memory = RAM::new();
        set_reset!(memory, 0x8001);

        cpu.reset();

        loop_cpu!(cpu, memory);

        assert_eq!(cpu.total_cycles, 7);

        assert_eq!(cpu.regs.pc, 0x8001);
        assert_eq!(cpu.regs.sp, 0xfd);
        assert_eq!(cpu.regs.p.bits(), 0x24);
    }

    #[test]
    fn it_can_interrupt_and_rti() {
        let mut cpu = CPU::new();
        cpu.debug = true;

        let mut memory = RAM::new();
        set_reset!(memory, 0x8001);

        memory.write(0xfffe, 0x00);
        memory.write(0xffff, 0x60);

        memory.write(0xfffa, 0x00);
        memory.write(0xfffb, 0xc0);

        memory.write(0x6000, 0x40);

        cpu.reset();

        loop_cpu!(cpu, memory);
        cpu.print_debug();

        let prev_cycle = cpu.total_cycles;

        cpu.regs.p |= StatusFlag::I;
        cpu.irq();

        loop_cpu!(cpu, memory);
        cpu.print_debug();

        assert_ne!(cpu.regs.pc, 0x6000);
        assert_ne!(cpu.total_cycles - prev_cycle, 7);

        let prev_cycle = cpu.total_cycles;
        cpu.regs.p &= !StatusFlag::I;
        cpu.irq();

        let pc = cpu.regs.pc;
        loop_cpu!(cpu, memory);
        cpu.print_debug();

        assert_eq!(cpu.regs.pc, 0x6000);
        assert_eq!(cpu.total_cycles - prev_cycle, 7);

        let prev_cycle = cpu.total_cycles;
        loop_cpu!(cpu, memory);
        cpu.print_debug();

        assert_eq!(cpu.regs.pc, pc);
        assert_eq!(cpu.total_cycles - prev_cycle, 6);
    }

    #[test]
    fn it_can_interrupt_nmi_and_rti() {
        let mut cpu = CPU::new();
        cpu.debug = true;

        let mut memory = RAM::new();
        set_reset!(memory, 0x8001);

        memory.write(0xfffe, 0x00);
        memory.write(0xffff, 0x60);

        memory.write(0xfffa, 0x00);
        memory.write(0xfffb, 0xc0);

        memory.write(0xc000, 0x40);
        memory.write(0xc001, 0x40);

        cpu.reset();

        loop_cpu!(cpu, memory);
        cpu.print_debug();

        let prev_cycle = cpu.total_cycles;

        cpu.regs.p |= StatusFlag::I;
        cpu.nmi();

        let pc = cpu.regs.pc;
        loop_cpu!(cpu, memory);
        cpu.print_debug();

        assert_eq!(cpu.regs.pc, 0xc000);
        assert_eq!(cpu.total_cycles - prev_cycle, 7);

        let prev_cycle = cpu.total_cycles;
        loop_cpu!(cpu, memory);
        cpu.print_debug();

        assert_eq!(cpu.regs.pc, pc);
        assert_eq!(cpu.total_cycles - prev_cycle, 6);
    }
}
