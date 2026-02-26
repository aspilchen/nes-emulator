const VOLUME_MAX: u8 = 15;

pub struct Envelope {
    pub is_reset_pending: bool,
    is_looping: bool,
    period_counter: u8,
    period_latch: u8,
    volume: u8,
}

impl Envelope {
    pub fn new() -> Self {
        Self {
            is_reset_pending: false,
            is_looping: false,
            period_counter: 0,
            period_latch: 0,
            volume: 0,
        }
    }

    pub fn read(&self) -> u8 {
        self.volume
    }

    pub fn step(&mut self) {
        if self.is_reset_pending {
            self.reset();
        } else if self.period_counter == 0 {
            self.decrement_volume();
        } else {
            self.decrement_period_counter();
        }
    }

    pub fn set_period_latch(&mut self, period_latch: u8) {
        self.period_latch = period_latch;
        self.is_reset_pending = true
    }

    fn decrement_volume(&mut self) {
        if self.volume == 0 {
            if self.is_looping {
                self.volume = VOLUME_MAX;
            }
        } else {
            self.volume -= 1;
        }
    }

    fn decrement_period_counter(&mut self) {
        if self.period_counter == 0 {
            self.period_counter = self.period_latch;
        } else {
            self.period_counter -= 1;
        }
    }

    fn reset(&mut self) {
        if self.is_reset_pending {
            self.period_counter = self.period_latch;
            self.volume = VOLUME_MAX;
            self.is_reset_pending = false;
        }
    }
}
