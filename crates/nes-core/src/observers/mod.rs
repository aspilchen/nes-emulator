pub mod cpu_observer;
pub mod ppu_observer;

pub use cpu_observer::{CpuObserver, NestestCpuObserver, NestestCpuStepTrace};

use crate::{
    cpu::{AddressMode, AddressResult, Op},
    observers::ppu_observer::NestestPpuStepTrace,
};

use std::fmt;

#[macro_export]
macro_rules! notify {
    ($observer:expr, $method:ident $(, $args:expr)* $(,)?) => {
        if let Some(obs) = $observer.as_deref_mut() {
            obs.$method($($args),*);
        }
    };
}

pub struct NestestStepTrace<'a> {
    pub cpu: &'a NestestCpuStepTrace,
    pub ppu: &'a NestestPpuStepTrace,
}

impl<'a> fmt::Display for NestestStepTrace<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let byte_str = self
            .cpu
            .bytes_fetched
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<String>>()
            .join(" ");

        let mut addr_str = match self.cpu.address {
            crate::cpu::AddressResult::Memory(mem) => match mem.mode {
                AddressMode::Absolute => {
                    format!("${:04X}", mem.effective_address)
                }
                AddressMode::AbsoluteX => {
                    format!("(${},X)", mem.effective_address)
                }
                AddressMode::AbsoluteY => {
                    format!("(${},Y)", mem.effective_address)
                }
                AddressMode::Accumulator => "A".into(),
                AddressMode::Immediate => {
                    format!("#${:02X}", self.cpu.bytes_fetched[1])
                }
                // AddressMode:: Implied => {}
                AddressMode::Indirect => {
                    format!("${:04X}", mem.effective_address)
                }
                AddressMode::IndirectX => {
                    format!("(${:02X},X)", self.cpu.bytes_fetched[1])
                }
                AddressMode::IndirectY => {
                    format!("(${:02X},Y)", self.cpu.bytes_fetched[1])
                }
                AddressMode::ZeroPage => match self.cpu.mnemonic {
                    Op::STX => format!("${:02X} = {:02X}", mem.effective_address, self.cpu.x),
                    _ => format!("${:02X}", mem.effective_address),
                },
                AddressMode::ZeroPageX => {
                    format!("${:02X},X", self.cpu.bytes_fetched[1])
                }
                AddressMode::ZeroPageY => {
                    format!("${:02X},Y", self.cpu.bytes_fetched[1])
                }
                AddressMode::Relative => {
                    format!("${:04X}", mem.effective_address)
                }
                // AddressMode::IndirectIndexed => "".into(),
                _ => "".into(),
            },
            AddressResult::Accumulator => "A".into(),
            _ => "".into(),
        };

        write!(
            f,
            "{:04X}  {: <8}  {:?}  {: <31} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} PPU:{:>3},{:>3} CYC:{}",
            self.cpu.pc,
            byte_str,
            self.cpu.mnemonic,
            addr_str,
            self.cpu.a,
            self.cpu.x,
            self.cpu.y,
            self.cpu.p,
            self.cpu.sp,
            self.ppu.scanline,
            self.ppu.cycle,
            self.cpu.cyc,
        )
    }
}
