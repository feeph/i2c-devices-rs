/*
    ASAIR AHTx0 Humidity and Temperature Sensor
*/

pub mod hw;

#[allow(unused_imports)]
use log::{debug, error, info, warn};

// ------------------------------------------------------------------------
// constants
// ------------------------------------------------------------------------

// the device's I²C bus address is always 0x38
// you must use an I²C bus multiplexer (e.g. TCA9548A) to connect multiple
// AHT10/AHT20's to the same I²C bus
static DEVICE_ADDRESS: u8 = 0x38;

// ------------------------------------------------------------------------
// send calibration command (AHT10)
// ------------------------------------------------------------------------

pub fn send_calibration_command_aht10<Ibd>(ibd: &mut Ibd)
where
    Ibd: crate::traits::I2cBusDevice,
{
    let aht10_enable: [u8; 3] = [0xE1, 0x08, 0x00];
    hw::send_command(ibd, DEVICE_ADDRESS, &aht10_enable);
}

// ------------------------------------------------------------------------
// send calibration command (AHT20)
// ------------------------------------------------------------------------

pub fn send_calibration_command_aht20<Ibd>(ibd: &mut Ibd)
where
    Ibd: crate::traits::I2cBusDevice,
{
    let aht20_enable: [u8; 3] = [0xBE, 0x08, 0x00];
    hw::send_command(ibd, DEVICE_ADDRESS, &aht20_enable);
}
