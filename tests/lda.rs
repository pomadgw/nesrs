mod test_utils;

#[macro_use]
#[path = "../src/macros.rs"]
mod macros;

#[cfg(test)]
mod cpu_lda_tests {
    use crate::test_utils::*;
    use nesrs::cpu::*;
    use nesrs::memory::*;

    #[test]
    fn it_can_fetch_imm() {
        println!("BEGIN TEST LDA");
        let mut cpu = CPU::new();

        let mut memory = RAM::new();
        set_reset!(memory, 0x8001);

        cpu.reset();

        loop_cpu!(cpu, memory);

        let prev_cycle = cpu.total_cycles;

        set_ram!(memory, 0x8001, [0xa9, 0x55]);

        loop_cpu!(cpu, memory);

        assert_eq!(cpu.regs.a, 0x55);
        assert_eq!(cpu.total_cycles - prev_cycle, 2);
    }

    #[test]
    fn it_can_fetch_abs() {
        println!("BEGIN TEST LDA");
        let mut cpu = CPU::new();

        let mut memory = RAM::new();
        set_reset!(memory, 0x8001);

        cpu.reset();

        loop_cpu!(cpu, memory);

        let prev_cycle = cpu.total_cycles;

        memory.write(0x5455, 0xaa);

        set_ram!(memory, 0x8001, [0xad, 0x55, 0x54]);

        loop_cpu!(cpu, memory);

        assert_eq!(cpu.regs.a, 0xaa);
        assert_eq!(cpu.total_cycles - prev_cycle, 4);
    }

    #[test]
    fn it_can_set_flags() {
        println!("BEGIN TEST LDA");
        let mut cpu = CPU::new();

        let mut memory = RAM::new();
        set_reset!(memory, 0x8001);

        cpu.reset();

        loop_cpu!(cpu, memory);

        set_ram!(memory, 0x8001, [0xa9, 0x00]);

        loop_cpu!(cpu, memory);

        assert_eq!(cpu.regs.a, 0x00);
        assert_eq!(cpu.regs.p.contains(nesrs::cpu::types::StatusFlag::Z), true);
        cpu.reset();

        loop_cpu!(cpu, memory);

        set_ram!(memory, 0x8001, [0xa9, 0x88]);

        loop_cpu!(cpu, memory);

        assert_eq!(cpu.regs.a, 0x88);
        assert_eq!(cpu.regs.p.contains(nesrs::cpu::types::StatusFlag::N), true);
    }
}
