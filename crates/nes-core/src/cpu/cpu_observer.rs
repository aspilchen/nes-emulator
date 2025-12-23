// use crate::cpu::addressing::AddressResult;
// use crate::cpu::{cpu6502::Cpu6502, instruction::Instruction};

// pub struct CpuTraceDetails {
//     pub cpu_snapshot: Cpu6502,
//     pub instruction: Instruction,
//     pub operand: AddressResult,
//     pub value: Option<u8>,
// }

// pub trait CpuObserver {
//     fn on_cpu(&mut self, details: CpuTraceDetails);
// }

// pub struct NestTestCpuObserver {
//     pub traces: Vec<CpuTraceDetails>,
// }

// impl NestTestCpuObserver {
//     pub fn new() -> Self {
//         Self { traces: vec![] }
//     }
// }

// impl CpuObserver for NestTestCpuObserver {
//     fn on_cpu(&mut self, details: CpuTraceDetails) {
//         self.traces.push(details);
//     }
// }
