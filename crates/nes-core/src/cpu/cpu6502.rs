use crate::bus::Bus;
use bitflags::bitflags;

bitflags! {
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

pub struct Cpu6502 {
    pub a: u8,
    pub x: u8,
    pub y: u8,

    pub pc: u16,
    pub sp: u8,
    pub status: Status,

    pub cycles: u8,
}

impl Cpu6502 {
    pub fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0xFD,
            status: Status::default(),
            cycles: 0,
        }
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

    pub fn increment_pc(&mut self, value: u16) {
        self.pc.wrapping_add(value);
    }

    fn get_reset_vector(&mut self, bus: &mut Bus) -> u16 {
        let addr_low = 0xFFFC;
        let addr_high = 0xFFFD;
        let low_byte = bus.cpu_read(addr_low) as u16;
        let high_byte = bus.cpu_read(addr_high) as u16;
        (high_byte << 4) | low_byte
    }
}

impl Default for Status {
    fn default() -> Self {
        Status::UNUSED | Status::IRQ_DISABLE
    }
}
