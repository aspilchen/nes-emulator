use crate::{
    cartridge::cartridge::Mirroring,
    ppu::ppu::{VRAM_BEGIN, VRAM_END},
};

const VRAM_SIZE: usize = 0x800;
const MIRROR_OFFSET: u16 = 0x1000;
const MIRROR_BEGIN: u16 = 0x3000;
const MIRROR_END: u16 = 0x3EFF;
const NAMETABLE_ADDRESS_REGION: u16 = 0x1000;
const NAMETABLE_SIZE: u16 = 0x0400;

pub struct VRam {
    data: [u8; VRAM_SIZE],
    mirroring: Mirroring,
}

impl VRam {
    pub fn new() -> Self {
        Self {
            data: [0; VRAM_SIZE],
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
        self.data = [0; VRAM_SIZE];
        self.mirroring = mirroring;
    }

    fn map_address(&self, address: u16) -> usize {
        let address = self.mirror_down(address);
        let normalized = (address - VRAM_BEGIN) % NAMETABLE_ADDRESS_REGION;
        match self.mirroring {
            Mirroring::Vertical => normalized as usize % VRAM_SIZE,
            Mirroring::Horizontal => {
                let table = normalized / NAMETABLE_SIZE;
                let offset = normalized % NAMETABLE_SIZE;
                match table {
                    0 | 2 => offset as usize,
                    1 | 3 => (offset % NAMETABLE_SIZE) as usize,
                    _ => panic!("Something went wrong"),
                }
            }
            Mirroring::FourScreen => normalized as usize,
            _ => todo!("Unsupported mirroring type"),
        }
    }

    fn mirror_down(&self, address: u16) -> u16 {
        match address {
            // VRAM_BEGIN..=VRAM_END => address,
            MIRROR_BEGIN..=MIRROR_END => address - MIRROR_OFFSET,
            _ => address,
        }
    }
}
