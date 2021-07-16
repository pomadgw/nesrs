use crate::mappers::*;
use crate::memory::Memory;
use modular_bitfield::prelude::*;
use std::io::prelude::*;
use std::io::Cursor;
use std::io::SeekFrom;
use std::mem;
use std::slice;

use std::sync::{Arc, Mutex};

#[derive(Debug, Copy, Clone)]
pub enum MirroringMode {
    Hardware,
    Horizontal,
    Vertical,
    SingleScreen,
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
    mapper: Box<dyn Mapper>,
    use_cartridge_data: bool,
}

pub type CartridgeRef = Arc<Mutex<Cartridge>>;

impl Cartridge {
    pub fn parse(buffer: &Vec<u8>) -> Result<Cartridge, String> {
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

        let mapper = match mapper_id {
            NROM::ID => Box::new(NROM::new(n_prg_banks, n_chr_banks)),
            _ => {
                return Err(format!("Mapper {} is not supported", mapper_id));
            }
        };

        Ok(Cartridge {
            header,
            prg_rom,
            n_prg_banks: n_prg_banks as usize,
            n_chr_banks: n_chr_banks as usize,
            chr_rom,
            chr_ram: vec![0; 8192],
            hw_mirroring,
            mapper_id,
            mapper,
            use_cartridge_data: false,
        })
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

    pub fn use_cartridge_data(&self) -> bool {
        self.use_cartridge_data
    }

    pub fn ppu_read(&mut self, address: usize, _is_read_only: bool) -> u8 {
        let mut mapped_address = 0;
        let result = self
            .mapper
            .map_ppu_read_address(address, &mut mapped_address);

        match result {
            MapperStatus::Read => {
                self.use_cartridge_data = true;
                return self.chr_rom()[mapped_address];
            }
            _ => {
                self.use_cartridge_data = false;
                return 0;
            }
        }
    }

    pub fn ppu_write(&mut self, address: usize, value: u8) {
        let mut mapped_address = 0;
        let result = self
            .mapper
            .map_ppu_read_address(address, &mut mapped_address);

        match result {
            MapperStatus::Read => {
                self.use_cartridge_data = true;
                self.chr_ram[mapped_address] = value;
            }
            _ => {
                self.use_cartridge_data = false;
            }
        }
    }
}

impl Memory for Cartridge {
    fn read(&mut self, address: usize, _is_read_only: bool) -> u8 {
        let mut mapped_address = 0;
        let result = self
            .mapper
            .map_cpu_read_address(address, &mut mapped_address);

        match result {
            MapperStatus::Read => {
                self.use_cartridge_data = true;
                return self.prg_rom()[mapped_address];
            }
            MapperStatus::ReadRam(data) => {
                self.use_cartridge_data = true;
                return data;
            }
            _ => {
                self.use_cartridge_data = false;
                return 0;
            }
        }
    }

    fn write(&mut self, address: usize, value: u8) {
        let mut mapped_address = 0;
        let result = self
            .mapper
            .map_cpu_write_address(address, &mut mapped_address, value);

        match result {
            MapperStatus::Read => {
                self.use_cartridge_data = true;
                self.prg_rom[mapped_address] = value;
            }
            MapperStatus::Write => {
                self.use_cartridge_data = true;
            }
            _ => {
                self.use_cartridge_data = false;
            }
        }
    }
}
