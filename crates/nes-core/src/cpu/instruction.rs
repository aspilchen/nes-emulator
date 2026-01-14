use std::os::linux::raw::stat;

use crate::cpu::bus::Bus;
use crate::cpu::cpu6502::{Cpu6502, CpuState, Status};
use crate::cpu::{addressing, AddressMode};
use addressing::AddressResult;

const MSB_BIT: u8 = 0x80; // Most Significant Bit (bit 7)
const LSB_BIT: u8 = 0x01; // Least Significant Bit (bit 0)
const OVERFLOW_BIT: u8 = 0x40; // Bit 6 for overflow in BIT

#[derive(Debug, Clone, Copy)]
pub enum Op {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DCP,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    ISB,
    JMP,
    JSR,
    LAX,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    RLA,
    ROL,
    ROR,
    RRA,
    RTI,
    RTS,
    SBC,
    SAX,
    SEC,
    SED,
    SEI,
    SLO,
    SRE,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
}

impl Default for Op {
    fn default() -> Self {
        Op::NOP
    }
}

type AddressFn = fn(&mut Cpu6502, &mut Bus) -> AddressResult;
type ExecutFn = fn(&mut Cpu6502, &mut Bus, &AddressResult) -> InstructionResult;

#[derive(Clone, Copy)]
pub struct Instruction {
    pub name: Op,
    pub opcode: u8,
    pub resolve_address: AddressFn,
    pub execute: ExecutFn,
    pub size: u8,
    pub cycles: u64,
    pub undocumented: bool,
}

pub struct InstructionResult {
    pub extra_cycles: u64,
}

macro_rules! opcode_table {
    ($($opcode:expr => $op:ident, $addr:expr, $exec:expr, $size:expr, $cycles:expr, $undocumented:expr),* $(,)?) => {
        impl From<u8> for Instruction {
            fn from(opcode: u8) -> Self {
                match opcode {
                    $($opcode => Instruction { name: Op::$op, opcode, resolve_address: $addr, execute: $exec, size: $size, cycles: $cycles, undocumented: $undocumented},)*
                    _ => Instruction { name: Op::NOP, opcode, resolve_address: addressing::implied, execute: nop, size: 1, cycles: 2, undocumented: false },
                }
            }
        }
    };
}

