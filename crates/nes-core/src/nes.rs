use crate::apu::Apu;
use crate::cartridge::Cartridge;

use crate::cpu::{self, Op};
use crate::frame::Frame;
use cpu::{Cpu6502, Ram};

use crate::ppu;
use crate::ppu::{Ppu, PpuStepResult};

macro_rules! CPU_BUS {
    ($self:ident) => {
        cpu::Bus {
            cart: &mut $self.cartridge,
            ram: &mut $self.ram,
            apu: &mut $self.apu,
            ppu: &mut $self.ppu,
        }
    };
}

macro_rules! PPU_BUS {
    ($self:ident) => {
        ppu::Bus {
            cart: &mut $self.cartridge,
            ram: &mut $self.ram,
        }
    };
}

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
        self.cpu.reset(CPU_BUS!(self));
        self.ppu.reset(PPU_BUS!(self));
    }

    pub fn step(&mut self) -> StepResult {
        let cpu_result = self.cpu.step(CPU_BUS!(self)).unwrap();
        let cycles = cpu_result.cycles;
        let ppu_cycles = cycles * 3;
        let ppu_result = self.ppu.step(PPU_BUS!(self), ppu_cycles);
        let frame = if ppu_result.nmi_inturrupt {
            self.nmi_inturrupt();
            let frame = self.ppu.get_frame(PPU_BUS!(self));
            Some(frame)
        } else {
            None
        };
        if let Some(page) = ppu_result.dma_page {
            self.init_dma_transfer(page as u16);
        }
        StepResult {
            cpu: cpu_result,
            ppu_result,
            frame,
        }
    }

    fn nmi_inturrupt(&mut self) {
        let address = self.cartridge.nmi_vector();
        self.cpu.hardware_interrupt(CPU_BUS!(self), address);
    }

    fn init_dma_transfer(&mut self, page: u16) {
        let mut buffer = [0; 256];
        let page = page << 8;
        let mut bus = CPU_BUS!(self);
        bus.write(0x2003, 0);
        for i in 0..256 {
            buffer[i] = bus.read(page + i as u16);
        }
        self.ppu.dma(&buffer);
    }
}
