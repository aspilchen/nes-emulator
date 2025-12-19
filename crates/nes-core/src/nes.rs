use crate::bus::Bus;
use crate::cartridge::cartridge::Cartridge;

pub struct Nes {
    bus: Bus,
}

impl Nes {
    pub fn new(rom: &[u8]) -> Result<Self, String> {
        let cartridge = Cartridge::new(rom)?;
        let bus = Bus::new(cartridge);
        Ok(Self { bus })
    }
}
