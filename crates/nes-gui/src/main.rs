// mod app;
use nes_core::input;
use nes_core::Nes;
use std::fs;
use std::io;

fn main() {
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "open" {
            let mut file_path = String::new();
            io::stdin()
                .read_line(&mut file_path)
                .expect("Failed to read line");

            let buffer = fs::read(file_path.trim());
            match buffer {
                Ok(data) => {
                    let nes = Nes::new(&data);
                    match nes {
                        Ok(_n) => println!("starting game"),
                        Err(e) => println!("cannot start game: {}", e),
                    }
                }
                _ => println!("file not found"),
            }
        } else {
            break;
        }
    }
}
