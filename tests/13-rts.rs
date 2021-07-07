#[cfg(test)]
mod cpu_instr_tests_rts {
    use nesrs::bus::*;
    use nesrs::cartridge::*;
    use nesrs::memory::*;

    #[test]
    fn rts_test() {
        let bytes = include_bytes!("./instr_test/13-rts.nes");
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
