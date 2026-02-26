use crate::apu::{
    frame_counter::{FrameCounter, FrameSignal},
    pulse::Pulse,
};
use bitflags::bitflags;
use ringbuf::{traits::Producer, HeapProd};

pub const PULSE1_CONTROL: u16 = 0x4000;
pub const PULSE1_SWEEP: u16 = 0x4001;
pub const PULSE1_TIMER_LOW: u16 = 0x4002;
pub const PULSE1_TIMER_HIGH: u16 = 0x4003;
pub const PULSE2_CONTROL: u16 = 0x4004;
pub const PULSE2_SWEEP: u16 = 0x4005;
pub const PULSE2_TIMER_LOW: u16 = 0x4006;
pub const PULSE2_TIMER_HIGH: u16 = 0x4007;
pub const TRIANGLE_CONTROL: u16 = 0x4008;
pub const TRIANGLE_TIMER_LOW: u16 = 0x400A;
pub const TRIANGLE_TIMER_HIGH: u16 = 0x400B;
pub const NOISE_CONTROL: u16 = 0x400C;
pub const NOISE_PERIOD: u16 = 0x400E;
pub const NOISE_LENGTH: u16 = 0x400F;
pub const DMC_FLAGS: u16 = 0x4010;
pub const DMC_DIRECT: u16 = 0x4011;
pub const DMC_ADDRESS: u16 = 0x4012;
pub const DMC_LENGTH: u16 = 0x4013;
pub const APU_STATUS: u16 = 0x4015;
pub const FRAME_COUNTER: u16 = 0x4017;

pub struct Apu {
    frame_counter: FrameCounter,
    pulse1: Pulse,
    pulse2: Pulse,
    status: ApuStatus,
    sample_counter: u8,
    buffer: Option<HeapProd<f32>>,
}

bitflags! {
    pub struct ApuStatus: u8 {
        const PULSE1_ENABLE   = 0b0000_0001;
        const PULSE2_ENABLE   = 0b0000_0010;
        const TRIANGLE_ENABLE = 0b0000_0100;
        const NOISE_ENABLE    = 0b0000_1000;
        const DMC_ENABLE      = 0b0001_0000;
        const FRAME_IRQ       = 0b0100_0000;
        const DMC_IRQ         = 0b1000_0000;
    }
}

impl Apu {
    pub fn new() -> Self {
        Self {
            frame_counter: FrameCounter::new(),
            pulse1: Pulse::new(true),
            pulse2: Pulse::new(false),
            status: ApuStatus::empty(),
            sample_counter: 0,
            buffer: None,
        }
    }

    pub fn set_audio_buffer(&mut self, buffer: HeapProd<f32>) {
        self.buffer = Some(buffer);
    }

    pub fn step(&mut self, cycles: u64) {
        for _ in 0..cycles {
            self.pulse1.step();
            self.pulse2.step();
            match self.frame_counter.step() {
                Some(FrameSignal::QuarterFrame) => {
                    self.pulse1.step_envelope();
                    self.pulse2.step_envelope();
                }
                Some(FrameSignal::HalfFrame) => {
                    self.pulse1.step_envelope();
                    self.pulse1.step_length_counter();
                    self.pulse1.step_sweep();
                    self.pulse2.step_envelope();
                    self.pulse2.step_length_counter();
                    self.pulse2.step_sweep();
                }
                Some(FrameSignal::IRQ) => {
                    self.pulse1.step_envelope();
                    self.pulse1.step_length_counter();
                    self.pulse1.step_sweep();
                    self.pulse2.step_envelope();
                    self.pulse2.step_length_counter();
                    self.pulse2.step_sweep();
                    // flag CPU interrupt
                }
                None => {}
            }
            self.sample_counter += 1;
            if self.sample_counter >= 41 {
                let sample = self.mix();
                self.sample_counter = 0;
                if let Some(buffer) = self.buffer.as_mut() {
                    buffer.try_push(sample);
                }
            }
        }
    }

    pub fn read(&mut self, address: u16) -> u8 {
        if address == APU_STATUS {
            self.status.remove(ApuStatus::FRAME_IRQ);
            self.status.bits()
        } else {
            0xFF
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            PULSE1_CONTROL => self.pulse1.write_control(value),
            PULSE1_SWEEP => self.pulse1.write_sweep(value),
            PULSE1_TIMER_LOW => self.pulse1.write_timer_low(value),
            PULSE1_TIMER_HIGH => self.pulse1.write_timer_high(value),
            PULSE2_CONTROL => self.pulse2.write_control(value),
            PULSE2_SWEEP => self.pulse2.write_sweep(value),
            PULSE2_TIMER_LOW => self.pulse2.write_timer_low(value),
            PULSE2_TIMER_HIGH => self.pulse2.write_timer_high(value),
            TRIANGLE_CONTROL => {}
            TRIANGLE_TIMER_LOW => {}
            TRIANGLE_TIMER_HIGH => {}
            NOISE_CONTROL => {}
            NOISE_PERIOD => {}
            NOISE_LENGTH => {}
            DMC_FLAGS => {}
            DMC_DIRECT => {}
            DMC_ADDRESS => {}
            DMC_LENGTH => {}
            APU_STATUS => self.write_status(value),
            FRAME_COUNTER => {}
            _ => {}
        }
    }

    fn write_status(&mut self, value: u8) {
        self.status = ApuStatus::from_bits_truncate(value);
        self.pulse1
            .write_enabled(self.status.contains(ApuStatus::PULSE1_ENABLE));
        self.pulse2
            .write_enabled(self.status.contains(ApuStatus::PULSE2_ENABLE));
    }

    fn mix(&mut self) -> f32 {
        let p1 = self.pulse1.get_output() as f32;
        let p2 = self.pulse2.get_output() as f32;
        if p1 == 0.0 && p2 == 0.0 {
            return 0.0;
        }
        // 95.88 / (8128.0 / (p1 + p2) + 100.0)
        (p1 + p2) / 30.0
    }
}
