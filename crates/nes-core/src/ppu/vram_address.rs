const COARSE_X: u16 = 0b00000_00000_11111;
const COARSE_Y: u16 = 0b00000_11111_00000;
const NAMETABLE: u16 = 0b00011_00000_00000;
const FINE_Y: u16 = 0b11100_00000_00000;

// yyy NN YYYYY XXXXX where:
// XXXXX = coarse X
// YYYYY = coarse Y
// NN   = nametable
// yyy  = fine_y
pub struct VramAddress {
    pub address: u16,
}

impl VramAddress {
    pub fn new() -> Self {
        Self { address: 0 }
    }

    pub fn coarse_x(&self) -> u16 {
        self.address & 0x1F
    }

    pub fn coarse_y(&self) -> u16 {
        (self.address >> 5) & 0x1F
    }

    pub fn fine_y(&self) -> u16 {
        (self.address >> 12) & 0x7
    }

    pub fn nametable(&self) -> u16 {
        (self.address >> 8) & 0x3
    }

    pub fn set_coarse_x(&mut self, value: u16) {
        self.address = (self.address & !COARSE_X) | (value & COARSE_X);
    }

    pub fn set_coarse_y(&mut self, value: u16) {
        self.address = (self.address & !COARSE_Y) | ((value & 0x1F) << 5);
    }

    pub fn set_fine_y(&mut self, value: u16) {
        self.address = (self.address & !FINE_Y) | ((value & 0x3) << 12);
    }

    pub fn set_nametable(&mut self, value: u8) {
        self.address = (self.address & !NAMETABLE) | (((value as u16) & 0x3) << 10);
    }

    pub fn increment(&mut self, value: u16) {
        self.address = self.address.wrapping_add(value) & 0x7FFF; // register is only 15 bits
    }

    pub fn increment_horizontal(&mut self) {
        if self.coarse_x() == 31 {
            self.address &= !0x001F;
            self.address ^= 0x0400; // switch horizontal nametable
        } else {
            self.address += 1;
        }
    }

    // Vertical increment (end of scanline)
    pub fn increment_vertical(&mut self) {
        if self.fine_y() < 7 {
            self.address += 0x1000; // fine Y++
        } else {
            self.address &= !0x7000; // fine Y = 0
            let mut y = self.coarse_y();
            if y == 29 {
                y = 0;
                self.address ^= 0x0800; // switch vertical nametable
            } else if y == 31 {
                y = 0; // coarse Y padding
            } else {
                y += 1;
            }
            self.address = (self.address & !0x03E0) | (y << 5);
        }
    }
}
