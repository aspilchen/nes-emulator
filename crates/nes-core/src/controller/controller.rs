use bitflags::bitflags;

pub const CONTROLLER_1: u16 = 0x4016;
pub const CONTROLLER_2: u16 = 0x4017;

bitflags! {
    #[derive(Debug)]
    pub struct Buttons: u8 {
        const A      = 0b0000_0001;
        const B      = 0b0000_0010;
        const SELECT = 0b0000_0100;
        const START  = 0b0000_1000;
        const UP     = 0b0001_0000;
        const DOWN   = 0b0010_0000;
        const LEFT   = 0b0100_0000;
        const RIGHT  = 0b1000_0000;
    }
}

pub struct Controller {
    state: Buttons,
    shift_register: u8,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            state: Buttons::empty(),
            shift_register: 0,
        }
    }

    pub fn reset(&mut self) {
        self.state = Buttons::empty();
        self.shift_register = 0;
    }

    pub fn on_button_press(&mut self, button: Buttons) {
        self.state.insert(button);
    }

    pub fn on_button_release(&mut self, button: Buttons) {
        self.state.remove(button);
    }

    pub fn strobe(&mut self) {
        self.shift_register = self.state.bits();
    }

    pub fn read(&mut self) -> u8 {
        let result = self.shift_register & 1;
        self.shift_register >>= 1;
        result
    }
}
