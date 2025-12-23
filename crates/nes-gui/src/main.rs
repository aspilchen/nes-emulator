// mod app;
use nes_core::input;
use nes_core::Nes;
use std::fs;
use std::io;

fn main() {
    let mut file_path = String::new();
    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to read line");

    let buffer = fs::read(file_path.trim()).expect("cannot open file");
    let mut nes = Nes::new(&buffer).expect("invalid nes file");
    nes.reset();
    nes.step();
}
