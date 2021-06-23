use crate::memory::Memory;
use modular_bitfield::prelude::*;
use std::io::prelude::*;
use std::io::Cursor;
use std::io::SeekFrom;
use std::mem;
use std::slice;

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
    mapper: Box<dyn Mapper>,
    use_cartridge_data: bool,
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

        let mapper = Box::new(NROM {
            prg_banks: n_prg_banks,
        });

        Cartridge {
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

    pub fn use_cartridge_data(&self) -> bool {
        self.use_cartridge_data
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
            _ => {
                self.use_cartridge_data = false;
                return 0;
            }
        }
    }

    fn write(&mut self, _address: usize, _value: u8) {
        // do nothing for now
    }
}

pub enum MapperStatus {
    Read,
    Write,
    Unreadable,
}

pub trait Mapper {
    fn map_cpu_read_address(&self, address: usize, mapped_address: &mut usize) -> MapperStatus;
}

pub struct NROM {
    pub prg_banks: u8,
}

impl NROM {
    pub fn new(prg_banks: u8) -> Self {
        Self { prg_banks }
    }
}

impl Mapper for NROM {
    fn map_cpu_read_address(&self, address: usize, mapped_address: &mut usize) -> MapperStatus {
        if address < 0x8000 {
            return MapperStatus::Unreadable;
        }

        *mapped_address = address & if self.prg_banks > 1 { 0x7FFF } else { 0x3FFF };

        return MapperStatus::Read;
    }
}
