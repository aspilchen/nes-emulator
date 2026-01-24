use crate::cartridge::{CHR_BEGIN, CHR_END, TILE_HEIGHT, TILE_SIZE, TILE_WIDTH};
use crate::frame::Frame;
use crate::ppu::oam::SpriteAttribute;
use crate::ppu::*;

const PRE_RENDER: u16 = 261;
const VBLANK_BEGIN: u16 = 241;
const VBLANK_END: u16 = 260;

const HBLANK_BEGIN: u16 = 257;

const DOTS_RENDER_BEGIN: u16 = 1;
const DOTS_RENDER_END: u16 = 256;
const SCANLINE_RENDER_BEGIN: u16 = 0;
const SCANLINE_RENDER_END: u16 = 239;

const NMI_SCANLINE: u16 = 241;
const NMI_DOT: u16 = 1;
const DOTS_PER_SCANLINE: u16 = 341;
const MAX_SCANLINE: u16 = 262;

pub struct Ppu {
    pub control: Control,
    pub mask: Mask,
    pub status: Status,
    pub oam: Oam,
    pub palette: Palette,
    pub vram: VRam,
    pub data_buffer: u8,
    pub address: Address,
    pub scroll: Scroll,
    pub dot: u16,
    pub cycles: u64,
    pub scanline: u16,
    dma_page: Option<u8>,
    vblank_active: bool,
}

pub struct PpuStepResult {
    pub cycles: u64,
    pub scanline: u16,
    pub nmi_inturrupt: bool,
    pub frame_complete: bool,
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
            scroll: Default::default(),
            dot: 0,
            cycles: 21,
            scanline: 0,
            dma_page: None,
            vblank_active: false,
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
        self.scroll.reset();
        self.cycles = 21;
        self.scanline = 0;
        self.dot = 0;
        self.dma_page = None;
        self.vblank_active = false;
    }

    pub fn step(
        &mut self,
        mut bus: Bus,
        mut frame: Option<&mut dyn Frame>,
        num_cycles: u64,
    ) -> PpuStepResult {
        let mut frame_complete = false;
        let cycles = self.cycles;
        let scanline = self.scanline;
        let mut nmi_interrupt = false;
        for _ in 0..num_cycles {
            self.dot += 1;
            if self.dot == DOTS_PER_SCANLINE {
                self.dot = 0;
                self.scanline += 1;
                if self.scanline == MAX_SCANLINE {
                    self.scanline = 0;
                }
            }

            if self.scanline == VBLANK_BEGIN && self.dot == 1 {
                nmi_interrupt = self.control.contains(Control::NMI_ON_VBLANK);
                self.status.insert(Status::VBLANK_STARTED);
                self.vblank_active = true;
                frame_complete = true;
            } else if self.scanline == 0 && self.dot == 1 {
                self.status.remove(Status::VBLANK_STARTED);
                self.vblank_active = false;
            }

            if self.dot == HBLANK_BEGIN && !self.vblank_active {
                if let Some(frame) = frame.as_deref_mut() {
                    self.draw_scanline(&mut bus, frame);
                }
            }
        }

        PpuStepResult {
            cycles,
            scanline,
            nmi_inturrupt: nmi_interrupt,
            frame_complete,
            dma_page: self.dma_page.take(),
        }
    }

    fn draw_scanline(&mut self, bus: &mut Bus, frame: &mut dyn Frame) {
        if self.scanline >= VBLANK_BEGIN || self.dot <= DOTS_RENDER_END {
            return;
        }
        self.draw_scanline_bg(bus, frame);
        self.draw_sprites(bus, frame);
    }

    fn draw_scanline_bg(&mut self, bus: &mut Bus, frame: &mut dyn Frame) {
        let bank = self.control.bg_pattern_table_base() / TILE_SIZE as u16;
        let vram_begin = (self.scanline / TILE_HEIGHT as u16) * vram::TILES_PER_ROW;
        let vram_end = vram_begin + vram::TILES_PER_ROW;
        for vram_index in vram_begin..vram_end {
            let tile_data = self.vram.get_nametable_entry(vram_index);
            let tile = bus.get_chr_tile(bank + tile_data.chr_index);
            let attributes = self.vram.get_attributes(tile_data.x, tile_data.y);
            let tile_y = self.scanline % TILE_HEIGHT as u16;
            let row = tile.get_row(tile_y);
            let palette_index = attributes.palette_index(tile_data.x, tile_data.y);
            let palette = self.palette.get_entry(palette_index);
            for (x, &row_value) in row.iter().enumerate() {
                let color_index = palette[row_value as usize];
                let frame_x = (tile_data.x as usize * TILE_WIDTH) + x;
                frame.set_pixel(frame_x, self.scanline as usize, color_index);
            }
        }
    }

    fn draw_sprites(&mut self, bus: &mut Bus, frame: &mut dyn Frame) {
        let bank = self.control.sprite_pattern_table_base();
        let oam_hits = self.oam.scan(self.scanline);
        for oam_entry in oam_hits {
            let tile = bus.get_chr_tile(bank + oam_entry.tile as u16);
            let palette_index = oam_entry.palette_index();
            let palette = self.sprite_palette(palette_index);
            let tile_row = if oam_entry.attribute.contains(SpriteAttribute::FLIP_V) {
                7 - (self.scanline - oam_entry.top())
            } else {
                self.scanline - oam_entry.top()
            };
            let row = tile.get_row(tile_row);
            for x in 0..TILE_WIDTH {
                let tile_x = if oam_entry.attribute.contains(SpriteAttribute::FLIP_H) {
                    7 - x
                } else {
                    x
                };
                if row[tile_x] == 0 {
                    continue;
                }
                let frame_x = oam_entry.x as usize + x;
                let color_index = palette[row[tile_x] as usize];

                if !frame.is_transparent(frame_x, self.scanline as usize)
                    && color_index != 0
                    && oam_entry.index == 0
                {
                    self.status.insert(Status::SPRITE_0_HIT);
                }

                frame.set_pixel(frame_x, self.scanline as usize, color_index);
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
        self.status.set(Status::VBLANK_STARTED, false);
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
        let address = self.address.read();
        let result = match address {
            CHR_BEGIN..=CHR_END => self.read_chr(address, bus),
            vram::BEGIN..=vram::END => self.read_vram(address),
            palette::BEGIN..=palette::END => self.palette.read(address),
            _ => 0,
        };
        self.increment_vram_address();
        result
    }

    fn write_data(&mut self, value: u8, bus: &mut Bus) {
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
        self.control = Control::from_bits_truncate(value);
    }

    fn write_mask(&mut self, value: u8) {
        self.mask = Mask::from_bits_truncate(value);
    }

    fn write_oam_address(&mut self, value: u8) {
        self.oam.address = value;
    }

    fn write_scroll(&mut self, value: u8) {
        self.scroll.write(value);
    }

    fn write_address(&mut self, value: u8) {
        self.address.write(value);
    }

    fn increment_vram_address(&mut self) {
        if self.control.contains(Control::VRAM_INC_32) {
            self.address.increment(32);
        } else {
            self.address.increment(1);
        };
    }
}
