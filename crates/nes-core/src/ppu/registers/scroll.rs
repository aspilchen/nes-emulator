pub struct Scroll {
    bytes: [u8; 2],
    latch: usize,
}

impl Default for Scroll {
    fn default() -> Self {
        Self {
            bytes: [0, 0],
            latch: 0,
        }
    }
}

impl Scroll {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn h_scroll(&self) -> u8 {
        self.bytes[0]
    }

    pub fn v_scroll(&self) -> u8 {
        self.bytes[1]
    }

    pub fn write(&mut self, value: u8) {
        self.bytes[self.latch] = value;
        self.latch = (self.latch + 1) % 2;
    }

    pub fn reset_latch(&mut self) {
        self.latch = 0;
    }

    pub fn reset(&mut self) {
        self.bytes = [0, 0];
        self.latch = 0;
    }
}
