/*
    low-level integration tests for AHTx0
    (using 'i2c_devices::ahtx0::hw')
*/

mod common;

use common::VirtualAHTx0;

use i2c_devices::ahtx0::{AHTx0, SensorData, SensorState};

// ------------------------------------------------------------------------

#[test]
fn trigger_reset_pass() {
    let mut vbd = create_ahtx0([0x1C, 0x44, 0xF2, 0xF5, 0xF9, 0xD6, 0x34]);
    let mut aht20 = i2c_devices::ahtx0::create_aht20();

    let computed = aht20.trigger_reset(&mut vbd);
    let expected = SensorState::Idle;

    assert_eq!(computed, expected);
    assert_eq!(vbd.lwb, [0xBA, 0xFF, 0xEE]);
}

#[test]
fn trigger_calibration_pass() {
    let mut vbd = create_ahtx0([0x1C, 0x44, 0xF2, 0xF5, 0xF9, 0xD6, 0x34]);
    let mut aht20 = i2c_devices::ahtx0::create_aht20();

    let computed = aht20.trigger_calibration(&mut vbd);
    let expected = SensorState::Idle;

    assert_eq!(computed, expected);
    assert_eq!(vbd.lwb, [0xBE, 0x08, 0x00]);
}

#[test]
fn get_sensor_status_not_initialized() {
    let mut vbd = create_ahtx0([0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    let mut aht20 = i2c_devices::ahtx0::create_aht20();

    let computed = aht20.get_sensor_status(&mut vbd);
    let expected = SensorState::NotInitialized;

    assert_eq!(computed, expected);
}

#[test]
fn get_sensor_status_not_calibrated() {
    let mut vbd = create_ahtx0([0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    let mut aht20 = i2c_devices::ahtx0::create_aht20();

    let computed = aht20.get_sensor_status(&mut vbd);
    let expected = SensorState::NotCalibrated;

    assert_eq!(computed, expected);
}

#[test]
fn get_sensor_status_busy() {
    let mut vbd = create_ahtx0([0x9C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    let mut aht20 = i2c_devices::ahtx0::create_aht20();

    let computed = aht20.get_sensor_status(&mut vbd);
    let expected = SensorState::Busy;

    assert_eq!(computed, expected);
}

#[test]
fn get_sensor_status_idle() {
    let mut vbd = create_ahtx0([0x1C, 0x44, 0xF2, 0xF5, 0xF9, 0xD6, 0x34]);
    let mut aht20 = i2c_devices::ahtx0::create_aht20();

    let computed = aht20.get_sensor_status(&mut vbd);
    let expected = SensorState::Idle;

    assert_eq!(computed, expected);
}

#[test]
fn trigger_measurement_pass() {
    let mut vbd = create_ahtx0([0x1C, 0x44, 0xF2, 0xF5, 0xF9, 0xD6, 0x34]);
    let mut aht20 = i2c_devices::ahtx0::create_aht20();

    let computed = aht20.trigger_measurement(&mut vbd);
    let expected = SensorState::Idle;

    assert_eq!(computed, expected);
}

#[test]
fn trigger_measurement_fail() {
    let mut vbd = create_ahtx0([0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    let mut aht20 = i2c_devices::ahtx0::create_aht20();

    let computed = aht20.trigger_measurement(&mut vbd);
    let expected = SensorState::NotInitialized;

    assert_eq!(computed, expected);
}

#[test]
fn get_sensor_data_pass() {
    let mut vbd = create_ahtx0([0x1C, 0x44, 0xF2, 0xF5, 0xF9, 0xD6, 0x34]);
    let mut aht20 = i2c_devices::ahtx0::create_aht20();

    let computed = aht20.get_sensor_data(&mut vbd);
    let expected = Ok(SensorData {
        temperature: 24.69902,
        humidity: 26.933193,
    });

    assert_eq!(computed, expected);
}

#[test]
fn get_sensor_data_busy() {
    let mut vbd = create_ahtx0([0x9C, 0x44, 0xF2, 0xF5, 0xF9, 0xD6, 0x00]);
    let mut aht10 = i2c_devices::ahtx0::create_aht10();

    let computed = aht10.get_sensor_data(&mut vbd);
    let expected = Err(SensorState::Busy);

    assert_eq!(computed, expected);
}

#[test]
fn get_sensor_data_crc_fail() {
    let mut vbd = create_ahtx0([0x1C, 0x44, 0xF2, 0xF5, 0xF9, 0xD6, 0x00]);
    let mut aht20 = i2c_devices::ahtx0::create_aht20();

    let computed = aht20.get_sensor_data(&mut vbd);
    let expected = Err(SensorState::DataInvalid);

    assert_eq!(computed, expected);
}

// ------------------------------------------------------------------------

fn create_ahtx0(data: [u8; 7]) -> VirtualAHTx0 {
    VirtualAHTx0 {
        data,
        lwb: [0xC0, 0xFF, 0xEE], // sentinel value
    }
}
