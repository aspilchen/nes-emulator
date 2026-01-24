use bitflags::bitflags;

bitflags! {
    #[derive(Clone)]
    pub struct Status: u8 {
        const SPRITE_OVERFLOW = 0b0010_0000;
        const SPRITE_0_HIT    = 0b0100_0000;
        const VBLANK_STARTED  = 0b1000_0000;
    }
}

impl Default for Status {
    fn default() -> Self {
        Self::from_bits_truncate(0)
    }
}
