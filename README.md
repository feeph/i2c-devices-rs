# I²C devices for Rust

Rust library for various I²C devices.

- [Supported devices](#supported-devices)
- [Requirements](#requirements)
- [Usage](#usage)
  - [augmented API (recommended)](#augmented-api-recommended)
  - [low-level API](#low-level-api)

## Supported devices

| module   | category                            | compatible with:                                     | support level |
| -------- | ----------------------------------- | ---------------------------------------------------- | ------------- |
| ads1x1x  | analog-to-digital converter         | ADS1013, ADS1013, ADS1015, ADS1113, ADS1113, ADS1115 | stub          |
| aht20    | temperature & humidity sensor       | AHT20                                                | stub          |
| at24c0xd | Serial EEPROM                       | AT24C01D, AT24C02D                                   | stub          |
| emc2101  | temperature sensor & fan controller | EMC2101, EMC2101-R                                   | usable        |
| ht16k33  | LED matrix controller               | HT16K33                                              | usable        |
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

Access to the I²C bus is hardware-specific. This code does not depend on
any specific hardware abstraction layer (esp-hal, rp-hal or similar). This
library provides the hardware-agnostic trait 'I2cBusDevice' which must be
implemented by the calling code.

## Usage

Please check the provided examples how to implement the trait and use this
library:

- [examples/esp32-c6/](examples/esp32-c6/) - Espressif32, using esp-hal
- [examples/rp2040/](examples/rp2040) - Raspberry Pi Pico, using rp-hal

### augmented API (recommended)

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

### low-level API

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
