use crate::apu::Apu;
use crate::cartridge::Cartridge;

use crate::cpu;
use crate::frame::Frame;
use crate::input::JoyPad;
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
            input_1: &mut $self.input_1,
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
    pub cpu: Cpu6502,
    pub ppu: Ppu,
    pub input_1: JoyPad,
    cartridge: Cartridge,
    ram: cpu::Ram,
    apu: Apu,
    dma_cycles_remaining: u64,
    nmi_pending: bool,
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
            dma_cycles_remaining: 0,
            nmi_pending: false,
            input_1: JoyPad::new(),
        })
    }

    pub fn reset(&mut self) {
        self.cpu.reset(CPU_BUS!(self));
        self.ppu.reset(PPU_BUS!(self));
        self.dma_cycles_remaining = 0;
        self.nmi_pending = false;
        self.input_1.reset();
    }

    pub fn step(&mut self, frame: Option<&mut dyn Frame>) -> StepResult {
        let cpu_result = self.cpu.step(CPU_BUS!(self)).unwrap();
        let cycles = cpu_result.cycles;
        let ppu_cycles = cycles * 3;
        let ppu_result = self.ppu.step(PPU_BUS!(self), frame, ppu_cycles);

        if ppu_result.nmi_inturrupt {
            self.nmi_inturrupt();
        }

        if let Some(page) = ppu_result.dma_page {
            self.dma_transfer(page as u16);
        }

        StepResult {
            cpu: cpu_result,
            ppu_result,
        }
    }

    fn nmi_inturrupt(&mut self) {
        let address = self.cartridge.nmi_vector();
        self.cpu.hardware_interrupt(CPU_BUS!(self), address);
    }

    fn dma_transfer(&mut self, page: u16) {
        let mut buffer = [0; 256];
        let page = page << 8;
        let mut bus = CPU_BUS!(self);
        for i in 0..256 {
            buffer[i] = bus.read(page + i as u16);
            bus.ppu.dma(&buffer);
        }
    }
}
