use std::collections::VecDeque;

use bitflags::bitflags;

use crate::{cartridge::TILE_HEIGHT, cpu::bus};

pub const ENTRY_SIZE: usize = 4;
pub const SIZE: usize = 256;
pub const NUM_ENTRIES: usize = SIZE / ENTRY_SIZE;
pub const SECONDARY_SIZE: usize = 64;
pub const SECONDARY_MAX_ENTRIES: usize = 8;

const FLIP: u16 = 7;

pub struct Oam {
    pub address: u8,
    data: [u8; SIZE],
}

#[derive(Clone, Copy)]
pub struct OamEntry {
    pub index: usize,
    pub y: u8,
    pub tile: u8,
    pub attribute: SpriteAttribute,
    pub x: u8,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct SpriteAttribute: u8 {
        const PALETTE_MASK       = 0b0000_0011;
        const PRIORITY_BEHIND_BG = 0b0010_0000;
        const FLIP_H             = 0b0100_0000;
        const FLIP_V             = 0b1000_0000;
    }
}

impl Oam {
    pub fn new() -> Self {
        Self {
            address: 0,
            data: [0xFF; SIZE],
        }
    }

    pub fn read(&mut self) -> u8 {
        let value = self.data[self.address as usize];
        value
    }

    pub fn write(&mut self, value: u8) {
        self.data[self.address as usize] = value;
        self.address = self.address.wrapping_add(1);
    }

    pub fn scan(&mut self, scanline: u16) -> Vec<OamEntry> {
        let mut result = Vec::new();
        for i in 0..NUM_ENTRIES {
            let entry = self.get_entry(i);
            let top = entry.top();
            if scanline >= top && scanline < top + 8 {
                result.push(entry);
            }
            if result.len() >= SECONDARY_MAX_ENTRIES {
                break;
            }
        }
        result
    }

    pub fn dma_transfer(&mut self, buffer: &[u8; 256]) {
        self.address = 0;
        self.data.copy_from_slice(buffer);
    }

    pub fn sprite_zero(&self) -> OamEntry {
        self.get_entry(0)
    }

    fn get_entry(&self, index: usize) -> OamEntry {
        let base = index * 4;
        OamEntry::from_bytes(index, &self.data[base..base + 4])
    }
}

impl OamEntry {
    pub fn from_bytes(index: usize, bytes: &[u8]) -> Self {
        Self {
            index,
            y: bytes[0],
            tile: bytes[1],
            attribute: SpriteAttribute::from_bits_truncate(bytes[2]),
            x: bytes[3],
        }
    }

    pub fn top(&self) -> u16 {
        self.y.wrapping_add(1) as u16
    }

    pub fn palette_index(&self) -> u16 {
        (self.attribute & SpriteAttribute::PALETTE_MASK).bits() as u16
    }
}
