# Rusty Arduino Uno

Getting up and running with rust on bare metal on an Arduino uno's ATmega328P-AU

I'll be following [the embedded rust book](https://docs.rust-embedded.org/book/intro/index.html) to get up and running with rust.  
The target will be a geekcraft uno r3 board.

# Setup.
## Tooling at the time of starting (yanked straight from the book)
- Rust 1.31, 1.31-beta, or a newer toolchain PLUS ARM Cortex-M compilation support.
- cargo-binutils ~0.1.4
- qemu-system-arm. Tested versions: 3.0.0
- OpenOCD >=0.8. Tested versions: v0.9.0 and v0.10.0
- GDB with ARM support. Version 7.12 or newer highly recommended. Tested versions: 7.10, 7.11, 7.12 and 8.1
- cargo-generate


## Installing the required tooling

```bash
xcode-select --install # if you haven't already done so
brew tap osx-cross/avr
brew install avr-gcc avrdude
```

To integrate flashing the board into the normal cargo workflow
```bash
cargo +stable install ravedude
```



# Setting up a project with the HAL

Set up a template using the [avr-hal-template](https://github.com/Rahix/avr-hal-template)
```bash
cargo install cargo-generate
cargo generate --git https://github.com/Rahix/avr-hal-template.git
```

Then just follow the instructions to generate a new project.

Notes:
- Check that the correct output hardware is created under '.cargo/config.toml' you'll find the line `target = "avr-specs/avr-atmega328p.json"` under `[build]`.

### Build and run the project

Build the project
```bash
cargo build
```

Flash the project
```bash
avrdude -pm328p -c arduino
```


<!-- # Setting up a project using the Peripheral Access Crates (PAC)
Following the instructions from the github page for the [avr-device](https://github.com/Rahix/avr-device), you'll need to find the name of your chip in their table.   
Then, add it to the `Cargo.toml` file in the `[dependencies]` section.
In this case it will be:

```bash
[dependencies.avr-device]
version = "0.5.4"
features = ["atmega328p"]
``` -->


