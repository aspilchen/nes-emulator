use crate::cartridge::{CHR_BEGIN, CHR_END, TILE_SIZE};
use crate::cpu::cpu6502::Status;
use crate::frame::Frame;
// use crate::ppu::vram_address::{self, VramAddress};
use crate::ppu::*;

// vram, Address, Bus, OamData, Palette, PpuControl, PpuMask, PpuStatus, RegisterName, VRam,
// };

#[rustfmt::skip]

pub static SYSTEM_PALLETE: [(u8,u8,u8); 64] = [
   (0x80, 0x80, 0x80), (0x00, 0x3D, 0xA6), (0x00, 0x12, 0xB0), (0x44, 0x00, 0x96), (0xA1, 0x00, 0x5E),
   (0xC7, 0x00, 0x28), (0xBA, 0x06, 0x00), (0x8C, 0x17, 0x00), (0x5C, 0x2F, 0x00), (0x10, 0x45, 0x00),
   (0x05, 0x4A, 0x00), (0x00, 0x47, 0x2E), (0x00, 0x41, 0x66), (0x00, 0x00, 0x00), (0x05, 0x05, 0x05),
   (0x05, 0x05, 0x05), (0xC7, 0xC7, 0xC7), (0x00, 0x77, 0xFF), (0x21, 0x55, 0xFF), (0x82, 0x37, 0xFA),
   (0xEB, 0x2F, 0xB5), (0xFF, 0x29, 0x50), (0xFF, 0x22, 0x00), (0xD6, 0x32, 0x00), (0xC4, 0x62, 0x00),
   (0x35, 0x80, 0x00), (0x05, 0x8F, 0x00), (0x00, 0x8A, 0x55), (0x00, 0x99, 0xCC), (0x21, 0x21, 0x21),
   (0x09, 0x09, 0x09), (0x09, 0x09, 0x09), (0xFF, 0xFF, 0xFF), (0x0F, 0xD7, 0xFF), (0x69, 0xA2, 0xFF),
   (0xD4, 0x80, 0xFF), (0xFF, 0x45, 0xF3), (0xFF, 0x61, 0x8B), (0xFF, 0x88, 0x33), (0xFF, 0x9C, 0x12),
   (0xFA, 0xBC, 0x20), (0x9F, 0xE3, 0x0E), (0x2B, 0xF0, 0x35), (0x0C, 0xF0, 0xA4), (0x05, 0xFB, 0xFF),
   (0x5E, 0x5E, 0x5E), (0x0D, 0x0D, 0x0D), (0x0D, 0x0D, 0x0D), (0xFF, 0xFF, 0xFF), (0xA6, 0xFC, 0xFF),
   (0xB3, 0xEC, 0xFF), (0xDA, 0xAB, 0xEB), (0xFF, 0xA8, 0xF9), (0xFF, 0xAB, 0xB3), (0xFF, 0xD2, 0xB0),
   (0xFF, 0xEF, 0xA6), (0xFF, 0xF7, 0x9C), (0xD7, 0xE8, 0x95), (0xA6, 0xED, 0xAF), (0xA2, 0xF2, 0xDA),
   (0x99, 0xFF, 0xFC), (0xDD, 0xDD, 0xDD), (0x11, 0x11, 0x11), (0x11, 0x11, 0x11)
];

const PRE_RENDER: u16 = 261;
const VBLANK_BEGIN: u16 = 241;
const VBLANK_END: u16 = 260;

const NMI_SCANLINE: u16 = 241;
const NMI_DOT: u16 = 1;

const RENDERING_SCANLINES_BEGIN: u16 = 0;
const RENDERING_SCANLINES_END: u16 = 239;
const RENDERING_DOTS_BEGIN: u16 = 1;
const RENDERING_DOTS_END: u16 = 256;

const DOTS_PER_SCANLINE: u16 = 341;
const MAX_SCANLINE: u16 = 262;

pub struct Ppu {
    pub control: PpuControl,
    pub mask: PpuMask,
    pub status: PpuStatus,
    pub oam: OamData,
    pub palette: Palette,
    pub vram: VRam,
    pub data_buffer: u8,
    pub address: Address,
    pub scroll: u8,
    pub dot: u16,
    pub cycles: u64,
    pub scanline: u16,

    vram_address: VramAddress,
    fine_x: u16,
}

