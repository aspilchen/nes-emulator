use crate::ppu::oam::{OamEntry, SpriteAttr};

const TOTAL_BYTES: usize = 32;
const BACKGROUND_BYTES: usize = 16;
const SPRITE_BYTES: usize = 16;
const BACKGROUND_BEGIN: usize = 0x3F00;
const BACKGROUND_END: usize = 0x3F0F;
const SPRITES_BEGIN: usize = 0x3F10;
const SPRITES_END: usize = 0x3F1F;
const MIRROR_MASK: usize = 0x1F;

const MASTER_PALETTE: [(u8, u8, u8); 64] = [
    (124, 124, 124),
    (0, 0, 252),
    (0, 0, 188),
    (68, 40, 188),
    (148, 0, 132),
    (168, 0, 32),
    (168, 16, 0),
    (136, 20, 0),
    (80, 48, 0),
    (0, 120, 0),
    (0, 104, 0),
    (0, 88, 0),
    (0, 64, 88),
    (0, 0, 0),
    (0, 0, 0),
    (0, 0, 0),
    (188, 188, 188),
    (0, 120, 248),
    (0, 88, 248),
    (104, 68, 252),
    (216, 0, 204),
    (228, 0, 88),
    (248, 56, 0),
    (228, 92, 16),
    (172, 124, 0),
    (0, 184, 0),
    (0, 168, 0),
    (0, 168, 68),
    (0, 136, 136),
    (0, 0, 0),
    (0, 0, 0),
    (0, 0, 0),
    (248, 248, 248),
    (60, 188, 252),
    (104, 136, 252),
    (152, 120, 248),
    (248, 120, 248),
    (248, 88, 152),
    (248, 120, 88),
    (252, 160, 68),
    (248, 184, 0),
    (184, 248, 24),
    (88, 216, 84),
    (88, 248, 152),
    (0, 232, 216),
    (120, 120, 120),
    (0, 0, 0),
    (0, 0, 0),
    (252, 252, 252),
    (164, 228, 252),
    (184, 184, 248),
    (216, 184, 248),
    (248, 184, 248),
    (248, 164, 192),
    (240, 208, 176),
    (252, 224, 168),
    (248, 216, 120),
    (216, 248, 120),
    (184, 248, 184),
    (184, 248, 216),
    (0, 252, 252),
    (248, 216, 248),
    (0, 0, 0),
    (0, 0, 0),
];

pub struct Palette {
    // data: [u8; ],
    background: [u8; BACKGROUND_BYTES],
    sprites: [u8; SPRITE_BYTES],
}

pub struct PaletteEntry {
    pub color0: u8,
    pub color1: u8,
    pub color2: u8,
    pub color3: u8,
}

enum PaletteAddress {
    Backgound(usize),
    Sprite(usize),
}

impl Palette {
    pub fn new() -> Self {
        Self {
            background: [0; BACKGROUND_BYTES],
            sprites: [0; SPRITE_BYTES],
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        let address = PaletteAddress::from_address(address);
        match address {
            PaletteAddress::Backgound(addr) => self.background[addr],
            PaletteAddress::Sprite(addr) => self.sprites[addr],
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        let address = PaletteAddress::from_address(address);
        match address {
            PaletteAddress::Backgound(addr) => self.background[addr] = value,
            PaletteAddress::Sprite(addr) => self.sprites[addr] = value,
        };
    }

    pub fn get_sprite_color(&self, pixel_bits: u8, palette_index: u8) {
        // let index: u8 = (sprite.attr & SpriteAttr::PALETTE_MASK).bits();
        // let entry = self.get_sprite_entry(index);
    }

    fn get_background_entry(&self, index: u8) -> PaletteEntry {
        let index = index as usize * 4;
        let mut entry = PaletteEntry::from_bytes(&self.background[index..index + 4]);
        entry.color0 = self.get_universal_background();
        entry
    }

    fn get_sprite_entry(&self, index: u8) -> PaletteEntry {
        let index = index as usize * 4;
        let mut entry = PaletteEntry::from_bytes(&self.sprites[index..index + 4]);
        entry.color0 = self.get_universal_background();
        entry
    }

    fn get_universal_background(&self) -> u8 {
        self.background[0]
    }
}

impl PaletteEntry {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            color0: bytes[0],
            color1: bytes[1],
            color2: bytes[2],
            color3: bytes[3],
        }
    }
}

impl PaletteAddress {
    pub fn from_address(address: u16) -> Self {
        let address = PaletteAddress::mirror_down(address);
        match address {
            BACKGROUND_BEGIN..=BACKGROUND_END => PaletteAddress::Backgound(address),
            SPRITES_BEGIN..=SPRITES_END => PaletteAddress::Sprite(address),
            _ => panic!("palette error"),
        }
    }

    fn mirror_down(address: u16) -> usize {
        MIRROR_MASK & address as usize
    }
}
