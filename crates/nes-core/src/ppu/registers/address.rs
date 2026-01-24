pub struct Address {
    bytes: [u8; 2],
    latch: usize,
}

impl Address {
    pub fn new() -> Self {
        Self {
            bytes: [0, 0],
            latch: 0,
        }
    }

    pub fn read(&self) -> u16 {
        u16::from_be_bytes(self.bytes)
    }

    pub fn write(&mut self, value: u8) {
        self.bytes[self.latch] = value;
        self.latch = (self.latch + 1) % 2;
    }

    pub fn increment(&mut self, step_size: u16) {
        let mut address = self.read();
        address = address.wrapping_add(step_size);
        self.bytes = address.to_be_bytes();
    }

    pub fn reset_latch(&mut self) {
        self.latch = 0;
    }
}
