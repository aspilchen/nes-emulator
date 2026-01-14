// mod app;
use nes_core::{
    self,
    cartridge::{Cartridge, ChrTile},
    frame::{self, Frame},
};
use std::fs;

use nes_gui::render::Renderer;

fn main() {
    let test_folder = format!("{}/src", env!("CARGO_MANIFEST_DIR"));
    let filepath = format!("{}/mario.nes", test_folder);
    let rom = fs::read(filepath).expect("cannot open file");
    let mut nes = nes_core::Nes::new(&rom).unwrap();
    nes.reset();

    let sdl = sdl2::init().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let mut renderer = Renderer::new();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }
        let result = nes.step();
        if let Some(frame) = result.frame {
            renderer.render_frame(&frame);
        }
    }
}

