macro_rules! imp {
    ($self:expr, $memory:expr) => {
        // do nothing...
    };
}

macro_rules! imm {
    ($self:expr, $memory:expr) => {
        $self.absolute_address = $self.next_pc();
    };
}

macro_rules! zp0 {
    ($self:expr, $memory:expr) => {
        let lo = $memory.read($self.next_pc(), false) as u16;
        $self.absolute_address = lo;
    };
}

macro_rules! zpx {
    ($self:expr, $memory:expr) => {
        let lo = $memory.read($self.next_pc(), false);
        $self.absolute_address = (lo.wrapping_add($self.x)) as u16;
    };
}

macro_rules! zpy {
    ($self:expr, $memory:expr) => {
        let lo = $memory.read($self.next_pc(), false);
        $self.absolute_address = (lo.wrapping_add($self.y)) as u16;
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
    ($self:expr, $memory:expr, $reg:expr) => {
        let lo = $memory.read($self.next_pc(), false) as u16;
        let hi = $memory.read($self.next_pc(), false) as u16;
        $self.absolute_address = (hi << 8) | lo;
        $self.absolute_address += ($reg as u16);
        if ($self.absolute_address & 0xff00) != (hi << 8) {
            $self.is_crossing_page = true;
        }
    };
}

/// params:
/// $self: CPU
/// $memory: memory trait
/// $skip_cross_page: whether to skip cross page or not
macro_rules! abx {
    ($self:expr, $memory:expr) => {
        abw!($self, $memory, $self.x);
    };
}

/// params:
/// $self: CPU
/// $memory: memory trait
/// $skip_cross_page: whether to skip cross page or not
macro_rules! aby {
    ($self:expr, $memory:expr) => {
        abw!($self, $memory, $self.y);
    };
}

macro_rules! izx {
    ($self:expr, $memory:expr) => {
        let mut temp = $memory.read($self.next_pc(), false);
        temp = temp.wrapping_add($self.x);
        let lo = $memory.read(temp.wrapping_add(0) as u16, false) as u16;
        let hi = $memory.read(temp.wrapping_add(1) as u16, false) as u16;
        $self.absolute_address = (hi << 8) | lo;
    };
}

macro_rules! izy {
    ($self:expr, $memory:expr) => {
        let temp = $memory.read($self.next_pc(), false);
        let lo = $memory.read(temp.wrapping_add(0) as u16, false) as u16;
        let hi = $memory.read(temp.wrapping_add(1) as u16, false) as u16;
        $self.absolute_address = (hi << 8) | lo;

        $self.absolute_address += ($self.y as u16);
        if ($self.absolute_address & 0xff00) != (hi << 8) {
            $self.is_crossing_page = true;
        }
    };
}

/// params:
/// $self: CPU
/// $memory: memory trait
/// $skip_cross_page: whether to skip cross page or not
macro_rules! ind {
    ($self:expr, $memory:expr) => {
        let lo_lo = $memory.read($self.next_pc(), false);
        let hi = $memory.read($self.next_pc(), false);
        let lo_hi = lo_lo.wrapping_add(1);
        let lo = toword!(lo_lo, hi);
        let hi = toword!(lo_hi, hi);

        $self.absolute_address = toword!(lo, hi);
    };
}
