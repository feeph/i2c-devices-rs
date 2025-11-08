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
fn get_alert_mask() {
    let mut vbd = create_emc2101();

    let computed = sut::get_alert_mask(&mut vbd);
    let expected = 0xA4;

    assert_eq!(computed, expected);
}

#[test]
fn set_alert_mask() {
    let mut vbd = create_emc2101();
    let val = create_random_value::<u8>();

    sut::set_alert_mask(&mut vbd, val);

    let computed = sut::get_alert_mask(&mut vbd);
    let expected = val;

    assert_eq!(computed, expected);
}

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
fn get_conversion_rate() {
    let mut vbd = create_emc2101();

    let computed = sut::get_conversion_rate(&mut vbd);
    let expected = 0x08;

    assert_eq!(computed, expected);
}

#[test]
fn set_conversion_rate() {
    let mut vbd = create_emc2101();
    let val = create_random_value::<u8>();

    sut::set_conversion_rate(&mut vbd, val);

    let computed = sut::get_conversion_rate(&mut vbd);
    let expected = val;

    assert_eq!(computed, expected);
}

#[test]
fn get_ets_averaging_filter() {
    let mut vbd = create_emc2101();

    let computed = sut::get_ets_averaging_filter(&mut vbd);
    let expected = 0x00;

    assert_eq!(computed, expected);
}

#[test]
fn set_ets_averaging_filter() {
    let mut vbd = create_emc2101();
    let val = create_random_value::<u8>();

    sut::set_ets_averaging_filter(&mut vbd, val);

    let computed = sut::get_ets_averaging_filter(&mut vbd);
    let expected = val;

    assert_eq!(computed, expected);
}

#[test]
fn get_ets_bcf() {
    let mut vbd = create_emc2101();

    let computed = sut::get_ets_bcf(&mut vbd);
    let expected = 0x08;

    assert_eq!(computed, expected);
}

#[test]
fn set_ets_bcf() {
    let mut vbd = create_emc2101();
    let val = create_random_value::<u8>();

    sut::set_ets_bcf(&mut vbd, val);

    let computed = sut::get_ets_bcf(&mut vbd);
    let expected = val;

    assert_eq!(computed, expected);
}

#[test]
fn get_ets_dif() {
    let mut vbd = create_emc2101();

    let computed = sut::get_ets_dif(&mut vbd);
    let expected = 0x12;

    assert_eq!(computed, expected);
}

#[test]
fn set_ets_dif() {
    let mut vbd = create_emc2101();
    let val = create_random_value::<u8>();

    sut::set_ets_dif(&mut vbd, val);

    let computed = sut::get_ets_dif(&mut vbd);
    let expected = val;

    assert_eq!(computed, expected);
}

#[test]
fn get_ets_tcrit_hysteresis() {
    let mut vbd = create_emc2101();

    let computed = sut::get_ets_tcrit_hysteresis(&mut vbd);
    let expected = 0x0A;

    assert_eq!(computed, expected);
}

#[test]
fn set_ets_tcrit_hysteresis() {
    let mut vbd = create_emc2101();
    let val = create_random_value::<u8>();

    sut::set_ets_tcrit_hysteresis(&mut vbd, val);

    let computed = sut::get_ets_tcrit_hysteresis(&mut vbd);
    let expected = val;

    assert_eq!(computed, expected);
}

#[test]
fn get_ets_tcrit_threshold() {
    let mut vbd = create_emc2101();

    let computed = sut::get_ets_tcrit_threshold(&mut vbd);
    let expected = 0x55;

    assert_eq!(computed, expected);
}

#[test]
fn set_ets_tcrit_threshold() {
    let mut vbd = create_emc2101();
    let val = create_random_value::<u8>();

    sut::set_ets_tcrit_threshold(&mut vbd, val);

    let computed = sut::get_ets_tcrit_threshold(&mut vbd);
    let expected = val;

    assert_eq!(computed, expected);
}

#[test]
fn get_external_temperature() {
    let mut vbd = create_emc2101();
    let val1 = create_random_value::<u8>();
    let val2 = create_random_value::<u8>();
    vbd.registers[0x01].0 = val1;
    vbd.registers[0x10].0 = val2;

    let computed = sut::get_external_temperature(&mut vbd);
    let expected = (val1, val2);

    assert_eq!(computed, expected);
}

#[test]
fn get_external_temperature_low_limit() {
    let mut vbd = create_emc2101();
    let val1 = create_random_value::<u8>();
    let val2 = create_random_value::<u8>();
    vbd.registers[0x08].0 = val1;
    vbd.registers[0x14].0 = val2;

    let computed = sut::get_external_temperature_low_limit(&mut vbd);
    let expected = (val1, val2);

    assert_eq!(computed, expected);
}

#[test]
fn set_external_temperature_low_limit() {
    let mut vbd = create_emc2101();
    let val1 = create_random_value::<u8>();
    let val2 = create_random_value::<u8>();

    sut::set_external_temperature_low_limit(&mut vbd, (val1, val2));

    let computed = sut::get_external_temperature_low_limit(&mut vbd);
    let expected = (val1, val2);

    assert_eq!(computed, expected);
}

#[test]
fn get_external_temperature_high_limit() {
    let mut vbd = create_emc2101();
    let val1 = create_random_value::<u8>();
    let val2 = create_random_value::<u8>();
    vbd.registers[0x07].0 = val1;
    vbd.registers[0x13].0 = val2;

    let computed = sut::get_external_temperature_high_limit(&mut vbd);
    let expected = (val1, val2);

    assert_eq!(computed, expected);
}

