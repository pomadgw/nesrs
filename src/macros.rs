macro_rules! set_ram {
    ($memory:ident, $start:expr, [ $( $content:expr ),* ]) => {
        {
            let mut offset: usize = 0;
            $(
                $memory.ram[$start + offset] = $content;
                offset += 1;
            )*
        }
    }
}

macro_rules! set_reset {
    ($memory:ident, $address:expr) => {
        let hi = (($address >> 8) & 0xff) as u8;
        let lo = ($address & 0xff) as u8;
        $memory.ram[INTERRUPT_RESET as usize] = lo;
        $memory.ram[(INTERRUPT_RESET + 1) as usize] = hi;
    };
}

macro_rules! loop_cpu {
    ($cpu:ident, $memory:ident) => {
        $cpu.clock(&mut $memory);
        while !$cpu.done() {
            $cpu.clock(&mut $memory);
        }
    };
}