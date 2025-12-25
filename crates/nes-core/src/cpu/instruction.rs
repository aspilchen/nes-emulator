use crate::bus::Bus;
use crate::cpu::addressing;
use crate::cpu::cpu6502::{Cpu6502, Status};
use crate::notify;
use crate::observers::*;
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
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
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
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
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

type AddressFn = fn(&mut Cpu6502, &mut Bus, &mut Option<Box<dyn CpuObserver>>) -> AddressResult;
type ExecutFn = fn(&mut Cpu6502, &mut Bus, AddressResult, &mut Option<Box<dyn CpuObserver>>);

// pub struct InstructionParams<'a> {
//     pub cpu: &'a mut Cpu6502,
//     pub bus: &'a mut Bus<'a>,
//     pub operand: &'a AddressResult,
//     pub observer: Option<&'a mut dyn CpuObserver>,
// }

#[derive(Clone, Copy)]
pub struct Instruction {
    pub name: Op,
    pub opcode: u8,
    pub resolve_address: AddressFn,
    pub execute: ExecutFn,
    pub size: u8,
    pub cycles: u8,
}

macro_rules! opcode_table {
    ($($opcode:expr => $op:ident, $addr:expr, $exec:expr, $size:expr, $cycles:expr),* $(,)?) => {
        impl From<u8> for Instruction {
            fn from(opcode: u8) -> Self {
                match opcode {
                    $($opcode => Instruction { name: Op::$op, opcode, resolve_address: $addr, execute: $exec, size: $size, cycles: $cycles },)*
                    _ => Instruction { name: Op::NOP, opcode, resolve_address: addressing::implied, execute: nop, size: 1, cycles: 2 },
                }
            }
        }
    };
}

