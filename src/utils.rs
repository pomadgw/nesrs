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

pub type PPUColor = (u8, u8, u8);

pub struct Screen {
    width: usize,
    height: usize,
    image: Vec<u8>,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Screen {
        Screen {
            width,
            height,
            image: vec![0; width * height * 4],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: PPUColor) {
        let pos = self.width * y + x;
        let pos = pos * 4;

        assert!(pos + 3 < self.width * self.height * 4);

        self.image[pos + 0] = color.0;
        self.image[pos + 1] = color.1;
        self.image[pos + 2] = color.2;
        self.image[pos + 3] = 255;
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn image(&self) -> &Vec<u8> {
        &self.image
    }
}
