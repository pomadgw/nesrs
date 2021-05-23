mod test_utils;

#[macro_use]
#[path = "../src/macros.rs"]
mod macros;

#[cfg(test)]
mod cpu_tests {
    use crate::test_utils::RAM;
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
}
