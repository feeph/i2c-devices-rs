/*
    common I²C-related functions
*/

#[allow(unused_imports)]
use log::{debug, error, info, warn};

/// scan the I²C bus for devices
/// - assumes 8 bit addressing (128 devices, 0..127)
/// - 10 bit addressing is not supported (1024 devices, 0..1024)
///
/// returns an array of booleans (Vector is not available in 'no_std')
pub fn scan_i2c_bus<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> [bool; 128]
where
    Dm: esp_hal::DriverMode,
{
    let mut d = [false; 128];

    // the I²C bus master is always present (address 0)
    d[0] = true;

    for addr in 1..=127u8 {
        debug!("Scanning for I²C device at address {addr}.");

        let res = i2c_bus.read(addr, &mut [0]);
        match res {
            Ok(_) => {
                d[addr as usize] = true;
                debug!("Found an I²C device at address {addr}.");
            }
            Err(_) => {
                debug!("Unable to find a device at address {addr}.");
            }
        }
    }

    // implicit return
    d
}

/// read a single byte from device register 'dr'
pub fn read_register_as_u8<Dm>(
    i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>,
    da: u8,
    dr: u8,
) -> u8
where
    Dm: esp_hal::DriverMode,
{
    let mut rb = [0u8; 1];
    // TODO add error handling for read_register_as_u8()
    let _ = i2c_bus.write_read(da, &[dr], &mut rb);

    // implicit return
    rb[0]
}

/// write a single byte to device register 'dr'
pub fn write_register_as_u8<Dm>(
    i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>,
    da: u8,
    dr: u8,
    byte: u8,
) where
    Dm: esp_hal::DriverMode,
{
    // TODO add error handling for write_register_as_u8()
    let _ = i2c_bus.write(da, &[dr, byte]);
}

// The master communicates with slave devices using I2C transactions.
// A transaction can be a write, a read, or a combination of both.
// The I2c driver provides methods for performing these transactions.
// -- https://docs.espressif.com/projects/rust/esp-hal/1.0.0-rc.0/esp32c6/esp_hal/i2c/master/index.html#usage

/// read two independent registers in the exact order provided
///
/// returns the two values in exactly the same order
// TODO this function's code is ugly (but it works) -> try to refactor later on
pub fn read_multibyte_register_as_u8<Dm>(
    i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>,
    da: u8,
    dr: [u8; 2],
) -> [u8; 2]
where
    Dm: esp_hal::DriverMode,
{
    let mut rb = [0u8; 2];

    // it's a bit overkill to use a loop for two iterations but that way we
    // avoid code duplication and it opens up the possibility of reading an
    // arbitrary number of values
    for (i, register) in dr.iter().enumerate() {
        let mut v = [0; 1];
        match i2c_bus.write_read(da, &[*register], &mut v) {
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

/// write two independent registers in the exact order provided
pub fn write_multibyte_register_as_u8<Dm>(
    i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>,
    da: u8,
    values: [[u8; 2]; 2],
) where
    Dm: esp_hal::DriverMode,
{
    for x in values.iter() {
        match i2c_bus.write(da, x) {
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
