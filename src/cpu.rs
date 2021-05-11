mod test;

pub fn hello() {
    println!("Hello");
}

pub struct CPURegisters {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub p: u8,
    pub pc: u16,
}

impl Default for CPURegisters {
    fn default() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            sp: 0,
            p: 0,
            pc: 0
        }
    }
}

pub struct CPU {
    pub regs: CPURegisters
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            regs: Default::default(),
        }
    }
}
