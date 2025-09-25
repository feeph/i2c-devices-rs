/*
    low-level integration tests for EMC2101
    (using 'i2c_devices::emc2101::hw')
*/

mod common;

use common::VirtualI2cBusDevice;
use rand::prelude::*;

use i2c_devices::emc2101::hw as sut;

// ------------------------------------------------------------------------

#[test]
fn get_config_register() {
    let mut vbd = create_emc2101();

    let computed = sut::get_config_register(&mut vbd);
    let expected = 0b0000_0000;

    assert_eq!(computed, expected);
}

#[test]
fn set_config_register() {
    let mut vbd = create_emc2101();
    let val = create_random_value::<u8>();

    sut::set_config_register(&mut vbd, val);

    let computed = sut::get_config_register(&mut vbd);
    let expected = val;

    assert_eq!(computed, expected);
}

#[test]
fn get_fan_config() {
    let mut vbd = create_emc2101();

    let computed = sut::get_fan_config(&mut vbd);
    let expected = 0x20;

    assert_eq!(computed, expected);
}

#[test]
fn set_fan_config() {
    let mut vbd = create_emc2101();
    let val = create_random_value::<u8>();

    // value is automatically clamped to range 0 ≤ x ≤ 31
    sut::set_fan_config(&mut vbd, val);

    let computed = sut::get_fan_config(&mut vbd);
    let expected = val.clamp(0, 31);

    assert_eq!(computed, expected);
}

#[test]
fn get_fan_speed() {
    let mut vbd = create_emc2101();

    let computed = sut::get_fan_speed(&mut vbd);
    let expected = 0x00;

    assert_eq!(computed, expected);
}

#[test]
fn set_fan_speed() {
    let mut vbd = create_emc2101();
    let val = create_random_value::<u8>();

    // value is automatically clamped to range 0 ≤ x ≤ 31
    sut::set_fan_speed(&mut vbd, val);

    let computed = sut::get_fan_speed(&mut vbd);
    let expected = val.clamp(0, 31);

    assert_eq!(computed, expected);
}

#[test]
fn get_hw_pid_emc2101() {
    let mut vbd = create_emc2101();

    let computed = sut::get_product_id(&mut vbd);
    let expected = 0x16u8;

    assert_eq!(computed, expected);
}

#[test]
fn get_hw_pid_emc2101r() {
    let mut vbd = create_emc2101r();

    let computed = sut::get_product_id(&mut vbd);
    let expected = 0x28u8;

    assert_eq!(computed, expected);
}

#[test]
fn get_hw_mid_smsc() {
    let mut vbd = create_emc2101();

    let computed = sut::get_manufacturer_id(&mut vbd);
    let expected = 0x5Du8;

    assert_eq!(computed, expected);
}

#[test]
fn get_hw_rev() {
    let mut vbd = create_emc2101();

    let computed = sut::get_product_revision(&mut vbd);
    let expected = 0x01u8;

    assert_eq!(computed, expected);
}

#[test]
fn get_pwm_frequency() {
    let mut vbd = create_emc2101();

    let computed = sut::get_pwm_frequency(&mut vbd);
    let expected = 0x17;

    assert_eq!(computed, expected);
}

#[test]
fn set_pwm_frequency() {
    let mut vbd = create_emc2101();
    let val = create_random_value::<u8>();

    // value is automatically clamped to range 0 ≤ x ≤ 31
    sut::set_pwm_frequency(&mut vbd, val);

    let computed = sut::get_pwm_frequency(&mut vbd);
    let expected = val.clamp(0, 31);

    assert_eq!(computed, expected);
}

#[test]
fn get_pwm_frequency_divider() {
    let mut vbd = create_emc2101();

    let computed = sut::get_pwm_frequency_divider(&mut vbd);
    let expected = 0x01;

    assert_eq!(computed, expected);
}

#[test]
fn set_pwm_frequency_divider() {
    let mut vbd = create_emc2101();
    let val = create_random_value::<u8>();

    // value is automatically clamped to range 0 ≤ x ≤ 31
    sut::set_pwm_frequency_divider(&mut vbd, val);

    let computed = sut::get_pwm_frequency_divider(&mut vbd);
    let expected = val;

    assert_eq!(computed, expected);
}
#[test]
fn get_status_register() {
    let mut vbd = create_emc2101();

    let computed = sut::get_status_register(&mut vbd);
    let expected = 0x00u8;

    assert_eq!(computed, expected);
}

