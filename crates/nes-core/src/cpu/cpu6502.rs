use crate::cpu::addressing::AddressResult;
use crate::cpu::bus::Bus;
use crate::cpu::collector::{Collector, MemoryAccess};
use crate::cpu::instruction::Instruction;
use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy)]
    pub struct Status: u8 {
        const CARRY       = 0b0000_0001;
        const ZERO        = 0b0000_0010;
        const IRQ_DISABLE = 0b0000_0100;
        const DECIMAL     = 0b0000_1000;
        const BREAK       = 0b0001_0000;
        const UNUSED      = 0b0010_0000;
        const OVERFLOW    = 0b0100_0000;
        const NEGATIVE    = 0b1000_0000;
    }
}

const STACK_BASE: u16 = 0x0100;
const RESET_VECTOR_LOW: u16 = 0xFFFC;
const RESET_VECTOR_HIGH: u16 = 0xFFFD;

pub struct CpuState {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub sp: u8,
    pub status: Status,
    pub cycles: u64,
}

pub struct Cpu6502 {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub sp: u8,
    pub status: Status,
    pub cycles: u64,
    pub collector: Option<Collector>,
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
            collector: None,
        }
    }

    pub fn reset(&mut self, bus: Bus) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.pc = bus.cart.reset_vector();
        self.sp = 0xFD;
        self.status = Status::default();
        self.cycles = 7;
        self.collector = None;
    }

    pub fn hardware_interrupt(&mut self, mut bus: Bus, address: u16) {
        let pc_bytes = self.pc.to_le_bytes();
        self.stack_push(&mut bus, pc_bytes[1]);
        self.stack_push(&mut bus, pc_bytes[0]);
        let status = self.status & !Status::BREAK;
        self.stack_push(&mut bus, status.bits());
        self.status.set(Status::IRQ_DISABLE, true);
        self.pc = address
    }

    pub fn step(&mut self, mut bus: Bus) -> Option<Collector> {
        self.collector = Some(Collector::new(self));
        let opcode = self.fetch(&mut bus);
        let instruction: Instruction = self.decode(opcode);
        let address_result = (instruction.resolve_address)(self, &mut bus);
        self.execute(&instruction, &address_result, &mut bus);
        if let Some(collector) = &mut self.collector {
            collector.address_result = address_result;
        }
        self.collector.take()
    }

    pub fn fetch(&mut self, bus: &mut Bus) -> u8 {
        let value = bus.read(self.pc);
        if let Some(collector) = &mut self.collector {
            collector.bytes_fetched.push(MemoryAccess {
                address: self.pc,
                value,
            });
        }
        self.increment_pc(1);
        value
    }

    pub fn decode(&mut self, opcode: u8) -> Instruction {
        let instruction = Instruction::from(opcode);
        if let Some(collector) = &mut self.collector {
            collector.op_name = instruction.name;
            collector.undocumented = instruction.undocumented;
        }
        instruction
    }

    pub fn execute(
        &mut self,
        instruction: &Instruction,
        address_result: &AddressResult,
        bus: &mut Bus,
    ) {
        let result = (instruction.execute)(self, bus, address_result);
        self.tick(instruction.cycles + result.extra_cycles);
    }

    pub fn read(&mut self, bus: &mut Bus, address: u16) -> u8 {
        let value = bus.read(address);
        if let Some(collector) = &mut self.collector {
            collector.bytes_read.push(MemoryAccess { address, value });
        }
        value
    }

    pub fn write(&mut self, bus: &mut Bus, address: u16, value: u8) {
        if let Some(collector) = &mut self.collector {
            // let curr_value = bus.read(address);
            // collector.bytes_overwrite.push(MemoryAccess {
            // address,
            // value: curr_value,
            // });
            bus.write(address, value);
            collector.bytes_write.push(MemoryAccess { address, value });
        }
    }

    pub fn stack_push(&mut self, bus: &mut Bus, value: u8) {
        let address = STACK_BASE.wrapping_add(self.sp as u16);
        bus.write(address, value);
        self.sp = self.sp.wrapping_sub(1);
    }

    pub fn stack_pop(&mut self, bus: &mut Bus) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        let address = STACK_BASE.wrapping_add(self.sp as u16);
        bus.read(address)
    }

    pub fn increment_pc(&mut self, value: u16) {
        self.pc = self.pc.wrapping_add(value);
    }

    pub fn set_zn(&mut self, value: u8) {
        self.status.set(Status::ZERO, value == 0);
        self.status.set(Status::NEGATIVE, (value & 0x80) != 0);
    }

    fn tick(&mut self, cycles: u64) {
        self.cycles += cycles;
        if let Some(collector) = &mut self.collector {
            collector.cycles = cycles;
        }
    }
}

impl Default for Status {
    fn default() -> Self {
        Status::UNUSED | Status::IRQ_DISABLE
    }
}

impl From<&Cpu6502> for CpuState {
    fn from(value: &Cpu6502) -> Self {
        Self {
            a: value.a,
            x: value.x,
            y: value.y,
            pc: value.pc,
            sp: value.sp,
            status: value.status,
            cycles: value.cycles,
        }
    }
}