opcode_table! {
    0x00 => BRK, addressing::implied, brk, 1, 7, false,
    0x01 => ORA, addressing::indirect_x, ora, 2, 6, false,
    0x05 => ORA, addressing::zero_page, ora, 2, 3, false,
    0x06 => ASL, addressing::zero_page, asl, 2, 5, false,
    0x08 => PHP, addressing::implied, php, 1, 3, false,
    0x09 => ORA, addressing::immediate, ora, 2, 2, false,
    0x0A => ASL, addressing::accumulator, asl, 1, 2, false,
    0x0D => ORA, addressing::absolute, ora, 3, 4, false,
    0x0E => ASL, addressing::absolute, asl, 3, 6, false,
    0x10 => BPL, addressing::relative, bpl, 2, 2, false,
    0x11 => ORA, addressing::indirect_y, ora, 2, 5, false,
    0x15 => ORA, addressing::zero_page_x, ora, 2, 4, false,
    0x16 => ASL, addressing::zero_page_x, asl, 2, 6, false,
    0x18 => CLC, addressing::implied, clc, 1, 2, false,
    0x19 => ORA, addressing::absolute_y, ora, 3, 4, false,
    0x1D => ORA, addressing::absolute_x, ora, 3, 4, false,
    0x1E => ASL, addressing::absolute_x, asl, 3, 7, false,
    0x20 => JSR, addressing::absolute, jsr, 3, 6, false,
    0x21 => AND, addressing::indirect_x, and, 2, 6, false,
    0x24 => BIT, addressing::zero_page, bit, 2, 3, false,
    0x25 => AND, addressing::zero_page, and, 2, 3, false,
    0x26 => ROL, addressing::zero_page, rol, 2, 5, false,
    0x28 => PLP, addressing::implied, plp, 1, 4, false,
    0x29 => AND, addressing::immediate, and, 2, 2, false,
    0x2A => ROL, addressing::accumulator, rol, 1, 2, false,
    0x2C => BIT, addressing::absolute, bit, 3, 4, false,
    0x2D => AND, addressing::absolute, and, 3, 4, false,
    0x2E => ROL, addressing::absolute, rol, 3, 6, false,
    0x30 => BMI, addressing::relative, bmi, 2, 2, false,
    0x31 => AND, addressing::indirect_y, and, 2, 5, false,
    0x35 => AND, addressing::zero_page_x, and, 2, 4, false,
    0x36 => ROL, addressing::zero_page_x, rol, 2, 6, false,
    0x38 => SEC, addressing::implied, sec, 1, 2, false,
    0x39 => AND, addressing::absolute_y, and, 3, 4, false,
    0x3D => AND, addressing::absolute_x, and, 3, 4, false,
    0x3E => ROL, addressing::absolute_x, rol, 3, 7, false,
    0x40 => RTI, addressing::implied, rti, 1, 6, false,
    0x41 => EOR, addressing::indirect_x, eor, 2, 6, false,
    0x45 => EOR, addressing::zero_page, eor, 2, 3, false,
    0x46 => LSR, addressing::zero_page, lsr, 2, 5, false,
    0x48 => PHA, addressing::implied, pha, 1, 3, false,
    0x49 => EOR, addressing::immediate, eor, 2, 2, false,
    0x4A => LSR, addressing::accumulator, lsr, 1, 2, false,
    0x4C => JMP, addressing::absolute, jmp, 3, 3, false,
    0x4D => EOR, addressing::absolute, eor, 3, 4, false,
    0x4E => LSR, addressing::absolute, lsr, 3, 6, false,
    0x50 => BVC, addressing::relative, bvc, 2, 2, false,
    0x51 => EOR, addressing::indirect_y, eor, 2, 5, false,
    0x55 => EOR, addressing::zero_page_x, eor, 2, 4, false,
    0x56 => LSR, addressing::zero_page_x, lsr, 2, 6, false,
    0x58 => CLI, addressing::implied, cli, 1, 2, false,
    0x59 => EOR, addressing::absolute_y, eor, 3, 4, false,
    0x5D => EOR, addressing::absolute_x, eor, 3, 4, false,
    0x5E => LSR, addressing::absolute_x, lsr, 3, 7, false,
    0x60 => RTS, addressing::implied, rts, 1, 6, false,
    0x61 => ADC, addressing::indirect_x, adc, 2, 6, false,
    0x65 => ADC, addressing::zero_page, adc, 2, 3, false,
    0x66 => ROR, addressing::zero_page, ror, 2, 5, false,
    0x68 => PLA, addressing::implied, pla, 1, 4, false,
    0x69 => ADC, addressing::immediate, adc, 2, 2, false,
    0x6A => ROR, addressing::accumulator, ror, 1, 2, false,
    0x6C => JMP, addressing::indirect, jmp, 3, 5, false,
    0x6D => ADC, addressing::absolute, adc, 3, 4, false,
    0x6E => ROR, addressing::absolute, ror, 3, 6, false,
    0x70 => BVS, addressing::relative, bvs, 2, 2, false,
    0x71 => ADC, addressing::indirect_y, adc, 2, 5, false,
    0x75 => ADC, addressing::zero_page_x, adc, 2, 4, false,
    0x76 => ROR, addressing::zero_page_x, ror, 2, 6, false,
    0x78 => SEI, addressing::implied, sei, 1, 2, false,
    0x79 => ADC, addressing::absolute_y, adc, 3, 4, false,
    0x7D => ADC, addressing::absolute_x, adc, 3, 4, false,
    0x7E => ROR, addressing::absolute_x, ror, 3, 7, false,
    0x81 => STA, addressing::indirect_x, sta, 2, 6, false,
    0x84 => STY, addressing::zero_page, sty, 2, 3, false,
    0x85 => STA, addressing::zero_page, sta, 2, 3, false,
    0x86 => STX, addressing::zero_page, stx, 2, 3, false,
    0x88 => DEY, addressing::implied, dey, 1, 2, false,
    0x8A => TXA, addressing::implied, txa, 1, 2, false,
    0x8C => STY, addressing::absolute, sty, 3, 4, false,
    0x8D => STA, addressing::absolute, sta, 3, 4, false,
    0x8E => STX, addressing::absolute, stx, 3, 4, false,
    0x90 => BCC, addressing::relative, bcc, 2, 2, false,
    0x91 => STA, addressing::indirect_y, sta, 2, 6, false,
    0x94 => STY, addressing::zero_page_x, sty, 2, 4, false,
    0x95 => STA, addressing::zero_page_x, sta, 2, 4, false,
    0x96 => STX, addressing::zero_page_y, stx, 2, 4, false,
    0x98 => TYA, addressing::implied, tya, 1, 2, false,
    0x99 => STA, addressing::absolute_y, sta, 3, 5, false,
    0x9A => TXS, addressing::implied, txs, 1, 2, false,
    0x9D => STA, addressing::absolute_x, sta, 3, 5, false,
    0xA0 => LDY, addressing::immediate, ldy, 2, 2, false,
    0xA1 => LDA, addressing::indirect_x, lda, 2, 6, false,
    0xA2 => LDX, addressing::immediate, ldx, 2, 2, false,
    0xA3 => LAX, addressing::indirect_x, lax, 2, 6, true,
    0xA4 => LDY, addressing::zero_page, ldy, 2, 3, false,
    0xA5 => LDA, addressing::zero_page, lda, 2, 3, false,
    0xA6 => LDX, addressing::zero_page, ldx, 2, 3, false,
    0xA7 => LAX, addressing::zero_page, lax, 2, 3, true,
    0xA8 => TAY, addressing::implied, tay, 1, 2, false,
    0xA9 => LDA, addressing::immediate, lda, 2, 2, false,
    0xAA => TAX, addressing::implied, tax, 1, 2, false,
    0xAC => LDY, addressing::absolute, ldy, 3, 4, false,
    0xAD => LDA, addressing::absolute, lda, 3, 4, false,
    0xAE => LDX, addressing::absolute, ldx, 3, 4, false,
    0xAF => LAX, addressing::absolute, lax, 3, 4, true,
    0xB0 => BCS, addressing::relative, bcs, 2, 2, false,
    0xB1 => LDA, addressing::indirect_y, lda, 2, 5, false,
    0xB3 => LAX, addressing::indirect_y, lax, 2, 5, true,
    0xB4 => LDY, addressing::zero_page_x, ldy, 2, 4, false,
    0xB5 => LDA, addressing::zero_page_x, lda, 2, 4, false,
    0xB6 => LDX, addressing::zero_page_y, ldx, 2, 4, false,
    0xB7 => LAX, addressing::zero_page_y, lax, 2, 4, true,
    0xB8 => CLV, addressing::implied, clv, 1, 2, false,
    0xB9 => LDA, addressing::absolute_y, lda, 3, 4, false,
    0xBA => TSX, addressing::implied, tsx, 1, 2, false,
    0xBC => LDY, addressing::absolute_x, ldy, 3, 4, false,
    0xBD => LDA, addressing::absolute_x, lda, 3, 4, false,
    0xB7 => LAX, addressing::absolute_y, lax, 3, 4, true,
    0xBE => LDX, addressing::absolute_y, ldx, 3, 4, false,
    0xBF => LAX, addressing::absolute_y, lax, 3, 4, true,
    0xC0 => CPY, addressing::immediate, cpy, 2, 2, false,
    0xC1 => CMP, addressing::indirect_x, cmp, 2, 6, false,
    0xC4 => CPY, addressing::zero_page, cpy, 2, 3, false,
    0xC5 => CMP, addressing::zero_page, cmp, 2, 3, false,
    0xC6 => DEC, addressing::zero_page, dec, 2, 5, false,
    0xC8 => INY, addressing::implied, iny, 1, 2, false,
    0xC9 => CMP, addressing::immediate, cmp, 2, 2, false,
    0xCA => DEX, addressing::implied, dex, 1, 2, false,
    0xCC => CPY, addressing::absolute, cpy, 3, 4, false,
    0xCD => CMP, addressing::absolute, cmp, 3, 4, false,
    0xCE => DEC, addressing::absolute, dec, 3, 6, false,
    0xD0 => BNE, addressing::relative, bne, 2, 2, false,
    0xD1 => CMP, addressing::indirect_y, cmp, 2, 5, false,
    0xD5 => CMP, addressing::zero_page_x, cmp, 2, 4, false,
    0xD6 => DEC, addressing::zero_page_x, dec, 2, 6, false,
    0xD8 => CLD, addressing::implied, cld, 1, 2, false,
    0xD9 => CMP, addressing::absolute_y, cmp, 3, 4, false,
    0xDD => CMP, addressing::absolute_x, cmp, 3, 4, false,
    0xDE => DEC, addressing::absolute_x, dec, 3, 7, false,
    0xE0 => CPX, addressing::immediate, cpx, 2, 2, false,
    0xE1 => SBC, addressing::indirect_x, sbc, 2, 6, false,
    0xE4 => CPX, addressing::zero_page, cpx, 2, 3, false,
    0xE5 => SBC, addressing::zero_page, sbc, 2, 3, false,
    0xE6 => INC, addressing::zero_page, inc, 2, 5, false,
    0xE8 => INX, addressing::implied, inx, 1, 2, false,
    0xE9 => SBC, addressing::immediate, sbc, 2, 2, false,
    0xEA => NOP, addressing::implied, nop, 1, 2, false,
    0xEC => CPX, addressing::absolute, cpx, 3, 4, false,
    0xED => SBC, addressing::absolute, sbc, 3, 4, false,
    0xEE => INC, addressing::absolute, inc, 3, 6, false,
    0xF0 => BEQ, addressing::relative, beq, 2, 2, false,
    0xF1 => SBC, addressing::indirect_y, sbc, 2, 5, false,
    0xF4 => NOP, addressing::zero_page_x, nop, 2, 4, true,
    0xF5 => SBC, addressing::zero_page_x, sbc, 2, 4, false,
    0xF6 => INC, addressing::zero_page_x, inc, 2, 6, false,
    0xF8 => SED, addressing::implied, sed, 1, 2, false,
    0xF9 => SBC, addressing::absolute_y, sbc, 3, 4, false,
    0xFA => NOP, addressing::implied, nop, 1, 2, true,
    0xFC => NOP, addressing::absolute_x, nop, 3, 4, true,
    0xFD => SBC, addressing::absolute_x, sbc, 3, 4, false,
    0xFE => INC, addressing::absolute_x, inc, 3, 7, false,
    // undocumented instructions
    0x1A => NOP, addressing::implied, nop, 1, 2, true,
    0x3A => NOP, addressing::implied, nop, 1, 2, true,
    0x5A => NOP, addressing::implied, nop, 1, 2, true,
    0x7A => NOP, addressing::implied, nop, 1, 2, true,
    0xDA => NOP, addressing::implied, nop, 1, 2, true,
    0xFA => NOP, addressing::implied, nop, 1, 2, true,
    0x80 => NOP, addressing::immediate, nop, 2, 2, true,
    0x82 => NOP, addressing::immediate, nop, 2, 2, true,
    0x89 => NOP, addressing::immediate, nop, 2, 2, true,
    0xC2 => NOP, addressing::immediate, nop, 2, 2, true,
    0xE2 => NOP, addressing::immediate, nop, 2, 2, true,
    0x04 => NOP, addressing::zero_page, nop, 2, 3, true,
    0x44 => NOP, addressing::zero_page, nop, 2, 3, true,
    0x64 => NOP, addressing::zero_page, nop, 2, 3, true,
    0x14 => NOP, addressing::zero_page_x, nop, 2, 4, true,
    0x34 => NOP, addressing::zero_page_x, nop, 2, 4, true,
    0x54 => NOP, addressing::zero_page_x, nop, 2, 4, true,
    0x74 => NOP, addressing::zero_page_x, nop, 2, 4, true,
    0xD4 => NOP, addressing::zero_page_x, nop, 2, 4, true,
    0xF4 => NOP, addressing::zero_page_x, nop, 2, 4, true,
    0x0C => NOP, addressing::absolute, nop, 3, 4, true,
    0x1C => NOP, addressing::absolute_x, nop, 3, 4, true,
    0x3C => NOP, addressing::absolute_x, nop, 3, 4, true,
    0x5C => NOP, addressing::absolute_x, nop, 3, 4, true,
    0x7C => NOP, addressing::absolute_x, nop, 3, 4, true,
    0xDC => NOP, addressing::absolute_x, nop, 3, 4, true,
    0xFC => NOP, addressing::absolute_x, nop, 3, 4, true,
    0x87 => SAX, addressing::zero_page, sax, 2, 3, true,
    0x97 => SAX, addressing::zero_page_y, sax, 2, 4, true,
    0x8F => SAX, addressing::absolute, sax, 3, 4, true,
    0x83 => SAX, addressing::indirect_x, sax, 2, 6, true,
    0xEB => SBC, addressing::immediate, sbc, 2, 2, true,
    0xC7 => DCP, addressing::zero_page, dcp, 2, 5, true,
    0xD7 => DCP, addressing::zero_page_x, dcp, 2, 6, true,
    0xCF => DCP, addressing::absolute, dcp, 3, 6, true,
    0xDF => DCP, addressing::absolute_x, dcp, 3, 7, true,
    0xDB => DCP, addressing::absolute_y, dcp, 3, 7, true,
    0xC3 => DCP, addressing::indirect_x, dcp, 2, 8, true,
    0xD3 => DCP, addressing::indirect_y, dcp, 2, 8, true,
    0xE3 => ISB, addressing::indirect_x, isb, 2, 8, true,
    0xE7 => ISB, addressing::zero_page, isb, 2, 5, true,
    0xEF => ISB, addressing::absolute, isb, 3, 6, true,
    0xF3 => ISB, addressing::indirect_y, isb, 2, 8, true,
    0xF7 => ISB, addressing::zero_page_x, isb, 2, 6, true,
    0xFB => ISB, addressing::absolute_y, isb, 3, 7, true,
    0xFF => ISB, addressing::absolute_x, isb, 3, 7, true,
    0x03 => SLO, addressing::indirect_x, slo, 2, 8, true,
    0x07 => SLO, addressing::zero_page, slo, 2, 5, true,
    0x0F => SLO, addressing::absolute, slo, 3, 6, true,
    0x13 => SLO, addressing::indirect_y, slo, 2, 8, true,
    0x17 => SLO, addressing::zero_page_x, slo, 2, 6, true,
    0x1B => SLO, addressing::absolute_y, slo, 3, 7, true,
    0x1F => SLO, addressing::absolute_x, slo, 3, 7, true,
    0x23 => RLA, addressing::indirect_x, rla, 2, 8, true,
    0x27 => RLA, addressing::zero_page, rla, 2, 5, true,
    0x2F => RLA, addressing::absolute, rla, 3, 6, true,
    0x33 => RLA, addressing::indirect_y, rla, 2, 8, true,
    0x37 => RLA, addressing::zero_page_x, rla, 2, 6, true,
    0x3B => RLA, addressing::absolute_y, rla, 3, 7, true,
    0x3F => RLA, addressing::absolute_x, rla, 3, 7, true,
    0x43 => SRE, addressing::indirect_x, sre, 2, 8, true,
    0x47 => SRE, addressing::zero_page, sre, 2, 5, true,
    0x4F => SRE, addressing::absolute, sre, 3, 6, true,
    0x53 => SRE, addressing::indirect_y, sre, 2, 8, true,
    0x57 => SRE, addressing::zero_page_x, sre, 2, 6, true,
    0x5B => SRE, addressing::absolute_y, sre, 3, 7, true,
    0x5F => SRE, addressing::absolute_x, sre, 3, 7, true,
    0x63 => RRA, addressing::indirect_x, rra, 2, 8, true,
    0x67 => RRA, addressing::zero_page, rra, 2, 5, true,
    0x6F => RRA, addressing::absolute, rra, 3, 6, true,
    0x73 => RRA, addressing::indirect_y, rra, 2, 8, true,
    0x77 => RRA, addressing::zero_page_x, rra, 2, 6, true,
    0x7B => RRA, addressing::absolute_y, rra, 3, 7, true,
    0x7F => RRA, addressing::absolute_x, rra, 3, 7, true,


}

