macro_rules! abs {
    ($self:expr, $memory:expr) => {
        let lo = $memory.read($self.next_pc(), false) as u16;
        let hi = $memory.read($self.next_pc(), false) as u16;
        $self.absolute_address = (hi << 8) | lo;
    };
}
