use crate::cartridge::ines::InesRom;
use crate::cartridge::mapper::{mapper0::Mapper0, Mapper, MapperType};

pub struct Cartridge {
    mapper: Box<dyn Mapper>,
}

impl Cartridge {
    pub fn new(rom: &[u8]) -> Result<Self, String> {
        let ines_rom = InesRom::new(rom)?;
        match ines_rom.header.mapper {
            MapperType::Mapper0 => {
                let mapper = Box::new(Mapper0::new(ines_rom));
                Ok(Self { mapper })
            }
            MapperType::Unsupported(id) => Err(format!("unsupported mapper id {}", id)),
        }
    }

    pub fn cpu_read(&mut self, address: u16) -> u8 {
        self.mapper.cpu_read(address)
    }

    pub fn cpu_write(&mut self, address: u16, value: u8) {
        self.mapper.cpu_write(address, value);
    }
}
