/*
    low-level integration tests for EMC2101
    (using 'i2c_devices::emc2101::hw')
*/

mod common;

use common::VirtualHt16K33;

use i2c_devices::ht16k33::hw as sut;

// ------------------------------------------------------------------------

#[test]
fn set_display_data_pass() {
    let mut vbd = create_ht16k33();

    sut::set_display_data(&mut vbd, 0x70, &[0u8; 16]);

    assert_eq!(vbd.dda, [0u8; 16]);
}

#[test]
fn set_blink_rate_pass() {
    let mut vbd = create_ht16k33();

    let computed = sut::set_blink_rate(&mut vbd, 0x70, 3);
    let expected = true;

    assert_eq!(computed, expected);
    assert_eq!(vbd.dis, 3);
}

#[test]
fn set_blink_rate_fail() {
    let mut vbd = create_ht16k33();

    let computed = sut::set_blink_rate(&mut vbd, 0x70, 99);
    let expected = false;

    assert_eq!(computed, expected);
    assert_eq!(vbd.dis, 0);
}

#[test]
fn set_brightness_level_pass() {
    let mut vbd = create_ht16k33();

    let computed = sut::set_brightness_level(&mut vbd, 0x70, 2);
    let expected = true;

    assert_eq!(computed, expected);
    assert_eq!(vbd.dim, 2);
}

#[test]
fn set_brightness_level_fail() {
    let mut vbd = create_ht16k33();

    let computed = sut::set_brightness_level(&mut vbd, 0x70, 99);
    let expected = false;

    assert_eq!(computed, expected);
    assert_eq!(vbd.dim, 0);
}

#[test]
fn set_output_select_pass() {
    let mut vbd = create_ht16k33();

    let computed = sut::set_output_select(&mut vbd, 0x70, 1);
    let expected = true;

    assert_eq!(computed, expected);
    assert_eq!(vbd.ris, 1);
}

#[test]
fn set_output_select_fail() {
    let mut vbd = create_ht16k33();

    let computed = sut::set_output_select(&mut vbd, 0x70, 99);
    let expected = false;

    assert_eq!(computed, expected);
    assert_eq!(vbd.ris, 0);
}

#[test]
fn set_oscillator_mode_pass() {
    let mut vbd = create_ht16k33();

    let computed = sut::set_oscillator_mode(&mut vbd, 0x70, 1);
    let expected = true;

    assert_eq!(computed, expected);
    assert_eq!(vbd.osc, 1);
}

#[test]
fn set_oscillator_mode_fail() {
    let mut vbd = create_ht16k33();

    let computed = sut::set_oscillator_mode(&mut vbd, 0x70, 99);
    let expected = false;

    assert_eq!(computed, expected);
    assert_eq!(vbd.osc, 0);
}

// ------------------------------------------------------------------------

fn create_ht16k33() -> VirtualHt16K33 {
    VirtualHt16K33 {
        dda: [0u8; 16],
        osc: 0x00,
        dis: 0x00,
        ris: 0x00,
        dim: 0x00,
    }
}
