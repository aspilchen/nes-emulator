use crate::apu::Apu;
use crate::cartridge::Cartridge;

use crate::cpu;
use cpu::{Cpu6502, Ram};

use crate::ppu;
use crate::ppu::{Ppu, PpuStepResult};

pub struct Nes {
    cartridge: Cartridge,
    cpu: Cpu6502,
    ppu: Ppu,
    ram: cpu::Ram,
    apu: Apu,
}

pub struct StepResult {
    pub cpu: cpu::Collector,
    pub ppu_result: PpuStepResult,
}

impl Nes {
    pub fn new(rom: &[u8]) -> Result<Self, String> {
        let cartridge = Cartridge::new(rom)?;
        Ok(Self {
            cartridge,
            cpu: Cpu6502::new(),
            ppu: Ppu::new(),
            ram: Ram::new(),
            apu: Apu::new(),
        })
    }

    pub fn reset(&mut self) {
        self.cpu.reset(cpu::Bus {
            cart: &mut self.cartridge,
            ram: &mut self.ram,
            apu: &mut self.apu,
            ppu: &mut self.ppu,
        });
        self.ppu.reset(ppu::Bus {
            cart: &mut self.cartridge,
            ram: &mut self.ram,
        });
    }

    pub fn step(&mut self) -> StepResult {
        let cpu_result = self
            .cpu
            .step(cpu::Bus {
                cart: &mut self.cartridge,
                ram: &mut self.ram,
                apu: &mut self.apu,
                ppu: &mut self.ppu,
            })
            .unwrap();
        let cycles = cpu_result.cycles;
        let ppu_cycles = cycles * 3;
        let ppu_result = self.ppu.step(
            ppu_cycles,
            ppu::Bus {
                cart: &mut self.cartridge,
                ram: &mut self.ram,
            },
        );
        StepResult {
            cpu: cpu_result,
            ppu_result,
        }
    }
}
