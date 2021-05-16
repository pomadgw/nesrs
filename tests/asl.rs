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
    fn it_can_fetch_acc() {
        println!("BEGIN TEST ASL");
        let mut cpu = CPU::new();

        let mut memory = RAM::new();
        set_reset!(memory, 0x8001);

        cpu.reset();

        loop_cpu!(cpu, memory);

        let prev_cycle = cpu.total_cycles;

        cpu.regs.a = 0x55;
        let expected = ((0x55 << 1) & 0xff) as u8;

        set_ram!(memory, 0x8001, [0x0a]);

        loop_cpu!(cpu, memory);

        assert_eq!(cpu.regs.a, expected);
        assert_eq!(cpu.total_cycles - prev_cycle, 2);
    }

    #[test]
    fn it_can_fetch_zp0() {
        println!("BEGIN TEST ASL");
        let mut cpu = CPU::new();

        let mut memory = RAM::new();
        set_reset!(memory, 0x8001);

        cpu.reset();

        loop_cpu!(cpu, memory);

        let prev_cycle = cpu.total_cycles;

        let loc = 0x0000;
        let value = 0x55;
        memory.write(0x0000, value);
        let expected = ((0x55 << 1) & 0xff) as u8;

        set_ram!(memory, 0x8001, [0x06, 0x00]);

        loop_cpu!(cpu, memory);

        assert_eq!(memory.ram[loc], expected);
        assert_eq!(cpu.total_cycles - prev_cycle, 5);
    }

    #[test]
    fn it_can_fetch_abs() {
        println!("BEGIN TEST ASL");
        let mut cpu = CPU::new();

        let mut memory = RAM::new();
        set_reset!(memory, 0x8001);

        cpu.reset();

        loop_cpu!(cpu, memory);

        let prev_cycle = cpu.total_cycles;
        let loc = 0x5000;

        let value = 0x55;
        memory.write(loc, value);
        let expected = ((0x55 << 1) & 0xff) as u8;

        set_ram!(memory, 0x8001, [0x0e, 0x00, 0x50]);

        loop_cpu!(cpu, memory);

        assert_eq!(memory.ram[loc], expected);
        assert_eq!(cpu.total_cycles - prev_cycle, 6);
    }

    #[test]
    fn it_can_fetch_abx() {
        println!("BEGIN TEST ASL");
        let mut cpu = CPU::new();
        let offset = 0x10;

        let mut memory = RAM::new();
        set_reset!(memory, 0x8001);

        cpu.reset();

        loop_cpu!(cpu, memory);

        let prev_cycle = cpu.total_cycles;
        let loc = 0x5000 + offset;
        cpu.regs.x = offset as u8;

        let value = 0x55;
        memory.write(loc, value);
        let expected = ((0x55 << 1) & 0xff) as u8;

        set_ram!(memory, 0x8001, [0x1e, 0x00, 0x50]);

        loop_cpu!(cpu, memory);

        assert_eq!(memory.ram[loc], expected);
        assert_eq!(cpu.total_cycles - prev_cycle, 7);
    }

    #[test]
    fn it_can_set_flags() {
        println!("BEGIN TEST ASL");
        let mut cpu = CPU::new();

        let mut memory = RAM::new();
        set_reset!(memory, 0x8001);

        cpu.reset();

        loop_cpu!(cpu, memory);

        let prev_cycle = cpu.total_cycles;

        cpu.regs.a = 0x00;
        let expected = ((0x00 << 1) & 0xff) as u8;

        set_ram!(memory, 0x8001, [0x0a]);

        loop_cpu!(cpu, memory);

        assert_eq!(cpu.regs.a, 0x00);
        assert_eq!(cpu.regs.p.contains(nesrs::cpu::types::StatusFlag::Z), true);
        assert_eq!(cpu.regs.p.contains(nesrs::cpu::types::StatusFlag::C), false);

        cpu.reset();
        loop_cpu!(cpu, memory);
        cpu.regs.a = 0x80;
        loop_cpu!(cpu, memory);
        assert_eq!(cpu.regs.a, 0x00);
        assert_eq!(cpu.regs.p.contains(nesrs::cpu::types::StatusFlag::C), true);

        cpu.reset();
        loop_cpu!(cpu, memory);
        cpu.regs.a = 0x7f;
        let expected = ((0x7f << 1) & 0xff) as u8;
        loop_cpu!(cpu, memory);
        assert_eq!(cpu.regs.a, expected);
        assert_eq!(cpu.regs.p.contains(nesrs::cpu::types::StatusFlag::N), true);
    }
}
