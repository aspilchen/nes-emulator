// use crate::cartridge::Cartridge;
// use crate::ppu::Ppu;
// use crate::apu::Apu;
// use crate::input::ControllerState;
use crate::cartridge::cartridge::Cartridge;

pub struct Bus {
    // Devices
    // pub ppu: Ppu,
    // pub apu: Apu,
    pub cartridge: Cartridge,
    cpu_ram: [u8; 2048],

    // Controllers
    // controller1: ControllerState,
    // controller2: ControllerState,
    controller_shift1: u8,
    controller_shift2: u8,
}

impl Bus {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            // ppu: Ppu::new(),
            // apu: Apu::new(),
            cartridge,
            cpu_ram: [0; 2048],
            // controller1: ControllerState::default(),
            // controller2: ControllerState::default(),
            controller_shift1: 0,
            controller_shift2: 0,
        }
    }

    pub fn cpu_read(&mut self, addr: u16) -> u8 {
        match addr {
            // $0000-$1FFF: 2KB internal RAM (mirrored every $800)
            0x0000..=0x1FFF => self.cpu_ram[(addr & 0x07FF) as usize],

            // $2000-$3FFF: PPU registers (mirrored every 8)
            // 0x2000..=0x3FFF => self.ppu.cpu_read(0x2000 + (addr & 0x7)),

            // $4000-$4017: APU & IO
            // 0x4000..=0x4013 => self.apu.cpu_read(addr),
            // 0x4014 => self.ppu.cpu_read(addr), // OAMDMA
            // 0x4015 => self.apu.cpu_read(addr),
            // 0x4016 => self.read_controller(1),
            // 0x4017 => self.read_controller(2),

            // $4020-$FFFF: Cartridge space
            0x4020..=0xFFFF => self.cartridge.cpu_read(addr),

            _ => 0,
        }
    }

    pub fn cpu_write(&mut self, address: u16, value: u8) {
        match address {
            // $0000-$1FFF: internal RAM
            0x0000..=0x1FFF => {
                self.cpu_ram[(address & 0x07FF) as usize] = value;
            }

            // $2000-$3FFF: PPU registers
            // 0x2000..=0x3FFF => {
            // self.ppu.cpu_write(0x2000 + (addr & 0x7), data);
            // }

            // $4000-$4017: APU & IO
            // 0x4000..=0x4013 => self.apu.cpu_write(addr, data),
            // 0x4014 => self.ppu.cpu_write(addr, data), // OAMDMA
            // 0x4015 => self.apu.cpu_write(addr, data),
            // 0x4016 => self.write_controller(data),
            // 0x4017 => self.apu.cpu_write(addr, data),

            // Cartridge
            0x4020..=0xFFFF => self.cartridge.cpu_write(address, value),
            _ => {}
        }
    }

    // -------------------------
    // CONTROLLERS
    // -------------------------
    // fn write_controller(&mut self, data: u8) {
    //     // Writing bit 0 latches controller state
    //     if data & 1 == 1 {
    //         self.controller_shift1 = self.controller1.to_bits();
    //         self.controller_shift2 = self.controller2.to_bits();
    //     }
    // }

    // fn read_controller(&mut self, player: u8) -> u8 {
    //     let shift = if player == 1 {
    //         &mut self.controller_shift1
    //     } else {
    //         &mut self.controller_shift2
    //     };

    //     let bit = (*shift & 0x80) >> 7;
    //     *shift <<= 1;
    //     bit | 0x40
    // }

    // // -------------------------
    // // INPUT API
    // // -------------------------
    // pub fn set_controller(&mut self, player: usize, state: ControllerState) {
    //     match player {
    //         1 => self.controller1 = state,
    //         2 => self.controller2 = state,
    //         _ => {}
    //     }
    // }
}