opcode_table! {
    0x00 => BRK, addressing::implied, brk, 1, 7,
    0x01 => ORA, addressing::indirect_x, ora, 2, 6,
    0x05 => ORA, addressing::zero_page, ora, 2, 3,
    0x06 => ASL, addressing::zero_page, asl, 2, 5,
    0x08 => PHP, addressing::implied, php, 1, 3,
    0x09 => ORA, addressing::immediate, ora, 2, 2,
    0x0A => ASL, addressing::accumulator, asl, 1, 2,
    0x0D => ORA, addressing::absolute, ora, 3, 4,
    0x0E => ASL, addressing::absolute, asl, 3, 6,
    0x10 => BPL, addressing::relative, bpl, 2, 2,
    0x11 => ORA, addressing::indirect_y, ora, 2, 5,
    0x15 => ORA, addressing::zero_page_x, ora, 2, 4,
    0x16 => ASL, addressing::zero_page_x, asl, 2, 6,
    0x18 => CLC, addressing::implied, clc, 1, 2,
    0x19 => ORA, addressing::absolute_y, ora, 3, 4,
    0x1D => ORA, addressing::absolute_x, ora, 3, 4,
    0x1E => ASL, addressing::absolute_x, asl, 3, 7,
    0x20 => JSR, addressing::absolute, jsr, 3, 6,
    0x21 => AND, addressing::indirect_x, and, 2, 6,
    0x24 => BIT, addressing::zero_page, bit, 2, 3,
    0x25 => AND, addressing::zero_page, and, 2, 3,
    0x26 => ROL, addressing::zero_page, rol, 2, 5,
    0x28 => PLP, addressing::implied, plp, 1, 4,
    0x29 => AND, addressing::immediate, and, 2, 2,
    0x2A => ROL, addressing::accumulator, rol, 1, 2,
    0x2C => BIT, addressing::absolute, bit, 3, 4,
    0x2D => AND, addressing::absolute, and, 3, 4,
    0x2E => ROL, addressing::absolute, rol, 3, 6,
    0x30 => BMI, addressing::relative, bmi, 2, 2,
    0x31 => AND, addressing::indirect_y, and, 2, 5,
    0x35 => AND, addressing::zero_page_x, and, 2, 4,
    0x36 => ROL, addressing::zero_page_x, rol, 2, 6,
    0x38 => SEC, addressing::implied, sec, 1, 2,
    0x39 => AND, addressing::absolute_y, and, 3, 4,
    0x3D => AND, addressing::absolute_x, and, 3, 4,
    0x3E => ROL, addressing::absolute_x, rol, 3, 7,
    0x40 => RTI, addressing::implied, rti, 1, 6,
    0x41 => EOR, addressing::indirect_x, eor, 2, 6,
    0x45 => EOR, addressing::zero_page, eor, 2, 3,
    0x46 => LSR, addressing::zero_page, lsr, 2, 5,
    0x48 => PHA, addressing::implied, pha, 1, 3,
    0x49 => EOR, addressing::immediate, eor, 2, 2,
    0x4A => LSR, addressing::accumulator, lsr, 1, 2,
    0x4C => JMP, addressing::absolute, jmp, 3, 3,
    0x4D => EOR, addressing::absolute, eor, 3, 4,
    0x4E => LSR, addressing::absolute, lsr, 3, 6,
    0x50 => BVC, addressing::relative, bvc, 2, 2,
    0x51 => EOR, addressing::indirect_y, eor, 2, 5,
    0x55 => EOR, addressing::zero_page_x, eor, 2, 4,
    0x56 => LSR, addressing::zero_page_x, lsr, 2, 6,
    0x58 => CLI, addressing::implied, cli, 1, 2,
    0x59 => EOR, addressing::absolute_y, eor, 3, 4,
    0x5D => EOR, addressing::absolute_x, eor, 3, 4,
    0x5E => LSR, addressing::absolute_x, lsr, 3, 7,
    0x60 => RTS, addressing::implied, rts, 1, 6,
    0x61 => ADC, addressing::indirect_x, adc, 2, 6,
    0x65 => ADC, addressing::zero_page, adc, 2, 3,
    0x66 => ROR, addressing::zero_page, ror, 2, 5,
    0x68 => PLA, addressing::implied, pla, 1, 4,
    0x69 => ADC, addressing::immediate, adc, 2, 2,
    0x6A => ROR, addressing::accumulator, ror, 1, 2,
    0x6C => JMP, addressing::indirect, jmp, 3, 5,
    0x6D => ADC, addressing::absolute, adc, 3, 4,
    0x6E => ROR, addressing::absolute, ror, 3, 6,
    0x70 => BVS, addressing::relative, bvs, 2, 2,
    0x71 => ADC, addressing::indirect_y, adc, 2, 5,
    0x75 => ADC, addressing::zero_page_x, adc, 2, 4,
    0x76 => ROR, addressing::zero_page_x, ror, 2, 6,
    0x78 => SEI, addressing::implied, sei, 1, 2,
    0x79 => ADC, addressing::absolute_y, adc, 3, 4,
    0x7D => ADC, addressing::absolute_x, adc, 3, 4,
    0x7E => ROR, addressing::absolute_x, ror, 3, 7,
    0x81 => STA, addressing::indirect_x, sta, 2, 6,
    0x84 => STY, addressing::zero_page, sty, 2, 3,
    0x85 => STA, addressing::zero_page, sta, 2, 3,
    0x86 => STX, addressing::zero_page, stx, 2, 3,
    0x88 => DEY, addressing::implied, dey, 1, 2,
    0x8A => TXA, addressing::implied, txa, 1, 2,
    0x8C => STY, addressing::absolute, sty, 3, 4,
    0x8D => STA, addressing::absolute, sta, 3, 4,
    0x8E => STX, addressing::absolute, stx, 3, 4,
    0x90 => BCC, addressing::relative, bcc, 2, 2,
    0x91 => STA, addressing::indirect_y, sta, 2, 6,
    0x94 => STY, addressing::zero_page_x, sty, 2, 4,
    0x95 => STA, addressing::zero_page_x, sta, 2, 4,
    0x96 => STX, addressing::zero_page_y, stx, 2, 4,
    0x98 => TYA, addressing::implied, tya, 1, 2,
    0x99 => STA, addressing::absolute_y, sta, 3, 5,
    0x9A => TXS, addressing::implied, txs, 1, 2,
    0x9D => STA, addressing::absolute_x, sta, 3, 5,
    0xA0 => LDY, addressing::immediate, ldy, 2, 2,
    0xA1 => LDA, addressing::indirect_x, lda, 2, 6,
    0xA2 => LDX, addressing::immediate, ldx, 2, 2,
    0xA4 => LDY, addressing::zero_page, ldy, 2, 3,
    0xA5 => LDA, addressing::zero_page, lda, 2, 3,
    0xA6 => LDX, addressing::zero_page, ldx, 2, 3,
    0xA8 => TAY, addressing::implied, tay, 1, 2,
    0xA9 => LDA, addressing::immediate, lda, 2, 2,
    0xAA => TAX, addressing::implied, tax, 1, 2,
    0xAC => LDY, addressing::absolute, ldy, 3, 4,
    0xAD => LDA, addressing::absolute, lda, 3, 4,
    0xAE => LDX, addressing::absolute, ldx, 3, 4,
    0xB0 => BCS, addressing::relative, bcs, 2, 2,
    0xB1 => LDA, addressing::indirect_y, lda, 2, 5,
    0xB4 => LDY, addressing::zero_page_x, ldy, 2, 4,
    0xB5 => LDA, addressing::zero_page_x, lda, 2, 4,
    0xB6 => LDX, addressing::zero_page_y, ldx, 2, 4,
    0xB8 => CLV, addressing::implied, clv, 1, 2,
    0xB9 => LDA, addressing::absolute_y, lda, 3, 4,
    0xBA => TSX, addressing::implied, tsx, 1, 2,
    0xBC => LDY, addressing::absolute_x, ldy, 3, 4,
    0xBD => LDA, addressing::absolute_x, lda, 3, 4,
    0xBE => LDX, addressing::absolute_y, ldx, 3, 4,
    0xC0 => CPY, addressing::immediate, cpy, 2, 2,
    0xC1 => CMP, addressing::indirect_x, cmp, 2, 6,
    0xC4 => CPY, addressing::zero_page, cpy, 2, 3,
    0xC5 => CMP, addressing::zero_page, cmp, 2, 3,
    0xC6 => DEC, addressing::zero_page, dec, 2, 5,
    0xC8 => INY, addressing::implied, iny, 1, 2,
    0xC9 => CMP, addressing::immediate, cmp, 2, 2,
    0xCA => DEX, addressing::implied, dex, 1, 2,
    0xCC => CPY, addressing::absolute, cpy, 3, 4,
    0xCD => CMP, addressing::absolute, cmp, 3, 4,
    0xCE => DEC, addressing::absolute, dec, 3, 6,
    0xD0 => BNE, addressing::relative, bne, 2, 2,
    0xD1 => CMP, addressing::indirect_y, cmp, 2, 5,
    0xD5 => CMP, addressing::zero_page_x, cmp, 2, 4,
    0xD6 => DEC, addressing::zero_page_x, dec, 2, 6,
    0xD8 => CLD, addressing::implied, cld, 1, 2,
    0xD9 => CMP, addressing::absolute_y, cmp, 3, 4,
    0xDD => CMP, addressing::absolute_x, cmp, 3, 4,
    0xDE => DEC, addressing::absolute_x, dec, 3, 7,
    0xE0 => CPX, addressing::immediate, cpx, 2, 2,
    0xE1 => SBC, addressing::indirect_x, sbc, 2, 6,
    0xE4 => CPX, addressing::zero_page, cpx, 2, 3,
    0xE5 => SBC, addressing::zero_page, sbc, 2, 3,
    0xE6 => INC, addressing::zero_page, inc, 2, 5,
    0xE8 => INX, addressing::implied, inx, 1, 2,
    0xE9 => SBC, addressing::immediate, sbc, 2, 2,
    0xEA => NOP, addressing::implied, nop, 1, 2,
    0xEC => CPX, addressing::absolute, cpx, 3, 4,
    0xED => SBC, addressing::absolute, sbc, 3, 4,
    0xEE => INC, addressing::absolute, inc, 3, 6,
    0xF0 => BEQ, addressing::relative, beq, 2, 2,
    0xF1 => SBC, addressing::indirect_y, sbc, 2, 5,
    0xF5 => SBC, addressing::zero_page_x, sbc, 2, 4,
    0xF6 => INC, addressing::zero_page_x, inc, 2, 6,
    0xF8 => SED, addressing::implied, sed, 1, 2,
    0xF9 => SBC, addressing::absolute_y, sbc, 3, 4,
    0xFD => SBC, addressing::absolute_x, sbc, 3, 4,
    0xFE => INC, addressing::absolute_x, inc, 3, 7,
}

