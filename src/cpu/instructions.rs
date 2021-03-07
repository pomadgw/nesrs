#[allow(unused_macros)]
macro_rules! inc {
    ($self:expr, $memory:expr) => {
        $memory.read($self.absolute_address, false);
        let mut temp = $memory.read($self.absolute_address, false);

        $memory.write($self.absolute_address, temp);
        temp = temp.wrapping_add(1);
        $memory.write($self.absolute_address, temp);
        $self.set_nz(temp);

        $self.is_crossing_page = false;
    };
}

#[allow(unused_macros)]
macro_rules! lda {
    ($self:expr, $memory:expr) => {
        $self.a = $memory.read($self.absolute_address, false);
        $self.set_nz($self.a);
    };
}

#[allow(unused_macros)]
macro_rules! ldx {
    ($self:expr, $memory:expr) => {
        $self.x = $memory.read($self.absolute_address, false);
        $self.set_nz($self.x);
    };
}
