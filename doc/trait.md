# Implementing the I²C Bus Device (IBD) trait

In order to provide a device-agnostic library a trait is used to define a
generic interface for I²C bus devices. The library does not care which
exact calls need to be made to the hardware, it just needs a generic way to
make them.

## Espressif32 (esp-hal v1.x)

- https://www.espressif.com/en/products/socs/esp32
- https://docs.espressif.com/projects/rust/index.html (device-dependent)

**CAUTION**: You must make sure to **use esp-hal 1.0**, e.g. '1.0.0-rc.0'.

- For a detailed description of the differences between 0.23 and 1.0 please
  refer to the [esp-hal 1.0.0 beta announcement](https://developer.espressif.com/blog/2025/02/rust-esp-hal-beta).
- Version 0.23.x will NOT work. Do not use 0.23 or any other non-1.x version!

The provided code was used with an ESP32-C6. Other Espressif chips may need
slightly different code. Please refer to the
[hardware-specific documentation](https://docs.espressif.com/projects/rust/esp-hal/1.0.0-rc.0/index.html).

```RUST
struct I2cBusDevice<'a, Dm: esp_hal::DriverMode> {
    i2c_bus: &'a mut esp_hal::i2c::master::I2c<'a, Dm>
}

impl<'a, Dm: esp_hal::DriverMode> i2c_devices::I2cBusDevice for I2cBusDevice<'a, Dm> {

    fn read_byte(&mut self, da: u8) -> Result<u8, &'static str> {
        let mut buf = [0, 1];

        let res = self.i2c_bus.read(da, &mut buf);
        match res {
            Ok(_) => {
                Ok(buf[0])
            }
            Err(_) => {
                Err("")
            }
        }
    }

    fn write_byte(&mut self, da: u8, byte: u8) {
        let _ = self.i2c_bus.write(da, &[byte]);
    }

    fn write_bytes(&mut self, da: u8, bytes: &[u8]) {
        let _ = self.i2c_bus.write(da, bytes);
    }

    fn read_register_as_byte(&mut self, da: u8, dr: u8) -> u8 {
        let mut rb = [0u8; 1];

        // TODO add error handling for read_register_as_u8()
        let _ = self.i2c_bus.write_read(da, &[dr], &mut rb);

        // implicit return
        rb[0]
    }

    fn write_register_as_byte(&mut self, da: u8, dr: u8, byte: u8) {
        // TODO add error handling for write_register_as_u8()
        let _ = self.i2c_bus.write(da, &[dr, byte]);
    }

    fn read_multibyte_register_as_u8<const N: usize>(&mut self, da: u8, dr: [u8; N]) -> [u8; N] {
        let mut rb = [0u8; N];

        // it's a bit overkill to use a loop for two iterations but that way we
        // avoid code duplication and it opens up the possibility of reading an
        // arbitrary number of values
        for (i, register) in dr.iter().enumerate() {
            let mut v = [0; 1];
            match self.i2c_bus.write_read(da, &[*register], &mut v) {
                Ok(_) => {
                    debug!(
                        "Successfully read register '{0:#04X}' (value: {1:#04X}).",
                        dr[i], rb[i]
                    );
                    rb[i] = v[0];
                }
                Err(reason) => warn!("Failed to read register '{0:#04X}': {reason}", dr[i]),
            }
        }

        // implicit return
        rb
    }

    fn write_multibyte_register_as_u8<const N: usize>(&mut self, da: u8, values: [[u8; 2]; N]) {
        for x in values.iter() {
            match self.i2c_bus.write(da, x) {
                Ok(_) => {
                    debug!(
                        "Successfully wrote register '{0:#04X}' (value: {1:#04X}).",
                        x[0], x[1]
                    );
                }
                Err(reason) => warn!("Failed to read register '{0:#04X}': {reason}", x[0]),
            }
        }
    }

    // some hardware functions require a little time to pass
    // - functions that sleep mention this fact in their documentation
    // - sleeping is hardware-dependent, no_std provides no abstraction
    fn sleep_ms(&mut self, milliseconds: u32) {
        esp_hal::delay::Delay::new().delay_millis(milliseconds);
    }
}
```

## Raspberry Pico (rp-hal)

- https://www.raspberrypi.com/products/rp2040
- https://www.raspberrypi.com/products/rp2350

```RUST
// TODO provide example code for rp2040/rp2350
```