pub fn adc(cpu: &mut Cpu6502, bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    let extra_cycles = match address_result {
        AddressResult::Memory(mem) => {
            let value = cpu.read(bus, mem.effective_address);
            adc_helper(cpu, value);
            add_cycle_if_page_crossed(mem.page_crossed)
        }
        _ => panic!("invalid memory mode"),
    };
    InstructionResult { extra_cycles }
}

pub fn and(cpu: &mut Cpu6502, bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    let extra_cycles = match address_result {
        AddressResult::Memory(mem) => {
            let value = cpu.read(bus, mem.effective_address);
            cpu.a &= value;
            cpu.set_zn(cpu.a);
            add_cycle_if_page_crossed(mem.page_crossed)
        }
        _ => panic!("invalid memory mode"),
    };
    InstructionResult { extra_cycles }
}

pub fn asl(cpu: &mut Cpu6502, bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    let value = match &address_result {
        AddressResult::Memory(mem) => cpu.read(bus, mem.effective_address),
        AddressResult::Accumulator => cpu.a,
        _ => panic!("invalid memory mode"),
    };
    let result = value << 1;
    cpu.status.set(Status::CARRY, value & MSB_BIT != 0);
    cpu.set_zn(result);
    match address_result {
        AddressResult::Memory(mem) => cpu.write(bus, mem.effective_address, result),
        AddressResult::Accumulator => cpu.a = result,
        _ => panic!("invalid memory mode"),
    };
    InstructionResult { extra_cycles: 0 }
}

