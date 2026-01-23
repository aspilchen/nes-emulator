use bitflags::bitflags;

use crate::cartridge::{cartridge::Mirroring, mapper::MapperType};

#[derive(Clone)]
pub struct InesHeader {
    pub ascii: Vec<u8>,
    pub prg_banks: u8,
    pub chr_banks: u8,
    pub flags6: Flags6,
    pub flags7: Flags7,
}

bitflags! {
    #[derive(Clone, Copy)]
    pub struct Flags6: u8 {
        const MIRROR_VERTICAL = 0b0000_0001;
        const BATTERY         = 0b0000_0010;
        const TRAINER         = 0b0000_0100;
        const FOUR_SCREEN     = 0b0000_1000;
        const MAPPER_LOWER    = 0b1111_0000;
    }
}

bitflags! {
    #[derive(Clone, Copy)]
    pub struct Flags7: u8 {
        const VS_UNISYSTEM       = 0b0000_0001;
        const PLAYCHOICE_10      = 0b0000_0010;
        const NES2               = 0b0000_1100;
        const MAPPER_UPPER       = 0b1111_0000;
    }
}

impl InesHeader {
    pub fn parse(data: &[u8]) -> Result<Self, String> {
        if data.len() < 16 {
            return Err("header".into());
        }

        let ascii = data[0..4].to_vec();
        let prg_banks = data[4];
        let chr_banks = data[5];

        if ascii != b"NES\x1A" {
            return Err("header".into());
        }
        Ok(Self {
            ascii,
            prg_banks,
            chr_banks,
            flags6: Flags6::from_bits_truncate(data[6]),
            flags7: Flags7::from_bits_truncate(data[7]),
        })
    }

    pub fn get_mapper_type(&self) -> MapperType {
        let lower = (self.flags6 & Flags6::MAPPER_LOWER).bits();
        let upper = (self.flags7 & Flags7::MAPPER_UPPER).bits() << 4;
        MapperType::from_id(upper + lower)
    }

    pub fn get_mirroring(&self) -> Mirroring {
        if self.flags6.contains(Flags6::FOUR_SCREEN) {
            Mirroring::FourScreen
        } else if self.flags6.contains(Flags6::MIRROR_VERTICAL) {
            Mirroring::Vertical
        } else {
            Mirroring::Horizontal
        }
    }
}
