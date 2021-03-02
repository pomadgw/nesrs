use nesrs;

pub struct DummyBus {
    pub ram: Vec<u8>,
}

impl DummyBus {
    pub fn new() -> DummyBus {
        DummyBus {
            ram: vec![0; 0x10000],
        }
    }
}

impl nesrs::utils::Memory for DummyBus {
    fn read(&self, addressing: u16, _is_read_only: bool) -> u8 {
        self.ram[addressing as usize]
    }

    fn write(&mut self, addressing: u16, value: u8) {
        self.ram[addressing as usize] = value;
    }
}

macro_rules! init_memory {
    ($bus:expr, $( $x:expr ),*) => {
        let mut pos: usize = 0;
        $(
            pos += 1;
            $bus.ram[pos - 1] = $x;
        )*
    }
}
