use crate::cartridge::{mapper::Mapper, ChrBank};

pub struct Mapper0 {
    prg_rom: Vec<u8>,
    // chr_rom: Vec<u8>,
    chr_bank: ChrBank,
}

impl Mapper0 {
    const PRG_BEGIN: u16 = 0x8000;

    pub fn new(prg_rom: Vec<u8>, chr_bank: ChrBank) -> Self {
        Self { prg_rom, chr_bank }
    }
}

impl Mapper for Mapper0 {
    fn cpu_read(&self, address: u16) -> u8 {
        let mapped_address = if address < Mapper0::PRG_BEGIN {
            0
        } else {
            (address - Mapper0::PRG_BEGIN) as usize % self.prg_rom.len()
        };
        self.prg_rom[mapped_address]
    }

    fn cpu_write(&mut self, _address: u16, _value: u8) {}

    fn ppu_read(&self, address: u16) -> u8 {
        self.chr_bank.data[address as usize]
    }

    fn ppu_write(&mut self, _address: u16, _value: u8) {
        // let mapped_address = address as usize % self.chr_rom.len();
        // self.chr_rom[mapped_address] = value;
    }

    fn get_chr_tile(&self, index: usize) -> crate::cartridge::ChrTile {
        self.chr_bank.get_tile(index)
    }

    fn chr_tile_count(&self) -> usize {
        self.chr_bank.chr_tile_count()
    }
}
