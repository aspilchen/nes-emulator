use crate::{notify, observers::ppu_observer::PpuObserver};

pub struct Ppu {
    // pub memory: [u8; 0x1000],
    // pub registers: [u8; 0x10],
    // pub sprite_memory: [u8; 0x100],
    pub cycles: u64,
    pub scanline: u16,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            // memory: [0; 0x1000],
            // registers: [0; 0x10],
            // sprite_memory: [0; 0x100],
            cycles: 21,
            scanline: 0,
        }
    }

    pub fn step(&mut self, num_cycles: u64, observer: &mut Option<Box<dyn PpuObserver>>) {
        notify!(observer, on_step_begin, self);
        let next = (self.cycles + num_cycles) % 341;
        if next < self.cycles {
            self.scanline = (self.scanline + 1) % 262;
        }
        self.cycles = next;
    }
}