pub fn bcc(cpu: &mut Cpu6502, _bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    let extra_cycles = match address_result {
        AddressResult::Memory(mem) => branch_helper(
            cpu,
            mem.effective_address,
            !cpu.status.contains(Status::CARRY),
        ),
        _ => panic!("invalid memory mode"),
    };
    InstructionResult { extra_cycles }
}

pub fn bcs(cpu: &mut Cpu6502, _bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    let extra_cycles = match address_result {
        AddressResult::Memory(mem) => branch_helper(
            cpu,
            mem.effective_address,
            cpu.status.contains(Status::CARRY),
        ),
        _ => panic!("invalid memory mode"),
    };
    InstructionResult { extra_cycles }
}

pub fn beq(cpu: &mut Cpu6502, _bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    let extra_cycles = match address_result {
        AddressResult::Memory(mem) => branch_helper(
            cpu,
            mem.effective_address,
            cpu.status.contains(Status::ZERO),
        ),
        _ => panic!("invalid memory mode"),
    };
    InstructionResult { extra_cycles }
}

pub fn bit(cpu: &mut Cpu6502, bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    match address_result {
        AddressResult::Memory(mem) => {
            let value = cpu.read(bus, mem.effective_address);
            let result = cpu.a & value;
            cpu.status.set(Status::ZERO, result == 0);
            cpu.status.set(Status::OVERFLOW, value & OVERFLOW_BIT != 0);
            cpu.status.set(Status::NEGATIVE, value & MSB_BIT != 0);
        }
        _ => panic!("invalid memory mode"),
    };
    InstructionResult { extra_cycles: 0 }
}

