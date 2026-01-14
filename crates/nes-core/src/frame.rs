pub const WIDTH: usize = 256;
pub const HEIGHT: usize = 240;

pub struct Frame {
    pub data: [[u8; WIDTH]; HEIGHT],
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            data: [[0; WIDTH]; HEIGHT],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: u8) {
        if x < WIDTH && y < HEIGHT {
            self.data[y][x] = value;
        }
    }

    pub fn clear(&mut self) {
        for row in &mut self.data {
            row.fill(0);
        }
    }
}
