pub struct FrameCounter {
    cycles: u16,
    mode: Mode,
    irq_disable: bool,
}

pub enum FrameSignal {
    QuarterFrame,
    HalfFrame,
    IRQ,
}

enum Mode {
    FourStep,
    FiveStep,
}

impl FrameCounter {
    pub fn new() -> Self {
        Self {
            cycles: 0,
            mode: Mode::FourStep,
            irq_disable: false,
        }
    }

    pub fn write(&mut self, value: u8) {
        self.mode = if (value & 0b1000_0000) == 0 {
            Mode::FourStep
        } else {
            Mode::FiveStep
        };
        self.irq_disable = (value & 0b0100_0000) != 0;
        self.cycles = 0;
    }

    pub fn step(&mut self) -> Option<FrameSignal> {
        self.cycles += 1;
        match (&self.mode, self.cycles) {
            (_, 3729) => Some(FrameSignal::QuarterFrame),
            (_, 7457) => Some(FrameSignal::HalfFrame),
            (_, 11186) => Some(FrameSignal::QuarterFrame),
            (Mode::FourStep, 14915) => {
                self.cycles = 0;
                if self.irq_disable {
                    Some(FrameSignal::HalfFrame)
                } else {
                    Some(FrameSignal::IRQ)
                }
            }
            (Mode::FiveStep, 18641) => {
                self.cycles = 0;
                Some(FrameSignal::HalfFrame)
            }
            _ => None,
        }
    }
}
