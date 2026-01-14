use crate::cartridge::{CHR_BEGIN, CHR_END, TILE_HEIGHT, TILE_SIZE, TILE_WIDTH};
use crate::frame::Frame;
use crate::ppu::oam::SpriteAttribute;
use crate::ppu::*;

const PRE_RENDER: u16 = 261;
const VBLANK_BEGIN: u16 = 241;
const VBLANK_END: u16 = 260;

const NMI_SCANLINE: u16 = 241;
const NMI_DOT: u16 = 1;
const DOTS_PER_SCANLINE: u16 = 341;
const MAX_SCANLINE: u16 = 262;

pub struct Ppu {
    pub control: PpuControl,
    pub mask: PpuMask,
    pub status: PpuStatus,
    pub oam: Oam,
    pub palette: Palette,
    pub vram: VRam,
    pub data_buffer: u8,
    pub address: Address,
    pub scroll: u8,
    pub dot: u16,
    pub cycles: u64,
    pub scanline: u16,
    dma_page: Option<u8>,
    vram_address: VramAddress,
}

pub struct PpuStepResult {
    pub cycles: u64,
    pub scanline: u16,
    pub nmi_inturrupt: bool,
    pub vblank: bool,
    pub dma_page: Option<u8>,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            control: Default::default(),
            mask: Default::default(),
            status: Default::default(),
            oam: Oam::new(),
            palette: Palette::new(),
            vram: VRam::new(),
            data_buffer: 0,
            address: Address::new(),
            scroll: 0,
            dot: 0,
            cycles: 21,
            scanline: 0,
            vram_address: VramAddress::new(),
            dma_page: None,
        }
    }

    pub fn reset(&mut self, bus: Bus) {
        self.control = Default::default();
        self.mask = Default::default();
        self.status = Default::default();
        self.oam = Oam::new();
        self.palette = Palette::new();
        self.vram.reset(bus.cart.get_mirroring());
        self.data_buffer = 0;
        self.address = Address::new();
        self.scroll = 0;
        self.cycles = 21;
        self.scanline = 0;
        self.dot = 0;
        self.vram_address = VramAddress { address: 0 };
        self.dma_page = None;
    }

    pub fn step(&mut self, mut _bus: Bus, num_cycles: u64) -> PpuStepResult {
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
            dma_page: self.dma_page.take(),
        }
    }

    pub fn get_frame(&mut self, mut bus: Bus) -> Frame {
        let mut frame = Frame::new();
        self.draw_background(&mut frame, &mut bus);
        self.draw_sprites(&mut frame, &mut bus);
        frame
    }

    fn draw_background(&mut self, frame: &mut Frame, bus: &mut Bus) {
        let bank = self.control.bg_pattern_table_base() / TILE_SIZE as u16;
        for i in vram::NAMETABLE_BEGIN..=vram::NAMETABLE_END {
            let tile_data = self.vram.get_nametable_entry(i);
            let tile = bus.get_chr_tile(bank + tile_data.value);
            let attribute = self.vram.get_attributes(tile_data.x, tile_data.y);
            let palette_index = attribute.palette_index(tile_data.x, tile_data.y);
            let palette = self.palette.get_entry(palette_index);
            for y in 0..8 {
                let row = tile.get_row(y);
                for x in 0..8 {
                    let value = row[x];
                    let value = palette[value as usize];
                    frame.set_pixel(
                        tile_data.x as usize * 8 + x,
                        tile_data.y as usize * 8 + y as usize,
                        value,
                    );
                }
            }
        }
    }

    fn draw_sprites(&mut self, frame: &mut Frame, bus: &mut Bus) {
        let bank = self.control.sprite_pattern_table_base();
        for scanline in 0..240 {
            let mut oam_hits = self.oam.scan(scanline);
            for (i, oam_entry) in oam_hits.iter().enumerate() {
                let tile = bus.get_chr_tile(bank + oam_entry.tile as u16);
                let palette = oam_entry.palette_index();
                let palette = self.sprite_palette(palette as u16);
                let height = if self.control.contains(PpuControl::SPRITE_8X16) {
                    // Use later for 8x16 sprites
                    16
                } else {
                    8
                };
                for y in 0..TILE_HEIGHT {
                    let row = tile.get_row(y as u16);
                    for x in 0..TILE_WIDTH {
                        if row[x] != 0 {
                            let pixel_x = if oam_entry.attribute.contains(SpriteAttribute::FLIP_H) {
                                oam_entry.x as usize + 7 - x
                            } else {
                                oam_entry.x as usize + x
                            };
                            let pixel_y = if oam_entry.attribute.contains(SpriteAttribute::FLIP_V) {
                                oam_entry.y as usize + 7 - y
                            } else {
                                oam_entry.y as usize + y
                            };

                            if i == 0 && frame.data[scanline as usize][pixel_x] != 0 {
                                self.status.insert(PpuStatus::SPRITE_0_HIT);
                            }

                            let value = palette[row[x] as usize];
                            frame.set_pixel(pixel_x, pixel_y, value);
                        }
                    }
                }
            }
        }
    }

    fn sprite_palette(&self, palette_index: u16) -> [u8; 4] {
        let address = palette_index + (0x11 / 4);
        self.palette.get_entry(address)
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
            RegisterName::OamDma => self.dma_page = Some(value),
            _ => {}
        };
    }

    pub fn dma(&mut self, buffer: &[u8; 256]) {
        self.oam.dma_transfer(buffer);
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
}
