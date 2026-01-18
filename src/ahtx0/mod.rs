/*
    ASAIR AHTx0 Humidity and Temperature Sensor

    The device's I²C bus address is always 0x38. You must use an I²C bus
    multiplexer (e.g. TCA9548A) to connect multiple AHT10/AHT20 sensors to
    the same I²C bus.
*/

mod data;
pub mod hw;

#[allow(unused_imports)]
use log::{debug, error, info, warn};

use core::result::Result;
pub use data::SensorData;

pub trait AHTx0 {
    fn trigger_reset<Ibd>(&mut self, ibd: &mut Ibd) -> SensorState
    where
        Ibd: crate::traits::I2cBusDevice;

    fn trigger_calibration<Ibd>(&mut self, ibd: &mut Ibd) -> SensorState
    where
        Ibd: crate::traits::I2cBusDevice;
    /// this is basically the same as a "take measurement" call except only the
    /// first byte is read (the remaining bytes containing the measurement data
    /// are ignored)
    fn get_sensor_status<Ibd>(&mut self, ibd: &mut Ibd) -> SensorState
    where
        Ibd: crate::traits::I2cBusDevice;

    fn trigger_measurement<Ibd>(&mut self, ibd: &mut Ibd) -> SensorState
    where
        Ibd: crate::traits::I2cBusDevice;

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
    fn get_sensor_data<Ibd>(&mut self, ibd: &mut Ibd) -> Result<SensorData, SensorState>
    where
        Ibd: crate::traits::I2cBusDevice;
}

// ------------------------------------------------------------------------
// AHT10
// ------------------------------------------------------------------------

pub struct AHT10 {
    da: u8, // device address
    dr: u8, // device reset register (type-specific)
}

pub fn create_aht10() -> AHT10 {
    AHT10 { da: 0x38, dr: 0xE1 }
}

impl AHTx0 for AHT10 {
    /// trigger device reset
    fn trigger_reset<Ibd>(&mut self, ibd: &mut Ibd) -> SensorState
    where
        Ibd: crate::traits::I2cBusDevice,
    {
        hw::trigger_reset(ibd, self.da);
        self.get_sensor_status(ibd)
    }

    /// trigger device calibration
    fn trigger_calibration<Ibd>(&mut self, ibd: &mut Ibd) -> SensorState
    where
        Ibd: crate::traits::I2cBusDevice,
    {
        hw::trigger_calibration(ibd, self.da, self.dr, &[0x08, 0x00]);
        self.get_sensor_status(ibd)
    }

    /// this is basically the same as a "take measurement" call except only the
    /// first byte is read (the remaining bytes containing the measurement data
    /// are ignored)
    fn get_sensor_status<Ibd>(&mut self, ibd: &mut Ibd) -> SensorState
    where
        Ibd: crate::traits::I2cBusDevice,
    {
        let status_byte = hw::get_sensor_status(ibd, self.da);

        // Adafruit's AHTx0 library indicates that calibration may fail
        // (sensor stays busy for more than 3 seconds after calibration)
        parse_status_byte(status_byte)
    }

    fn trigger_measurement<Ibd>(&mut self, ibd: &mut Ibd) -> SensorState
    where
        Ibd: crate::traits::I2cBusDevice,
    {
        hw::trigger_measurement(ibd, self.da);
        self.get_sensor_status(ibd)
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
    fn get_sensor_data<Ibd>(&mut self, ibd: &mut Ibd) -> Result<SensorData, SensorState>
    where
        Ibd: crate::traits::I2cBusDevice,
    {
        let data = hw::get_sensor_data::<Ibd, 6>(ibd, self.da);

        let status = parse_status_byte(data[0]);
        if status == SensorState::Idle {
            Ok(data::parse_data_without_crc(&data))
        } else {
            Err(status)
        }
    }
}

// ------------------------------------------------------------------------
// AHT20
// ------------------------------------------------------------------------

pub struct AHT20 {
    da: u8, // device address
    dr: u8, // device reset register (type-specific)
}

pub fn create_aht20() -> AHT20 {
    AHT20 { da: 0x38, dr: 0xBE }
}

impl AHTx0 for AHT20 {
    /// trigger device reset
    fn trigger_reset<Ibd>(&mut self, ibd: &mut Ibd) -> SensorState
    where
        Ibd: crate::traits::I2cBusDevice,
    {
        hw::trigger_reset(ibd, self.da);
        self.get_sensor_status(ibd)
    }

