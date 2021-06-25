pub enum MapperStatus {
    Read,
    Write,
    Unreadable,
}

pub trait Mapper {
    fn map_cpu_read_address(&self, address: usize, mapped_address: &mut usize) -> MapperStatus;
    fn map_cpu_write_address(
        &mut self,
        address: usize,
        mapped_address: &mut usize,
        value: u8,
    ) -> MapperStatus;

    fn map_ppu_read_address(&self, address: usize, mapped_address: &mut usize) -> MapperStatus;
    fn map_ppu_write_address(
        &mut self,
        address: usize,
        mapped_address: &mut usize,
        value: u8,
    ) -> MapperStatus;
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

    fn map_cpu_write_address(
        &mut self,
        _address: usize,
        _mapped_address: &mut usize,
        _value: u8,
    ) -> MapperStatus {
        MapperStatus::Unreadable
    }

    fn map_ppu_read_address(&self, address: usize, mapped_address: &mut usize) -> MapperStatus {
        if address >= 0x2000 {
            return MapperStatus::Unreadable;
        }

        *mapped_address = address;

        return MapperStatus::Read;
    }

    fn map_ppu_write_address(
        &mut self,
        _address: usize,
        _mapped_address: &mut usize,
        _value: u8,
    ) -> MapperStatus {
        MapperStatus::Unreadable
    }
}
