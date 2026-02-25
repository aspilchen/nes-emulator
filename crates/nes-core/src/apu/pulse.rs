use super::envelope::Envelope;

const DUTY_SEQUENCES: [[u8; 8]; 4] = [
    [0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 1, 1, 1],
    [0, 1, 1, 1, 1, 1, 1, 0],
];

const LENGTH_TABLE: [u8; 32] = [
    10, 254, 20, 2, 40, 4, 80, 6, 160, 8, 60, 10, 14, 12, 26, 14, 12, 16, 24, 18, 48, 20, 96, 22,
    192, 24, 72, 26, 16, 28, 32, 30,
];

pub struct Pulse {
    pub duty: u8,
    pub length_halt: bool,
    pub is_constant_volume: bool,
    pub constant_volume: u8,

    envelope: Envelope,

    pub sweep_enabled: bool,
    pub sweep_period: u8,
    pub sweep_negate: bool,
    pub sweep_shift: u8,
    pub sweep_reload: bool,

    timer_latch: u16,
    pub length_load: u8,

    pub timer: u16,
    pub target_timer: u16,

    pub sweep_divider: u8,

    pub length_counter: u8,

    pub duty_index: u8,
    pub timer_counter: u16,

    pub enabled: bool,
}

impl Pulse {
    pub fn new() -> Self {
        Self {
            duty: 0,
            length_halt: false,
            is_constant_volume: false,
            constant_volume: 0,
            envelope: Envelope::new(),
            sweep_enabled: false,
            sweep_period: 0,
            sweep_negate: false,
            sweep_shift: 0,
            sweep_reload: false,

            timer_latch: 0,
            length_load: 0,

            timer: 0,
            target_timer: 0,

            sweep_divider: 0,

            length_counter: 0,

            duty_index: 0,
            timer_counter: 0,

            enabled: false,
        }
    }

    pub fn write_control(&mut self, value: u8) {
        self.duty = (value >> 6) & 0b11;
        self.length_halt = (value & 0b0010_0000) != 0;
        self.is_constant_volume = (value & 0b0001_0000) != 0;
        self.constant_volume = value & 0b0000_1111;
        self.envelope.set_period(value & 0b0000_1111);
    }

    pub fn write_sweep(&mut self, value: u8) {
        self.sweep_enabled = (value & 0b1000_0000) != 0;
        self.sweep_period = (value >> 4) & 0b111;
        self.sweep_negate = (value & 0b0000_1000) != 0;
        self.sweep_shift = value & 0b111;
        self.sweep_reload = true;
    }

    pub fn write_timer_low(&mut self, value: u8) {
        self.timer_latch &= 0xFF00;
        self.timer_latch |= value as u16;
    }

    pub fn write_timer_high(&mut self, value: u8) {
        self.timer_latch &= 0x00FF;
        self.timer_latch |= (value as u16) << 8;
        if self.enabled {
            self.length_counter = LENGTH_TABLE[self.length_load as usize];
        }
        self.duty_index = 0;
        self.envelope.is_reset_pending = true;
    }

    pub fn step(&mut self) {
        if self.timer == 0 {
            self.increment_duty_index();
            self.timer = self.timer_latch;
        } else {
            self.decrement_timer();
        }
    }

    pub fn read_output(&self) -> u8 {
        let is_muted = self.length_counter == 0 || self.timer_latch < 8;
        if is_muted {
            0
        } else if self.is_constant_volume {
            self.constant_volume
        } else {
            self.envelope.read()
        }
    }

    pub fn decrement_timer(&mut self) {
        self.timer -= 1;
    }

    pub fn increment_duty_index(&mut self) {
        self.duty_index = (self.duty_index + 1) % 8;
    }

    pub fn decrement_length_counter(&mut self) {
        if self.length_counter == 0 {
            return;
        }
        self.length_counter -= 1;
    }
}