macro_rules! define_op {
    ($name:ident, $body:block) => {
        pub fn $name(
            cpu: &mut Cpu6502,
            bus: &mut Bus,
            operand: AddressResult,
            observer: &mut Option<Box<dyn CpuObserver>>
        ) $body
    };
}

pub fn adc(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    // define_op!(adc, {
    match operand {
        AddressResult::Memory(addr) => {
            let param = cpu.read(bus, addr.effective_address, observer);
            adc_helper(cpu, bus, param);
        }
        _ => panic!("invalid memory mode"),
    }
}

pub fn and(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    match operand {
        AddressResult::Memory(addr) => {
            let param = cpu.read(bus, addr.effective_address, observer);
            notify!(observer, on_execute, 0);
            cpu.a &= param;
            cpu.set_zn(cpu.a);
        }
        _ => panic!("invalid memory mode"),
    }
}

pub fn asl(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    let param = match &operand {
        AddressResult::Memory(addr) => cpu.read(bus, addr.effective_address, observer),
        AddressResult::Accumulator => cpu.a,
        _ => panic!("invalid memory mode"),
    };
    let result = param << 1;
    cpu.status.set(Status::CARRY, param & MSB_BIT != 0);
    cpu.set_zn(result);
    match operand {
        AddressResult::Memory(addr) => cpu.write(bus, addr.effective_address, result, observer),
        AddressResult::Accumulator => cpu.a = result,
        _ => panic!("invalid memory mode"),
    };
}

