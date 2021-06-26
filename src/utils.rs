pub struct XORShiftRand {
    state: u64,
}

impl XORShiftRand {
    pub fn new(init: u64) -> XORShiftRand {
        XORShiftRand { state: init }
    }

    pub fn rand(&mut self) -> u64 {
        let mut x = self.state;

        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;

        self.state = x;

        x
    }
}
