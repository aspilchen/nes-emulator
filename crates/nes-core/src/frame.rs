pub const WIDTH: usize = 256;
pub const HEIGHT: usize = 240;


// pub struct Frame {
//    pub data: Vec<u8>,
// }

// impl Frame {
//    const WIDTH: usize = 256;
//    const HIGHT: usize = 240;

//    pub fn new() -> Self {
//        Frame {
//            data: vec![0; (Frame::WIDTH) * (Frame::HIGHT) * 3],
//        }
//    }

//    pub fn set_pixel(&mut self, x: usize, y: usize, rgb: (u8, u8, u8)) {
//        let base = y * 3 * Frame::WIDTH + x * 3;
//        if base + 2 < self.data.len() {
//            self.data[base] = rgb.0;
//            self.data[base + 1] = rgb.1;
//            self.data[base + 2] = rgb.2;
//        }
//    }
// }


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
