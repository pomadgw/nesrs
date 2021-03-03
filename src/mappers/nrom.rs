use crate::mappers::*;

pub struct MapperNROM {
    pub prg_rom_num: u8,
    pub chr_rom_num: u8,
}

impl MapperNROM {
    pub fn new(prg_rom_num: u8, chr_rom_num: u8) -> MapperNROM {
        MapperNROM {
            prg_rom_num,
            chr_rom_num,
        }
    }
}

impl Mapper for MapperNROM {
    fn cpu_map_read(
        &self,
        address: u16,
        mapped_address: &mut usize,
        result: &mut u8,
    ) -> MapperStatus {
        if address >= 0x6000 && address < 0x8000 {
            *mapped_address = address as usize;
            return MapperStatus::VRAM;
        }

        if address >= 0x8000 {
            *mapped_address =
                (address & if self.prg_rom_num > 1 { 0x7fff } else { 0x3fff }) as usize;
            return MapperStatus::Read;
        }

        MapperStatus::Unreadable
    }

    fn cpu_map_write(&self, address: u16, mapped_address: &mut usize, value: u8) -> MapperStatus {
        if address >= 0x6000 && address < 0x8000 {
            *mapped_address = address as usize;
            return MapperStatus::VRAM;
        }

        if address >= 0x8000 {
            *mapped_address =
                (address & if self.prg_rom_num > 1 { 0x7fff } else { 0x3fff }) as usize;
            return MapperStatus::Read;
        }

        MapperStatus::Unreadable
    }

    fn ppu_map_read(
        &self,
        address: u16,
        mapped_address: &mut usize,
        result: &mut u8,
    ) -> MapperStatus {
        if address < 0x2000 {
            *mapped_address = address as usize;
            MapperStatus::Read
        } else {
            MapperStatus::Unreadable
        }
    }

    fn ppu_map_write(&self, address: u16, mapped_address: &mut usize, value: u8) -> MapperStatus {
        MapperStatus::Unreadable
    }
}
