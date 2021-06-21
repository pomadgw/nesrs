use nesrs;
use nesrs::cpu::*;
use nesrs::memory::*;
// use std::time::Instant;

use std::fs::File;
use std::io::prelude::*;

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

fn main() -> std::io::Result<()> {
    let mut cpu = CPU::new();

    let mut file = File::open("./6502_test.rom")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut memory = RAM { a: buffer };

    cpu.debug = true;
    // cpu.reset();
    // loop_cpu!(cpu, memory);

    cpu.regs.pc = 0x0400;
    println!(
        "${:04X}: A: ${:02X} X: ${:02X} Y: ${:02X}",
        cpu.regs.pc, cpu.regs.a, cpu.regs.x, cpu.regs.y
    );

    for _i in 0..5 {
        loop_cpu!(cpu, memory);
        cpu.print_debug();
    }

    // let now = Instant::now();

    loop {
        // let prev = now.elapsed().as_nanos();
        loop_cpu!(cpu, memory);
        cpu.print_debug();
        // let current = now.elapsed().as_nanos();
        // println!("time: {} ns", current - prev);

        let status = memory.read(0x8000, true);

        if status != 0 {
            // success
            if status == 0x80 {
                println!("DONE");
                break;
            }
            // not success
            if status != 0x80 {
                println!("ERROR: {:02X}", status);
                break;
            }
        }
    }

    println!(
        "${:04X}: A: ${:02X} X: ${:02X} Y: ${:02X}",
        cpu.regs.pc, cpu.regs.a, cpu.regs.x, cpu.regs.y
    );

    Ok(())
}
