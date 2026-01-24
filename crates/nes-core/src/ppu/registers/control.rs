use bitflags::bitflags;

bitflags! {
    #[derive(Clone)]
    pub struct Control: u8 {
        const NAMETABLE_0      = 0b0000_0001;
        const NAMETABLE_1      = 0b0000_0010;
        const VRAM_INC_32      = 0b0000_0100;
        const SPRITE_TABLE_1000= 0b0000_1000;
        const BG_TABLE_1000    = 0b0001_0000;
        const SPRITE_8X16      = 0b0010_0000;
        const MASTER_SLAVE     = 0b0100_0000;
        const NMI_ON_VBLANK    = 0b1000_0000;
    }
}

impl Default for Control {
    fn default() -> Self {
        Self::from_bits_truncate(0)
    }
}

impl Control {
    pub fn bg_pattern_table_base(&self) -> u16 {
        if self.contains(Self::BG_TABLE_1000) {
            0x1000
        } else {
            0
        }
    }

    pub fn sprite_pattern_table_base(&self) -> u16 {
        if self.contains(Self::SPRITE_TABLE_1000) {
            0x0100
        } else {
            0
        }
    }
}
