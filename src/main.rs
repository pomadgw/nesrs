use nesrs;
use nesrs::cpu::*;
use nesrs::memory::*;

#[macro_use]
mod macros;

struct RAM {
    a: Vec<u8>,
}

impl Memory for RAM {
    fn read(&self, address: usize, _is_read_only: bool) -> u8 {
        self.a[address]
    }

    fn write(&mut self, address: usize, value: u8) {
        self.a[address] = value;
    }
}

fn main() {
    let mut cpu = nesrs::cpu::CPU::new();

    let mut memory = RAM {
        a: vec![0; 0x10000],
    };

    set_reset!(memory, 0x8000);
    cpu.reset();
    loop_cpu!(cpu, memory);

    set_ram!(memory, 0x8000, [0xa9, 0x10, 0xaa, 0xa8]);
    loop_cpu!(cpu, memory);
    loop_cpu!(cpu, memory);
    loop_cpu!(cpu, memory);

    println!(
        "${:04X}: A: ${:02X} X: ${:02X} Y: ${:02X}",
        cpu.regs.pc, cpu.regs.a, cpu.regs.x, cpu.regs.y
    );
}
