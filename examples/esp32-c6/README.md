# Espressif ESP32-C6

Demonstrate usage of this library on Espressif micro controllers.

- https://www.espressif.com/en/products/socs/esp32
- https://docs.espressif.com/projects/rust/index.html (device-dependent)

**CAUTION**: You must make sure to **use esp-hal 1.0**, e.g. '1.0.0'.

- For a detailed description of the differences between 0.23 and 1.0 please
  refer to the [esp-hal 1.0.0 release announcement](https://developer.espressif.com/blog/2025/10/esp-hal-1).
- Version 0.23.x will NOT work. Do not use 0.23 or any other non-1.x version!

The provided code was used with an ESP32-C6. Other Espressif chips may need
slightly different code. Please refer to the
[hardware-specific documentation](https://docs.espressif.com/projects/rust/esp-hal/1.0.0-rc.0/index.html).

## Overview

This example demonstrates the implementation of the required trait and
calls a library function. In addition to the core functionality is uses
the [embassy framework](https://embassy.dev)

noteworthy files:

- **[.cargo/config.toml](.cargo/config.toml)** - define the build target and flags
- **[src/main.rs](src/main.rs)** - demonstrate usage
- **[Cargo.toml](Cargo.toml)** - define required libraries
- **[rust-toolchain.toml](rust-toolchain.toml)** - auto-install required targets

## Building the example

```BASH
cd examples/esp32-c6
cargo build
```

Compiling for ESP32-C6 requires specific targets to be enabled.
The file `rust-toolchain.toml` defines the required targets and this
ensures `cargo build` is able to do the right thing automatically.

## Initialize a new project

You can use the `esp-generate` tool to initialize a new project. It provides
a convenient way to select the features you want.

install the utility:

```BASH
cargo install esp-config --features=tui --locked
cargo install esp-generate --locked
```

generate a new project:

```BASH
esp-generate --chip=esp32c6 <project_name>

# e.g.:
esp-generate --chip esp32c6 -o unstable-hal -o embassy -o alloc -o wifi -o log -o esp-backtrace -o ci -o vscode hello_world
cargo add i2c_devices --no-default-features
```