pub fn bcc(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    match operand {
        AddressResult::Memory(addr) => branch_helper(
            cpu,
            addr.effective_address,
            !cpu.status.contains(Status::CARRY),
        ),
        _ => panic!("invalid memory mode"),
    }
}

pub fn bcs(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    match operand {
        AddressResult::Memory(addr) => {
            branch_helper(
                cpu,
                addr.effective_address,
                cpu.status.contains(Status::CARRY),
            );
        }
        _ => panic!("invalid memory mode"),
    }
}

pub fn beq(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    match operand {
        AddressResult::Memory(addr) => {
            branch_helper(
                cpu,
                addr.effective_address,
                cpu.status.contains(Status::ZERO),
            );
        }
        _ => panic!("invalid memory mode"),
    }
}

pub fn bit(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    match operand {
        AddressResult::Memory(addr) => {
            let param = cpu.read(bus, addr.effective_address, observer);
            let result = cpu.a & param;
            cpu.set_zn(result);
            cpu.status.set(Status::OVERFLOW, param & OVERFLOW_BIT != 0);
        }
        _ => panic!("invalid memory mode"),
    }
}

pub fn bmi(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    match operand {
        AddressResult::Memory(addr) => {
            branch_helper(
                cpu,
                addr.effective_address,
                cpu.status.contains(Status::NEGATIVE),
            );
        }
        _ => panic!("invalid memory mode"),
    }
}

