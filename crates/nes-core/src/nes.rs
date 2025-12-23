use crate::bus::Bus;
use crate::cartridge::cartridge::Cartridge;
use crate::cpu::{addressing::AddressResult, cpu6502::Cpu6502, instruction::Instruction};
use crate::observers::NesObserver;
use crate::ppu::ppu::Ppu;

pub struct Nes {
    cartridge: Cartridge,
    cpu: Cpu6502,
    ppu: Ppu,
    cpu_ram: [u8; 2048],
    pub observer: Option<Box<dyn NesObserver>>,
}

impl Nes {
    pub fn new(rom: &[u8]) -> Result<Self, String> {
        let cartridge = Cartridge::new(rom)?;
        Ok(Self {
            cartridge,
            cpu: Cpu6502::new(),
            ppu: Ppu::new(),
            cpu_ram: [0; 2048],
            observer: None,
        })
    }

    pub fn reset(&mut self) {
        let mut bus = Bus {
            cartridge: &mut self.cartridge,
            cpu_ram: &mut self.cpu_ram,
        };
        self.cpu.reset(&mut bus);
    }

    pub fn step(&mut self) {
        let mut bus = Bus {
            cartridge: &mut self.cartridge,
            cpu_ram: &mut self.cpu_ram,
        };
        // let observer = ;
        let cpu_cycles = self.cpu.step(&mut bus, &mut self.observer);
        let ppu_cycles = cpu_cycles * 3;
        self.ppu.step(ppu_cycles, &mut self.observer);
    }
}