pub fn bmi(cpu: &mut Cpu6502, _bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    let extra_cycles = match address_result {
        AddressResult::Memory(mem) => branch_helper(
            cpu,
            mem.effective_address,
            cpu.status.contains(Status::NEGATIVE),
        ),
        _ => panic!("invalid memory mode"),
    };
    InstructionResult { extra_cycles }
}

pub fn bne(cpu: &mut Cpu6502, _bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    let extra_cycles = match address_result {
        AddressResult::Memory(mem) => branch_helper(
            cpu,
            mem.effective_address,
            !cpu.status.contains(Status::ZERO),
        ),
        _ => panic!("invalid memory mode"),
    };
    InstructionResult { extra_cycles }
}

pub fn bpl(cpu: &mut Cpu6502, _bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    let extra_cycles = match address_result {
        AddressResult::Memory(mem) => branch_helper(
            cpu,
            mem.effective_address,
            !cpu.status.contains(Status::NEGATIVE),
        ),
        _ => panic!("invalid memory mode"),
    };
    InstructionResult { extra_cycles }
}

pub fn brk(cpu: &mut Cpu6502, bus: &mut Bus, _address_result: &AddressResult) -> InstructionResult {
    cpu.increment_pc(1);
    cpu.push_pc(bus);
    let mut status = cpu.status;
    status.set(Status::BREAK, true);
    cpu.stack_push(bus, status.bits());
    cpu.status.set(Status::IRQ_DISABLE, true);
    cpu.pc = bus.cart.brk_vector();
    InstructionResult { extra_cycles: 0 }
}

pub fn bvc(cpu: &mut Cpu6502, _bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    let extra_cycles = match address_result {
        AddressResult::Memory(mem) => branch_helper(
            cpu,
            mem.effective_address,
            !cpu.status.contains(Status::OVERFLOW),
        ),
        _ => panic!("invalid memory mode"),
    };
    InstructionResult { extra_cycles }
}

pub fn bvs(cpu: &mut Cpu6502, _bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    let extra_cycles = match address_result {
        AddressResult::Memory(mem) => branch_helper(
            cpu,
            mem.effective_address,
            cpu.status.contains(Status::OVERFLOW),
        ),
        _ => panic!("invalid memory mode"),
    };
    InstructionResult { extra_cycles }
}

pub fn clc(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _address_result: &AddressResult,
) -> InstructionResult {
    cpu.status.set(Status::CARRY, false);
    InstructionResult { extra_cycles: 0 }
}

pub fn cld(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _address_result: &AddressResult,
) -> InstructionResult {
    cpu.status.set(Status::DECIMAL, false);
    InstructionResult { extra_cycles: 0 }
}

pub fn cli(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _address_result: &AddressResult,
) -> InstructionResult {
    cpu.status.set(Status::IRQ_DISABLE, false);
    InstructionResult { extra_cycles: 0 }
}

pub fn clv(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _address_result: &AddressResult,
) -> InstructionResult {
    cpu.status.set(Status::OVERFLOW, false);
    InstructionResult { extra_cycles: 0 }
}

pub fn cmp(cpu: &mut Cpu6502, bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    let extra_cycles = match address_result {
        AddressResult::Memory(mem) => {
            let value = cpu.read(bus, mem.effective_address);
            compare(cpu, cpu.a, value);
            // if matches!(mem.mode, Absol)
            add_cycle_if_page_crossed(mem.page_crossed)
        }
        _ => panic!("invalid memory mode"),
    };
    InstructionResult { extra_cycles }
}

pub fn cpx(cpu: &mut Cpu6502, bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    match address_result {
        AddressResult::Memory(mem) => {
            let value = cpu.read(bus, mem.effective_address);
            compare(cpu, cpu.x, value);
        }
        _ => panic!("invalid memory mode"),
    };
    InstructionResult { extra_cycles: 0 }
}

pub fn cpy(cpu: &mut Cpu6502, bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    match address_result {
        AddressResult::Memory(mem) => {
            let value = cpu.read(bus, mem.effective_address);
            compare(cpu, cpu.y, value);
        }
        _ => panic!("invalid memory mode"),
    };
    InstructionResult { extra_cycles: 0 }
}

pub fn dec(cpu: &mut Cpu6502, bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    match address_result {
        AddressResult::Memory(mem) => {
            let value = cpu.read(bus, mem.effective_address);
            let result = decrement_helper(cpu, value);
            cpu.write(bus, mem.effective_address, result);
        }
        _ => panic!("invalid memory mode"),
    };
    InstructionResult { extra_cycles: 0 }
}

pub fn dex(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _address_result: &AddressResult,
) -> InstructionResult {
    cpu.x = decrement_helper(cpu, cpu.x);
    InstructionResult { extra_cycles: 0 }
}

pub fn dey(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _address_result: &AddressResult,
) -> InstructionResult {
    cpu.y = decrement_helper(cpu, cpu.y);
    InstructionResult { extra_cycles: 0 }
}

