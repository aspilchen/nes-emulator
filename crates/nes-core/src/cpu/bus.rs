use crate::apu;
use crate::cartridge::{Cartridge, ROM_BEGIN, ROM_END};
use crate::cpu::ram::{Ram, RAM_BEGIN, RAM_END};
use crate::ppu;
use apu::Apu;
use ppu::Ppu;

pub struct Bus<'a> {
    pub cart: &'a mut Cartridge,
    pub ram: &'a mut Ram,
    pub ppu: &'a mut Ppu,
    pub apu: &'a mut Apu,
}

impl<'a> Bus<'a> {
    pub fn read(&mut self, address: u16) -> u8 {
        match address {
            RAM_BEGIN..=RAM_END => self.ram.read(address),
            ppu::REGISTERS_BEGIN..=ppu::REGISTERS_END => self.read_ppu(address),
            apu::VOICE_BEGIN..=apu::VOICE_END => self.apu.read(address),
            ROM_BEGIN..=ROM_END => self.cart.cpu_read(address),
            apu::ENABLE_LEN => self.apu.read(address),
            apu::FRAME_COUNTER => self.apu.read(address),
            _ => 0,
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            RAM_BEGIN..=RAM_END => self.ram.write(address, value),
            ppu::REGISTERS_BEGIN..=ppu::REGISTERS_END => self.write_ppu(address, value),
            ppu::OAM_DMA => self.write_ppu(address, value),
            apu::VOICE_BEGIN..=apu::VOICE_END => self.apu.write(address, value),
            ROM_BEGIN..=ROM_END => self.cart.cpu_write(address, value),
            apu::ENABLE_LEN => self.apu.write(address, value),
            apu::FRAME_COUNTER => self.apu.write(address, value),
            _ => {},
            // _ => panic!("write error {:04X} = {:02X}", address, value),
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
