use core::panic;
use std::ops::RangeInclusive;

use crate::{cartridge::cartridge::Mirroring, ppu::registers::Address};

pub const BEGIN: u16 = 0x2000;
pub const END: u16 = 0x3EFF;
pub const NAMETABLE_BEGIN: u16 = 0x2000;
pub const NAMETABLE_END: u16 = 0x23BF;
pub const ATTRIBUTES_BEGIN: u16 = 0x23C0;
pub const ATTRIBUTES_END: u16 = 0x23FF;
pub const TILES_PER_ROW: u16 = 32;

const SIZE: usize = 0x800;
const NAMETABLE_SIZE: u16 = 0x400;

pub struct VRam {
    pub data: [u8; SIZE],
    mirroring: Mirroring,
}

pub struct NameTableEntry {
    pub chr_index: u16,
    pub x: u16,
    pub y: u16,
}

pub struct TileAttributes {
    pub value: u8,
}

impl VRam {
    pub fn new() -> Self {
        Self {
            data: [0; SIZE],
            mirroring: Mirroring::Unsupported,
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

    pub fn reset(&mut self, mirroring: Mirroring) {
        self.data = [0; SIZE];
        self.mirroring = mirroring;
    }

    pub fn get_nametable_entry(&self, nametable_index: u16) -> NameTableEntry {
        let address = nametable_index + NAMETABLE_BEGIN;
        let value = self.read(address);
        NameTableEntry::new(address, value)
    }

    pub fn get_attributes(&self, tile_x: u16, tile_y: u16) -> TileAttributes {
        let address = tile_y / 4 * 8 + tile_x / 4;
        let value = self.read(ATTRIBUTES_BEGIN + address);
        TileAttributes { value }
    }

    fn map_address(&self, address: u16) -> usize {
        let address = self.mirror_down(address) - BEGIN;
        let name_table = address / NAMETABLE_SIZE;
        let result = match (&self.mirroring, name_table) {
            (Mirroring::Vertical, 2) | (Mirroring::Vertical, 3) => address - 0x800,
            (Mirroring::Horizontal, 2) => address - 0x400,
            (Mirroring::Horizontal, 1) => address - 0x400,
            (Mirroring::Horizontal, 3) => address - 0x800,
            _ => address,
        };
        result as usize
    }

    fn mirror_down(&self, address: u16) -> u16 {
        address & 0b10111111111111
    }
}

impl NameTableEntry {
    pub fn new(index: u16, value: u8) -> Self {
        Self {
            chr_index: value as u16,
            x: (index - BEGIN) % TILES_PER_ROW,
            y: (index - BEGIN) / TILES_PER_ROW,
        }
    }
}

impl TileAttributes {
    pub fn palette_index(&self, tile_x: u16, tile_y: u16) -> u16 {
        let shift = match (tile_x % 4 / 2, tile_y % 4 / 2) {
            (0, 0) => 0,
            (1, 0) => 2,
            (0, 1) => 4,
            (1, 1) => 6,
            (_, _) => unreachable!(),
        };
        (self.value as u16 >> shift) & 0b0000_0011
    }
}