#[test]
fn set_external_temperature_high_limit() {
    let mut vbd = create_emc2101();
    let val1 = create_random_value::<u8>();
    let val2 = create_random_value::<u8>();

    sut::set_external_temperature_high_limit(&mut vbd, (val1, val2));

    let computed = sut::get_external_temperature_high_limit(&mut vbd);
    let expected = (val1, val2);

    assert_eq!(computed, expected);
}

#[test]
fn set_external_temperature_override() {
    let mut vbd = create_emc2101();
    let val = create_random_value::<u8>();

    sut::set_external_temperature_override(&mut vbd, val);

    let computed = vbd.registers[0x0C].0;
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
    let expected = 0x16;

    assert_eq!(computed, expected);
}

#[test]
fn get_hw_pid_emc2101r() {
    let mut vbd = create_emc2101r();

    let computed = sut::get_product_id(&mut vbd);
    let expected = 0x28;

    assert_eq!(computed, expected);
}

#[test]
fn get_hw_mid_smsc() {
    let mut vbd = create_emc2101();

    let computed = sut::get_manufacturer_id(&mut vbd);
    let expected = 0x5D;

    assert_eq!(computed, expected);
}

#[test]
fn get_hw_rev() {
    let mut vbd = create_emc2101();

    let computed = sut::get_product_revision(&mut vbd);
    let expected = 0x01;

    assert_eq!(computed, expected);
}

#[test]
fn get_internal_temperature() {
    let mut vbd = create_emc2101();
    let val = (create_random_value::<u8>() / 3).clamp(0, 85);
    vbd.registers[0x00].0 = val;

    let computed = sut::get_internal_temperature(&mut vbd);
    let expected = val;

    assert_eq!(computed, expected);
}

#[test]
fn get_internal_temperature_high_limit() {
    let mut vbd = create_emc2101();

    let computed = sut::get_internal_temperature_high_limit(&mut vbd);
    let expected = 0x46;

    assert_eq!(computed, expected);
}

#[test]
fn set_internal_temperature_high_limit() {
    let mut vbd = create_emc2101();
    let val = (create_random_value::<u8>() / 3).clamp(0, 85);

    // value is rejected if out of range (0 ≤ x ≤ 85)
    sut::set_internal_temperature_high_limit(&mut vbd, val);

    let computed = sut::get_internal_temperature_high_limit(&mut vbd);
    let expected = val;

    assert_eq!(computed, expected);
}

#[test]
fn get_lookup_table() {
    let mut vbd = create_emc2101();

    let computed = sut::get_lookup_table(&mut vbd);
    let expected = [
        (0x7F, 0x3F), // Lookup Table Setting 1
        (0x7F, 0x3F), // Lookup Table Setting 2
        (0x7F, 0x3F), // Lookup Table Setting 3
        (0x7F, 0x3F), // Lookup Table Setting 4
        (0x7F, 0x3F), // Lookup Table Setting 5
        (0x7F, 0x3F), // Lookup Table Setting 6
        (0x7F, 0x3F), // Lookup Table Setting 7
        (0x7F, 0x3F), // Lookup Table Setting 8
    ];

    assert_eq!(computed, expected);
}

#[test]
fn set_lookup_table() {
    let mut vbd = create_emc2101();
    let values = [
        (0x00, 0x01), // Lookup Table Setting 1
        (0x02, 0x03), // Lookup Table Setting 2
        (0x04, 0x05), // Lookup Table Setting 3
        (0x06, 0x07), // Lookup Table Setting 4
        (0x08, 0x09), // Lookup Table Setting 5
        (0x0A, 0x0B), // Lookup Table Setting 6
        (0x0C, 0x0D), // Lookup Table Setting 7
        (0x0E, 0x0F), // Lookup Table Setting 8
    ];

    sut::set_lookup_table(&mut vbd, values);

    let computed = sut::get_lookup_table(&mut vbd);
    let expected = values;

    assert_eq!(computed, expected);
}

#[test]
fn get_lookup_table_hysteresis() {
    let mut vbd = create_emc2101();

    let computed = sut::get_lookup_table_hysteresis(&mut vbd);
    let expected = 0x04;

    assert_eq!(computed, expected);
}

#[test]
fn set_lookup_table_hysteresis() {
    let mut vbd = create_emc2101();
    let val = create_random_value::<u8>();

    sut::set_lookup_table_hysteresis(&mut vbd, val);

    let computed = sut::get_lookup_table_hysteresis(&mut vbd);
    let expected = val;

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
    let expected = 0x00;

    assert_eq!(computed, expected);
}

#[test]
fn get_scratch_register1() {
    let mut vbd = create_emc2101();

    let computed = sut::get_scratch_register1(&mut vbd);
    let expected = 0x00;

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
    let expected = 0x00;

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

#[test]
fn reset_device_registers() {
    let mut vbd = create_emc2101();
    let registers_orig = vbd.registers;

    // randomize all writeable registers
    for x in sut::defaults::DEFAULTS {
        let dr = x[0];
        vbd.registers[dr as usize].0 = create_random_value::<u8>();
    }
    // reset all writeable registers to their initial state
    sut::reset_device_registers(&mut vbd);

    let computed = vbd.registers;
    let expected = registers_orig;

    assert_eq!(computed, expected);
}

#[test]
fn trigger_one_shot() {
    let mut vbd = create_emc2101();
    vbd.registers[0x0F].0 = 0xAB;

    // the actual value written to the OneShot register is irrelevant but
    // the library writes 0x00 and we can test that implementation detail
    sut::trigger_one_shot(&mut vbd);

    let computed = vbd.registers[0x0F].0;
    let expected = 0x00;

    assert_eq!(computed, expected);
}

#[test]
fn validate_device_registers() {
    let mut vbd = create_emc2101();

    let computed = sut::validate_device_registers(&mut vbd);
    let expected = true;

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
