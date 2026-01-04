use bitflags::bitflags;

pub const ENTRY_SIZE: usize = 4;
pub const PRIMARY_SIZE: usize = 256;
pub const PRIMARY_NUM_ENTRIES: usize = PRIMARY_SIZE / ENTRY_SIZE;
pub const SECONDARY_SIZE: usize = 64;
pub const SECONDARY_MAX_ENTRIES: usize = SECONDARY_SIZE / ENTRY_SIZE;

const FLIP: u16 = 7;

pub struct OamData {
    pub address: u8,
    pub secondary: Vec<OamEntry>,
    primary: [u8; PRIMARY_SIZE],
}

#[derive(Clone, Copy)]
pub struct OamEntry {
    pub y: u8,
    pub tile: u8,
    pub attr: SpriteAttr,
    pub x: u8,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct SpriteAttr: u8 {
        const PALETTE_MASK       = 0b0000_0011;
        const PRIORITY_BEHIND_BG = 0b0010_0000;
        const FLIP_H             = 0b0100_0000;
        const FLIP_V             = 0b1000_0000;
    }
}

impl OamData {
    pub fn new() -> Self {
        Self {
            address: 0,
            secondary: Vec::with_capacity(SECONDARY_MAX_ENTRIES),
            primary: [0; PRIMARY_SIZE],
        }
    }

    pub fn read(&mut self) -> u8 {
        let value = self.primary[self.address as usize];
        value
    }

    pub fn write(&mut self, value: u8) {
        self.primary[self.address as usize] = value;
        self.address = self.address.wrapping_add(1);
    }

    pub fn scan(&mut self, scanline_y: u16) -> bool {
        self.secondary.clear();
        for i in 0..PRIMARY_NUM_ENTRIES {
            let entry = self.get_entry(i);
            if entry.top() == scanline_y {
                if self.secondary.len() < SECONDARY_MAX_ENTRIES {
                    self.secondary.push(entry);
                } else {
                    return true;
                }
            }
        }
        false
    }

    fn get_entry(&self, index: usize) -> OamEntry {
        let base = index * 4;
        OamEntry::from_bytes(&self.primary[base..base + 4])
    }
}

impl OamEntry {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            y: bytes[0],
            tile: bytes[1],
            attr: SpriteAttr::from_bits_truncate(bytes[2]),
            x: bytes[3],
        }
    }

    pub fn top(&self) -> u16 {
        self.y.wrapping_add(1) as u16
    }

    pub fn tile_y(&self, pixel_y: u16) -> u8 {
        if self.attr.contains(SpriteAttr::FLIP_V) {
            (FLIP - (pixel_y - self.y as u16)) as u8
        } else {
            (pixel_y - self.y as u16) as u8
        }
    }

    pub fn tile_x(&self, pixel_x: u16) -> u8 {
        if self.attr.contains(SpriteAttr::FLIP_H) {
            (FLIP - (pixel_x - self.x as u16)) as u8
        } else {
            (pixel_x - self.x as u16) as u8
        }
    }
}
