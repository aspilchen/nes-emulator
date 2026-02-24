const DUTY_SEQUENCES: [[u8; 8]; 4] = [
    [0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 1, 1, 1],
    [0, 1, 1, 1, 1, 1, 1, 0],
];

pub struct Pulse {
    pub duty: u8,
    pub length_halt: bool,
    pub constant_volume: bool,
    pub volume: u8,

    pub sweep_enabled: bool,
    pub sweep_period: u8,
    pub sweep_negate: bool,
    pub sweep_shift: u8,
    pub sweep_reload: bool,

    timer_latch: u16,
    pub length_load: u8,

    pub timer: u16,
    pub target_timer: u16,

    pub envelope_divider: u8,
    pub envelope_decay: u8,
    pub envelope_start: bool,

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
            constant_volume: false,
            volume: 0,

            sweep_enabled: false,
            sweep_period: 0,
            sweep_negate: false,
            sweep_shift: 0,
            sweep_reload: false,

            timer_latch: 0,
            length_load: 0,

            timer: 0,
            target_timer: 0,

            envelope_divider: 0,
            envelope_decay: 0,
            envelope_start: false,

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
        self.constant_volume = (value & 0b0001_0000) != 0;
        self.volume = value & 0b0000_1111;
        self.envelope_start = true;
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
            // self.length_counter = length_table[self.length_load as usize];
        }
        self.envelope_start = true;
        self.duty_index = 0; // reset waveform sequence
    }

    pub fn step(&mut self) {
        self.decrement_timer();
        if self.timer == 0 {
            self.increment_duty_index();
            self.timer = self.timer_latch;
        }
    }

    pub fn decrement_timer(&mut self) {
        self.timer -= 1;
    }

    pub fn increment_duty_index(&mut self) {
        self.duty_index = (self.duty_index + 1) % 8;
    }
}
