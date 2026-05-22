# NES Emulator

A NES emulator written in Rust, implementing the 6502 CPU, PPU, and APU 
with instruction-step synchronization. Each CPU instruction is fetched and 
executed, then the PPU and APU are stepped for the corresponding number of 
cycles to maintain synchronization.

Currently supports Mapper 0 cartridge format. Can boot and run games 
including Super Mario Bros. and Pac-Man. Additional mappers planned once 
the core is fully stable.

## Features

- 6502 CPU emulation
- PPU (picture processing unit) rendering
- APU (audio processing unit) emulation
- Mapper 0 cartridge support
- Instruction-step CPU/PPU/APU synchronization
- Modular library architecture — emulator core is decoupled from the 
  frontend, making it straightforward to swap UI/graphics backends
- Basic multithreading to isolate UI from emulation timing

## Requirements

- Rust (stable)
- SDL2

## Usage

Clone the repo and run with cargo, providing a path to a `.nes` ROM file:

```bash
cargo run --release /path/to/rom.nes
```

Tested on Linux. Windows support not guaranteed.

## Controls

| Keyboard | NES Gamepad |
|----------|-------------|
| WASD     | D-Pad       |
| J        | A           |
| K        | B           |
| Enter    | Start       |
| O        | Select      |

## Status

- [x] 6502 CPU
- [x] PPU rendering
- [x] APU audio
- [x] Mapper 0
- [ ] Additional mappers
- [ ] Expanded test coverage
