use crate::observers::*;
use crate::{bus::Bus, cpu::cpu6502::Cpu6502};

#[derive(Debug, Clone, Copy)]
pub enum AddressMode {
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Accumulator,
    Immediate,
    Implied,
    Indirect,
    IndirectX,
    IndirectY,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    IndirectIndexed,
}

#[derive(Clone, Copy)]
pub enum AddressResult {
    Accumulator,
    Implied,
    Memory(MemoryAddress),
}

#[derive(Clone, Copy)]
pub struct MemoryAddress {
    pub mode: AddressMode,
    pub operand: u16,
    pub effective_address: u16,
    pub page_crossed: bool,
}

pub fn accumulator(
    _cpu: &mut Cpu6502,
    _bus: &mut Bus,
    observer: &mut Option<Box<dyn CpuObserver>>,
) -> AddressResult {
    AddressResult::Accumulator
}

pub fn implied(
    _cpu: &mut Cpu6502,
    _bus: &mut Bus,
    observer: &mut Option<Box<dyn CpuObserver>>,
) -> AddressResult {
    AddressResult::Implied
}

pub fn absolute(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    observer: &mut Option<Box<dyn CpuObserver>>,
) -> AddressResult {
    let operand = fetch_16(cpu, bus, observer);
    AddressResult::Memory(MemoryAddress {
        mode: AddressMode::Absolute,
        operand,
        effective_address: operand,
        page_crossed: false,
    })
}

pub fn absolute_x(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    observer: &mut Option<Box<dyn CpuObserver>>,
) -> AddressResult {
    let operand = fetch_16(cpu, bus, observer);
    let index = cpu.x as u16;
    let address = operand.wrapping_add(index);
    AddressResult::Memory(MemoryAddress {
        mode: AddressMode::AbsoluteX,
        operand,
        effective_address: address,
        page_crossed: false,
    })
}

pub fn absolute_y(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    observer: &mut Option<Box<dyn CpuObserver>>,
) -> AddressResult {
    let operand = fetch_16(cpu, bus, observer);
    let index = cpu.y as u16;
    let address = operand.wrapping_add(index);
    AddressResult::Memory(MemoryAddress {
        mode: AddressMode::AbsoluteY,
        operand,
        effective_address: address,
        page_crossed: false,
    })
}

pub fn immediate(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    observer: &mut Option<Box<dyn CpuObserver>>,
) -> AddressResult {
    let address = cpu.pc;
    let operand = cpu.fetch(bus, observer) as u16;
    AddressResult::Memory(MemoryAddress {
        mode: AddressMode::Immediate,
        operand,
        effective_address: address,
        page_crossed: false,
    })
}

pub fn indirect(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    observer: &mut Option<Box<dyn CpuObserver>>,
) -> AddressResult {
    let operand = cpu.fetch(bus, observer) as u16;
    let mut bytes = [0, 0];
    bytes[0] = cpu.read(bus, operand, observer);
    bytes[1] = cpu.read(bus, operand.wrapping_add(1), observer);
    let address = u16::from_le_bytes(bytes);
    AddressResult::Memory(MemoryAddress {
        mode: AddressMode::Indirect,
        operand,
        effective_address: address,
        page_crossed: false,
    })
}

pub fn indirect_x(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    observer: &mut Option<Box<dyn CpuObserver>>,
) -> AddressResult {
    let operand = cpu.fetch(bus, observer) as u16;
    let index = cpu.x as u16;
    let indirect1 = operand.wrapping_add(index);
    let indirect2 = indirect1.wrapping_add(1);
    let mut bytes = [0, 0];
    bytes[0] = cpu.read(bus, indirect1, observer);
    bytes[1] = cpu.read(bus, indirect2, observer);
    let address = u16::from_le_bytes(bytes);
    AddressResult::Memory(MemoryAddress {
        mode: AddressMode::IndirectX,
        operand,
        effective_address: address,
        page_crossed: false,
    })
}

pub fn indirect_y(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    observer: &mut Option<Box<dyn CpuObserver>>,
) -> AddressResult {
    let operand = cpu.fetch(bus, observer) as u16;
    let mut bytes = [0, 0];
    bytes[0] = cpu.read(bus, operand, observer);
    bytes[1] = cpu.read(bus, operand.wrapping_add(1), observer);
    let index = cpu.y as u16;
    let address = u16::from_le_bytes(bytes).wrapping_add(index);
    AddressResult::Memory(MemoryAddress {
        mode: AddressMode::IndirectY,
        operand,
        effective_address: address,
        page_crossed: false,
    })
}

pub fn relative(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    observer: &mut Option<Box<dyn CpuObserver>>,
) -> AddressResult {
    let operand = cpu.fetch(bus, observer) as i8;
    let pc = cpu.pc as i16;
    let address = pc.wrapping_add(operand as i16) as u16;
    AddressResult::Memory(MemoryAddress {
        mode: AddressMode::Relative,
        operand: operand as u16,
        effective_address: address,
        page_crossed: false,
    })
}

pub fn zero_page(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    observer: &mut Option<Box<dyn CpuObserver>>,
) -> AddressResult {
    let operand = cpu.fetch(bus, observer) as u16;
    let address = operand;
    AddressResult::Memory(MemoryAddress {
        mode: AddressMode::ZeroPage,
        operand,
        effective_address: address,
        page_crossed: false,
    })
}

pub fn zero_page_x(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    observer: &mut Option<Box<dyn CpuObserver>>,
) -> AddressResult {
    let operand = cpu.fetch(bus, observer);
    let index = cpu.x;
    let address = operand.wrapping_add(index) as u16;
    AddressResult::Memory(MemoryAddress {
        mode: AddressMode::ZeroPageX,
        operand: operand as u16,
        effective_address: address,
        page_crossed: false,
    })
}

pub fn zero_page_y(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    observer: &mut Option<Box<dyn CpuObserver>>,
) -> AddressResult {
    let operand = cpu.fetch(bus, observer);
    let index = cpu.y;
    let address = operand.wrapping_add(index) as u16;
    AddressResult::Memory(MemoryAddress {
        mode: AddressMode::ZeroPageY,
        operand: operand as u16,
        effective_address: address,
        page_crossed: false,
    })
}

fn fetch_16(cpu: &mut Cpu6502, bus: &mut Bus, observer: &mut Option<Box<dyn CpuObserver>>) -> u16 {
    let mut bytes = [0, 0];
    bytes[0] = cpu.fetch(bus, observer);
    bytes[1] = cpu.fetch(bus, observer);
    u16::from_le_bytes(bytes)
}