#[test]
fn get_scratch_register1() {
    let mut vbd = create_emc2101();

    let computed = sut::get_scratch_register1(&mut vbd);
    let expected = 0x00u8;

    assert_eq!(computed, expected);
}

#[test]
fn set_scratch_register1() {
    let mut vbd = create_emc2101();
    let val = create_random_value::<u8>();

    sut::set_scratch_register1(&mut vbd, val);

    let computed = sut::get_scratch_register1(&mut vbd);
    let expected = val;

    assert_eq!(computed, expected);
}

#[test]
fn get_scratch_register2() {
    let mut vbd = create_emc2101();

    let computed = sut::get_scratch_register2(&mut vbd);
    let expected = 0x00u8;

    assert_eq!(computed, expected);
}

#[test]
fn set_scratch_register2() {
    let mut vbd = create_emc2101();
    let val = create_random_value::<u8>();

    sut::set_scratch_register2(&mut vbd, val);

    let computed = sut::get_scratch_register2(&mut vbd);
    let expected = val;

    assert_eq!(computed, expected);
}

#[test]
fn get_spin_up_behavior() {
    let mut vbd = create_emc2101();

    let computed = sut::get_spin_up_behavior(&mut vbd);
    let expected = 0x3F;

    assert_eq!(computed, expected);
}

#[test]
fn set_spin_up_behavior() {
    let mut vbd = create_emc2101();
    let val = create_random_value::<u8>();

    sut::set_spin_up_behavior(&mut vbd, val);

    let computed = sut::get_spin_up_behavior(&mut vbd);
    let expected = val.clamp(0, 31);

    assert_eq!(computed, expected);
}

#[test]
fn get_tach_limit() {
    let mut vbd = create_emc2101();

    let computed = sut::get_tach_limit(&mut vbd);
    let expected = 0xFFFF;

    assert_eq!(computed, expected);
}

#[test]
fn set_tach_limit() {
    let mut vbd = create_emc2101();
    let val = create_random_value::<u16>();

    sut::set_tach_limit(&mut vbd, val);

    let computed = sut::get_tach_limit(&mut vbd);
    let expected = val;

    assert_eq!(computed, expected);
}

#[test]
fn get_tach_reading() {
    let mut vbd = create_emc2101();

    let computed = sut::get_tach_reading(&mut vbd);
    let expected = 0xFFFF;

    assert_eq!(computed, expected);
}

// ------------------------------------------------------------------------

fn create_emc2101() -> VirtualI2cBusDevice {
    let mut registers = [(0u8, false); 256];

    // set read-only registers
    registers[0x00] = (0x00, false);
    registers[0x01] = (0x00, false);
    registers[0x02] = (0x00, false);
    registers[0x10] = (0x00, false);
    registers[0x46] = (0xFF, false);
    registers[0x47] = (0xFF, false);
    registers[0xFD] = (0x16, false);
    registers[0xFE] = (0x5D, false);
    registers[0xFF] = (0x01, false);

    // set read-write registers
    for tuple in sut::defaults::DEFAULTS {
        let dr = tuple[0];
        let dv = tuple[1];

        registers[dr as usize] = (dv, true);
    }

    VirtualI2cBusDevice { registers }
}

fn create_emc2101r() -> VirtualI2cBusDevice {
    let mut registers = [(0u8, false); 256];

    // set read-only registers
    registers[0xFD] = (0x28, false);
    registers[0xFE] = (0x5D, false);
    registers[0xFF] = (0x01, false);

    // set read-write registers
    for tuple in sut::defaults::DEFAULTS {
        let dr = tuple[0];
        let dv = tuple[1];

        registers[dr as usize] = (dv, true);
    }

    VirtualI2cBusDevice { registers }
}

fn create_random_value<T>() -> T
where
    rand::distr::StandardUniform: rand::distr::Distribution<T>,
{
    let mut rng = rand::rng();

    rng.random::<T>()
}