pub struct PpuStepResult {
    pub cycles: u64,
    pub scanline: u16,
    pub nmi_inturrupt: bool,
    pub vblank: bool,
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
            data_buffer: 0,
            address: Address::new(),
            scroll: 0,
            dot: 0,
            cycles: 21,
            scanline: 0,
            vram_address: VramAddress::new(),
            fine_x: 0,
        }
    }

    pub fn reset(&mut self, bus: Bus) {
        self.control = Default::default();
        self.mask = Default::default();
        self.status = Default::default();
        self.oam = OamData::new();
        self.palette = Palette::new();
        self.vram.reset(bus.cart.get_mirroring());
        self.data_buffer = 0;
        self.address = Address::new();
        self.scroll = 0;
        self.cycles = 21;
        self.scanline = 0;
        self.dot = 0;
        self.vram_address = VramAddress { address: 0 };
        self.fine_x = 0;
    }

    pub fn step(&mut self, num_cycles: u64, mut bus: Bus) -> PpuStepResult {
        let cycles = self.cycles;
        let scanline = self.scanline;
        let mut nmi_inturrupt = false;
        let mut vblank = self.status.contains(PpuStatus::VBLANK_STARTED);
        for _ in 0..num_cycles {
            self.dot += 1;

            if self.dot == DOTS_PER_SCANLINE {
                self.dot = 0;
                self.scanline += 1;

                if self.scanline == MAX_SCANLINE {
                    self.scanline = 0;
                }
            }

            vblank = vblank || (self.scanline == VBLANK_BEGIN && self.dot == 1);
            if self.scanline == VBLANK_END && self.dot == 1 {
                vblank = false;
            }

            nmi_inturrupt = nmi_inturrupt
                || (self.scanline == NMI_SCANLINE
                    && self.dot == NMI_DOT
                    && self.control.contains(PpuControl::NMI_ON_VBLANK));
        }
        self.status.set(PpuStatus::VBLANK_STARTED, vblank);
        PpuStepResult {
            cycles,
            scanline,
            nmi_inturrupt,
            vblank,
        }
    }

    pub fn get_frame(&mut self, bus: &mut Bus) -> Frame {
        let mut frame = Frame::new();
        let bank = self.control.bg_pattern_table_base();
        for i in vram::BEGIN..=vram::READ_END {
            let tile_i = self.vram.read(i) as u16;
            let tile = bus.get_chr_tile(bank + tile_i);
            let tile_x = (i - vram::BEGIN) as usize % 32;
            let tile_y = (i - vram::BEGIN) as usize / 32;
            let palette = self.bg_pallette(tile_x, tile_y);
            for y in 0..=7 {
                let row = tile.get_row(y);
                for x in (0..=7).rev() {
                    let value = row[x];
                    let value = palette[value as usize];
                    frame.set_pixel(tile_x * 8 + x, tile_y * 8 + y as usize, value);
                }
            }
        }
        frame
    }

    fn bg_pallette(&self, tile_column: usize, tile_row: usize) -> [u8; 4] {
        let attr_table_idx = tile_row / 4 * 8 + tile_column / 4;
        let attr_byte = self.vram.read(0x23C0 + attr_table_idx as u16);
        // let attr_byte = self.vram.data[0x3c0 + attr_table_idx]; // note: still using hardcoded first nametable
        let pallet_idx = match (tile_column % 4 / 2, tile_row % 4 / 2) {
            (0, 0) => attr_byte & 0b11,
            (1, 0) => (attr_byte >> 2) & 0b11,
            (0, 1) => (attr_byte >> 4) & 0b11,
            (1, 1) => (attr_byte >> 6) & 0b11,
            (_, _) => panic!("should not happen"),
        };
        let palette_start = palette::BEGIN + pallet_idx as u16 * 4;
        [
            self.palette.read(0),
            self.palette.read(palette_start),
            self.palette.read(palette_start + 1),
            self.palette.read(palette_start + 2),
        ]
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

    fn read_status(&mut self) -> u8 {
        let result = self.status.bits();
        self.status.set(PpuStatus::VBLANK_STARTED, false);
        self.address.reset_latch();
        result
    }

    fn read_oam_data(&mut self) -> u8 {
        self.oam.read()
    }

    fn write_oam_data(&mut self, value: u8) {
        self.oam.write(value);
    }

    fn read_data(&mut self, bus: &mut Bus) -> u8 {
        // let address = self.vram_address.address;
        let address = self.address.read();
        let result = match address {
            CHR_BEGIN..=CHR_END => {
                let result = self.data_buffer;
                self.data_buffer = self.read_chr(address, bus);
                result
            }
            vram::BEGIN..=vram::END => {
                let result = self.data_buffer;
                self.data_buffer = self.read_vram(address);
                result
            }
            palette::BEGIN..=palette::END => self.palette.read(address),
            _ => 0,
        };
        self.increment_vram_address();
        result
    }

    fn write_data(&mut self, value: u8, bus: &mut Bus) {
        // let address = self.vram_address.address;
        let address = self.address.read();
        match address {
            CHR_BEGIN..=CHR_END => bus.write_chr(address, value),
            vram::BEGIN..=vram::END => self.vram.write(address, value),
            palette::BEGIN..=palette::END => self.palette.write(address, value),
            _ => {}
        }
        self.increment_vram_address();
    }

    fn read_chr(&mut self, address: u16, bus: &mut Bus) -> u8 {
        let value = bus.read_chr(address);
        let result = self.data_buffer;
        self.data_buffer = value;
        result
    }

    fn read_vram(&mut self, address: u16) -> u8 {
        let value = self.vram.read(address);
        let result = self.data_buffer;
        self.data_buffer = value;
        result
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

    fn increment_vram_address(&mut self) {
        if self.control.contains(PpuControl::VRAM_INC_32) {
            self.address.increment(32);
        } else {
            self.address.increment(1);
        };
    }

    fn oam_dma(&mut self, value: u8, bus: &Bus) {}
}
