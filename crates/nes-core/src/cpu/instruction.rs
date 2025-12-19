use crate::bus::Bus;
use crate::cpu::cpu6502::Status;
use crate::cpu::{addressing::AddressResult, cpu6502::Cpu6502};

type AddressFn = fn(&mut Cpu6502, &mut Bus) -> AddressResult;
type ExecutFn = fn(&mut Cpu6502, &mut Bus, Option<AddressResult>);
type TraceFn = fn(&Cpu6502, AddressResult);

#[derive(Debug)]
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

pub struct Instruction {
    name: String,
    address: AddressFn,
    execute: ExecutFn,
    trace: TraceFn,
    cycles: u8,
}

pub fn adc(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let addr = address.expect("addressing error");
    let param = cpu.read(bus, addr.address);
    adc_helper(bus, param);
}

pub fn and(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let addr = address.expect("addressing error");
    let param = cpu.read(bus, addr.address);
    cpu.a &= param;
    // test_and_set_negative_flag(cpu, result);
    // test_and_set_zero_flag(cpu, result);
}

pub fn asl(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let address = &address;
    let param = match address {
        Some(a) => cpu.read(bus, a.address),
        _ => cpu.a,
    };
    // const BITMASK: u8 = 0x80;
    // let is_carry = (param & BITMASK) != 0;
    let result = param << 1;
    // bus.cpu.status.set_flags(Status::Carry, is_carry);
    // test_and_set_negative_flag(&mut bus.cpu, result);
    // test_and_set_zero_flag(&mut bus.cpu, result);
    match address {
        Some(a) => cpu.write(bus, a.address, result),
        _ => cpu.a = result,
    };
}

pub fn bcc(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    if !cpu.status.contains(Status::CARRY) {
        cpu.pc = address.expect("address error").address;
    }
}

pub fn bcs(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    if cpu.status.contains(Status::CARRY) {
        cpu.pc = address.expect("address error").address;
    }
}

pub fn beq(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    if cpu.status.contains(Status::ZERO) {
        cpu.pc = address.expect("address error").address;
    }
}

pub fn bit(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let addr = address.expect("addressing error");
    let param = cpu.read(bus, addr.address);
    let result = cpu.a & param;
    // test_and_set_zero_flag(cpu, result);
    // test_and_set_negative_flag(cpu, result);
    let is_overflow = result & 0b01000000 != 0;
    // cpu.status.set_flags(Status::Overflow, is_overflow);
}

pub fn bmi(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    if cpu.status.contains(Status::NEGATIVE) {
        cpu.pc = address.expect("addressing error").address;
    }
}

pub fn bne(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    if !cpu.status.contains(Status::ZERO) {
        cpu.pc = address.expect("addressing error").address;
    }
}

pub fn bpl(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    if !cpu.status.contains(Status::NEGATIVE) {
        cpu.pc = address.expect("addressing error").address;
    }
}

pub fn brk(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    todo!("BRK not implemented");
}

pub fn bvc(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let is_overflow = bus.cpu.status.contains(Status::Overflow);
    if (!is_overflow) {
        bus.cpu.program_counter = address as u16;
    }
}

pub fn bvs(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let is_overflow = bus.cpu.status.contains(Status::Overflow);
    if (is_overflow) {
        bus.cpu.program_counter = address as u16;
    }
}

pub fn clc(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    bus.cpu.status &= Status::InvertedCarry;
}

pub fn cld(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    bus.cpu.status &= Status::InvertedDecimal;
}

pub fn cli(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    bus.cpu.status &= Status::InvertedInturruptDisable;
}

pub fn clv(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    bus.cpu.status &= Status::InvertedOverflow;
}

pub fn cmp(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let addr = address.expect("addressing error");
    let param = cpu.read(bus, addr.address);
    let cpu = &mut bus.cpu;
    let accumulator = cpu.accumulator;
    let is_carry = accumulator >= param;
    cpu.status.set_flags(Status::Carry, is_carry);
    let is_zero = accumulator == param;
    cpu.status.set_flags(Status::Zero, is_zero);
    let is_negative = accumulator < param;
    cpu.status.set_flags(Status::Negative, is_negative);
}

pub fn cpx(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let addr = address.expect("addressing error");
    let param = cpu.read(bus, addr.address);
    let cpu = &mut bus.cpu;
    let index_x = cpu.index_x;
    let is_carry = index_x >= param;
    cpu.status.set_flags(Status::Carry, is_carry);
    let is_zero = index_x == param;
    cpu.status.set_flags(Status::Zero, is_zero);
    let is_negative = index_x < param;
    cpu.status.set_flags(Status::Negative, is_negative);
}

pub fn cpy(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let addr = address.expect("addressing error");
    let param = cpu.read(bus, addr.address);
    let cpu = &mut bus.cpu;
    let index_y = cpu.index_y;
    let is_carry = index_y >= param;
    cpu.status.set_flags(Status::Carry, is_carry);
    let is_zero = index_y == param;
    cpu.status.set_flags(Status::Zero, is_zero);
    let is_negative = index_y < param;
    cpu.status.set_flags(Status::Negative, is_negative);
}

