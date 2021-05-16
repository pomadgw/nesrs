mod test_utils;

#[macro_use]
#[path = "../src/macros.rs"]
mod macros;

#[cfg(test)]
mod cpu_addresing_modes_tests {
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
    fn it_can_fetch_abx() {
        println!("BEGIN TEST LDA");
        let mut cpu = CPU::new();

        let mut memory = RAM::new();
        set_reset!(memory, 0x8001);

        cpu.reset();

        loop_cpu!(cpu, memory);

        let prev_cycle = cpu.total_cycles;
        let offset = 0x10;

        memory.write(0x5455, 0xaa);
        memory.write(0x5455 + offset, 0x01);

        cpu.regs.x = offset as u8;

        // no page crossing case
        set_ram!(memory, 0x8001, [0xbd, 0x55, 0x54]);

        loop_cpu!(cpu, memory);

        assert_eq!(cpu.regs.a, 0x01);
        assert_eq!(cpu.total_cycles - prev_cycle, 4);

        // page crossing case

        cpu.reset();
        loop_cpu!(cpu, memory);

        let prev_cycle = cpu.total_cycles;

        memory.write(0x54f5, 0xaa);
        memory.write(0x54f5 + offset, 0x02);

        cpu.regs.x = offset as u8;

        // no page crossing case
        set_ram!(memory, 0x8001, [0xbd, 0xf5, 0x54]);

        loop_cpu!(cpu, memory);

        assert_eq!(cpu.regs.a, 0x02);
        assert_eq!(cpu.total_cycles - prev_cycle, 5);
    }

    #[test]
    fn it_can_fetch_aby() {
        println!("BEGIN TEST LDA");
        let mut cpu = CPU::new();

        let mut memory = RAM::new();
        set_reset!(memory, 0x8001);

        cpu.reset();

        loop_cpu!(cpu, memory);

        let prev_cycle = cpu.total_cycles;
        let offset = 0x10;

        memory.write(0x5455, 0xaa);
        memory.write(0x5455 + offset, 0x01);

        cpu.regs.y = offset as u8;

        // no page crossing case
        set_ram!(memory, 0x8001, [0xb9, 0x55, 0x54]);

        loop_cpu!(cpu, memory);

        assert_eq!(cpu.regs.a, 0x01);
        assert_eq!(cpu.total_cycles - prev_cycle, 4);

        // page crossing case

        cpu.reset();
        loop_cpu!(cpu, memory);

        let prev_cycle = cpu.total_cycles;

        memory.write(0x54f5, 0xaa);
        memory.write(0x54f5 + offset, 0x02);

        cpu.regs.x = offset as u8;

        // no page crossing case
        set_ram!(memory, 0x8001, [0xb9, 0xf5, 0x54]);

        loop_cpu!(cpu, memory);

        assert_eq!(cpu.regs.a, 0x02);
        assert_eq!(cpu.total_cycles - prev_cycle, 5);
    }

    #[test]
    fn it_can_fetch_zp0() {
        println!("BEGIN TEST LDA");
        let mut cpu = CPU::new();

        let mut memory = RAM::new();
        set_reset!(memory, 0x8001);

        cpu.reset();

        loop_cpu!(cpu, memory);

        let prev_cycle = cpu.total_cycles;

        memory.write(0x55, 0xaa);

        set_ram!(memory, 0x8001, [0xa5, 0x55]);

        loop_cpu!(cpu, memory);

        assert_eq!(cpu.regs.a, 0xaa);
        assert_eq!(cpu.total_cycles - prev_cycle, 3);
    }

    #[test]
    fn it_can_fetch_zpx() {
        println!("BEGIN TEST LDA");
        let mut cpu = CPU::new();
        let offset = 0x10;

        let mut memory = RAM::new();
        set_reset!(memory, 0x8001);

        cpu.reset();

        loop_cpu!(cpu, memory);
        cpu.regs.x = offset as u8;

        let prev_cycle = cpu.total_cycles;

        memory.write(0x55 + offset, 0xaa);

        set_ram!(memory, 0x8001, [0xb5, 0x55]);

        loop_cpu!(cpu, memory);

        assert_eq!(cpu.regs.a, 0xaa);
        assert_eq!(cpu.total_cycles - prev_cycle, 4);
    }

    #[test]
    fn it_can_fetch_zpy() {
        println!("BEGIN TEST LDx");
        let mut cpu = CPU::new();
        let offset = 0x10;

        let mut memory = RAM::new();
        set_reset!(memory, 0x8001);

        cpu.reset();

        loop_cpu!(cpu, memory);
        cpu.regs.y = offset as u8;

        let prev_cycle = cpu.total_cycles;

        memory.write(0x55 + offset, 0xaa);

        set_ram!(memory, 0x8001, [0xb6, 0x55]);

        loop_cpu!(cpu, memory);

        assert_eq!(cpu.regs.x, 0xaa);
        assert_eq!(cpu.total_cycles - prev_cycle, 4);
    }
}
