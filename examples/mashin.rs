use nesrs;
use nesrs::cpu::*;
use nesrs::memory::*;

use std::fs::File;
use std::io::prelude::*;

#[macro_use]
#[path = "../src/macros.rs"]
mod macros;

struct RAM {
    a: Vec<u8>,
}

impl Memory for RAM {
    fn read(&mut self, address: usize, _is_read_only: bool) -> u8 {
        self.a[address]
    }

    fn write(&mut self, address: usize, value: u8) {
        self.a[address] = value;
    }
}

fn main() -> std::io::Result<()> {
    let mut cpu = CPU::new();
    let mut file = File::open("./rom/test/test.rom")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut memory = RAM { a: buffer };

    cpu.debug = true;
    cpu.reset();
    loop_cpu!(cpu, memory);

    for _i in 0..300 {
        loop_cpu!(cpu, memory);
        cpu.print_debug();
    }

    Ok(())
}
