/*
    a virtual I²C block device (used for testing without hardware)
*/

pub struct VirtualI2cBusDevice {
    // TODO store read-only and read-write registers
}

impl i2c_devices::I2cBusDevice for VirtualI2cBusDevice {
    fn read_byte(&mut self, _da: u8) -> Result<u8, &'static str> {
        // ...

        Ok(0)
    }

    fn write_byte(&mut self, _da: u8, _byte: u8) {

        // ...
    }

    fn write_bytes(&mut self, _da: u8, _bytes: &[u8]) {

        // ...
    }

    fn read_register_as_byte(&mut self, _da: u8, _dr: u8) -> u8 {
        // ...

        0
    }

    fn write_register_as_byte(&mut self, _da: u8, _dr: u8, _byte: u8) {

        // ...
    }

    fn read_multibyte_register_as_u8<const N: usize>(&mut self, _da: u8, _dr: [u8; N]) -> [u8; N] {
        let mut rb = [0u8; N];

        // ...

        rb
    }

    fn write_multibyte_register_as_u8<const N: usize>(&mut self, _da: u8, _values: [[u8; 2]; N]) {

        // ...
    }

    // some hardware functions require a little time to pass
    // - functions that sleep mention this fact in their documentation
    // - sleeping is hardware-dependent, no_std provides no abstraction
    fn sleep_ms(&mut self, _milliseconds: u32) {

        // ...
    }
}
