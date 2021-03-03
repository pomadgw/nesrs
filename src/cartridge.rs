use std::fs::File;
use std::mem;
use std::rc::Rc;
use std::slice;
use std::io::Cursor;

use std::io::prelude::*;
use std::io::SeekFrom;

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
    pub mapperid : u8,
    pub hardware_mirror_mode : MirrorMode,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub ram: Vec<u8>,
    pub header: NESHeader,
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

        Cartridge {
            mapperid,
            prg_rom,
            chr_rom,
            hardware_mirror_mode,
            ram: vec![0; 0x2000],
            header,
        }
    }
}
