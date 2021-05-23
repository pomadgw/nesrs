#[allow(unused_macros)]
macro_rules! step {
    ($self:ident ; $n:expr; ) => {};
    ($self:ident ; $n:expr; $block:block $(, $rest:block)*) => {
        if ($self.cycles == $n) {
            $block
        } else {
            step!($self; $n + 1; $($rest),*);
        }
    };
    ($self:ident, $($blocks:block)+) => { step!($self; 0; $($blocks),*); };
}

#[allow(unused_macros)]
macro_rules! set_ram {
    ($memory:ident, $start:expr, [ $( $content:expr ),* ]) => {
        {
            let mut offset: usize = 0;
            $(
                $memory.write($start + offset, $content);
                offset += 1;
            )*
        }
    }
}

#[allow(unused_macros)]
macro_rules! set_ram_from_vec {
    ($memory:ident, $start:expr, $content:expr) => {{
        let mut offset: usize = 0;

        for v in $content {
            $memory.ram[$start + offset] = *v;
            offset += 1;
        }
    }};
}

#[allow(unused_macros)]
macro_rules! set_reset {
    ($memory:ident, $address:expr) => {
        let hi = (($address >> 8) & 0xff) as u8;
        let lo = ($address & 0xff) as u8;
        $memory.write(INTERRUPT_RESET as usize, lo);
        $memory.write((INTERRUPT_RESET + 1) as usize, hi);
    };
}

#[allow(unused_macros)]
macro_rules! loop_cpu {
    ($cpu:ident, $memory:ident) => {
        $cpu.clock(&mut $memory);
        while !$cpu.done() {
            $cpu.clock(&mut $memory);
        }
    };
}
