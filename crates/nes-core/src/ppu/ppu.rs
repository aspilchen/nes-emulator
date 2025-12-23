use crate::observers::{NesObserver, PpuTraceDetails};

pub struct Ppu {
    // pub memory: [u8; 0x1000],
    // pub registers: [u8; 0x10],
    // pub sprite_memory: [u8; 0x100],
    pub cycles: u64,
    pub scanline: u16,
}

pub trait PpuObserver {
    fn on_ppu(&mut self, ppu: &Ppu);
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            // memory: [0; 0x1000],
            // registers: [0; 0x10],
            // sprite_memory: [0; 0x100],
            cycles: 0,
            scanline: 0,
        }
    }

    pub fn step(&mut self, num_cycles: u64, observer: &mut Option<Box<dyn NesObserver>>) {
        if let Some(observer) = observer {
            let trace_details = PpuTraceDetails {
                scanline: self.scanline,
                cycle: self.cycles,
            };
            observer.on_ppu(trace_details);
        }
        let next = (self.cycles + num_cycles) % 341;
        if next < self.cycles {
            self.scanline = (self.scanline + 1) % 262;
        }
        self.cycles = next;
    }
}
