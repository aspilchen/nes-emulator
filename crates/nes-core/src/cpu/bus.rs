use crate::apu;
use crate::cartridge::{Cartridge, ROM_BEGIN, ROM_END};
use crate::cpu::ram::{self, Ram};
use crate::ppu;
use apu::Apu;
use ppu::Ppu;

pub struct Bus<'a> {
    pub cart: &'a mut Cartridge,
    pub ram: &'a mut Ram,
    pub ppu: &'a mut Ppu,
    pub apu: &'a mut Apu,
}

enum Hardware {
    Cart,
    Ram,
    Ppu,
    Apu,
    NotImplemented,
}

impl<'a> Bus<'a> {
    pub fn read(&mut self, address: u16) -> u8 {
        let hardware = Hardware::from_address(address);
        match hardware {
            Hardware::Ram => self.ram.read(address),
            Hardware::Ppu => self.read_ppu(address),
            Hardware::Apu => self.apu.read(address),
            Hardware::Cart => self.cart.cpu_read(address),
            _ => 0,
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        let hardware = Hardware::from_address(address);
        match hardware {
            Hardware::Ram => self.ram.write(address, value),
            Hardware::Ppu => self.write_ppu(address, value),
            Hardware::Apu => self.apu.write(address, value),
            Hardware::Cart => self.cart.cpu_write(address, value),
            _ => {}
        }
    }

    fn read_ppu(&mut self, address: u16) -> u8 {
        self.ppu.read_register(
            address,
            &mut ppu::Bus {
                cart: self.cart,
                ram: self.ram,
            },
        )
    }

    fn write_ppu(&mut self, address: u16, value: u8) {
        self.ppu.write_register(
            address,
            value,
            &mut ppu::Bus {
                cart: self.cart,
                ram: self.ram,
            },
        );
    }
}

impl Hardware {
    pub fn from_address(address: u16) -> Self {
        match address {
            ram::BEGIN..=ram::END => Self::Ram,
            ppu::REGISTERS_BEGIN..=ppu::REGISTERS_END => Self::Ppu,
            ppu::OAM_DMA => Self::Ppu,
            ROM_BEGIN..=ROM_END => Self::Cart,
            apu::ENABLE_LEN => Self::Apu,
            apu::FRAME_COUNTER => Self::Apu,
            _ => Self::NotImplemented,
        }
    }
}
