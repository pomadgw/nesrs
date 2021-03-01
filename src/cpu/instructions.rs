macro_rules! set_instruction {
    ($self:expr, $cycles:expr, $block:block) => {{
        if $self.steps == 0 {
            $block
        }

        let cycle_required = if $self.is_crossing_page {
            $cycles
        } else {
            $cycles - 1
        };

        if $self.steps == cycle_required {
            $self.sync = true;
        }
    }};
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

macro_rules! lda {
    ($self:expr, $memory:expr) => {
        $self.a = $memory.read($self.absolute_address, false);
    };
}

// INC opcode invokes double read-write
macro_rules! inc {
    ($self:expr, $memory:expr) => {
        let mut temp = $memory.read($self.absolute_address, false);
        temp = $memory.read($self.absolute_address, false);

        $memory.write($self.absolute_address, temp);
        temp = temp.wrapping_add(1);
        $memory.write($self.absolute_address, temp);
    };
}
