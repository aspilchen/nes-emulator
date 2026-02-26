use super::envelope::Envelope;
use super::sweep::Sweep;

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
    duty: u8,
    length_halt: bool,
    is_constant_volume: bool,
    constant_volume: u8,
    envelope: Envelope,
    sweep: Sweep,
    timer_counter: u16,
    timer_latch: u16,
    length_counter: u8,
    duty_index: u8,
    is_enabled: bool,
    is_pulse_1: bool,
}

impl Pulse {
    pub fn new(is_pulse_1: bool) -> Self {
        Self {
            duty: 0,
            length_halt: false,
            is_constant_volume: false,
            constant_volume: 0,
            envelope: Envelope::new(),
            sweep: Sweep::new(is_pulse_1),
            timer_counter: 0,
            timer_latch: 0,
            length_counter: 0,
            duty_index: 0,
            is_enabled: false,
            is_pulse_1,
        }
    }

    pub fn step(&mut self) {
        if self.timer_counter == 0 {
            self.duty_index = (self.duty_index + 1) % 8;
            self.timer_counter = self.timer_latch;
        } else {
            self.timer_counter = self.timer_counter.saturating_sub(1);
        }
    }

    pub fn step_envelope(&mut self) {
        self.envelope.step();
    }

    pub fn step_length_counter(&mut self) {
        if !self.length_halt {
            self.length_counter = self.length_counter.saturating_sub(1);
        }
    }

    pub fn step_sweep(&mut self) {
        self.sweep.step(self.timer_latch);
        self.timer_latch = self.sweep.get_output();
    }

    pub fn write_enabled(&mut self, is_enabled: bool) {
        self.is_enabled = is_enabled;
    }

    pub fn write_control(&mut self, value: u8) {
        self.duty = (value >> 6) & 0b11;
        self.length_halt = (value & 0b0010_0000) != 0;
        self.is_constant_volume = (value & 0b0001_0000) != 0;
        self.constant_volume = value & 0b0000_1111;
        self.envelope.set_period_latch(value & 0b0000_1111);
    }

    pub fn write_sweep(&mut self, value: u8) {
        self.sweep.write(value);
    }

    pub fn write_timer_low(&mut self, value: u8) {
        self.timer_latch &= 0xFF00;
        self.timer_latch |= value as u16;
    }

    pub fn write_timer_high(&mut self, value: u8) {
        let timer_bits = value & 0b0000_0111;
        let length_index = (value >> 3) & 0b0001_1111;
        self.timer_latch &= 0x00FF;
        self.timer_latch |= (timer_bits as u16) << 8;
        if self.is_enabled {
            self.length_counter = LENGTH_TABLE[length_index as usize];
        }
        self.duty_index = 0;
        self.envelope.is_reset_pending = true;
    }

    pub fn get_output(&self) -> u8 {
        let is_muted = self.length_counter == 0
            || self.timer_latch < 8
            || DUTY_SEQUENCES[self.duty as usize][self.duty_index as usize] == 0;
        if is_muted {
            0
        } else if self.is_constant_volume {
            self.constant_volume
        } else {
            self.envelope.read()
        }
    }
}
