mod lib;

use lib::DummyBus;
use nesrs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inc_abs() {
        let mut bus = DummyBus::new();
        let mut cpu = nesrs::cpu::CPU::new();
        bus.ram[0x0000] = 0xee;
        bus.ram[0x0001] = 0x00;
        bus.ram[0x0002] = 0x01;
        bus.ram[0x0100] = 0x10;

        loop {
            cpu.clock(&mut bus);

            if cpu.is_clocking_done() {
                break;
            }
        }

        assert_eq!(bus.ram[0x0100], 0x11);
        assert_eq!(cpu.cycles, 6);
    }

    #[test]
    fn inc_set_z() {
        let mut bus = DummyBus::new();
        let mut cpu = nesrs::cpu::CPU::new();
        bus.ram[0x0000] = 0xee;
        bus.ram[0x0001] = 0x00;
        bus.ram[0x0002] = 0x01;
        bus.ram[0x0100] = 0xff;

        loop {
            cpu.clock(&mut bus);

            if cpu.is_clocking_done() {
                break;
            }
        }

        assert_eq!(bus.ram[0x0100], 0x00);
        assert_eq!(cpu.get_status(nesrs::cpu::CPUStatus::Z), true);
    }

    #[test]
    fn inc_set_n() {
        let mut bus = DummyBus::new();
        let mut cpu = nesrs::cpu::CPU::new();
        bus.ram[0x0000] = 0xee;
        bus.ram[0x0001] = 0x00;
        bus.ram[0x0002] = 0x01;
        bus.ram[0x0100] = 0x7f;

        loop {
            cpu.clock(&mut bus);

            if cpu.is_clocking_done() {
                break;
            }
        }

        assert_eq!(bus.ram[0x0100], 0x80);
        assert_eq!(cpu.get_status(nesrs::cpu::CPUStatus::N), true);
    }

    #[test]
    fn inc_abx() {
        let mut bus = DummyBus::new();
        let mut cpu = nesrs::cpu::CPU::new();
        bus.ram[0x0000] = 0xfe;
        bus.ram[0x0001] = 0x00;
        bus.ram[0x0002] = 0x01;
        bus.ram[0x0101] = 0x10;
        cpu.x = 0x01;

        loop {
            cpu.clock(&mut bus);

            if cpu.is_clocking_done() {
                break;
            }
        }

        assert_eq!(bus.ram[0x0101], 0x11);
        assert_eq!(cpu.cycles, 7);
    }
}
