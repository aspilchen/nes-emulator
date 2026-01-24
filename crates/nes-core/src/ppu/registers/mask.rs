use bitflags::bitflags;

bitflags! {
    #[derive(Clone)]
    pub struct Mask: u8 {
        const GRAYSCALE           = 0b0000_0001;
        const SHOW_BG_LEFT_8      = 0b0000_0010;
        const SHOW_SPRITES_LEFT_8 = 0b0000_0100;
        const SHOW_BG             = 0b0000_1000;
        const SHOW_SPRITES        = 0b0001_0000;
        const EMPHASIZE_RED       = 0b0010_0000;
        const EMPHASIZE_GREEN     = 0b0100_0000;
        const EMPHASIZE_BLUE      = 0b1000_0000;
    }
}

impl Default for Mask {
    fn default() -> Self {
        Self::from_bits_truncate(0)
    }
}
