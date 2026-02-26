pub struct Sweep {
    is_pulse_1: bool,
    is_enabled: bool,
    negate: bool,
    is_reload_pending: bool,
    period_counter: u8,
    period_latch: u8,
    shift: u8,
    output_result: u16,
}

impl Sweep {
    pub fn new(is_pulse_1: bool) -> Self {
        Self {
            is_pulse_1,
            is_enabled: false,
            is_reload_pending: false,
            negate: false,
            period_counter: 0,
            period_latch: 0,
            shift: 0,
            output_result: 0,
        }
    }

    pub fn write(&mut self, value: u8) {
        self.is_enabled = (value & 0b1000_0000) != 0;
        self.period_latch = (value >> 4) & 0b111;
        self.negate = (value & 0b0000_1000) != 0;
        self.shift = value & 0b0000_0111;
        self.is_reload_pending = true;
    }

    pub fn get_output(&self) -> u16 {
        self.output_result
    }

    pub fn step(&mut self, timer_latch: u16) {
        let next_timer_latch = self.calculate_next_timer_latch(timer_latch);
        let is_muting = timer_latch < 8 || next_timer_latch > 0x7FF;

        if self.is_reload_pending || self.period_counter == 0 {
            self.reload();
        } else {
            self.period_counter = self.period_counter.saturating_sub(1);
        }

        self.output_result = if is_muting || !self.is_enabled {
            timer_latch
        } else {
            next_timer_latch
        };
    }

    fn calculate_next_timer_latch(&self, timer_latch: u16) -> u16 {
        let delta = timer_latch >> self.shift;
        if self.negate {
            if self.is_pulse_1 {
                timer_latch.wrapping_sub(delta + 1)
            } else {
                timer_latch.wrapping_sub(delta)
            }
        } else {
            timer_latch + delta
        }
    }

    fn is_muting(&self, timer_period: u16, target: u16) -> bool {
        timer_period < 8 || target > 0x7FF
    }

    fn reload(&mut self) {
        self.period_counter = self.period_latch;
        self.is_reload_pending = false;
    }
}
