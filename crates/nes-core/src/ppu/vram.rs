use crate::cartridge::cartridge::Mirroring;

pub const BEGIN: u16 = 0x2000;
pub const END: u16 = 0x3EFF;
pub const READ_END: u16 = 0x23C0;

const SIZE: usize = 0x800;
const MIRROR_OFFSET: u16 = 0x1000;
const MIRROR_BEGIN: u16 = 0x3000;
const MIRROR_END: u16 = 0x3EFF;
const NAMETABLE_ADDRESS_REGION: u16 = 0x1000;
const NAMETABLE_SIZE: u16 = 0x0400;

pub struct VRam {
    pub data: [u8; SIZE],
    mirroring: Mirroring,
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

    fn map_address(&self, address: u16) -> usize {
        let mirrored_vram = address & 0b10111111111111; // mirror down 0x3000-0x3eff to 0x2000 - 0x2eff
        let vram_index = mirrored_vram - 0x2000; // to vram vector
        let name_table = vram_index / 0x400;
        let result = match (&self.mirroring, name_table) {
            (Mirroring::Vertical, 2) | (Mirroring::Vertical, 3) => vram_index - 0x800,
            (Mirroring::Horizontal, 2) => vram_index - 0x400,
            (Mirroring::Horizontal, 1) => vram_index - 0x400,
            (Mirroring::Horizontal, 3) => vram_index - 0x800,
            _ => vram_index,
        };
        result as usize
        // let address = self.mirror_down(address);
        // let normalized = (address - BEGIN) % NAMETABLE_ADDRESS_REGION;
        // match self.mirroring {
        //     Mirroring::Vertical => normalized as usize % SIZE,
        //     Mirroring::Horizontal => {
        //         let table = normalized / NAMETABLE_SIZE;
        //         let offset = normalized % NAMETABLE_SIZE;
        //         match table {
        //             0 | 2 => offset as usize,
        //             1 | 3 => (offset % NAMETABLE_SIZE) as usize,
        //             _ => panic!("Something went wrong"),
        //         }
        //     }
        //     Mirroring::FourScreen => normalized as usize,
        //     _ => todo!("Unsupported mirroring type"),
        // }
    }

    fn mirror_down(&self, address: u16) -> u16 {
        match address {
            // VRAM_BEGIN..=VRAM_END => address,
            MIRROR_BEGIN..=MIRROR_END => address - MIRROR_OFFSET,
            _ => address,
        }
    }
}
