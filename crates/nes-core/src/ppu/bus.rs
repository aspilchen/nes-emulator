use crate::cartridge::{Cartridge, ChrTile};
use crate::cpu::ram::Ram;

pub struct Bus<'a> {
    pub cart: &'a mut Cartridge,
    pub ram: &'a mut Ram,
}

impl<'a> Bus<'a> {
    pub fn read_chr(&self, address: u16) -> u8 {
        self.cart.ppu_read(address)
    }

    pub fn write_chr(&mut self, address: u16, value: u8) {
        self.cart.ppu_write(address, value);
    }

    pub fn get_chr_tile(&'_ self, index: u16) -> ChrTile<'_> {
        self.cart.get_chr_tile(index)
    }

    pub fn dma_read(&self, address: u16) -> u8 {
        self.ram.read(address)
    }
}