pub fn eor(cpu: &mut Cpu6502, bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    let extra_cycles = match address_result {
        AddressResult::Memory(mem) => {
            let value = cpu.read(bus, mem.effective_address);
            let accumulator = cpu.a;
            let result = accumulator ^ value;
            cpu.a = result;
            cpu.set_zn(result);
            add_cycle_if_page_crossed(mem.page_crossed)
        }
        _ => panic!("invalid memory mode"),
    };
    InstructionResult { extra_cycles }
}

pub fn inc(cpu: &mut Cpu6502, bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    match address_result {
        AddressResult::Memory(mem) => {
            let value = cpu.read(bus, mem.effective_address);
            let result = increment_helper(cpu, value);
            cpu.write(bus, mem.effective_address, result);
        }
        _ => panic!("invalid memory mode"),
    };
    InstructionResult { extra_cycles: 0 }
}

pub fn inx(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _address_result: &AddressResult,
) -> InstructionResult {
    cpu.x = increment_helper(cpu, cpu.x);
    InstructionResult { extra_cycles: 0 }
}

pub fn iny(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _address_result: &AddressResult,
) -> InstructionResult {
    cpu.y = increment_helper(cpu, cpu.y);
    InstructionResult { extra_cycles: 0 }
}

pub fn jmp(cpu: &mut Cpu6502, bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    match address_result {
        AddressResult::Memory(mem) => {
            match mem.mode {
                AddressMode::Absolute => cpu.pc = mem.effective_address,
                _ => {
                    let mut bytes = [0, 0];
                    let low = mem.effective_address;
                    let high = (low & 0xFF00) + (low.wrapping_add(1) & 0xFF);
                    bytes[0] = cpu.read(bus, low);
                    bytes[1] = cpu.read(bus, high);
                    cpu.pc = u16::from_le_bytes(bytes);
                }
            }
            // todo!("Implement the hardware bug regarding crossing pages");
        }
        _ => panic!("invalid memory mode"),
    };
    InstructionResult { extra_cycles: 0 }
}

pub fn jsr(cpu: &mut Cpu6502, bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    match address_result {
        AddressResult::Memory(mem) => {
            let bytes = cpu.pc.wrapping_sub(1).to_le_bytes();
            cpu.stack_push(bus, bytes[1]);
            cpu.stack_push(bus, bytes[0]);
            cpu.pc = mem.effective_address;
        }
        _ => panic!("invalid memory mode"),
    }
    InstructionResult { extra_cycles: 0 }
}

pub fn lda(cpu: &mut Cpu6502, bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    let extra_cycles = match address_result {
        AddressResult::Memory(mem) => {
            let value = cpu.read(bus, mem.effective_address);
            cpu.a = value;
            cpu.set_zn(cpu.a);
            add_cycle_if_page_crossed(mem.page_crossed)
        }
        _ => panic!("invalid memory mode"),
    };
    InstructionResult { extra_cycles }
}

pub fn ldx(cpu: &mut Cpu6502, bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    let extra_cycles = match address_result {
        AddressResult::Memory(mem) => {
            let value = cpu.read(bus, mem.effective_address);
            cpu.x = value;
            cpu.set_zn(cpu.x);
            add_cycle_if_page_crossed(mem.page_crossed)
        }
        _ => panic!("invalid memory mode"),
    };
    InstructionResult { extra_cycles }
}

pub fn ldy(cpu: &mut Cpu6502, bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    let extra_cycles = match address_result {
        AddressResult::Memory(mem) => {
            let value = cpu.read(bus, mem.effective_address);
            cpu.y = value;
            cpu.set_zn(cpu.y);
            add_cycle_if_page_crossed(mem.page_crossed)
        }
        _ => panic!("invalid memory mode"),
    };
    InstructionResult { extra_cycles }
}

pub fn lsr(cpu: &mut Cpu6502, bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    let value = match &address_result {
        AddressResult::Accumulator => cpu.a,
        AddressResult::Memory(mem) => cpu.read(bus, mem.effective_address),
        _ => panic!("invalid memory mode"),
    };
    let result = value >> 1;
    cpu.status.set(Status::CARRY, value & LSB_BIT != 0);
    cpu.set_zn(result);
    match address_result {
        AddressResult::Accumulator => cpu.a = result,
        AddressResult::Memory(mem) => cpu.write(bus, mem.effective_address, result),
        _ => panic!("invalid memory mode"),
    };
    InstructionResult { extra_cycles: 0 }
}

pub fn nop(cpu: &mut Cpu6502, bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    // No operation
    let extra_cycles = match address_result {
        AddressResult::Memory(mem) => match mem.mode {
            _ => {
                cpu.read(bus, mem.effective_address);
                add_cycle_if_page_crossed(mem.page_crossed)
            }
        },
        _ => 0,
    };
    InstructionResult { extra_cycles }
}

pub fn ora(cpu: &mut Cpu6502, bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    let extra_cycles = match address_result {
        AddressResult::Memory(mem) => {
            let value = cpu.read(bus, mem.effective_address);
            cpu.a |= value;
            cpu.set_zn(cpu.a);
            add_cycle_if_page_crossed(mem.page_crossed)
        }
        _ => panic!("invalid memory mode"),
    };
    InstructionResult { extra_cycles }
}

pub fn pha(cpu: &mut Cpu6502, bus: &mut Bus, _address_result: &AddressResult) -> InstructionResult {
    cpu.stack_push(bus, cpu.a);
    InstructionResult { extra_cycles: 0 }
}

pub fn php(cpu: &mut Cpu6502, bus: &mut Bus, _address_result: &AddressResult) -> InstructionResult {
    let value = (cpu.status | Status::UNUSED | Status::BREAK).bits();
    cpu.stack_push(bus, value);
    InstructionResult { extra_cycles: 0 }
}

