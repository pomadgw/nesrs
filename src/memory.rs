pub trait Memory {
    fn read(&self, address: usize, is_read_only: bool) -> u8;
    fn write(&mut self, address: usize, value: u8);
}
