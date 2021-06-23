use nesrs;
use nesrs::cartridge::*;
use nesrs::cpu::*;
use nesrs::memory::*;

use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;
use std::io::SeekFrom;
use std::mem;
use std::slice;
use std::time::Instant;

#[macro_use]
mod macros;

struct NesMapper {
    ram: Vec<u8>,
    cartridge: Cartridge,
}

impl Memory for NesMapper {
    fn read(&mut self, address: usize, is_read_only: bool) -> u8 {
        let data = self.cartridge.read(address, is_read_only);
        if self.cartridge.use_cartridge_data() {
            return data;
        } else {
            self.ram[address]
        }
    }

    fn write(&mut self, address: usize, value: u8) {
        if address < 0x8000 {
            self.ram[address] = value;
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut cpu = CPU::new();

    let mut file = File::open("./rom/nestest.nes")?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    let mut memory = NesMapper {
        ram: vec![0; 0x2000],
        cartridge: Cartridge::parse(&buffer),
    };

    cpu.debug = true;
    cpu.reset();
    loop_cpu!(cpu, memory);

    cpu.regs.pc = 0xc000;

    let now = Instant::now();
    let start = now.elapsed().as_micros();

    for _i in 0..8991 {
        loop_cpu!(cpu, memory);
        cpu.print_debug();
    }

    let end = now.elapsed().as_micros();
    let dur = end - start;
    let dur_per_cycle = dur as f32 / cpu.total_cycles as f32;
    let freq = cpu.total_cycles as f32 / (dur as f32 / 1_000_000.0);

    eprintln!("duration: {:}us, cycles: {}", dur, cpu.total_cycles);
    eprintln!("duration / cycles: {} us/cycle", dur_per_cycle);
    if freq < 1_000_000.0 {
        eprintln!("freq: {} Hz", freq);
    } else if freq > 1_000_000_000.0 {
        eprintln!("freq: {} KHz", freq / 1_000.0);
    } else {
        eprintln!("freq: {} MHz", freq / 1_000_000.0);
    }

    Ok(())
}
