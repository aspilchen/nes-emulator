pub const WIDTH: usize = 256;
pub const HEIGHT: usize = 240;

pub trait Frame {
    fn set_pixel(&mut self, x: usize, y: usize, value: u8);
    fn is_transparent(&self, x: usize, y: usize) -> bool;
    fn clear(&mut self);
}
