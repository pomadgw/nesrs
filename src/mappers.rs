pub mod nrom;

pub enum MapperStatus {
    Unreadable,
    Read,
    ReadWrite,
    VRAM,
}

pub trait Mapper {
    fn cpu_map_read(
        &self,
        address: u16,
        mapped_address: &mut usize,
        result: &mut u8,
    ) -> MapperStatus;
    fn cpu_map_write(&self, address: u16, mapped_address: &mut usize, value: u8) -> MapperStatus;
    fn ppu_map_read(
        &self,
        address: u16,
        mapped_address: &mut usize,
        result: &mut u8,
    ) -> MapperStatus;
    fn ppu_map_write(&self, address: u16, mapped_address: &mut usize, value: u8) -> MapperStatus;
}
