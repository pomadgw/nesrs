#[allow(unused_macros)]
macro_rules! brk {
    ($self:expr, $memory:expr) => {
        if $self.irq_triggers == 0 {
            $self.next_pc();
        }

        let hi: u8 = (($self.pc >> 8) & 0xff) as u8;
        let lo: u8 = ($self.pc & 0xff) as u8;

        $self.push_stack(
            $memory,
            if $self.check_trigger(IRQStatus::Reset) {
                0
            } else {
                hi
            },
        );

        $self.push_stack(
            $memory,
            if $self.check_trigger(IRQStatus::Reset) {
                0
            } else {
                lo
            },
        );

        $self.set_status(CPUStatus::B, true);
        $self.set_status(CPUStatus::U, true);

        if $self.check_trigger(IRQStatus::Reset) {
            $self.push_stack($memory, 0);
        } else {
            $self.push_stack($memory, $self.p);
            $self.set_status(CPUStatus::U, false);
        }

        $self.set_status(CPUStatus::B, false);
        $self.set_status(CPUStatus::I, true);

        let vector_address: u16 = if $self.check_trigger(IRQStatus::Reset) {
            0xfffc
        } else if $self.check_trigger(IRQStatus::NMI) {
            0xfffa
        } else {
            0xfffe
        };

        let lo = $memory.read(vector_address, false) as u16;
        let hi = $memory.read(vector_address + 1, false) as u16;

        $self.pc = (hi << 8) | lo;

        if $self.check_trigger(IRQStatus::Reset) {
            $self.clear_trigger(IRQStatus::Reset);
        } else if $self.check_trigger(IRQStatus::NMI) {
            $self.clear_trigger(IRQStatus::NMI);
        }
    };
}

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