pub fn dec(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let addr = address.expect("addressing error");
    let param = cpu.read(bus, addr.address);
    let result = param.wrapping_sub(1);
    bus.write(address, result);
    let cpu = &mut bus.cpu;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
}

pub fn dex(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let cpu = &mut bus.cpu;
    let param = cpu.index_x;
    let result = param.wrapping_sub(1);
    cpu.index_x = result;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
}

pub fn dey(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let cpu = &mut bus.cpu;
    let param = cpu.index_y;
    let result = param.wrapping_sub(1);
    cpu.index_y = result;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
}

pub fn eor(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let addr = address.expect("addressing error");
    let param = cpu.read(bus, addr.address);
    let cpu = &mut bus.cpu;
    let accumulator = cpu.accumulator;
    let result = accumulator ^ param;
    cpu.accumulator = result;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
}

pub fn inc(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let addr = address.expect("addressing error");
    let param = cpu.read(bus, addr.address);
    let result = param.wrapping_add(1);
    bus.write(address, result);
    let cpu = &mut bus.cpu;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
}

pub fn inx(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let cpu = &mut bus.cpu;
    let param = cpu.index_x;
    let result = param.wrapping_add(1);
    cpu.index_x = result;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
}

pub fn iny(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let cpu = &mut bus.cpu;
    let param = cpu.index_y;
    let result = param.wrapping_add(1);
    cpu.index_y = result;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
}

pub fn jmp(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    bus.cpu.program_counter = address as u16;
    // todo!("Implement the hardware bug regarding crossing pages");
}

pub fn jsr(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let param = decode_address(bus, address_mode);
    let cpu = &mut bus.cpu;
    let bytes = cpu.program_counter.wrapping_sub(1).to_le_bytes();
    cpu.stack_push(bytes[1]);
    cpu.stack_push(bytes[0]);
    bus.cpu.program_counter = param as u16;
}

pub fn lda(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let addr = address.expect("addressing error");
    let param = cpu.read(bus, addr.address);
    let cpu = &mut bus.cpu;
    cpu.accumulator = param;
    test_and_set_negative_flag(cpu, param);
    test_and_set_zero_flag(cpu, param);
}

pub fn ldx(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let addr = address.expect("addressing error");
    let param = cpu.read(bus, addr.address);
    let cpu = &mut bus.cpu;
    cpu.index_x = param;
    test_and_set_negative_flag(cpu, param);
    test_and_set_zero_flag(cpu, param);
}

pub fn ldy(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let addr = address.expect("addressing error");
    let param = cpu.read(bus, addr.address);
    let cpu = &mut bus.cpu;
    cpu.index_y = param;
    test_and_set_negative_flag(cpu, param);
    test_and_set_zero_flag(cpu, param);
}

pub fn lsr(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let (param, address) = match address_mode {
        AddressMode::Accumulator => (bus.cpu.accumulator, 0),
        _ => {
            let temp = cpu.read(bus, address.address);
            (temp, address)
        }
    };
    const BITMASK: u8 = 0x01;
    let is_carry = (param & BITMASK) != 0;
    let result = param >> 1;
    bus.cpu.status.set_flags(Status::Carry, is_carry);
    bus.cpu.status &= Status::InvertedNegative;
    test_and_set_zero_flag(&mut bus.cpu, result);
    match address_mode {
        AddressMode::Accumulator => bus.cpu.accumulator = result,
        _ => {
            bus.write(address, result);
        }
    };
}

pub fn nop(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    // todo!("NOP not implemented");
}

pub fn ora(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let addr = address.expect("addressing error");
    let param = cpu.read(bus, addr.address);
    let cpu = &mut bus.cpu;
    let accumulator = cpu.accumulator;
    let result = accumulator | param;
    cpu.accumulator = result;
    test_and_set_negative_flag(cpu, result);
    test_and_set_zero_flag(cpu, result);
}

pub fn pha(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let value = bus.cpu.accumulator;
    bus.cpu.stack_push(value);
}

pub fn php(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let value = bus.cpu.status.bits();
    bus.cpu.stack_push(value);
}

pub fn pla(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let value = bus.cpu.stack_pop();
    bus.cpu.accumulator = value;
}

pub fn plp(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let value = bus.cpu.stack_pop();
    bus.cpu.status = Status::from(value);
}

