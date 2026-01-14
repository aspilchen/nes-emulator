use bitflags::bitflags;

const CTRL: u16 = 0;
const MASK: u16 = 1;
const STATUS: u16 = 2;
const OAM_ADDR: u16 = 3;
const OAM_DATA: u16 = 4;
const SCROLL: u16 = 5;
const ADDR: u16 = 6;
const DATA: u16 = 7;
pub const OAM_DMA: u16 = 0x4014;
pub const REGISTERS_BEGIN: u16 = 0x2000;
pub const REGISTERS_END: u16 = 0x3FFF;

pub enum RegisterName {
    Control,
    Mask,
    Status,
    OamAddress,
    OamData,
    Scroll,
    Address,
    Data,
    OamDma,
}

bitflags! {
    #[derive(Clone)]
    pub struct PpuControl: u8 {
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

bitflags! {
    #[derive(Clone)]
    pub struct PpuMask: u8 {
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

bitflags! {
    #[derive(Clone)]
    pub struct PpuStatus: u8 {
        const SPRITE_OVERFLOW = 0b0010_0000;
        const SPRITE_0_HIT    = 0b0100_0000;
        const VBLANK_STARTED  = 0b1000_0000;
    }
}

pub struct Address {
    bytes: [u8; 2],
    latch: usize,
}

impl Default for PpuControl {
    fn default() -> Self {
        Self::from_bits_truncate(0)
    }
}

impl Default for PpuMask {
    fn default() -> Self {
        Self::from_bits_truncate(0)
    }
}

impl Default for PpuStatus {
    fn default() -> Self {
        Self::from_bits_truncate(0)
    }
}

impl PpuControl {
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

impl Address {
    pub fn new() -> Self {
        Self {
            bytes: [0, 0],
            latch: 0,
        }
    }

    pub fn read(&self) -> u16 {
        u16::from_be_bytes(self.bytes)
    }

    pub fn write(&mut self, value: u8) {
        self.bytes[self.latch] = value;
        self.latch = (self.latch + 1) % 2;
    }

    pub fn increment(&mut self, step_size: u16) {
        let mut address = self.read();
        address = address.wrapping_add(step_size);
        self.bytes = address.to_be_bytes();
    }

    pub fn reset_latch(&mut self) {
        self.latch = 0;
    }
}

impl RegisterName {
    pub fn from_address(address: u16) -> Option<Self> {
        let address = Self::mirror_down(address);
        match address {
            CTRL => Some(Self::Control),
            MASK => Some(Self::Mask),
            STATUS => Some(Self::Status),
            OAM_ADDR => Some(Self::OamAddress),
            OAM_DATA => Some(Self::OamData),
            SCROLL => Some(Self::Scroll),
            ADDR => Some(Self::Address),
            DATA => Some(Self::Data),
            OAM_DMA => Some(Self::OamDma),
            _ => None,
        }
    }

    fn mirror_down(address: u16) -> u16 {
        match address {
            REGISTERS_BEGIN..=REGISTERS_END => address % 8,
            _ => address,
        }
    }
}
