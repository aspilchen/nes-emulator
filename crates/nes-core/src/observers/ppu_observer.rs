use crate::ppu::ppu::Ppu;
use std::any::Any;

pub trait PpuObserver {
    fn on_step_begin(&mut self, ppu: &Ppu);
    fn on_step_end(&mut self, ppu: &Ppu);
    fn as_any(&self) -> &dyn Any;
}

pub struct NestestPpuObserver {
    pub traces: Vec<NestestPpuStepTrace>,
}

pub struct NestestPpuStepTrace {
    pub cycle: u64,
    pub scanline: u16,
}

impl NestestPpuObserver {
    pub fn new() -> Self {
        Self { traces: Vec::new() }
    }
}

impl PpuObserver for NestestPpuObserver {
    fn on_step_begin(&mut self, ppu: &Ppu) {
        self.traces.push(NestestPpuStepTrace {
            cycle: ppu.cycles,
            scanline: ppu.scanline,
        });
    }

    fn on_step_end(&mut self, ppu: &Ppu) {
        todo!()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
