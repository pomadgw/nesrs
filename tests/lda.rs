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
}