pub fn bne(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    match operand {
        AddressResult::Memory(addr) => {
            branch_helper(
                cpu,
                addr.effective_address,
                !cpu.status.contains(Status::ZERO),
            );
        }
        _ => panic!("invalid memory mode"),
    }
}

pub fn bpl(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    match operand {
        AddressResult::Memory(addr) => {
            branch_helper(
                cpu,
                addr.effective_address,
                !cpu.status.contains(Status::NEGATIVE),
            );
        }
        _ => panic!("invalid memory mode"),
    }
}

pub fn brk(
    _cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _operand: AddressResult,
    _observer: &mut Option<Box<dyn CpuObserver>>,
) {
    todo!("BRK not implemented");
}

pub fn bvc(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    match operand {
        AddressResult::Memory(addr) => {
            branch_helper(
                cpu,
                addr.effective_address,
                !cpu.status.contains(Status::OVERFLOW),
            );
        }
        _ => panic!("invalid memory mode"),
    }
}

pub fn bvs(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    match operand {
        AddressResult::Memory(addr) => {
            branch_helper(
                cpu,
                addr.effective_address,
                cpu.status.contains(Status::OVERFLOW),
            );
        }
        _ => panic!("invalid memory mode"),
    }
}

pub fn clc(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    cpu.status.set(Status::CARRY, false);
}

pub fn cld(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    cpu.status.set(Status::DECIMAL, false);
}

pub fn cli(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    cpu.status.set(Status::IRQ_DISABLE, false);
}

pub fn clv(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    cpu.status.set(Status::OVERFLOW, false);
}

pub fn cmp(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    match operand {
        AddressResult::Memory(addr) => {
            let param = cpu.read(bus, addr.effective_address, observer);
            compare(cpu, cpu.a, param);
        }
        _ => panic!("invalid memory mode"),
    }
}

pub fn cpx(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    match operand {
        AddressResult::Memory(addr) => {
            let param = cpu.read(bus, addr.effective_address, observer);
            compare(cpu, cpu.x, param);
        }
        _ => panic!("invalid memory mode"),
    }
}

pub fn cpy(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    match operand {
        AddressResult::Memory(addr) => {
            let param = cpu.read(bus, addr.effective_address, observer);
            compare(cpu, cpu.y, param);
        }
        _ => panic!("invalid memory mode"),
    }
}

pub fn dec(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    match operand {
        AddressResult::Memory(addr) => {
            let param = cpu.read(bus, addr.effective_address, observer);
            let result = decrement_helper(cpu, param);
            cpu.write(bus, addr.effective_address, result, observer);
        }
        _ => panic!("invalid memory mode"),
    }
}

pub fn dex(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    cpu.x = decrement_helper(cpu, cpu.x);
}

pub fn dey(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    cpu.y = decrement_helper(cpu, cpu.y);
}

pub fn eor(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    match operand {
        AddressResult::Memory(addr) => {
            let param = cpu.read(bus, addr.effective_address, observer);
            let accumulator = cpu.a;
            let result = accumulator ^ param;
            cpu.a = result;
            cpu.set_zn(result);
        }
        _ => panic!("invalid memory mode"),
    }
}

pub fn inc(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    match operand {
        AddressResult::Memory(addr) => {
            let param = cpu.read(bus, addr.effective_address, observer);
            let result = increment_helper(cpu, param);
            cpu.write(bus, addr.effective_address, result, observer);
        }
        _ => panic!("invalid memory mode"),
    }
}

pub fn inx(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    cpu.x = increment_helper(cpu, cpu.x);
}

pub fn iny(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    cpu.y = increment_helper(cpu, cpu.y);
}

pub fn jmp(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    match operand {
        AddressResult::Memory(addr) => {
            cpu.pc = addr.effective_address;
            // todo!("Implement the hardware bug regarding crossing pages");
        }
        _ => panic!("invalid memory mode"),
    }
}

