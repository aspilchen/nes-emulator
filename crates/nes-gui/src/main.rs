use nes_core::controller::Buttons;
// mod app;
use nes_core::{self, frame::Frame};
use nes_gui::render::{FrameMessage, ProducerFrame, Renderer, SdlFrame};
use sdl2::EventPump;
use std::fs;
use std::sync::mpsc::{channel, sync_channel, Receiver, Sender};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;

enum ControlMessage {
    ButtonPress(Buttons),
    ButtonRelease(Buttons),
}

fn game_loop(
    running: Arc<AtomicBool>,
    user_input: Receiver<ControlMessage>,
    frame: &mut ProducerFrame,
) {
    let test_folder = format!("{}/src", env!("CARGO_MANIFEST_DIR"));
    let filepath = format!("{}/pacman.nes", test_folder);
    let rom = fs::read(filepath).expect("cannot open file");
    let mut nes = nes_core::Nes::new(&rom).unwrap();
    nes.reset();
    while running.load(Ordering::Acquire) {
        while let Ok(inpt) = user_input.try_recv() {
            match inpt {
                ControlMessage::ButtonPress(button) => nes.controller_1.on_button_press(button),
                ControlMessage::ButtonRelease(button) => nes.controller_1.on_button_release(button),
                _ => {}
            }
        }

        let result = nes.step(Some(frame));
        if result.ppu_result.frame_complete {
            frame.send_frame();
        }
    }
}

fn handle_button_press(event_pump: &mut EventPump, input_sender: &Sender<ControlMessage>) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            sdl2::event::Event::KeyDown {
                keycode: Some(key), ..
            } => match key {
                sdl2::keyboard::Keycode::W => {
                    input_sender.send(ControlMessage::ButtonPress(Buttons::UP));
                }
                sdl2::keyboard::Keycode::A => {
                    input_sender.send(ControlMessage::ButtonPress(Buttons::LEFT));
                }
                sdl2::keyboard::Keycode::S => {
                    input_sender.send(ControlMessage::ButtonPress(Buttons::DOWN));
                }
                sdl2::keyboard::Keycode::D => {
                    input_sender.send(ControlMessage::ButtonPress(Buttons::RIGHT));
                }
                sdl2::keyboard::Keycode::Return => {
                    input_sender.send(ControlMessage::ButtonPress(Buttons::START));
                }
                sdl2::keyboard::Keycode::J => {
                    input_sender.send(ControlMessage::ButtonPress(Buttons::A));
                }
                sdl2::keyboard::Keycode::K => {
                    input_sender.send(ControlMessage::ButtonPress(Buttons::B));
                }
                sdl2::keyboard::Keycode::O => {
                    input_sender.send(ControlMessage::ButtonPress(Buttons::SELECT));
                }
                _ => {}
            },

            sdl2::event::Event::KeyUp {
                keycode: Some(key), ..
            } => match key {
                sdl2::keyboard::Keycode::W => {
                    input_sender.send(ControlMessage::ButtonRelease(Buttons::UP));
                }
                sdl2::keyboard::Keycode::A => {
                    input_sender.send(ControlMessage::ButtonRelease(Buttons::LEFT));
                }
                sdl2::keyboard::Keycode::S => {
                    input_sender.send(ControlMessage::ButtonRelease(Buttons::DOWN));
                }
                sdl2::keyboard::Keycode::D => {
                    input_sender.send(ControlMessage::ButtonRelease(Buttons::RIGHT));
                }
                sdl2::keyboard::Keycode::Return => {
                    input_sender.send(ControlMessage::ButtonRelease(Buttons::START));
                }
                sdl2::keyboard::Keycode::J => {
                    input_sender.send(ControlMessage::ButtonRelease(Buttons::A));
                }
                sdl2::keyboard::Keycode::K => {
                    input_sender.send(ControlMessage::ButtonRelease(Buttons::B));
                }
                sdl2::keyboard::Keycode::O => {
                    input_sender.send(ControlMessage::ButtonRelease(Buttons::SELECT));
                }
                _ => {}
            },
            sdl2::event::Event::Quit { .. } => return false,
            _ => {}
        }
    }
    true
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let (tx, rx) = sync_channel::<FrameMessage>(1);
    let (input_sender, input_receiver) = channel::<ControlMessage>();
    let mut event_pump = sdl.event_pump().unwrap();
    let mut renderer = Renderer::new();
    let running = Arc::new(AtomicBool::new(true));
    let mut producer = ProducerFrame::new(tx);
    let mut frame = SdlFrame::new();
    let emulator_thread = {
        let running = Arc::clone(&running);
        thread::spawn(move || game_loop(running, input_receiver, &mut producer))
    };
    let mut is_running = true;
    while is_running {
        is_running = handle_button_press(&mut event_pump, &input_sender);

        match rx.recv() {
            Ok(FrameMessage::Frame(data)) => {
                frame.from_indexes(&data);
                renderer.render_frame(&frame);
            }
            Ok(FrameMessage::Clear) => frame.clear(),
            _ => {}
        }
    }
    running.store(is_running, Ordering::Release);
    while let Ok(_) = rx.try_recv() {}
    emulator_thread.join();
}
