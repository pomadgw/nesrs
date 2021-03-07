pub trait Memory {
    fn read(&self, address: u16, is_read_only: bool) -> u8;
    fn write(&mut self, address: u16, value: u8);
}
