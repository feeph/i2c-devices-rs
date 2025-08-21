# I²C devices for Rust

Rust library for various I²C devices.

- [Supported devices](#supported-devices)
- [Requirements](#requirements)
- [Usage](#usage)
  - [initialize the bus](#initialize-the-bus)
  - [scan the bus for connected devices](#scan-the-bus-for-connected-devices)
  - [use a device](#use-a-device)
    - [augmented API (recommended)](#augmented-api-recommended)
    - [low-level API](#low-level-api)

## Supported devices

| module   | category                            | compatible with:                                     | support level |
| -------- | ----------------------------------- | ---------------------------------------------------- | ------------- |
| ads1x1x  | analog-to-digital converter         | ADS1013, ADS1013, ADS1015, ADS1113, ADS1113, ADS1115 | stub          |
| aht20    | temperature & humidity sensor       | AHT20                                                | stub          |
| at24c0xd | Serial EEPROM                       | AT24C01D, AT24C02D                                   | stub          |
| emc2101  | temperature sensor & fan controller | EMC2101, EMC2101-R                                   | usable        |
| ht16k33  | matrix driver with key scan         | HT16K33                                              | wip           |
| tca953x  | I/O Expander                        | TCA9536, TCA9537                                     | stub          |
| tca9548a | I²C bus multiplexer                 | TCA9548A                                             | stub          |

support levels:

- stub: Don't expect anything.
- wip: Work in progress. Some functionality exists.
- usable: The code is in a usable state but the interface may still change.
- stable: The code's implementation and the interface has stabilized.
- maintenance: The code is in maintenance mode, no additional features will be added.
- deprecated: The code is considered deprecated and scheduled for removal.

## Requirements

This code requires esp_hal (1.0.0-rc0) with feature 'unstable' enabled:

```shell
cargo add esp-hal@=1.0.0-rc.0 -F esp32c6,log-04,unstable
```

You MUST NOT use esp_hal (0.23.x) or esp32-hal!

See https://developer.espressif.com/blog/2025/02/rust-esp-hal-beta for details.

[2025-08-03] _Version 1.0.0-rc0 is the most recent release._

## Usage

### initialize the bus

This configuration assumes to run on an Espressif ESP32-C6.

Other options are:  ESP32, ESP32-C2, ESP32-C3, ESP32-H2, ESP32-S2, ESP32-S3.

- Please check https://docs.espressif.com/projects/rust/esp-hal/1.0.0-rc.0/
  for instructions for these other supported devices.
- Please check https://docs.espressif.com/projects/rust/ for an overview of
  all esp_hal-related modules.

```rust
// use the same pins that would be used in ESP32-C6's "Low Power" mode
// to make circuit layouts compatible with both modes
let pin_sda = peripherals.GPIO6;
let pin_scl = peripherals.GPIO7;

// set the bus frequency
// - I²C standard mode: 100kHz
// - I²C fast mode:     400kHz
let i2c_config = esp_hal::i2c::master::Config::default()
    .with_frequency(esp_hal::time::Rate::from_khz(100));

info!("Initialize I²C channel 0. (SCL: GPIO{}, SDA: GPIO{})", pin_scl.number(), pin_sda.number());
let mut i2c_bus0 = esp_hal::i2c::master::I2c::new(peripherals.I2C0, i2c_config)
    .unwrap()
    .with_scl(pin_scl)
    .with_sda(pin_sda);
```

### scan the bus for connected devices

```rust
info!("Scanning the I²C bus for devices.");
let i2c_dev = i2c_devices::scan_i2c_bus(&mut i2c_bus0);

let mut devices_found = false;
for (addr, found) in i2c_dev.iter().enumerate() {
    if addr > 0 && *found {
        info!("Found an I²C device at address {:#04X}.", addr);
        devices_found = true;
    }
}

if !devices_found {
    warn!("Unable to find any I²C devices connected to this bus!");
}
```

### use a device

#### augmented API (recommended)

An augmented, higher-level API is provided for user convenience. This API
parses and validates the provided data.

Typical usage scenarios would be exploration and application code.

```rust
// read & parse the hardware registers
// (use auto-completion to explore all available fields)
let hwd = i2c_devices::emc2101::get_hardware_details(&mut i2c_bus0);
// -> returns struct 'HardwareDetails'
let cfg = i2c_devices::emc2101::get_config_register(&mut i2c_bus0);
// -> returns struct 'ConfigRegister'

// print the parsed values
info!("Hardware Details: {} {} (rev {})", hwd.manufacturer, hwd.product, hwd.revision);
info!("Config Register:");
info!("  mask:        {}", cfg.mask);
info!("  standby:     {}", cfg.standby);
```

#### low-level API

Direct access to the hardware is possible using the low-level API. This API
does not perform any parsing or validation. The device's data sheet is
required to understand the data.

Typical usage scenarios would be extremely resource-constrained devices or
debugging a device's functionality.

```rust
// read the hardware registers (without any parsing or validation)
let mid = i2c_devices::emc2101::hw::get_manufacturer_id(&mut i2c_bus0);
let pid = i2c_devices::emc2101::hw::get_product_id(&mut i2c_bus0);
let rev = i2c_devices::emc2101::hw::get_product_revision(&mut i2c_bus0);
let cfg = i2c_devices::emc2101::hw::get_config_register(&mut i2c_bus0);

// print the unparsed values
// (requires data sheet to discern meaning)
info!("Manufacturer ID:  {:#04X}", mid);
info!("Product ID:       {:#04X}", pid);
info!("Product Revision: {:#04X}", rev);
info!("Config Register:  {:#04X}", cfg);
```
