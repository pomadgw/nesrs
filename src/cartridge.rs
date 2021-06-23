use modular_bitfield::prelude::*;
use std::io::prelude::*;
use std::io::Cursor;
use std::io::SeekFrom;
use std::mem;
use std::slice;
// use crate::memory::Memory;

#[derive(Debug, Copy, Clone)]
pub enum MirroringMode {
    Hardware,
    Horizontal,
    Vertical,
}

#[allow(dead_code)]
#[bitfield]
pub struct Flag6iNES {
    mirroring: B1,
    has_prg_ram: bool,
    has_trainer: bool,
    use_four_screen_vram: B1,
    mapper_lower: B4,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct NESHeader {
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

#[allow(dead_code)]
pub struct Cartridge {
    header: NESHeader,
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    chr_ram: Vec<u8>,
    n_prg_banks: usize,
    n_chr_banks: usize,
    hw_mirroring: MirroringMode,
    mapper_id: u8,
}

impl Cartridge {
    pub fn parse(buffer: &Vec<u8>) -> Cartridge {
        let mut cursor = Cursor::new(buffer);

        // read nesrom.nes
        let mut header: NESHeader = unsafe { mem::zeroed() };

        let header_size = mem::size_of::<NESHeader>();

        unsafe {
            let header_slice =
                slice::from_raw_parts_mut(&mut header as *mut _ as *mut u8, header_size);

            cursor.read_exact(header_slice).unwrap();
        }

        let flag6 = Flag6iNES::from_bytes([header.mapper1]);
        let mapper_id = (header.mapper2 & 0xf0) | flag6.mapper_lower();

        if flag6.has_trainer() {
            cursor.seek(SeekFrom::Current(512)).unwrap();
        }

        let hw_mirroring = if flag6.mirroring() == 0 {
            MirroringMode::Horizontal
        } else {
            MirroringMode::Vertical
        };

        let mut prg_rom: Vec<u8> = Vec::new();
        let mut chr_rom: Vec<u8> = Vec::new();

        let n_prg_banks = header.prg_rom_chunks;
        let n_chr_banks = header.chr_rom_chunks;

        prg_rom.resize((n_prg_banks as usize) * 16384, 0);
        cursor.read(&mut prg_rom).unwrap();

        if n_chr_banks > 0 {
            chr_rom.resize((n_chr_banks as usize) * 8192, 0);
            cursor.read(&mut chr_rom).unwrap();
        }

        Cartridge {
            header,
            prg_rom,
            n_prg_banks: n_prg_banks as usize,
            n_chr_banks: n_chr_banks as usize,
            chr_rom,
            chr_ram: vec![0; 8192],
            hw_mirroring,
            mapper_id,
        }
    }

    pub fn prg_rom(&self) -> &Vec<u8> {
        &self.prg_rom
    }

    pub fn chr_rom(&self) -> &Vec<u8> {
        if self.n_chr_banks == 0 {
            &self.chr_ram
        } else {
            &self.chr_rom
        }
    }

    pub fn header(&self) -> NESHeader {
        self.header
    }

    pub fn mirroring(&self) -> MirroringMode {
        self.hw_mirroring
    }
}
