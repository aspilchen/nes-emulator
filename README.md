# NES Emulator
This is a basic NES emulator built in Rust, mostly for me to learn/practice working with Rust. Because this is just a personal project to learn from, it has only been tested in linux, and probably won't be tested in Windows or anything else. More extensive error handling and GUI may come in the future once I iron out the actual emulation part. 

In its current state it won't pass the Nestest unit test I set up. It had passed the unit test previously indicating the CPU instructions and implementation is pretty solid, but the way data was extracted during testing caused some issuing around timing and memory access during actual gameplay. I have to make some adjustments to how test data is collected before I can demonstrate that test again.

# Capablities
Currently it only works with games using Mapper0 (cartridge mapping type). Still a lot to do, but it can boot and run simple games like Mario and Pacman. I will add more cartridge mappers once I have the core fully implemented and running properly.

# How to use
At the moment it needs SDL2 to display graphics. Clone the repot and build/run using cargo, providing a .nes filepath you wish to play.

# Button Mapping
Keyboard -> NES gamepad
- WASD -> directions
- j -> A
- k -> B
- Enter -> Start
- O -> Select

# Story and Basic Implementation Details
I figured this would be a fun project overall to learn Rust in. And that it demonstrates a pretty wide range of skills from low level bit-twiddling and replicating hardware behavour, to high level details like program design and some graphics. And even some basic multi-threading to make sure the UI/graphics dont interfere with emulation timing.

I tried to keep the project code as organized and modular as I could. Many of my program design choices are made for readability. Things like wrapping raw bytes in a helper class to clarify what the bytes/bits represent. The actual emulator core is just a library that can be imported, making it easy for me to test different UI and graphics libraries around it.
