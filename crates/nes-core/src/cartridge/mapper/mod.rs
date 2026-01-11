mod mapper0;
use crate::cartridge::{mapper::mapper0::Mapper0, ChrTile, InesRom};

pub trait Mapper {
    fn cpu_read(&self, address: u16) -> u8;
    fn cpu_write(&mut self, address: u16, value: u8);
    fn ppu_read(&self, address: u16) -> u8;
    fn ppu_write(&mut self, address: u16, value: u8);
    fn get_chr_tile(&self, index: u16) -> ChrTile;
    fn chr_tile_count(&self) -> usize;
}

#[derive(Debug)]
pub enum MapperType {
    Mapper0,
    Unsupported(u8),
}

#[derive(Debug)]
pub enum MapperError {
    UnsupportedMapper(MapperType),
}

impl MapperType {
    pub fn from_id(id: u8) -> Self {
        match id {
            0 => MapperType::Mapper0,
            _ => MapperType::Unsupported(id),
        }
    }
}

// pub fn from_type(
//     mapper_type: MapperType,
//     prg_rom: Vec<u8>,
//     chr_rom: Vec<u8>,
// ) -> Result<Box<dyn Mapper>, MapperError> {
//     match mapper_type {
//         MapperType::Mapper0 => Ok(Box::new(Mapper0::new(prg_rom, chr_rom))),
//         _ => Err(MapperError::UnsupportedMapper(mapper_type)),
//     }
// }

pub fn from_ines(ines: InesRom) -> Result<Box<dyn Mapper>, MapperError> {
    let mapper_type = ines.header.get_mapper_type();
    match mapper_type {
        MapperType::Mapper0 => Ok(Box::new(Mapper0::new(ines.prg_rom, ines.chr_rom))),
        _ => Err(MapperError::UnsupportedMapper(mapper_type)),
    }
}
