#[cfg(test)]
mod cpu_lda_tests {
    use nesrs::*;

    struct RAM {
        a: Vec<u8>,
    }

    impl memory::Memory for RAM {
        fn read(&self, address: usize, _is_read_only: bool) -> u8 {
            self.a[address]
        }

        fn write(&mut self, address: usize, value: u8) {
            self.a[address] = value;
        }
    }

    #[test]
    fn it_can_fetch_imm() {
        println!("BEGIN TEST LDA");
        let mut cpu = nesrs::cpu::CPU::new();

        let mut memory = RAM {
            a: vec![0; 0x10000],
        };

        memory.a[cpu::INTERRUPT_RESET as usize] = 0x01;
        memory.a[(cpu::INTERRUPT_RESET + 1) as usize] = 0x80;

        cpu.reset();

        cpu.clock(&mut memory);
        while !cpu.done() {
            cpu.clock(&mut memory);
        }

        let prev_cycle = cpu.total_cycles;

        memory.a[0x8001] = 0xa9;
        memory.a[0x8002] = 0x55;

        cpu.clock(&mut memory);
        while !cpu.done() {
            cpu.clock(&mut memory);
        }

        assert_eq!(cpu.regs.a, 0x55);
        assert_eq!(cpu.total_cycles - prev_cycle, 2);
    }

    #[test]
    fn it_can_set_flags() {
        println!("BEGIN TEST LDA");
        let mut cpu = nesrs::cpu::CPU::new();

        let mut memory = RAM {
            a: vec![0; 0x10000],
        };

        memory.a[cpu::INTERRUPT_RESET as usize] = 0x01;
        memory.a[(cpu::INTERRUPT_RESET + 1) as usize] = 0x80;

        cpu.reset();

        cpu.clock(&mut memory);
        while !cpu.done() {
            cpu.clock(&mut memory);
        }

        memory.a[0x8001] = 0xa9;
        memory.a[0x8002] = 0x00;

        cpu.clock(&mut memory);
        while !cpu.done() {
            cpu.clock(&mut memory);
        }

        assert_eq!(cpu.regs.a, 0x00);
        assert_eq!(cpu.regs.p.contains(nesrs::cpu::types::StatusFlag::Z), true);
        cpu.reset();

        cpu.clock(&mut memory);
        while !cpu.done() {
            cpu.clock(&mut memory);
        }

        memory.a[0x8002] = 0x88;

        cpu.clock(&mut memory);
        while !cpu.done() {
            cpu.clock(&mut memory);
        }
        assert_eq!(cpu.regs.a, 0x88);
        assert_eq!(cpu.regs.p.contains(nesrs::cpu::types::StatusFlag::N), true);
    }
}
