/*
    low-level integration tests for AHTx0
    (using 'i2c_devices::ahtx0::hw')
*/

mod common;

use common::VirtualAHTx0;

use i2c_devices::ahtx0::hw as sut;

// ------------------------------------------------------------------------

#[test]
fn trigger_reset_pass() {
    let mut vbd = create_ahtx0([0x00; 7]);

    // pub fn trigger_reset<Ibd>(ibd: &mut Ibd, da: u8) -> bool
    let computed = sut::trigger_reset(&mut vbd, 0x38);
    let expected = true;

    assert_eq!(computed, expected);
    assert_eq!(vbd.lwb, [0xBA, 0xFF, 0xEE]);
}

#[test]
fn trigger_calibration_pass() {
    let mut vbd = create_ahtx0([0x00; 7]);

    // pub fn trigger_calibration<Ibd>(ibd: &mut Ibd, da: u8, dr: u8, command: &[u8; 2]) -> bool
    let computed = sut::trigger_calibration(&mut vbd, 0x38, 0xBE, &[0x08, 0x00]);
    let expected = true;

    assert_eq!(computed, expected);
    assert_eq!(vbd.lwb, [0xBE, 0x08, 0x00]);
}

#[test]
fn trigger_trigger_measurement() {
    let mut vbd = create_ahtx0([0x00; 7]);

    // pub fn trigger_measurement<Ibd>(ibd: &mut Ibd, da: u8)
    sut::trigger_measurement(&mut vbd, 0x38);

    let computed = vbd.lwb;
    let expected = [0xAC, 0x33, 0x00];

    assert_eq!(computed, expected);
}

#[test]
fn get_sensor_status_pass() {
    // sensor ready with valid data
    let mut vbd = create_ahtx0([0x1C, 0x44, 0xF2, 0xF5, 0xF9, 0xD6, 0x34]);

    // pub fn get_sensor_status<Ibd>(ibd: &mut Ibd, da: u8) -> u8
    let computed = sut::get_sensor_status(&mut vbd, 0x38);
    let expected = 0x1C;

    assert_eq!(computed, expected);
}

#[test]
fn get_sensor_status_busy() {
    let mut vbd = create_ahtx0([0x9C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

    // pub fn get_sensor_status<Ibd>(ibd: &mut Ibd, da: u8) -> u8
    let computed = sut::get_sensor_status(&mut vbd, 0x38);
    let expected = 0x9C;

    assert_eq!(computed, expected);
}

#[test]
fn get_sensor_data_without_crc_pass() {
    let mut vbd = create_ahtx0([0x1C, 0x44, 0xF2, 0xF5, 0xF9, 0xD6, 0x00]);

    // pub fn get_sensor_data<Ibd, const N: usize>(ibd: &mut Ibd, da: u8) -> [u8; N]
    let computed = sut::get_sensor_data::<VirtualAHTx0, 6>(&mut vbd, 0x38);
    let expected = [0x1C, 0x44, 0xF2, 0xF5, 0xF9, 0xD6];

    assert_eq!(computed, expected);
}

#[test]
fn get_sensor_data_with_crc_pass() {
    let mut vbd = create_ahtx0([0x1C, 0x44, 0xF2, 0xF5, 0xF9, 0xD6, 0x34]);

    // pub fn get_sensor_data<Ibd, const N: usize>(ibd: &mut Ibd, da: u8) -> [u8; N]
    let computed = sut::get_sensor_data::<VirtualAHTx0, 7>(&mut vbd, 0x38);
    let expected = [0x1C, 0x44, 0xF2, 0xF5, 0xF9, 0xD6, 0x34];

    assert_eq!(computed, expected);
}

#[test]
fn request_measurement_pass() {
    let mut vbd = create_ahtx0([0x00; 7]);

    // pub fn request_measurement<Ibd>(ibd: &mut Ibd, da: u8) -> bool
    let computed = sut::request_measurement(&mut vbd, 0x38);
    let expected = true;

    assert_eq!(computed, expected);
}

// ------------------------------------------------------------------------

fn create_ahtx0(data: [u8; 7]) -> VirtualAHTx0 {
    VirtualAHTx0 {
        data,
        lwb: [0xC0, 0xFF, 0xEE], // sentinel value
    }
}
