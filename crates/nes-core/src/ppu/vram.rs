use crate::{cartridge::cartridge::Mirroring, ppu::registers::address};

pub const BEGIN: u16 = 0x2000;
pub const END: u16 = 0x3EFF;
pub const NAMETABLE_BEGIN: u16 = 0x2000;
pub const NAMETABLE_END: u16 = 0x23BF;
pub const ATTRIBUTES_BEGIN: u16 = 0x23C0;
pub const ATTRIBUTES_END: u16 = 0x23FF;
pub const TILES_PER_ROW: u16 = 32;
pub const TILES_PER_COLUMN: u16 = 30;
pub const ATTRIBUTES_OFFSET: u16 = 0x3C0;

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
    pub palette_index: u16,
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

    pub fn get_nametable_entry(
        &self,
        mut nametable: u16,
        mut x: u16,
        mut y: u16,
    ) -> NameTableEntry {
        if y >= TILES_PER_COLUMN {
            y -= TILES_PER_COLUMN;
            nametable += NAMETABLE_SIZE * 2;
        }
        if x >= TILES_PER_ROW {
            x -= TILES_PER_ROW;
            nametable += NAMETABLE_SIZE;
        }
        let offset = (y * TILES_PER_ROW) + x;
        let address = nametable + offset;
        let chr_index = self.read(address) as u16;
        let attributes = self.get_attributes(nametable, x, y);
        NameTableEntry {
            chr_index,
            x,
            y,
            palette_index: attributes.palette_index(x, y),
        }
    }

    pub fn get_attributes(&self, nametable: u16, x: u16, y: u16) -> TileAttributes {
        let offset = (y / 4) * 8 + (x / 4);
        let address = nametable + ATTRIBUTES_OFFSET + offset;
        let value = self.read(address);
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

impl TileAttributes {
    pub fn palette_index(&self, x: u16, y: u16) -> u16 {
        let shift = match (x % 4 / 2, y % 4 / 2) {
            (0, 0) => 0,
            (1, 0) => 2,
            (0, 1) => 4,
            (1, 1) => 6,
            (_, _) => unreachable!(),
        };
        (self.value as u16 >> shift) & 0b0000_0011
    }
}