pub fn pla(cpu: &mut Cpu6502, bus: &mut Bus, _address_result: &AddressResult) -> InstructionResult {
    cpu.a = cpu.stack_pop(bus);
    cpu.set_zn(cpu.a);
    InstructionResult { extra_cycles: 0 }
}

pub fn plp(cpu: &mut Cpu6502, bus: &mut Bus, _address_result: &AddressResult) -> InstructionResult {
    let value = cpu.stack_pop(bus);
    let mut new_status = Status::from_bits_truncate(value);
    new_status.set(Status::UNUSED, cpu.status.contains(Status::UNUSED));
    new_status.set(Status::BREAK, cpu.status.contains(Status::BREAK));
    cpu.status = new_status;
    InstructionResult { extra_cycles: 0 }
}

pub fn rol(cpu: &mut Cpu6502, bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    let value = match &address_result {
        AddressResult::Accumulator => cpu.a,
        AddressResult::Memory(mem) => cpu.read(bus, mem.effective_address),
        _ => panic!("invalid memory mode"),
    };
    let mut result = value << 1;
    if cpu.status.contains(Status::CARRY) {
        result |= 1;
    }
    cpu.status.set(Status::CARRY, value & MSB_BIT != 0);
    cpu.set_zn(result);
    match address_result {
        AddressResult::Accumulator => cpu.a = result,
        AddressResult::Memory(mem) => cpu.write(bus, mem.effective_address, result),
        _ => panic!("invalid memory mode"),
    };
    InstructionResult { extra_cycles: 0 }
}

pub fn ror(cpu: &mut Cpu6502, bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    let value = match &address_result {
        AddressResult::Accumulator => cpu.a,
        AddressResult::Memory(mem) => cpu.read(bus, mem.effective_address),
        _ => panic!("invalid memory mode"),
    };
    let mut result = value >> 1;
    if cpu.status.contains(Status::CARRY) {
        result |= 0x80;
    }
    cpu.status.set(Status::CARRY, value & LSB_BIT != 0);
    cpu.set_zn(result);
    match address_result {
        AddressResult::Accumulator => cpu.a = result,
        AddressResult::Memory(mem) => cpu.write(bus, mem.effective_address, result),
        _ => panic!("invalid memory mode"),
    };
    InstructionResult { extra_cycles: 0 }
}

pub fn rti(cpu: &mut Cpu6502, bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    plp(cpu, bus, address_result);
    let mut bytes = [0, 0];
    bytes[0] = cpu.stack_pop(bus);
    bytes[1] = cpu.stack_pop(bus);
    cpu.pc = u16::from_le_bytes(bytes);
    InstructionResult { extra_cycles: 0 }
}

pub fn rts(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _address_result: &AddressResult,
) -> InstructionResult {
    let mut bytes = [0, 0];
    bytes[0] = cpu.stack_pop(_bus);
    bytes[1] = cpu.stack_pop(_bus);
    cpu.pc = u16::from_le_bytes(bytes).wrapping_add(1);
    InstructionResult { extra_cycles: 0 }
}

pub fn sbc(cpu: &mut Cpu6502, bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    let extra_cycles = match address_result {
        AddressResult::Memory(mem) => {
            let value = cpu.read(bus, mem.effective_address);
            adc_helper(cpu, !value);
            add_cycle_if_page_crossed(mem.page_crossed)
        }
        _ => panic!("invalid memory mode"),
    };
    InstructionResult { extra_cycles }
}

pub fn sec(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _address_result: &AddressResult,
) -> InstructionResult {
    cpu.status.set(Status::CARRY, true);
    InstructionResult { extra_cycles: 0 }
}

pub fn sed(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _address_result: &AddressResult,
) -> InstructionResult {
    cpu.status.set(Status::DECIMAL, true);
    InstructionResult { extra_cycles: 0 }
}

pub fn sei(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _address_result: &AddressResult,
) -> InstructionResult {
    cpu.status.set(Status::IRQ_DISABLE, true);
    InstructionResult { extra_cycles: 0 }
}

pub fn sta(cpu: &mut Cpu6502, bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    match address_result {
        AddressResult::Memory(mem) => {
            cpu.write(bus, mem.effective_address, cpu.a);
        }
        _ => panic!("invalid memory mode"),
    };
    InstructionResult { extra_cycles: 0 }
}

pub fn stx(cpu: &mut Cpu6502, bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    match address_result {
        AddressResult::Memory(mem) => {
            cpu.write(bus, mem.effective_address, cpu.x);
        }
        _ => panic!("invalid memory mode"),
    };
    InstructionResult { extra_cycles: 0 }
}

pub fn sty(cpu: &mut Cpu6502, bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    match address_result {
        AddressResult::Memory(mem) => {
            cpu.write(bus, mem.effective_address, cpu.y);
        }
        _ => panic!("invalid memory mode"),
    };
    InstructionResult { extra_cycles: 0 }
}

pub fn tax(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _address_result: &AddressResult,
) -> InstructionResult {
    cpu.x = cpu.a;
    cpu.set_zn(cpu.x);
    InstructionResult { extra_cycles: 0 }
}

pub fn tay(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _address_result: &AddressResult,
) -> InstructionResult {
    cpu.y = cpu.a;
    cpu.set_zn(cpu.y);
    InstructionResult { extra_cycles: 0 }
}

pub fn tsx(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _address_result: &AddressResult,
) -> InstructionResult {
    cpu.x = cpu.sp;
    cpu.set_zn(cpu.x);
    InstructionResult { extra_cycles: 0 }
}

pub fn txa(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _address_result: &AddressResult,
) -> InstructionResult {
    cpu.a = cpu.x;
    cpu.set_zn(cpu.a);
    InstructionResult { extra_cycles: 0 }
}

pub fn txs(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _address_result: &AddressResult,
) -> InstructionResult {
    cpu.sp = cpu.x;
    InstructionResult { extra_cycles: 0 }
}

