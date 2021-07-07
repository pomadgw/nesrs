#[cfg(test)]
mod cpu_instr_tests_stack {
    use nesrs::bus::*;
    use nesrs::cartridge::*;
    use nesrs::memory::*;

    #[test]
    fn stack_test() {
        let bytes = include_bytes!("./instr_test/11-stack.nes");
        let buffer = bytes.to_vec();
        let cartridge = Cartridge::parse(&buffer);

        let mut bus = Bus::new(cartridge);

        // bus.cpu.debug = true;
        bus.reset();

        loop {
            bus.clock();

            if bus.memory().read(0x6000, true) == 0x80 {
                break;
            }
        }

        loop {
            bus.clock();

            if bus.memory().read(0x6000, true) != 0x80 {
                break;
            }
        }

        assert_eq!(bus.memory().read(0x6000, true), 0x00);
    }
}
