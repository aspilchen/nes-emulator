const SIZE: usize = 32;
pub const BEGIN: u16 = 0x3F00;
pub const END: u16 = 0x3FFF;

pub struct Palette {
    data: [u8; SIZE],
}

impl Palette {
    pub fn new() -> Self {
        Self { data: [0; SIZE] }
    }

    pub fn get_entry(&self, index: u16) -> [u8; 4] {
        let address = self.mirror_down(index) * 4;
        let mut result = [0; 4];
        result[0] = self.data[0];
        for i in 1..4 {
            result[i] = self.data[address as usize + i];
        }
        result
    }

    pub fn read(&self, address: u16) -> u8 {
        let address = self.mirror_down(address);
        self.data[address]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        let address = self.mirror_down(address);
        self.data[address] = value;
    }

    fn mirror_down(&self, address: u16) -> usize {
        (address & 0x1F) as usize
    }
}
