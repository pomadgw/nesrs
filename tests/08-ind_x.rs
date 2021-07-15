#[cfg(test)]
mod cpu_instr_tests_ind_x {
    use nesrs::bus::*;
    use nesrs::memory::*;

    #[test]
    fn ind_x_test() {
        let bytes = include_bytes!("./instr_test/08-ind_x.nes");
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
