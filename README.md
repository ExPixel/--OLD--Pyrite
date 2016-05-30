# Pyrite 
![[Travis](https://travis-ci.org/ExPixel/pyrite)](https://travis-ci.org/ExPixel/pyrite.svg?branch=master) 
![[AppVeyor](https://ci.appveyor.com/project/ExPixel/pyrite)](https://ci.appveyor.com/api/projects/status/b77f7lieol52ghjm?svg=true)

If compilation doesn't work due to a conflict with `libc`. Use `cargo update -p libc:0.2.2` or just 'cargo update -p libc'
first to see what specs are available for you to use. Not sure about any other ways to fix this
issue at the moment.

A GBA Emulator

- For quick tests of the emulator's functionality, use ```cargo build``` and ```cargo run -- [ROM]``` (< 60 FPS, maybe < 30 FPS)
- To run the emulator at full speed, ```cargo build --release``` and ```cargo run --release -- [ROM]``` (60 FPS+)

Some Screenshots:

![Pokemon Fire Red](https://raw.githubusercontent.com/ExPixel/Pyrite2/master/misc/screenshots/games/Pokemon-Fire-Red.png)

**Don't mind the 12FPS, I ran this in debug mode.**  

More regarding the ARM Wrestler test in [here](https://github.com/ExPixel/Pyrite2/tree/master/misc/screenshots/arm-wrestler).  
**NOTE**: The first 4 LDM tests are supposed to fail (they do on real hardware.)