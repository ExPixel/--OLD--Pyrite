# Pyrite

A GBA Emulator

- For quick tests of the emulator's functionality, use ```cargo build``` and ```cargo run -- [ROM]``` (< 60 FPS, maybe < 30 FPS)
- To run the emulator at full speed, ```cargo build --release``` and ```cargo run --release -- [ROM]``` (60 FPS+)

Some Screenshots:

![Affine Background Demo](https://raw.githubusercontent.com/ExPixel/Pyrite2/master/misc/screenshots/sbb-aff/PyriteSbbAffGif.gif)

More regarding the ARM Wrestler test in [here](https://github.com/ExPixel/Pyrite2/tree/master/misc/screenshots/arm-wrestler).  
**NOTE**: The first 4 LDM tests are supposed to fail (they do on real hardware.)