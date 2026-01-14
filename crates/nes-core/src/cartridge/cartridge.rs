use crate::cartridge::mapper;
use crate::cartridge::{ChrTile, InesHeader, InesRom, Mapper};

// use crate::cartridge::ines::InesRom;

// use crate::cartridge::mapper::;

pub const ROM_BEGIN: u16 = 0x4020;
pub const ROM_END: u16 = 0xFFFF;
pub const CHR_BEGIN: u16 = 0;
pub const CHR_END: u16 = 0x1FFF;

pub struct Cartridge {
    mapper: Box<dyn Mapper>,
    header: InesHeader,
}

pub enum Mirroring {
    Horizontal,
    Vertical,
    OneScreen,
    FourScreen,
    Unsupported,
}

impl Cartridge {
    pub fn new(rom: &[u8]) -> Result<Self, String> {
        let ines_rom = InesRom::new(rom)?;
        let header = ines_rom.header.clone();
        let mapper: Box<dyn Mapper> = mapper::from_ines(ines_rom).expect("Mapper error");
        Ok(Self { mapper, header })
    }

    pub fn cpu_read(&self, address: u16) -> u8 {
        self.mapper.cpu_read(address)
    }

    pub fn cpu_write(&mut self, address: u16, value: u8) {
        self.mapper.cpu_write(address, value);
    }

    pub fn ppu_read(&self, address: u16) -> u8 {
        self.mapper.ppu_read(address)
    }

    pub fn ppu_write(&mut self, address: u16, value: u8) {
        self.mapper.ppu_write(address, value);
    }

    pub fn get_mirroring(&self) -> Mirroring {
        self.header.get_mirroring()
    }

    pub fn get_chr_tile(&self, index: u16) -> ChrTile {
        self.mapper.get_chr_tile(index)
    }

    pub fn chr_tile_count(&self) -> usize {
        self.mapper.chr_tile_count()
    }

    pub fn reset_vector(&self) -> u16 {
        let mut bytes = [0, 0];
        let address = 0xFFFC;
        bytes[0] = self.mapper.cpu_read(address);
        bytes[1] = self.mapper.cpu_read(address + 1);
        u16::from_le_bytes(bytes)
    }

    pub fn nmi_vector(&self) -> u16 {
        let mut bytes = [0, 0];
        let address = 0xFFFA;
        bytes[0] = self.mapper.cpu_read(address);
        bytes[1] = self.mapper.cpu_read(address + 1);
        u16::from_le_bytes(bytes)
    }

    pub fn brk_vector(&self) -> u16 {
        let mut bytes = [0, 0];
        let address = 0xFFFE;
        bytes[0] = self.mapper.cpu_read(address);
        bytes[1] = self.mapper.cpu_read(address + 1);
        u16::from_le_bytes(bytes)
    }
}
