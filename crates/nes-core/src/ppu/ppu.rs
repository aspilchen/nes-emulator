use crate::cartridge::{CHR_BEGIN, CHR_END};
use crate::ppu::{
    Address, Bus, OamData, Palette, PpuControl, PpuMask, PpuStatus, RegisterName, VRam,
};

pub const VRAM_BEGIN: u16 = 0x2000;
pub const VRAM_END: u16 = 0x3EFF;
pub const PALETTE_BEGIN: u16 = 0x3F00;
pub const PALETTE_END: u16 = 0x3FFF;
pub const OAM_BEGIN: u8 = 0;
pub const OAM_END: u8 = 255;

pub struct Ppu {
    pub control: PpuControl,
    pub mask: PpuMask,
    pub status: PpuStatus,
    pub oam: OamData,
    pub palette: Palette,
    pub vram: VRam,
    pub data: u8,
    pub address: Address,
    pub scroll: u8,
    pub cycles: u64,
    pub scanline: u16,
}

pub struct PpuStepResult {
    pub cycles: u64,
    pub scanline: u16,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            control: Default::default(),
            mask: Default::default(),
            status: Default::default(),
            oam: OamData::new(),
            palette: Palette::new(),
            vram: VRam::new(),
            data: 0,
            address: Address::new(),
            scroll: 0,
            cycles: 21,
            scanline: 0,
        }
    }

    pub fn reset(&mut self, bus: Bus) {
        self.control = Default::default();
        self.mask = Default::default();
        self.status = Default::default();
        self.oam = OamData::new();
        self.palette = Palette::new();
        self.vram.reset(bus.cart.get_mirroring());
        self.data = 0;
        self.address = Address::new();
        self.scroll = 0;
        self.cycles = 21;
        self.scanline = 0;
    }

    pub fn step(&mut self, num_cycles: u64, mut bus: Bus) -> PpuStepResult {
        let cycles = self.cycles;
        let scanline = self.scanline;
        let next = (self.cycles + num_cycles) % 341;
        if next < self.cycles {
            self.scanline = (self.scanline + 1) % 262;
        }
        self.cycles = next;
        PpuStepResult { cycles, scanline }
    }

    pub fn read_register(&mut self, address: u16, bus: &mut Bus) -> u8 {
        let register = RegisterName::from_address(address).unwrap();
        match register {
            RegisterName::Status => self.read_status(),
            RegisterName::OamData => self.read_oam_data(),
            RegisterName::Data => self.read_data(bus),
            _ => 0,
        }
    }

    pub fn write_register(&mut self, address: u16, value: u8, bus: &mut Bus) {
        let register = RegisterName::from_address(address).unwrap();
        match register {
            RegisterName::Control => self.write_control(value),
            RegisterName::Mask => self.write_mask(value),
            RegisterName::OamAddress => self.write_oam_address(value),
            RegisterName::OamData => self.write_oam_data(value),
            RegisterName::Scroll => self.write_scroll(value),
            RegisterName::Address => self.write_address(value),
            RegisterName::Data => self.write_data(value, bus),
            RegisterName::OamDma => self.oam_dma(value, bus),
            _ => {}
        };
    }

    fn read_status(&self) -> u8 {
        self.status.bits()
    }

    fn read_oam_data(&mut self) -> u8 {
        self.oam.read()
    }

    fn write_oam_data(&mut self, value: u8) {
        self.oam.write(value);
    }

    fn read_data(&mut self, bus: &mut Bus) -> u8 {
        let address = self.address.read();
        self.increment_address();
        match address {
            CHR_BEGIN..=CHR_END => self.read_chr(address, bus),
            VRAM_BEGIN..=VRAM_END => self.read_vram(address),
            PALETTE_BEGIN..=PALETTE_END => self.palette.read(address),
            _ => 0,
        }
    }

    fn read_chr(&mut self, address: u16, bus: &mut Bus) -> u8 {
        let value = bus.read_chr(address);
        let result = self.data;
        self.data = value;
        result
    }

    fn read_vram(&mut self, address: u16) -> u8 {
        let value = self.vram.read(address);
        let result = self.data;
        self.data = value;
        result
    }

    fn write_data(&mut self, value: u8, bus: &mut Bus) {
        let address = self.address.read();
        match address {
            CHR_BEGIN..=CHR_END => bus.write_chr(address, value),
            VRAM_BEGIN..=VRAM_END => self.vram.write(address, value),
            PALETTE_BEGIN..=PALETTE_END => self.palette.write(address, value),
            _ => {}
        }
        self.increment_address();
    }

    fn write_control(&mut self, value: u8) {
        self.control = PpuControl::from_bits_truncate(value);
    }

    fn write_mask(&mut self, value: u8) {
        self.mask = PpuMask::from_bits_truncate(value);
    }

    fn write_oam_address(&mut self, value: u8) {
        self.oam.address = value;
    }

    fn write_scroll(&mut self, value: u8) {
        self.scroll = value;
    }

    fn write_address(&mut self, value: u8) {
        self.address.write(value);
    }

    fn increment_address(&mut self) {
        let vertical_increment = 32;
        let horizontal_increment = 1;
        if self.control.contains(PpuControl::VRAM_INC_32) {
            self.address.increment(vertical_increment);
        } else {
            self.address.increment(horizontal_increment);
        }
    }

    fn oam_dma(&mut self, value: u8, bus: &Bus) {}
}
