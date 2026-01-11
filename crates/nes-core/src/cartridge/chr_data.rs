use std::ops::Shr;

pub const TILE_SIZE: usize = 16;
const ROW_SIZE: usize = 8;

pub struct ChrBank {
    pub data: Vec<u8>,
}

pub struct ChrTile<'a> {
    pub data: &'a [u8; TILE_SIZE],
}

impl ChrBank {
    pub fn new(chr_rom: Vec<u8>) -> Self {
        Self { data: chr_rom }
    }

    pub fn get_tile(&self, index: u16) -> ChrTile {
        let index = index as usize;
        let start = index * TILE_SIZE;
        let end = start + TILE_SIZE;
        let bytes: &[u8; TILE_SIZE] = self.data[start..end].try_into().unwrap();
        ChrTile { data: bytes }
    }

    pub fn chr_tile_count(&self) -> usize {
        self.data.len() / TILE_SIZE
    }
}

impl<'a> ChrTile<'a> {
    pub fn get_pixel(&self, x: u16, y: u16) -> u8 {
        let x = x as usize;
        let y = y as usize;
        let plane0 = self.data[y];
        let plane1 = self.data[ROW_SIZE + y];
        let shift = (ROW_SIZE - 1) - x;
        let bit0 = (plane0 >> shift) & 1;
        let bit1 = (plane1 >> shift) & 1;
        (bit1 << 1) | bit0
    }

    pub fn get_row(&self, y: u16) -> [u8; ROW_SIZE] {
        let y = y as usize;
        let mut result = [0; ROW_SIZE];
        let mut upper = self.data[y];
        let mut lower = self.data[y + ROW_SIZE];
        for x in (0..8).rev() {
            result[x] = (1 & upper) << 1 | (1 & lower);
            upper = upper >> 1;
            lower = lower >> 1;
        }
        result
    }
}
