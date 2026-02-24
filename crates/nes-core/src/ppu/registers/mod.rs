pub mod address;
pub mod control;
pub mod mask;
pub mod scroll;
pub mod status;

pub use address::Address;
pub use control::Control;
pub use mask::Mask;
pub use scroll::Scroll;
pub use status::Status;

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
