use nesrs;
use nesrs::cpu::*;
use nesrs::memory::*;

use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::mem;
use std::slice;

#[macro_use]
mod macros;

struct RAM {
    prg_banks: u8,
    prg_rom: Vec<u8>,
    ram: Vec<u8>,
}

impl Memory for RAM {
    fn read(&self, address: usize, _is_read_only: bool) -> u8 {
        if address < 0x8000 {
            self.ram[address]
        } else {
            let new_address = address & if self.prg_banks > 1 { 0x7FFF } else { 0x3FFF };
            self.prg_rom[new_address]
        }
    }

    fn write(&mut self, address: usize, value: u8) {
        if address < 0x8000 {
            self.ram[address] = value;
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
struct NESHeader {
    name: [u8; 4],
    prg_rom_chunks: u8,
    chr_rom_chunks: u8,
    mapper1: u8,
    mapper2: u8,
    prg_ram_size: u8,
    tv_system1: u8,
    tv_system2: u8,
    unused: [u8; 5],
}

fn main() -> std::io::Result<()> {
    let mut cpu = CPU::new();

    let mut file = File::open("./rom/nestest.nes")?;

    // read nesrom.nes
    let mut header: NESHeader = unsafe { mem::zeroed() };

    let header_size = mem::size_of::<NESHeader>();

    unsafe {
        let header_slice = slice::from_raw_parts_mut(&mut header as *mut _ as *mut u8, header_size);

        file.read_exact(header_slice).unwrap();
    }

    if (header.mapper1 & 0x04) > 0 {
        file.seek(SeekFrom::Current(512)).unwrap();
    }

    let mut prg_memory: Vec<u8> = Vec::new();
    let mut chr_memory: Vec<u8> = Vec::new();

    let n_prg_banks = header.prg_rom_chunks;
    let n_chr_banks = header.chr_rom_chunks;

    // println!("n_prg_banks: {}, n_chr_banks: {}", n_prg_banks, n_chr_banks);

    prg_memory.resize((n_prg_banks as usize) * 16384, 0);
    file.read(&mut prg_memory).unwrap();

    if n_chr_banks == 0 {
        chr_memory.resize(8192, 0);
    } else {
        chr_memory.resize((n_chr_banks as usize) * 8192, 0);
        file.read(&mut chr_memory).unwrap();
    }

    let mut memory = RAM {
        prg_rom: prg_memory,
        prg_banks: n_prg_banks,
        ram: vec![0; 0x8000],
    };

    cpu.debug = true;
    cpu.reset();
    loop_cpu!(cpu, memory);
    cpu.regs.pc = 0xc000;

    for _i in 0..8991 {
        loop_cpu!(cpu, memory);
        cpu.print_debug();
    }

    Ok(())
}
