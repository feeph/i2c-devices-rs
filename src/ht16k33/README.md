# Holtek HT16K33 matrix driver

The Holtek HT16K33 is a RAM mapping 16×8 LED controller driver with keyscan.

## Usage

```RUST
// configure the I²C bus
let mut i2c_bus0 = esp_hal::i2c::master::I2c::new(peripherals.I2C0, i2c_config)
    .unwrap()
    .with_scl(pin_scl)
    .with_sda(pin_sda);

// define the device ID and number of available digits
let did = 0;     // 0 to 7
let digits = 4;  // 2, 4, 6, 8

// set display parameters (no blinking and 50% brightness)
i2c_devices::ht16k33::set_blink_rate(&mut i2c_bus, did, i2c_devices::ht16k33::BlinkRate::Steady);
i2c_devices::ht16k33::set_brightness_level(&mut i2c_bus, did, 7);

// display a value
i2c_devices::ht16k33::show_number(&mut i2c_bus, did, 1234, digits);

// indicate an alert condition by blinking the display
i2c_devices::ht16k33::set_blink_rate(&mut i2c_bus, did, i2c_devices::ht16k33::BlinkRate::Slow);

// clear display and turn off again
i2c_devices::ht16k33::clear(&mut i2c_bus, did);
```
