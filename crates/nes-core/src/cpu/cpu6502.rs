use crate::bus::Bus;
use crate::cpu::instruction::Op;
use crate::cpu::{addressing::AddressResult, instruction::Instruction};
use crate::observers::{CpuTraceDetails, NesObserver};
use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy)]
    pub struct Status: u8 {
        const CARRY     = 0b0000_0001;
        const ZERO      = 0b0000_0010;
        const IRQ_DISABLE = 0b0000_0100;
        const DECIMAL   = 0b0000_1000;
        const BREAK     = 0b0001_0000;
        const UNUSED    = 0b0010_0000;
        const OVERFLOW  = 0b0100_0000;
        const NEGATIVE  = 0b1000_0000;
    }
}

const STACK_BASE: u16 = 0x0100;
const RESET_VECTOR_LOW: u16 = 0xFFFC;
const RESET_VECTOR_HIGH: u16 = 0xFFFD;

#[derive(Clone, Copy)]
pub struct Cpu6502 {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub sp: u8,
    pub status: Status,
    pub cycles: usize,
}

impl Cpu6502 {
    pub fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            pc: 0xC000,
            sp: 0xFD,
            status: Status::default(),
            cycles: 7,
        }
    }

    pub fn step(&mut self, bus: &mut Bus, observer: &mut Option<Box<dyn NesObserver>>) -> u64 {
        let cpu_snapshot = self.clone();
        let opcode = self.fetch(bus);
        let instruction: Instruction = self.decode(opcode);
        let operand = (instruction.operand)(self, bus);
        let mut trace_details = CpuTraceDetails {
            cpu_snapshot,
            instruction: instruction.clone(),
            operand: operand.clone(),
            value: None,
        };
        self.execute(&instruction, operand, bus, Some(&mut trace_details));

        if let Some(observer) = observer {
            observer.on_cpu(trace_details);
        }
        instruction.cycles as u64
    }

    pub fn decode(&mut self, opcode: u8) -> Instruction {
        Instruction::from(opcode)
    }

    pub fn execute(
        &mut self,
        instruction: &Instruction,
        operand: AddressResult,
        bus: &mut Bus,
        trace_details: Option<&mut CpuTraceDetails>,
    ) {
        (instruction.execute)(self, bus, operand, trace_details);
        self.cycles += instruction.cycles as usize;
    }

    pub fn reset(&mut self, bus: &mut Bus) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.pc = self.get_reset_vector(bus);
        self.sp = 0xFD;
        self.status = Status::default();
        self.cycles = 7;
    }

    pub fn fetch(&mut self, bus: &mut Bus) -> u8 {
        let result = bus.cpu_read(self.pc);
        self.increment_pc(1);
        result
    }

    pub fn read(&mut self, bus: &mut Bus, address: u16) -> u8 {
        bus.cpu_read(address)
    }

    pub fn write(&mut self, bus: &mut Bus, address: u16, value: u8) {
        bus.cpu_write(address, value);
    }

    pub fn stack_push(&mut self, bus: &mut Bus, value: u8) {
        let address = STACK_BASE.wrapping_add(self.sp as u16);
        bus.cpu_write(address, value);
        self.sp = self.sp.wrapping_sub(1);
    }

    pub fn stack_pop(&mut self, bus: &mut Bus) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        let address = STACK_BASE.wrapping_add(self.sp as u16);
        bus.cpu_read(address)
    }

    pub fn increment_pc(&mut self, value: u16) {
        self.pc = self.pc.wrapping_add(value);
    }

    pub fn set_zn(&mut self, value: u8) {
        self.status.set(Status::ZERO, value == 0);
        self.status.set(Status::NEGATIVE, (value & 0x80) != 0);
    }

    fn get_reset_vector(&mut self, bus: &mut Bus) -> u16 {
        let low_byte = bus.cpu_read(RESET_VECTOR_LOW) as u16;
        let high_byte = bus.cpu_read(RESET_VECTOR_HIGH) as u16;
        (high_byte << 8) | low_byte
    }
}

impl Default for Status {
    fn default() -> Self {
        Status::UNUSED | Status::IRQ_DISABLE
    }
}
