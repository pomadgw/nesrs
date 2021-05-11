#[cfg(test)]
mod cpu_tests {
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
    fn it_can_reset() {
        let mut cpu = nesrs::cpu::CPU::new();

        let mut memory = RAM {
            a: vec![0; 0x10000],
        };

        memory.a[cpu::INTERRUPT_RESET as usize] = 0x01;
        memory.a[(cpu::INTERRUPT_RESET + 1) as usize] = 0x80;

        cpu.reset(&mut memory);

        assert_eq!(cpu.regs.pc, 0x8001);
    }
}
