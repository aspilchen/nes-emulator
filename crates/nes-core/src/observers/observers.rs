use std::any::Any;

use crate::cpu::*;
use crate::ppu::ppu::Ppu;
use crate::Nes;

pub trait CpuObserver {
    fn on_step_begin(&mut self, cpu: Cpu6502);
    fn on_fetch(&mut self, value: u8);
    fn on_decode(&mut self, instruction: Instruction);
    fn on_resolve_address(&mut self, address: AddressResult);
    fn on_execute(&mut self, value: u8);
    fn on_read(&mut self, value: u8);
    fn on_step_end(&mut self);
}

// pub struct NesObserver {
//     pub cpu_observer: dyn CpuObserver,
// }

pub trait NesObserverrr: Any {
    fn on_cpu(&mut self, observer_result: CpuTraceDetails);
    fn on_ppu(&mut self, observer_result: PpuTraceDetails);
}

#[derive(Clone, Copy)]
pub struct CpuTraceDetails {
    pub cpu_snapshot: Cpu6502,
    pub instruction: Instruction,
    pub operand: AddressResult,
    pub value: Option<u8>,
}

#[derive(Clone, Copy)]
pub struct PpuTraceDetails {
    pub scanline: u16,
    pub cycle: u64,
}

pub struct StepTrace {
    pc: u16,
    bytes: Vec<u8>,
    op: Op,
    address: AddressResult,
    a: u8,
    x: u8,
    y: u8,
    p: u8,
    scanline: u8,
    ppu_cycles: u64,
    cpu_cycles: u64,
}

#[derive(Clone)]
pub struct NestTestNesObserver {
    pub traces: Vec<StepTrace>,
}

impl NestTestNesObserver {
    pub fn new() -> Self {
        Self { traces: Vec::new() }
    }
}

impl NesObserverrr for NestTestNesObserver {
    fn on_cpu(&mut self, observer_result: CpuTraceDetails) {
        self.cpu_traces.push(observer_result);
    }

    fn on_ppu(&mut self, observer_result: PpuTraceDetails) {
        self.ppu_traces.push(observer_result);
    }
}
