#[macro_use]
mod lib;

use lib::DummyBus;
use nesrs;

#[cfg(test)]
mod tests_ldx {
    use super::*;

    #[test]
    fn ldx() {
        let mut bus = DummyBus::new();
        let mut cpu = nesrs::cpu::CPU::new();
        bus.ram[0x0000] = 0xa2;
        bus.ram[0x0001] = 0xff;

        loop {
            cpu.clock(&mut bus);

            if cpu.is_clocking_done() {
                break;
            }
        }

        assert_eq!(cpu.x, 0xff);
        assert_eq!(cpu.cycles, 2);
    }

    #[test]
    fn ldx_set_z() {
        let mut bus = DummyBus::new();
        let mut cpu = nesrs::cpu::CPU::new();
        bus.ram[0x0000] = 0xae;
        bus.ram[0x0001] = 0x00;
        bus.ram[0x0002] = 0x01;
        bus.ram[0x0100] = 0x00;

        loop {
            cpu.clock(&mut bus);

            if cpu.is_clocking_done() {
                break;
            }
        }

        assert_eq!(cpu.x, 0x00);
        assert_eq!(cpu.get_status(nesrs::cpu::CPUStatus::Z), true);
    }

    #[test]
    fn ldx_set_n() {
        let mut bus = DummyBus::new();
        let mut cpu = nesrs::cpu::CPU::new();
        bus.ram[0x0000] = 0xae;
        bus.ram[0x0001] = 0x00;
        bus.ram[0x0002] = 0x01;
        bus.ram[0x0100] = 0x81;

        loop {
            cpu.clock(&mut bus);

            if cpu.is_clocking_done() {
                break;
            }
        }

        assert_eq!(cpu.x, 0x81);
        assert_eq!(cpu.get_status(nesrs::cpu::CPUStatus::N), true);
    }
}
