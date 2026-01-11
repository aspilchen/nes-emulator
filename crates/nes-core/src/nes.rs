use crate::apu::Apu;
use crate::cartridge::Cartridge;

use crate::cpu::{self, Op};
use crate::frame::Frame;
use cpu::{Cpu6502, Ram};

use crate::ppu;
use crate::ppu::{Ppu, PpuStepResult};

pub struct Nes {
    cartridge: Cartridge,
    pub cpu: Cpu6502,
    pub ppu: Ppu,
    ram: cpu::Ram,
    apu: Apu,
}

pub struct StepResult {
    pub cpu: cpu::Collector,
    pub ppu_result: PpuStepResult,
    pub frame: Option<Frame>,
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
        let cpu_result = self.step_cpu().unwrap();
        let cycles = cpu_result.cycles;
        let ppu_cycles = cycles * 3;
        let ppu_result = self.ppu.step(
            ppu_cycles,
            ppu::Bus {
                cart: &mut self.cartridge,
                ram: &mut self.ram,
            },
        );

        let frame = if ppu_result.nmi_inturrupt {
            self.cpu_interrupt(self.cartridge.nmi_vector());
            let frame = self.ppu.get_frame(&mut ppu::Bus {
                cart: &mut self.cartridge,
                ram: &mut self.ram,
            });
            Some(frame)
        } else {
            None
        };

        StepResult {
            cpu: cpu_result,
            ppu_result,
            frame,
        }
    }

    fn step_cpu(&mut self) -> Option<cpu::Collector> {
        self.cpu.step(cpu::Bus {
            cart: &mut self.cartridge,
            ram: &mut self.ram,
            apu: &mut self.apu,
            ppu: &mut self.ppu,
        })
    }

    fn cpu_interrupt(&mut self, address: u16) {
        self.cpu.hardware_interrupt(
            cpu::Bus {
                cart: &mut self.cartridge,
                ram: &mut self.ram,
                apu: &mut self.apu,
                ppu: &mut self.ppu,
            },
            address,
        );
    }
}
