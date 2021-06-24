use nesrs;
use nesrs::bus::*;
use nesrs::cartridge::*;

use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

#[macro_use]
mod macros;

fn main() -> std::io::Result<()> {
    let mut file = File::open("./rom/nestest.nes")?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;
    let cartridge = Cartridge::parse(&buffer);

    let mut bus = Bus::new(cartridge);

    bus.cpu.debug = false;
    bus.reset();
    bus.cpu.regs.pc = 0xc000;

    let now = Instant::now();
    let start = now.elapsed().as_micros();

    for _i in 0..(8991 * 3) {
        bus.clock();

        if bus.cpu.done() {
            bus.cpu.print_debug();
        }
    }

    let end = now.elapsed().as_micros();
    let dur = end - start;
    let dur_per_cycle = dur as f32 / bus.cpu.total_cycles as f32;
    let freq = bus.cpu.total_cycles as f32 / (dur as f32 / 1_000_000.0);

    eprintln!("duration: {:}us, cycles: {}", dur, bus.cpu.total_cycles);
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
