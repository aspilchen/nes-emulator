use crate::cpu::{cpu6502::CpuState, AddressResult, Cpu6502, Op};

pub struct MemoryAccess {
    pub address: u16,
    pub value: u8,
}

pub struct Collector {
    pub cpu_snapshot: CpuState,
    pub bytes_fetched: Vec<MemoryAccess>,
    pub bytes_read: Vec<MemoryAccess>,
    pub bytes_write: Vec<MemoryAccess>,
    pub bytes_overwrite: Vec<MemoryAccess>,
    pub op_name: Op,
    pub undocumented: bool,
    pub address_result: AddressResult,
    pub cycles: u64,
}

impl Collector {
    pub fn new(cpu: &Cpu6502) -> Self {
        Collector {
            cpu_snapshot: CpuState::from(cpu),
            bytes_fetched: Vec::new(),
            bytes_read: Vec::new(),
            bytes_write: Vec::new(),
            bytes_overwrite: Vec::new(),
            op_name: Default::default(),
            undocumented: false,
            address_result: AddressResult::Implied,
            cycles: 0,
        }
    }
}