pub fn rol(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let (param, address) = match address_mode {
        AddressMode::Accumulator => (bus.cpu.accumulator, 0),
        _ => {
            let temp = cpu.read(bus, address.address);
            (temp, address)
        }
    };
    let bitmask = 0x80;
    let is_carry = (param & bitmask) != 0;
    let mut result = param << 1;
    if (bus.cpu.status.contains(Status::Carry)) {
        result += 1;
    }
    bus.cpu.status.set_flags(Status::Carry, is_carry);
    test_and_set_negative_flag(&mut bus.cpu, result);
    test_and_set_zero_flag(&mut bus.cpu, result);
    match address_mode {
        AddressMode::Accumulator => bus.cpu.accumulator = result,
        _ => {
            bus.write(address, result);
        }
    };
}

pub fn ror(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let (param, address) = match address_mode {
        AddressMode::Accumulator => (bus.cpu.accumulator, 0),
        _ => {
            let temp = cpu.read(bus, address.address);
            (temp, address)
        }
    };
    let bitmask = 0x01;
    let is_carry = (param & bitmask) != 0;
    let mut result = param >> 1;
    if (bus.cpu.status.contains(Status::Carry)) {
        result += 0x80;
    }
    bus.cpu.status.set_flags(Status::Carry, is_carry);
    test_and_set_negative_flag(&mut bus.cpu, result);
    test_and_set_zero_flag(&mut bus.cpu, result);
    match address_mode {
        AddressMode::Accumulator => bus.cpu.accumulator = result,
        _ => {
            bus.write(address, result);
        }
    };
}

pub fn rti(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    todo!("RTI not implemented");
}

pub fn rts(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let mut bytes = [0, 0];
    bytes[0] = bus.cpu.stack_pop();
    bytes[1] = bus.cpu.stack_pop();
    bus.cpu.program_counter = u16::from_le_bytes(bytes).wrapping_add(1);
}

pub fn sbc(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let param = !cpu.read(bus, address.address);
    adc_helper(bus, param);
}

pub fn sec(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    bus.cpu.status |= Status::Carry;
}

pub fn sed(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    bus.cpu.status |= Status::Decimal;
}

pub fn sei(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    bus.cpu.status |= Status::InturruptDisable;
}

pub fn sta(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let value = bus.cpu.accumulator;
    bus.write(address, value);
}

pub fn stx(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let value = bus.cpu.index_x;
    bus.write(address, value);
}

pub fn sty(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let value = bus.cpu.index_y;
    bus.write(address, value);
}

pub fn tax(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let cpu = &mut bus.cpu;
    let value = cpu.accumulator;
    cpu.index_x = value;
    test_and_set_negative_flag(cpu, value);
    test_and_set_zero_flag(cpu, value);
}

pub fn tay(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let cpu = &mut bus.cpu;
    let value = cpu.accumulator;
    cpu.index_y = value;
    test_and_set_negative_flag(cpu, value);
    test_and_set_zero_flag(cpu, value);
}

pub fn tsx(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let cpu = &mut bus.cpu;
    let value = cpu.stack_ptr;
    cpu.index_x = value;
    test_and_set_negative_flag(cpu, value);
    test_and_set_zero_flag(cpu, value);
}

pub fn txa(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let cpu = &mut bus.cpu;
    let value = cpu.index_x;
    cpu.accumulator = value;
    test_and_set_negative_flag(cpu, value);
    test_and_set_zero_flag(cpu, value);
}

pub fn txs(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let cpu = &mut bus.cpu;
    let value = cpu.index_x;
    cpu.stack_ptr = value;
}

pub fn tya(cpu: &mut Cpu6502, bus: &mut Bus, address: Option<AddressResult>) {
    let cpu = &mut bus.cpu;
    let value = cpu.index_y;
    cpu.accumulator = value;
    test_and_set_negative_flag(cpu, value);
    test_and_set_zero_flag(cpu, value);
}

fn test_and_set_negative_flag(cpu: &mut Ricoh6502, result: u8) {
    let is_negative = (result as i8) < 0;
    cpu.status.set_flags(Status::Negative, is_negative);
}

fn test_and_set_overflow_flag(cpu: &mut Ricoh6502, acc: u8, param: u8, result: u8) {
    let is_overflow = ((result ^ acc) & (result ^ param) & 0x80) != 0;
    cpu.status.set_flags(Status::Overflow, is_overflow);
}

fn test_and_set_zero_flag(cpu: &mut Ricoh6502, result: u8) {
    let is_zero = result == 0;
    cpu.status.set_flags(Status::Zero, is_zero);
}

fn adc_helper(bus: &mut Bus, param: u8) {
    let cpu = &mut bus.cpu;
    let accumulator = cpu.accumulator;
    let mut carry2 = false;
    let (mut result, carry1) = accumulator.overflowing_add(param);
    if (cpu.status.contains(Status::Carry)) {
        (result, carry2) = result.overflowing_add(1);
    }
    let is_carry = carry1 || carry2;
    cpu.status.set_flags(Status::Carry, is_carry);
    test_and_set_overflow_flag(cpu, accumulator, param, result);
    test_and_set_zero_flag(cpu, result);
    test_and_set_negative_flag(cpu, result);
    cpu.accumulator = result;
}
