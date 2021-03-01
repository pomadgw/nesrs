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

macro_rules! lda {
    ($self:expr, $memory:expr) => {
        $self.a = $memory.read($self.absolute_address, false);
        $self.set_nz($self.a);
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
        $self.set_nz(temp);
    };
}