pub fn tya(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _address_result: &AddressResult,
) -> InstructionResult {
    cpu.a = cpu.y;
    cpu.set_zn(cpu.a);
    InstructionResult { extra_cycles: 0 }
}

pub fn lax(cpu: &mut Cpu6502, bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    let extra_cycles = match address_result {
        AddressResult::Memory(mem) => {
            let value = cpu.read(bus, mem.effective_address);
            cpu.a = value;
            cpu.x = value;
            cpu.set_zn(value);
            add_cycle_if_page_crossed(mem.page_crossed)
        }
        _ => 0,
    };
    InstructionResult { extra_cycles }
}

pub fn sax(cpu: &mut Cpu6502, bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    if let AddressResult::Memory(mem) = address_result {
        let value = cpu.a & cpu.x;
        cpu.write(bus, mem.effective_address, value);
    }
    InstructionResult { extra_cycles: 0 }
}

pub fn dcp(cpu: &mut Cpu6502, bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    if let AddressResult::Memory(mem) = address_result {
        let value = cpu.read(bus, mem.effective_address);
        let result = value.wrapping_sub(1);
        cpu.write(bus, mem.effective_address, result);
        compare(cpu, cpu.a, result);
    }
    InstructionResult { extra_cycles: 0 }
}

pub fn isb(cpu: &mut Cpu6502, bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    if let AddressResult::Memory(mem) = address_result {
        let value = cpu.read(bus, mem.effective_address).wrapping_add(1);
        adc_helper(cpu, !value);
        cpu.write(bus, mem.effective_address, value);
    }
    InstructionResult { extra_cycles: 0 }
}

pub fn slo(cpu: &mut Cpu6502, bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    if let AddressResult::Memory(mem) = address_result {
        let value = cpu.read(bus, mem.effective_address);
        let shifted = value << 1;
        cpu.write(bus, mem.effective_address, shifted);
        cpu.a |= shifted;
        cpu.status.set(Status::CARRY, value & MSB_BIT != 0);
        cpu.set_zn(cpu.a);
    }
    InstructionResult { extra_cycles: 0 }
}

pub fn rla(cpu: &mut Cpu6502, bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    if let AddressResult::Memory(mem) = address_result {
        let value = cpu.read(bus, mem.effective_address);
        let mut result = value << 1;
        if cpu.status.contains(Status::CARRY) {
            result |= 1;
        }
        cpu.write(bus, mem.effective_address, result);
        cpu.a &= result;
        cpu.status.set(Status::CARRY, value & MSB_BIT != 0);
        cpu.set_zn(cpu.a);
    }
    InstructionResult { extra_cycles: 0 }
}

pub fn sre(cpu: &mut Cpu6502, bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    if let AddressResult::Memory(mem) = address_result {
        let value = cpu.read(bus, mem.effective_address);
        let shifted = value >> 1;
        cpu.write(bus, mem.effective_address, shifted);
        cpu.a ^= shifted;
        cpu.status.set(Status::CARRY, value & LSB_BIT != 0);
        cpu.set_zn(cpu.a);
    }
    InstructionResult { extra_cycles: 0 }
}

pub fn rra(cpu: &mut Cpu6502, bus: &mut Bus, address_result: &AddressResult) -> InstructionResult {
    if let AddressResult::Memory(mem) = address_result {
        let value = cpu.read(bus, mem.effective_address);
        let mut result = value >> 1;
        if cpu.status.contains(Status::CARRY) {
            result |= MSB_BIT;
        }
        cpu.status.set(Status::CARRY, value & LSB_BIT != 0);
        cpu.write(bus, mem.effective_address, result);
        adc_helper(cpu, result);
    }
    InstructionResult { extra_cycles: 0 }
}

// pub fn iny(cpu: &mut Cpu6502, bus: &mut CpuBus, address_result: &AddressResult) -> InstructionResult {
//     cpu.y.wrapping_add(1);
//     cpu.set_zn(cpu.y);
//     InstructionResult { extra_cycles: 0 }
// }

fn adc_helper(cpu: &mut Cpu6502, value: u8) {
    let accumulator = cpu.a;
    let mut carry2 = false;
    let (mut result, carry1) = accumulator.overflowing_add(value);
    if cpu.status.contains(Status::CARRY) {
        (result, carry2) = result.overflowing_add(1);
    }
    cpu.status.set(Status::CARRY, carry1 || carry2);
    cpu.status.set(
        Status::OVERFLOW,
        ((result ^ accumulator) & (result ^ value) & MSB_BIT) != 0,
    );
    cpu.set_zn(result);
    cpu.a = result;
}

fn compare(cpu: &mut Cpu6502, register: u8, value: u8) {
    cpu.status.set(Status::CARRY, register >= value);
    cpu.status.set(Status::ZERO, register == value);
    cpu.status.set(
        Status::NEGATIVE,
        register.wrapping_sub(value) & MSB_BIT != 0,
    );
}

fn decrement_helper(cpu: &mut Cpu6502, value: u8) -> u8 {
    let result = value.wrapping_sub(1);
    cpu.set_zn(result);
    result
}

fn increment_helper(cpu: &mut Cpu6502, value: u8) -> u8 {
    let result = value.wrapping_add(1);
    cpu.set_zn(result);
    result
}

fn branch_helper(cpu: &mut Cpu6502, address: u16, condition: bool) -> u64 {
    let mut extra_cycles = 0;
    if condition {
        let is_different_page = cpu.pc & 0xFF00 != address & 0xFF00;
        if is_different_page {
            extra_cycles += 1;
        }
        cpu.pc = address;
        extra_cycles += 1;
    }
    extra_cycles
}

fn add_cycle_if_page_crossed(page_crossed: bool) -> u64 {
    if page_crossed {
        1
    } else {
        0
    }
}
