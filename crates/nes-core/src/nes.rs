use crate::bus::Bus;
use crate::cartridge::cartridge::Cartridge;
use crate::cpu::{addressing::AddressResult, cpu6502::Cpu6502, instruction::Instruction};
use crate::observers::ppu_observer::PpuObserver;
use crate::observers::{self, *};
use crate::ppu::ppu::Ppu;

pub struct Nes {
    cartridge: Cartridge,
    cpu: Cpu6502,
    ppu: Ppu,
    cpu_ram: [u8; 2048],
    pub cpu_observer: Option<Box<dyn CpuObserver>>,
    pub ppu_observer: Option<Box<dyn PpuObserver>>,
}

impl Nes {
    pub fn new(rom: &[u8]) -> Result<Self, String> {
        let cartridge = Cartridge::new(rom)?;
        Ok(Self {
            cartridge,
            cpu: Cpu6502::new(),
            ppu: Ppu::new(),
            cpu_ram: [0; 2048],
            cpu_observer: None,
            ppu_observer: None,
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
        let cpu_cycles = self.cpu.step(&mut bus, &mut self.cpu_observer);
        let ppu_cycles = cpu_cycles * 3;
        self.ppu.step(ppu_cycles, &mut self.ppu_observer);
    }
}
