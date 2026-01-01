use crate::bus::Bus;
use crate::cpu::cpu6502::Cpu6502;

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

#[derive(Clone)]
pub enum AddressResult {
    Accumulator,
    Implied,
    Memory(MemoryAddress),
}

#[derive(Clone)]
pub struct MemoryAddress {
    pub mode: AddressMode,
    pub base_address: u16,
    pub effective_address: u16,
    pub indexed_offset: Option<u16>,
    pub page_crossed: bool,
}

pub fn accumulator(_cpu: &mut Cpu6502, _bus: &mut Bus) -> AddressResult {
    AddressResult::Accumulator
}

pub fn implied(_cpu: &mut Cpu6502, _bus: &mut Bus) -> AddressResult {
    AddressResult::Implied
}

pub fn absolute(cpu: &mut Cpu6502, bus: &mut Bus) -> AddressResult {
    let bytes = fetch_16(cpu, bus);
    let operand = u16::from_le_bytes(bytes);
    AddressResult::Memory(MemoryAddress {
        mode: AddressMode::Absolute,
        base_address: operand,
        effective_address: operand,
        indexed_offset: None,
        page_crossed: false,
    })
}

pub fn absolute_x(cpu: &mut Cpu6502, bus: &mut Bus) -> AddressResult {
    let bytes = fetch_16(cpu, bus);
    let operand = u16::from_le_bytes(bytes);
    let effective_address = operand.wrapping_add(cpu.x as u16);
    let page_crossed = crosses_page_boundary(operand, effective_address);
    AddressResult::Memory(MemoryAddress {
        mode: AddressMode::AbsoluteX,
        base_address: operand,
        effective_address,
        indexed_offset: Some(effective_address),
        page_crossed,
    })
}

pub fn absolute_y(cpu: &mut Cpu6502, bus: &mut Bus) -> AddressResult {
    let bytes = fetch_16(cpu, bus);
    let operand = u16::from_le_bytes(bytes);
    let effective_address = operand.wrapping_add(cpu.y as u16);
    let page_crossed = crosses_page_boundary(operand, effective_address);
    AddressResult::Memory(MemoryAddress {
        mode: AddressMode::AbsoluteY,
        base_address: operand,
        effective_address,
        indexed_offset: Some(effective_address),
        page_crossed,
    })
}

pub fn immediate(cpu: &mut Cpu6502, bus: &mut Bus) -> AddressResult {
    let effective_address = cpu.pc;
    let operand = cpu.fetch(bus) as u16;
    AddressResult::Memory(MemoryAddress {
        mode: AddressMode::Immediate,
        base_address: operand,
        effective_address,
        indexed_offset: None,
        page_crossed: false,
    })
}

pub fn indirect(cpu: &mut Cpu6502, bus: &mut Bus) -> AddressResult {
    let bytes = fetch_16(cpu, bus);
    let operand = u16::from_le_bytes(bytes);
    // let mut effective_bytes = [0, 0];
    // effective_bytes[0] = cpu.read(bus, operand);
    // effective_bytes[1] = cpu.read(bus, operand + 1);
    let effective_address = u16::from_le_bytes(bytes);
    AddressResult::Memory(MemoryAddress {
        mode: AddressMode::Indirect,
        base_address: operand,
        effective_address,
        indexed_offset: None,
        page_crossed: false,
    })
}

pub fn indirect_x(cpu: &mut Cpu6502, bus: &mut Bus) -> AddressResult {
    let operand = cpu.fetch(bus);
    let base = operand.wrapping_add(cpu.x);
    let base_high = base.wrapping_add(1);
    let mut bytes = [0, 0];
    bytes[0] = cpu.read(bus, base as u16);
    bytes[1] = cpu.read(bus, base_high as u16);
    let effective_address = u16::from_le_bytes(bytes);
    let page_crossed = crosses_page_boundary(operand as u16, base as u16);
    AddressResult::Memory(MemoryAddress {
        mode: AddressMode::IndirectX,
        base_address: operand as u16,
        effective_address,
        indexed_offset: Some(base as u16),
        page_crossed,
    })
}

pub fn indirect_y(cpu: &mut Cpu6502, bus: &mut Bus) -> AddressResult {
    let operand = cpu.fetch(bus);
    let mut bytes = [0, 0];
    bytes[0] = cpu.read(bus, operand as u16);
    bytes[1] = cpu.read(bus, operand.wrapping_add(1) as u16);
    let base_address = u16::from_le_bytes(bytes);
    let effective_address = base_address.wrapping_add(cpu.y as u16);
    let page_crossed = crosses_page_boundary(base_address, effective_address);
    AddressResult::Memory(MemoryAddress {
        mode: AddressMode::IndirectY,
        base_address,
        effective_address,
        indexed_offset: Some(base_address),
        page_crossed,
    })
}

pub fn relative(cpu: &mut Cpu6502, bus: &mut Bus) -> AddressResult {
    let operand = cpu.fetch(bus) as i8;
    let pc = cpu.pc as i16;
    let effective_address = pc.wrapping_add(operand as i16) as u16;
    AddressResult::Memory(MemoryAddress {
        mode: AddressMode::Relative,
        base_address: operand as u16,
        effective_address,
        indexed_offset: None,
        page_crossed: false,
    })
}

pub fn zero_page(cpu: &mut Cpu6502, bus: &mut Bus) -> AddressResult {
    let operand = cpu.fetch(bus) as u16;
    let effective_address = operand;
    AddressResult::Memory(MemoryAddress {
        mode: AddressMode::ZeroPage,
        base_address: operand,
        effective_address,
        indexed_offset: None,
        page_crossed: false,
    })
}

pub fn zero_page_x(cpu: &mut Cpu6502, bus: &mut Bus) -> AddressResult {
    let operand = cpu.fetch(bus);
    let effective_address = operand.wrapping_add(cpu.x) as u16;
    AddressResult::Memory(MemoryAddress {
        mode: AddressMode::ZeroPageX,
        base_address: operand as u16,
        effective_address,
        indexed_offset: Some(effective_address),
        page_crossed: false,
    })
}

pub fn zero_page_y(cpu: &mut Cpu6502, bus: &mut Bus) -> AddressResult {
    let operand = cpu.fetch(bus);
    let effective_address = operand.wrapping_add(cpu.y) as u16;
    AddressResult::Memory(MemoryAddress {
        mode: AddressMode::ZeroPageY,
        base_address: operand as u16,
        effective_address,
        indexed_offset: Some(effective_address),
        page_crossed: false,
    })
}

fn fetch_16(cpu: &mut Cpu6502, bus: &mut Bus) -> [u8; 2] {
    let mut bytes = [0, 0];
    bytes[0] = cpu.fetch(bus);
    bytes[1] = cpu.fetch(bus);
    bytes
}

fn crosses_page_boundary(a: u16, b: u16) -> bool {
    (a & 0xFF00) != (b & 0xFF00)
}
