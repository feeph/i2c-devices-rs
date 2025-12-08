# Holtek HT16K33 matrix driver

The Holtek HT16K33 is a RAM mapping 16×8 LED controller driver with
keyscan support. It can be used to drive a 7-segment display, 14-segment
display or LED matrix.

## Design

The low-level interface provided by the hardware is exposed as
`i2c_devices::ht16k33::*`. This interface is a bit tricky because the
order of operations is important and the meaning of addressable bits
changes according to the actual output controlled by the HT16K33 and
how it is wired up.

To provide a better user experience a high-level, more convenient
interface is offered to the user. The following characteristics apply:

- The order of operations does not matter, user can configure settings in
  whatever order they want. The correct order of operations is applied when
  sending the desired output to the display.
- Provided input is sanity-checked. Device ID, brightness level and display
  mode are limited to the values supported by the hardware.
- User can provide their own conversion functions to allow display of
  customized character sets.

In order to support multi-threaded execution the I²C bus is provided as a
parameter. This ensures the I²C bus is locked and held exactly as long as
needed and no longer. The Rust concurrency model ensures that no parallel
access to the I²C bus occurs.

## Usage

See 'examples/' for fully working examples on how to use the I²C devices
library with Espressif32 and Raspberry Pico.

The following steps must be followed:

1.) Initialize the I²C bus as required by hardware.
2.) Create a hardware-agnostic I²C bus device.
3.) Use this library to control I²C bus devices.

### create I²C bus (Espressif32)

initialize the I²C bus as required by hardware

```RUST
let pin_sda = peripherals.GPIO6;
let pin_scl = peripherals.GPIO7;

let i2c_config =
    esp_hal::i2c::master::Config::default().with_frequency(esp_hal::time::Rate::from_khz(400));

#[allow(unused_mut)]
let mut i2c_bus0 = esp_hal::i2c::master::I2c::new(peripherals.I2C0, i2c_config)
    .unwrap()
    .with_scl(pin_scl)
    .with_sda(pin_sda);
```

create a hardware-agnostic I²C bus device

```RUST
let mut ibd = I2cBusDevice {
    i2c_bus: &mut i2c_bus,
    timer,
};
```

### create I²C bus (Raspberry Pico)

initialize the I²C bus as required by hardware

```RUST
let sda_pin: hal::gpio::Pin<_, hal::gpio::FunctionI2C, _> = pins.gpio18.reconfigure();
let scl_pin: hal::gpio::Pin<_, hal::gpio::FunctionI2C, _> = pins.gpio19.reconfigure();

let mut i2c_bus = hal::I2C::i2c1(
    pac.I2C1,
    sda_pin,
    scl_pin,
    400.kHz(),
    &mut pac.RESETS,
    &clocks.system_clock,
);

let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
```

create a hardware-agnostic I²C bus device

```RUST
let mut ibd = I2cBusDevice {
    i2c_bus: &mut i2c_bus,
    timer,
};
```

### control a 7-segment display

use this library to control I²C bus devices

```RUST
// initialize device handler
let mut sd1 = i2c_devices::ht16k33::Segment7x4 {
    // device identifier (range: 0-7)
    did: 1,
    // brightness level (range: 0-15)
    brightness_level: 1,
    // On, Off, BlinkSlow, BlinkMedium, BlinkFast
    display_mode: i2c_devices::ht16k33::DisplayMode::On,
    // this functions defines how to map character to segments
    convert: i2c_devices::ht16k33::convert_7,
};

// show something
sd1.show_string(&mut ibd, "12:34");

// change blink rate and brightness
sd1.set_display_mode(&mut ibd, i2c_devices::ht16k33::DisplayMode::BlinkSlow);
sd1.set_brightness_level(&mut ibd, 4);

// show something else
sd2.show_number(&mut ibd, 1.234);
```

_The `did` parameter is the device's ID in binary representation._

|  A0     |  A1     |  A2     | did |
|:-------:|:-------:|:-------:|:---:|
| open    | open    | open    |  0  |
| bridged | open    | open    |  1  |
| open    | bridged | open    |  2  |
| bridged | bridged | open    |  3  |
| open    | open    | bridged |  4  |
| bridged | open    | bridged |  5  |
| open    | bridged | bridged |  6  |
| bridged | bridged | bridged |  7  |
