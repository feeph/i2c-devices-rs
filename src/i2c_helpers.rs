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

// The master communicates with slave devices using I2C transactions.
// A transaction can be a write, a read, or a combination of both.
// The I2c driver provides methods for performing these transactions.
// -- https://docs.espressif.com/projects/rust/esp-hal/1.0.0-rc.0/esp32c6/esp_hal/i2c/master/index.html#usage
