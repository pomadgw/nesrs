#[cfg(test)]
mod cpu_instr_tests_rti {
    use nesrs::bus::*;
    use nesrs::memory::*;

    #[test]
    fn rti_test() {
        let bytes = include_bytes!("./instr_test/14-rti.nes");
        let buffer = bytes.to_vec();
        let mut bus = Bus::new_from_array(&buffer).unwrap();

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
