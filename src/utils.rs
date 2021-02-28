pub trait Memory {
    fn read(&self, addressing: u16, is_read_only: bool) -> u8;
    fn write(&mut self, addressing: u16, value: u8);
}
