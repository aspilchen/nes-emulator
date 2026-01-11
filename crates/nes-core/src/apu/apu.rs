// Mock to pass nestest
pub const VOICE_BEGIN: u16 = 0x4000;
pub const VOICE_END: u16 = 0x4013;
pub const ENABLE_LEN: u16 = 0x4015;
pub const FRAME_COUNTER: u16 = 0x4017;

pub struct Apu {
    pub status: u8,
    pub duty: u8,
    pub sweep: u8,
    pub timer_low: u8,
    pub length: u8,
}

impl Apu {
    pub fn new() -> Self {
        Self {
            status: 0,
            duty: 0,
            sweep: 0,
            timer_low: 0,
            length: 0,
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        0xFF
        // match address {
        //     0x4015 => self.status,
        //     0x4004 => self.duty,
        //     0x4005 => self.sweep,
        //     0x4006 => self.timer_low,
        //     0x4007 => self.length,
        //     _ => panic!("error"),
        // }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0x4015 => self.status = value,
            0x4004 => self.duty = value,
            0x4005 => self.sweep = value,
            0x4006 => self.timer_low = value,
            0x4007 => self.length = value,
            _ => {},
        }
    }
}
