use crate::cartridge::{Cartridge, ROM_BEGIN, ROM_END};
use crate::cpu::ram::{self, Ram};
use crate::input::JoyPad;
use crate::ppu;
use crate::{apu, input};
use apu::Apu;
use ppu::Ppu;

pub struct Bus<'a> {
    pub cart: &'a mut Cartridge,
    pub ram: &'a mut Ram,
    pub ppu: &'a mut Ppu,
    pub apu: &'a mut Apu,
    pub input_1: &'a mut JoyPad,
}

enum Hardware {
    Cart,
    Ram,
    Ppu,
    Apu,
    Input1,
    Input2,
    NotImplemented,
}

impl<'a> Bus<'a> {
    pub fn read(&mut self, address: u16) -> u8 {
        let hardware = Hardware::from(address, 'r');
        match hardware {
            Hardware::Ram => self.ram.read(address),
            Hardware::Ppu => self.read_ppu(address),
            Hardware::Apu => self.apu.read(address),
            Hardware::Cart => self.cart.cpu_read(address),
            Hardware::Input1 => self.input_1.read(),
            _ => 0,
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        let hardware = Hardware::from(address, 'w');
        match hardware {
            Hardware::Ram => self.ram.write(address, value),
            Hardware::Ppu => self.write_ppu(address, value),
            Hardware::Apu => self.apu.write(address, value),
            Hardware::Cart => self.cart.cpu_write(address, value),
            Hardware::Input1 => self.input_1.write(value),
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
    pub fn from(address: u16, read_write: char) -> Self {
        match (address, read_write) {
            (ram::BEGIN..=ram::END, _) => Self::Ram,
            (ppu::REGISTERS_BEGIN..=ppu::REGISTERS_END, _) => Self::Ppu,
            (ppu::OAM_DMA, _) => Self::Ppu,
            (ROM_BEGIN..=ROM_END, _) => Self::Cart,
            (apu::ENABLE_LEN, _) => Self::Apu,
            (apu::FRAME_COUNTER, 'w') => Self::Apu,
            (input::CONTROLLER_1, _) => Hardware::Input1,
            (input::CONTROLLER_2, 'r') => Hardware::Input2,
            _ => Self::NotImplemented,
        }
    }
}
