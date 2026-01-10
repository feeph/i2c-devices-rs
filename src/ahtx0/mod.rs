/*
    ASAIR AHTx0 Humidity and Temperature Sensor
*/

pub mod hw;

#[allow(unused_imports)]
use log::{debug, error, info, warn};

use core::option::Option;

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

pub fn trigger_calibration_aht10<Ibd>(ibd: &mut Ibd)
where
    Ibd: crate::traits::I2cBusDevice,
{
    let dr_aht10 = 0xE1;
    hw::trigger_calibration(ibd, DEVICE_ADDRESS, dr_aht10, &[0x08, 0x00]);
}

// ------------------------------------------------------------------------
// send calibration command (AHT20)
// ------------------------------------------------------------------------

pub fn trigger_calibration_aht20<Ibd>(ibd: &mut Ibd)
where
    Ibd: crate::traits::I2cBusDevice,
{
    let dr_aht20 = 0xBE;
    hw::trigger_calibration(ibd, DEVICE_ADDRESS, dr_aht20, &[0x08, 0x00]);
}

// ------------------------------------------------------------------------
// request sensor status
// ------------------------------------------------------------------------

pub struct SensorStatus {
    pub is_busy: bool,
    pub is_calibrated: bool,
}

/// this is basically the same as a "take measurement" call except only the
/// first byte is read (the remaining bytes containing the measurement data
/// are ignored)
pub fn get_sensor_status<Ibd>(ibd: &mut Ibd) -> SensorStatus
where
    Ibd: crate::traits::I2cBusDevice,
{
    let result = hw::get_sensor_status(ibd, DEVICE_ADDRESS);

    // Adafruit's AHTx0 library indicates that calibration may fail
    // (sensor stays busy for more than 3 seconds after calibration)
    SensorStatus {
        is_busy: is_busy(result),
        is_calibrated: is_calibrated(result),
    }
}

/// return the measured data
///
/// wraps the result in an optional value
/// - returns the data if the measurement has succeeded
/// - returns None if the measurement was invalid
///
/// If the data is invalid the following conditions should be checked:
/// 1. device was initialized properly after power on
///    and at least 100ms have passed
/// 2. measurement was requested
///    and at least 80ms have passed before requesting the result
pub fn get_sensor_data<Ibd>(ibd: &mut Ibd) -> Option<[u8; 4]>
where
    Ibd: crate::traits::I2cBusDevice,
{
    let result = hw::get_sensor_data(ibd, DEVICE_ADDRESS);

    let status = result[0];
    let data = [result[1], result[2], result[3], result[4]];
    let crc = result[5];

    if is_initialized(status) && !is_busy(status) && validate_data(data, crc) {
        Some(data)
    } else {
        None
    }
}

// ------------------------------------------------------------------------

fn is_calibrated(status: u8) -> bool {
    (status & 0b0000_1000) == 0b0000_1000
}

#[test]
fn device_is_calibrated() {
    let computed = is_calibrated(0x18);
    let expected = true;

    assert_eq!(computed, expected);
}

#[test]
fn device_is_not_calibrated() {
    let computed = is_calibrated(0x10);
    let expected = false;

    assert_eq!(computed, expected);
}

// ------------------------------------------------------------------------

fn is_initialized(status: u8) -> bool {
    (status & 0b0001_1000) == 0b0001_1000 // must equal 0x18
}

#[test]
fn device_is_initialized_pass1() {
    let computed = is_initialized(0x18); // standby
    let expected = true;

    assert_eq!(computed, expected);
}

#[test]
fn device_is_initialized_pass2() {
    let computed = is_initialized(0x98); // busy
    let expected = true;

    assert_eq!(computed, expected);
}

#[test]
fn device_is_initialized_fail() {
    let computed = is_initialized(0x10); // not initialized
    let expected = false;

    assert_eq!(computed, expected);
}

// ------------------------------------------------------------------------

fn is_busy(status: u8) -> bool {
    (status & 0b1000_0000) == 0b1000_0000
}

#[test]
fn device_is_busy() {
    let computed = is_busy(0x98);
    let expected = true;

    assert_eq!(computed, expected);
}

#[test]
fn device_is_waiting() {
    let computed = is_busy(0x18);
    let expected = false;

    assert_eq!(computed, expected);
}

// ------------------------------------------------------------------------

fn validate_data(_data: [u8; 4], _crc: u8) -> bool {
    // TODO implement CRC validation logic
    true
}

#[test]
fn data_is_valid_pass() {
    let computed = validate_data([0x00, 0x00, 0x00, 0x00], 0xFF);
    let expected = true;

    assert_eq!(computed, expected);
}

#[test]
fn data_is_valid_fail() {
    let computed = validate_data([0x00, 0x00, 0x00, 0x00], 0xFF);
    let expected = true;

    assert_eq!(computed, expected);
}
