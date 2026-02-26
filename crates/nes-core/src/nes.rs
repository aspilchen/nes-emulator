use crate::apu::Apu;
use crate::cartridge::Cartridge;
use ringbuf::HeapProd;

use crate::cpu::{self, Op};
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
    dma_active: bool,
    dma_page: u16,
    nmi_pending: bool,
}

pub struct StepResult {
    pub cpu: Option<cpu::Collector>,
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
            dma_active: false,
            nmi_pending: false,
            dma_page: 0,
            input_1: JoyPad::new(),
        })
    }

    pub fn set_audio_buffer(&mut self, buffer: HeapProd<f32>) {
        self.apu.set_audio_buffer(buffer);
    }

    pub fn reset(&mut self) {
        self.cpu.reset(CPU_BUS!(self));
        self.ppu.reset(PPU_BUS!(self));
        self.dma_active = false;
        self.dma_page = 0;
        self.nmi_pending = false;
        self.input_1.reset();
    }

    pub fn step(&mut self, frame: Option<&mut dyn Frame>) -> StepResult {
        if self.dma_active {
            self.dma_active = false;
            let ppu_result = self.ppu.step(PPU_BUS!(self), frame, 256 * 3);
            self.apu.step(256);
            return StepResult {
                cpu: None,
                ppu_result,
            };
        }
        let cpu_result = self.cpu.step(CPU_BUS!(self)).unwrap();
        let cycles = cpu_result.cycles;
        let ppu_cycles = cycles * 3;
        let ppu_result = self.ppu.step(PPU_BUS!(self), frame, ppu_cycles);
        self.apu.step(cycles);
        if ppu_result.nmi_inturrupt {
            self.nmi_inturrupt();
        }

        if let Some(page) = ppu_result.dma_page {
            self.dma_active = true;
            self.dma_transfer(page as u16);
        }

        StepResult {
            cpu: Some(cpu_result),
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
        }
        bus.ppu.dma(&buffer);
        self.dma_active = true;
    }
}
