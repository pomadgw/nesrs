mod lib;

use lib::DummyBus;
use nesrs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lda_imm() {
        let mut bus = DummyBus::new();
        let mut cpu = nesrs::cpu::CPU::new();
        bus.ram[0x0000] = 0xa9;
        bus.ram[0x0001] = 0xff;

        loop {
            cpu.clock(&mut bus);

            if cpu.is_clocking_done() {
                break;
            }
        }

        assert_eq!(cpu.a, 0xff);
        assert_eq!(cpu.cycles, 2);
    }

    #[test]
    fn lda_set_z() {
        let mut bus = DummyBus::new();
        let mut cpu = nesrs::cpu::CPU::new();
        bus.ram[0x0000] = 0xad;
        bus.ram[0x0001] = 0x00;
        bus.ram[0x0002] = 0x01;
        bus.ram[0x0100] = 0x00;

        loop {
            cpu.clock(&mut bus);

            if cpu.is_clocking_done() {
                break;
            }
        }

        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.get_status(nesrs::cpu::CPUStatus::Z), true);
    }

    #[test]
    fn lda_set_n() {
        let mut bus = DummyBus::new();
        let mut cpu = nesrs::cpu::CPU::new();
        bus.ram[0x0000] = 0xad;
        bus.ram[0x0001] = 0x00;
        bus.ram[0x0002] = 0x01;
        bus.ram[0x0100] = 0x81;

        loop {
            cpu.clock(&mut bus);

            if cpu.is_clocking_done() {
                break;
            }
        }

        assert_eq!(cpu.a, 0x81);
        assert_eq!(cpu.get_status(nesrs::cpu::CPUStatus::N), true);
    }
}
