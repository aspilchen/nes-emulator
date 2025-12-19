pub mod mapper0;

pub enum MapperType {
    Mapper0,
    Unsupported(u8),
}

impl MapperType {
    pub fn from_id(id: u8) -> Self {
        match id {
            0 => MapperType::Mapper0,
            _ => MapperType::Unsupported(id),
        }
    }
}

pub trait Mapper {
    fn cpu_read(&mut self, address: u16) -> u8;
    fn cpu_write(&mut self, address: u16, value: u8);
    fn ppu_read(&mut self, address: u16) -> u8;
    fn ppu_write(&mut self, address: u16, value: u8);
}
