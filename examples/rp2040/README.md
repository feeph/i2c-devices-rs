# Raspberry Pi Pico 2040

Demonstrate usage of this library on Raspberry Pi Pico micro controllers.

- Raspberry Pi Pico & Raspberry Pi Pico W ([product page](https://www.raspberrypi.com/products/raspberry-pi-pico/))
- Raspberry Pi Pico 2 & Raspberry Pi Pico 2 W ([product page](https://www.raspberrypi.com/products/raspberry-pi-pico-2/))

## Overview

This minimalistic example demonstrates the implementation of the required
trait and calls a library function.

noteworthy files:

- **[.cargo/config.toml](.cargo/config.toml)** - define the build target and flags
- **[src/main.rs](src/main.rs)** - demonstrate usage
- **[Cargo.toml](Cargo.toml)** - define required libraries
- **[rust-toolchain.toml](rust-toolchain.toml)** - auto-install required targets

required files (_copied from [github.com/rp-rs/rp-hal/rp2040-hal-examples/](https://github.com/rp-rs/rp-hal/tree/main/rp2040-hal-examples)_)

- build.rs
- memory.x

## Building the example

```BASH
cd examples/rp2040
cargo build
```

Compiling for Raspberry Pico requires specific targets to be enabled.
The file `rust-toolchain.toml` defines the required targets and this
ensures `cargo build` is able to do the right thing automatically.
