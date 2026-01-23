use std::ops::Shr;

pub const TILE_SIZE: usize = 16;
pub const TILE_WIDTH: usize = 8;
pub const TILE_HEIGHT: usize = 8;

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
        let start = index as usize * TILE_SIZE;
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
        let plane1 = self.data[TILE_WIDTH + y];
        let shift = (TILE_WIDTH - 1) - x;
        let bit0 = (plane0 >> shift) & 1;
        let bit1 = (plane1 >> shift) & 1;
        (bit1 << 1) | bit0
    }

    pub fn get_row(&self, y: u16) -> [u8; TILE_WIDTH] {
        let y = y as usize;
        let mut result = [0; TILE_WIDTH];
        let mut lo = self.data[y];
        let mut hi = self.data[y + TILE_WIDTH];
        for x in (0..8).rev() {
            result[x] = (1 & hi) << 1 | (1 & lo);
            hi >>= 1;
            lo >>= 1;
        }
        result
    }
}
