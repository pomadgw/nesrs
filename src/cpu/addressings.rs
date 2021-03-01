macro_rules! imm {
    ($self:expr, $memory:expr) => {
        $self.absolute_address = $self.next_pc();
    };
}

macro_rules! abs {
    ($self:expr, $memory:expr) => {
        let lo = $memory.read($self.next_pc(), false) as u16;
        let hi = $memory.read($self.next_pc(), false) as u16;
        $self.absolute_address = (hi << 8) | lo;
    };
}

macro_rules! abw {
    ($self:expr, $memory:expr, $reg:expr, false) => {
        let lo = $memory.read($self.next_pc(), false) as u16;
        let hi = $memory.read($self.next_pc(), false) as u16;
        $self.absolute_address = (hi << 8) | lo;
        $self.absolute_address += ($reg as u16);
        if ($self.absolute_address & 0xff00) != (hi << 8) {
            $self.is_crossing_page = true;
        }
    };

    ($self:expr, $memory:expr, $reg:expr, true) => {
        let lo = $memory.read($self.next_pc(), false) as u16;
        let hi = $memory.read($self.next_pc(), false) as u16;
        $self.absolute_address = (hi << 8) | lo;
        $self.absolute_address += ($reg as u16);
    };
}

/// params:
/// $self: CPU
/// $memory: memory trait
/// $skip_cross_page: whether to skip cross page or not
macro_rules! abx {
    ($self:expr, $memory:expr, false) => {
        abw!($self, $memory, $self.x, false);
    };
    ($self:expr, $memory:expr, true) => {
        abw!($self, $memory, $self.x, true);
    };
}

/// params:
/// $self: CPU
/// $memory: memory trait
/// $skip_cross_page: whether to skip cross page or not
macro_rules! aby {
    ($self:expr, $memory:expr, false) => {
        abw!($self, $memory, $self.y, false);
    };
    ($self:expr, $memory:expr, true) => {
        abw!($self, $memory, $self.y, true);
    };
}