    /// trigger device calibration
    fn trigger_calibration<Ibd>(&mut self, ibd: &mut Ibd) -> SensorState
    where
        Ibd: crate::traits::I2cBusDevice,
    {
        hw::trigger_calibration(ibd, self.da, self.dr, &[0x08, 0x00]);
        self.get_sensor_status(ibd)
    }

    /// this is basically the same as a "take measurement" call except only the
    /// first byte is read (the remaining bytes containing the measurement data
    /// are ignored)
    fn get_sensor_status<Ibd>(&mut self, ibd: &mut Ibd) -> SensorState
    where
        Ibd: crate::traits::I2cBusDevice,
    {
        let status_byte = hw::get_sensor_status(ibd, self.da);

        // Adafruit's AHTx0 library indicates that calibration may fail
        // (sensor stays busy for more than 3 seconds after calibration)
        parse_status_byte(status_byte)
    }

    fn trigger_measurement<Ibd>(&mut self, ibd: &mut Ibd) -> SensorState
    where
        Ibd: crate::traits::I2cBusDevice,
    {
        hw::trigger_measurement(ibd, self.da);
        self.get_sensor_status(ibd)
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
    fn get_sensor_data<Ibd>(&mut self, ibd: &mut Ibd) -> Result<SensorData, SensorState>
    where
        Ibd: crate::traits::I2cBusDevice,
    {
        let data = hw::get_sensor_data::<Ibd, 7>(ibd, self.da);

        let status = parse_status_byte(data[0]);
        if status == SensorState::Idle {
            let result = data::parse_data_with_crc(&data);
            match result {
                Some(x) => Ok(x),
                None => Err(SensorState::DataInvalid),
            }
        } else {
            Err(status)
        }
    }
}

// ------------------------------------------------------------------------
// request sensor status
// ------------------------------------------------------------------------

pub struct SensorStatus {
    pub is_busy: bool,
    pub is_calibrated: bool,
}

// ------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub enum SensorState {
    NotInitialized,
    NotCalibrated,
    Busy,
    Idle,
    DataInvalid,
}

fn parse_status_byte(status_byte: u8) -> SensorState {
    // according to data sheet, section 7.3:
    // 0b...._..XX retain (typically 0)
    // 0b...._.X.. retain (sometimes 1)
    // 0b...._X... CAL enable (1 - calibrated, 0 - uncalibrated)
    // 0b...X.... retain (typically 1)
    // 0b.XX._.... retain (typically 0)
    // 0bX..._.... busy indication (1 - busy, 0 - idle)
    //
    // commonly seen values are: 0x08, 0x18, 0x1C and 0x98
    if (status_byte & 0b0000_1000) != 0b0000_1000 {
        SensorState::NotCalibrated
    } else if (status_byte & 0b0001_1000) != 0b0001_1000 {
        SensorState::NotInitialized
    } else if (status_byte & 0b1000_0000) == 0b1000_0000 {
        SensorState::Busy
    } else {
        SensorState::Idle
    }
}

#[test]
fn device_is_not_calibrated() {
    let computed = parse_status_byte(0x10);
    let expected = SensorState::NotCalibrated;

    assert_eq!(computed, expected);
}

#[test]
fn device_is_not_initialized() {
    let computed = parse_status_byte(0x08);
    let expected = SensorState::NotInitialized;

    assert_eq!(computed, expected);
}

#[test]
fn device_is_busy() {
    let computed = parse_status_byte(0x98);
    let expected = SensorState::Busy;

    assert_eq!(computed, expected);
}

#[test]
fn device_is_idle_1() {
    let computed = parse_status_byte(0x18);
    let expected = SensorState::Idle;

    assert_eq!(computed, expected);
}

#[test]
fn device_is_idle_2() {
    let computed = parse_status_byte(0x1C);
    let expected = SensorState::Idle;

    assert_eq!(computed, expected);
}
