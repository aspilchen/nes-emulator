use std::any::Any;

use crate::{
    cpu::{addressing::AddressResult, cpu6502::Cpu6502, instruction::Instruction},
    ppu::ppu::Ppu,
    Nes,
};

pub trait NesObserver : Any {
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

#[derive(Clone)]
pub struct NestTestNesObserver {
    pub cpu_traces: Vec<CpuTraceDetails>,
    pub ppu_traces: Vec<PpuTraceDetails>,
}

impl NestTestNesObserver {
    pub fn new() -> Self {
        Self {
            cpu_traces: Vec::new(),
            ppu_traces: Vec::new(),
        }
    }
}

impl NesObserver for NestTestNesObserver {
    fn on_cpu(&mut self, observer_result: CpuTraceDetails) {
        self.cpu_traces.push(observer_result);
    }

    fn on_ppu(&mut self, observer_result: PpuTraceDetails) {
        self.ppu_traces.push(observer_result);
    }
}
