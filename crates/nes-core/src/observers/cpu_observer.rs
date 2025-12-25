use crate::cpu::*;

use std::any::Any;

pub trait CpuObserver {
    fn on_step_begin(&mut self, cpu: &Cpu6502);
    fn on_fetch(&mut self, value: u8);
    fn on_decode(&mut self, instruction: &Instruction);
    fn on_resolve_address(&mut self, address: &AddressResult);
    fn on_execute(&mut self, value: u8);
    fn on_read(&mut self, value: u8);
    fn on_write(&mut self, value: u8);
    fn on_step_end(&mut self, cpu: &Cpu6502);
    fn as_any(&self) -> &dyn Any;
}
pub struct NestestCpuObserver {
    pub traces: Vec<NestestCpuStepTrace>,
    curr_step: Option<NestestCpuStepTrace>,
}

pub struct NestestCpuStepTrace {
    pub pc: u16,
    pub bytes_fetched: Vec<u8>,
    pub bytes_read: Vec<u8>,
    pub bytes_write: Vec<u8>,
    pub mnemonic: Op,
    pub address: AddressResult,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub p: u8,
    pub sp: u8,
    pub cyc: u64,
}

impl NestestCpuObserver {
    pub fn new() -> Self {
        Self {
            traces: Vec::new(),
            curr_step: None,
        }
    }
}

impl CpuObserver for NestestCpuObserver {
    fn on_step_begin(&mut self, cpu: &Cpu6502) {
        self.curr_step = Some(NestestCpuStepTrace {
            pc: cpu.pc,
            bytes_fetched: Vec::new(),
            bytes_read: Vec::new(),
            bytes_write: Vec::new(),
            mnemonic: Default::default(),
            address: AddressResult::Implied,
            a: cpu.a,
            x: cpu.x,
            y: cpu.y,
            p: cpu.status.bits(),
            sp: cpu.sp,
            cyc: cpu.cycles,
        })
    }

    fn on_fetch(&mut self, value: u8) {
        if let Some(t) = self.curr_step.as_mut() {
            t.bytes_fetched.push(value);
        }
    }

    fn on_decode(&mut self, instruction: &Instruction) {
        if let Some(t) = self.curr_step.as_mut() {
            t.mnemonic = instruction.name;
        }
    }

    fn on_resolve_address(&mut self, address: &AddressResult) {
        if let Some(t) = self.curr_step.as_mut() {
            t.address = address.clone();
        }
    }

    fn on_execute(&mut self, value: u8) {}

    fn on_read(&mut self, value: u8) {
        if let Some(t) = self.curr_step.as_mut() {
            t.bytes_read.push(value);
        }
    }

    fn on_write(&mut self, value: u8) {
        if let Some(t) = self.curr_step.as_mut() {
            t.bytes_write.push(value);
        }
    }

    fn on_step_end(&mut self, cpu: &Cpu6502) {
        if let Some(t) = self.curr_step.take() {
            self.traces.push(t);
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
