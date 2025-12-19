use crate::cartridge::{ines::InesRom, mapper::Mapper};

pub struct Mapper0 {
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
}

impl Mapper0 {
    const PRG_BEGIN: u16 = 0x8000;

    pub fn new(rom: InesRom) -> Self {
        Self {
            prg_rom: rom.prg_rom,
            chr_rom: rom.chr_rom,
        }
    }
}

impl Mapper for Mapper0 {
    fn cpu_read(&mut self, address: u16) -> u8 {
        let mapped_address = if address < Mapper0::PRG_BEGIN {
            0
        } else {
            (address - Mapper0::PRG_BEGIN) as usize % self.prg_rom.len()
        };
        self.prg_rom[mapped_address]
    }

    fn cpu_write(&mut self, _address: u16, _value: u8) {}

    fn ppu_read(&mut self, address: u16) -> u8 {
        self.chr_rom[address as usize]
    }

    fn ppu_write(&mut self, address: u16, value: u8) {
        let mapped_address = address as usize % self.chr_rom.len();
        self.chr_rom[mapped_address] = value;
    }
}
