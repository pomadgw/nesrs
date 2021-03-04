use std::io::Cursor;
use std::mem;
use std::slice;

use std::io::prelude::*;
use std::io::SeekFrom;

use crate::mappers::nrom::*;
use crate::mappers::*;
use crate::utils::*;

pub enum MirrorMode {
    Hardware,
    Vertical,
    Horizontal,
    FourScreen,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct NESHeader {
    name: [u8; 4],
    pub prg_rom_chunks: u8,
    pub chr_rom_chunks: u8,
    pub mapper1: u8,
    pub mapper2: u8,
    prg_ram_size: u8,
    tv_system1: u8,
    tv_system2: u8,
    unused: [u8; 5],
}

pub struct Cartridge {
    pub mapperid: u8,
    pub hardware_mirror_mode: MirrorMode,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub ram: Vec<u8>,
    pub header: NESHeader,
    pub mapper: Option<Box<dyn Mapper>>,
}

impl Cartridge {
    pub fn new_from_file(data: Vec<u8>) -> Cartridge {
        let mut file = Cursor::new(data);
        let mut header: NESHeader = unsafe { mem::zeroed() };

        let header_size = mem::size_of::<NESHeader>();

        unsafe {
            let header_slice =
                slice::from_raw_parts_mut(&mut header as *mut _ as *mut u8, header_size);

            file.read_exact(header_slice).unwrap();

            // println!("{:?}", header);
        }

        if (header.mapper1 & 0x04) > 0 {
            file.seek(SeekFrom::Current(512)).unwrap();
        }

        let mapperid = ((header.mapper2 >> 4) << 4) | (header.mapper1 >> 4);
        let hardware_mirror_mode = if (header.mapper1 & 0x01) > 0 {
            MirrorMode::Vertical
        } else {
            MirrorMode::Horizontal
        };

        let mut n_prg_banks = 0;
        let mut n_chr_banks = 0;

        let mut prg_rom: Vec<u8> = Vec::new();
        let mut chr_rom: Vec<u8> = Vec::new();

        let n_filetype = 1;

        if n_filetype == 0 {}
        if n_filetype == 1 {
            n_prg_banks = header.prg_rom_chunks;
            n_chr_banks = header.chr_rom_chunks;

            println!("n_prg_banks: {}, n_chr_banks: {}", n_prg_banks, n_chr_banks);

            prg_rom.resize((n_prg_banks as usize) * 16384, 0);
            file.read(&mut prg_rom).unwrap();

            if n_chr_banks == 0 {
                chr_rom.resize(8192, 0);
            } else {
                chr_rom.resize((n_chr_banks as usize) * 8192, 0);
                file.read(&mut chr_rom).unwrap();
            }
        }
        if n_filetype == 2 {}

        let mapper: Option<Box<dyn Mapper>> = match mapperid {
            0 => Some(Box::new(MapperNROM::new(n_prg_banks, n_chr_banks))),
            _ => None,
        };

        Cartridge {
            mapperid,
            prg_rom,
            chr_rom,
            hardware_mirror_mode,
            ram: vec![0; 0x2000],
            header,
            mapper,
        }
    }

    pub fn cpu_read(&self, addressing: u16, result: &mut u8) -> bool {
        let mut mapped_address: usize = 0;

        match self
            .mapper
            .as_ref()
            .unwrap()
            .cpu_map_read(addressing, &mut mapped_address, result)
        {
            MapperStatus::ReadWrite => true,
            MapperStatus::VRAM => {
                *result = self.ram[mapped_address];
                true
            }
            MapperStatus::Read => {
                *result = self.prg_rom[mapped_address];
                true
            }
            MapperStatus::Unreadable => false,
        }
    }

    pub fn cpu_write(&mut self, addressing: u16, value: u8) -> bool {
        let mut mapped_address: usize = 0;

        match self
            .mapper
            .as_ref()
            .unwrap()
            .cpu_map_write(addressing, &mut mapped_address, value)
        {
            MapperStatus::ReadWrite => true,
            MapperStatus::VRAM => {
                self.ram[mapped_address] = value;
                true
            }
            MapperStatus::Read => {
                self.prg_rom[mapped_address] = value;
                true
            }
            MapperStatus::Unreadable => false,
        }
    }
}
