use crate::bus::Bus;
use crate::cartridge::cartridge::Cartridge;
use crate::cpu::cpu6502::CpuStepResult;
use crate::cpu::step_collector::CpuStepCollector;
use crate::cpu::{addressing::AddressResult, cpu6502::Cpu6502, instruction::Instruction};

use crate::ppu::ppu::{Ppu, PpuStepResult};

pub struct Nes {
    cartridge: Cartridge,
    cpu: Cpu6502,
    ppu: Ppu,
    cpu_ram: [u8; 2048],
}

pub struct StepResult {
    pub cpu: CpuStepCollector,
    pub ppu_result: PpuStepResult,
}

impl Nes {
    pub fn new(rom: &[u8]) -> Result<Self, String> {
        let cartridge = Cartridge::new(rom)?;
        Ok(Self {
            cartridge,
            cpu: Cpu6502::new(),
            ppu: Ppu::new(),
            cpu_ram: [0; 2048],
        })
    }

    pub fn reset(&mut self) {
        let mut bus = Bus {
            cartridge: &mut self.cartridge,
            cpu_ram: &mut self.cpu_ram,
        };
        self.cpu.reset(&mut bus);
    }

    pub fn step(&mut self) -> StepResult {
        let mut bus = Bus {
            cartridge: &mut self.cartridge,
            cpu_ram: &mut self.cpu_ram,
        };
        let cpu_result = self.cpu.step(&mut bus).unwrap();
        let cycles = cpu_result.cycles;
        let ppu_cycles = cycles * 3;
        let ppu_result = self.ppu.step(ppu_cycles);
        StepResult {
            cpu: cpu_result,
            ppu_result,
        }
    }
}
