#[allow(unused_imports)]
use log::{debug, error, info, warn};

// the device's I²C bus address is always 0x4C
// you must use an I²C bus multiplexer (e.g. TCA9548A) to connect multiple
// EMC2101's to the same I²C bus
static DEVICE_ADDRESS: u8 = 0x4C;

/// read a single byte from device register 'dr'
pub fn read_register_as_u8<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>, dr: u8) -> u8
where
    Dm: esp_hal::DriverMode,
{
    let mut rb = [0u8; 1];
    // TODO add error handling for read_register_as_u8()
    let _ = i2c_bus.write_read(DEVICE_ADDRESS, &[dr], &mut rb);

    // implicit return
    rb[0]
}

/// write a single byte to device register 'dr'
pub fn write_register_as_u8<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>, dr: u8, byte: u8)
where
    Dm: esp_hal::DriverMode,
{
    // TODO add error handling for write_register_as_u8()
    let _ = i2c_bus.write(DEVICE_ADDRESS, &[dr, byte]);
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
        match i2c_bus.write_read(DEVICE_ADDRESS, &[*register], &mut v) {
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