pub fn jsr(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    match operand {
        AddressResult::Memory(addr) => {
            let return_addr = cpu.pc.wrapping_sub(1);
            let bytes = return_addr.to_le_bytes();
            cpu.stack_push(bus, bytes[1]);
            cpu.stack_push(bus, bytes[0]);
            cpu.pc = addr.effective_address;
        }
        _ => panic!("invalid memory mode"),
    }
}

pub fn lda(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    match operand {
        AddressResult::Memory(addr) => {
            cpu.a = cpu.read(bus, addr.effective_address, observer);
            cpu.set_zn(cpu.a);
        }
        _ => panic!("invalid memory mode"),
    }
}

pub fn ldx(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    match operand {
        AddressResult::Memory(addr) => {
            cpu.x = cpu.read(bus, addr.effective_address, observer);
            cpu.set_zn(cpu.x);
        }
        _ => panic!("invalid memory mode"),
    }
}

pub fn ldy(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    match operand {
        AddressResult::Memory(addr) => {
            cpu.y = cpu.read(bus, addr.effective_address, observer);
            cpu.set_zn(cpu.y);
        }
        _ => panic!("invalid memory mode"),
    }
}

pub fn lsr(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    let param = match &operand {
        AddressResult::Accumulator => cpu.a,
        AddressResult::Memory(addr) => cpu.read(bus, addr.effective_address, observer),
        _ => panic!("invalid memory mode"),
    };
    let result = param >> 1;
    cpu.status.set(Status::CARRY, param & LSB_BIT != 0);
    cpu.set_zn(result);
    match operand {
        AddressResult::Accumulator => cpu.a = result,
        AddressResult::Memory(addr) => cpu.write(bus, addr.effective_address, result, observer),
        _ => panic!("invalid memory mode"),
    };
}

pub fn nop(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    // No operation
}

pub fn ora(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    match operand {
        AddressResult::Memory(addr) => {
            let param = cpu.read(bus, addr.effective_address, observer);
            cpu.a |= param;
            cpu.set_zn(cpu.a);
        }
        _ => panic!("invalid memory mode"),
    }
}

pub fn pha(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    _operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    cpu.stack_push(bus, cpu.a);
}

pub fn php(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    _operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    let value = cpu.status.bits();
    cpu.stack_push(bus, value);
}

pub fn pla(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    _operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    cpu.a = cpu.stack_pop(bus);
}

pub fn plp(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    _operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    let value = cpu.stack_pop(bus);
    cpu.status = Status::from_bits_truncate(value);
}

pub fn rol(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    let param = match &operand {
        AddressResult::Accumulator => cpu.a,
        AddressResult::Memory(addr) => cpu.read(bus, addr.effective_address, observer),
        _ => panic!("invalid memory mode"),
    };
    let mut result = param << 1;
    if cpu.status.contains(Status::CARRY) {
        result |= 1;
    }
    cpu.status.set(Status::CARRY, param & MSB_BIT != 0);
    cpu.set_zn(result);
    match operand {
        AddressResult::Accumulator => cpu.a = result,
        AddressResult::Memory(addr) => cpu.write(bus, addr.effective_address, result, observer),
        _ => panic!("invalid memory mode"),
    };
}

pub fn ror(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    let param = match &operand {
        AddressResult::Accumulator => cpu.a,
        AddressResult::Memory(addr) => cpu.read(bus, addr.effective_address, observer),
        _ => panic!("invalid memory mode"),
    };
    let mut result = param >> 1;
    if cpu.status.contains(Status::CARRY) {
        result |= 0x80;
    }
    cpu.status.set(Status::CARRY, param & LSB_BIT != 0);
    cpu.set_zn(result);
    match operand {
        AddressResult::Accumulator => cpu.a = result,
        AddressResult::Memory(addr) => cpu.write(bus, addr.effective_address, result, observer),
        _ => panic!("invalid memory mode"),
    };
}

