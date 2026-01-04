pub const RAM_SIZE: usize = 0x0800;
pub const RAM_BEGIN: u16 = 0;
pub const RAM_END: u16 = 0x07FF;

pub struct Ram {
    data: [u8; RAM_SIZE],
}

impl Ram {
    pub fn new() -> Self {
        Self {
            data: [0; RAM_SIZE],
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        let address = self.map_address(address);
        self.data[address]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        let address = self.map_address(address);
        self.data[address] = value;
    }

    fn map_address(&self, address: u16) -> usize {
        let mask = 0x7FF;
        (address & mask) as usize
    }
}