pub fn rti(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    todo!("RTI not implemented");
}

pub fn rts(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    let mut bytes = [0, 0];
    bytes[0] = cpu.stack_pop(_bus);
    bytes[1] = cpu.stack_pop(_bus);
    cpu.pc = u16::from_le_bytes(bytes).wrapping_add(1);
}

pub fn sbc(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    match operand {
        AddressResult::Memory(addr) => {
            let param = !cpu.read(bus, addr.effective_address, observer);
            adc_helper(cpu, bus, param);
        }
        _ => panic!("invalid memory mode"),
    }
}

pub fn sec(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    cpu.status.set(Status::CARRY, true);
}

pub fn sed(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    cpu.status.set(Status::DECIMAL, true);
}

pub fn sei(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    cpu.status.set(Status::IRQ_DISABLE, true);
}

pub fn sta(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    match operand {
        AddressResult::Memory(addr) => {
            cpu.write(bus, addr.effective_address, cpu.a, observer);
        }
        _ => panic!("invalid memory mode"),
    }
}

pub fn stx(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    match operand {
        AddressResult::Memory(addr) => {
            cpu.write(bus, addr.effective_address, cpu.x, observer);
        }
        _ => panic!("invalid memory mode"),
    }
}

pub fn sty(
    cpu: &mut Cpu6502,
    bus: &mut Bus,
    operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    match operand {
        AddressResult::Memory(addr) => {
            cpu.write(bus, addr.effective_address, cpu.y, observer);
        }
        _ => panic!("invalid memory mode"),
    }
}

pub fn tax(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    cpu.x = cpu.a;
    cpu.set_zn(cpu.x);
}

pub fn tay(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    cpu.y = cpu.a;
    cpu.set_zn(cpu.y);
}

pub fn tsx(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    cpu.x = cpu.sp;
    cpu.set_zn(cpu.x);
}

pub fn txa(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    cpu.a = cpu.x;
    cpu.set_zn(cpu.a);
}

pub fn txs(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    cpu.sp = cpu.x;
    cpu.set_zn(cpu.sp);
}

pub fn tya(
    cpu: &mut Cpu6502,
    _bus: &mut Bus,
    _operand: AddressResult,
    observer: &mut Option<Box<dyn CpuObserver>>,
) {
    cpu.a = cpu.y;
    cpu.set_zn(cpu.a);
}

fn adc_helper(cpu: &mut Cpu6502, bus: &mut Bus, param: u8) {
    let accumulator = cpu.a;
    let mut carry2 = false;
    let (mut result, carry1) = accumulator.overflowing_add(param);
    if cpu.status.contains(Status::CARRY) {
        (result, carry2) = result.overflowing_add(1);
    }
    cpu.status.set(Status::CARRY, carry1 || carry2);
    cpu.status.set(
        Status::OVERFLOW,
        ((result ^ accumulator) & (result ^ param) & MSB_BIT) != 0,
    );
    cpu.set_zn(result);
    cpu.a = result;
}

fn compare(cpu: &mut Cpu6502, register: u8, param: u8) {
    cpu.status.set(Status::CARRY, register >= param);
    cpu.status.set(Status::ZERO, register == param);
    cpu.status.set(Status::NEGATIVE, register < param);
}

fn decrement_helper(cpu: &mut Cpu6502, param: u8) -> u8 {
    let result = param.wrapping_sub(1);
    cpu.set_zn(result);
    result
}

fn increment_helper(cpu: &mut Cpu6502, param: u8) -> u8 {
    let result = param.wrapping_add(1);
    cpu.set_zn(result);
    result
}

fn branch_helper(cpu: &mut Cpu6502, address: u16, condition: bool) {
    if condition {
        cpu.pc = address;
        cpu.cycles += 1;
    }
}
