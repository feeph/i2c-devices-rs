/*
    high-level integration tests for EMC2101
    (using 'i2c_devices::emc2101')
*/

mod common;

use common::VirtualI2cBusDevice;
use rand::prelude::*;

use i2c_devices::emc2101 as sut;

// ------------------------------------------------------------------------

#[test]
fn get_alert_mask() {
    let mut vbd = create_emc2101();

    let computed = sut::get_alert_mask(&mut vbd);
    let expected = sut::AlertMask {
        int_mask: false,
        hi_mask: false,
        lo_mask: false,
        tcrit_mask: false,
        tach_mask: false,
    };

    assert_eq!(computed, expected);
}

#[test]
fn set_alert_mask() {
    let mut vbd = create_emc2101();
    let val = sut::AlertMask {
        int_mask: false,
        hi_mask: true,
        lo_mask: false,
        tcrit_mask: false,
        tach_mask: false,
    };

    sut::set_alert_mask(&mut vbd, val);

    let computed = sut::get_alert_mask(&mut vbd);
    let expected = sut::AlertMask {
        int_mask: false,
        hi_mask: true,
        lo_mask: false,
        tcrit_mask: false,
        tach_mask: false,
    };

    assert_eq!(computed, expected);
}

#[test]
fn get_config_register() {
    let mut vbd = create_emc2101();

    let computed = sut::get_config_register(&mut vbd);
    let expected = sut::ConfigRegister {
        mask: false,
        standby: false,
        fan_standby: false,
        dac: false,
        dis_to: false,
        alt_tach: false,
        tcrit_ovrd: false,
        queue: false,
    };

    assert_eq!(computed, expected);
}

#[test]
fn set_config_register() {
    let mut vbd = create_emc2101();
    let cr = sut::ConfigRegister {
        mask: false,
        standby: true,
        fan_standby: false,
        dac: false,
        dis_to: false,
        alt_tach: false,
        tcrit_ovrd: false,
        queue: false,
    };

    sut::set_config_register(&mut vbd, cr);

    let computed = sut::get_config_register(&mut vbd);
    // let expected = val;

    // assert_eq!(computed, expected);
    assert_eq!(computed.mask, false);
    assert_eq!(computed.standby, true);
    assert_eq!(computed.fan_standby, false);
    assert_eq!(computed.dac, false);
    assert_eq!(computed.dis_to, false);
    assert_eq!(computed.alt_tach, false);
    assert_eq!(computed.tcrit_ovrd, false);
    assert_eq!(computed.queue, false);
}

#[test]
fn get_conversion_rate() {
    let mut vbd = create_emc2101();

    let computed = sut::get_conversion_rate(&mut vbd);
    let expected = sut::ConversionRate::Sps16;

    assert_eq!(computed, expected);
}

#[test]
fn set_conversion_rate() {
    let mut vbd = create_emc2101();
    let val = sut::ConversionRate::Sps8;

    sut::set_conversion_rate(&mut vbd, val);

    let computed = sut::get_conversion_rate(&mut vbd);
    let expected = sut::ConversionRate::Sps8;
    assert_eq!(computed, expected);
}

#[test]
fn get_ets_averaging_filter() {
    let mut vbd = create_emc2101();

    let computed = sut::get_ets_averaging_filter(&mut vbd);
    let expected = sut::AveragingFilter {
        filter_mode: sut::AveragingFilterMode::Disabled,
        pin_mode: sut::AlertPinMode::Interrupt,
    };

    assert_eq!(computed, expected);
}

#[test]
fn set_ets_averaging_filter() {
    let mut vbd = create_emc2101();
    let val = sut::AveragingFilter {
        filter_mode: sut::AveragingFilterMode::Disabled,
        pin_mode: sut::AlertPinMode::Interrupt,
    };

    sut::set_ets_averaging_filter(&mut vbd, val);

    let computed = sut::get_ets_averaging_filter(&mut vbd);
    let expected = sut::AveragingFilter {
        filter_mode: sut::AveragingFilterMode::Disabled,
        pin_mode: sut::AlertPinMode::Interrupt,
    };

    assert_eq!(computed, expected);
}

#[test]
fn get_ets_bcf() {
    let mut vbd = create_emc2101();

    let computed = sut::get_ets_bcf(&mut vbd);
    let expected = sut::BetaCompensation {
        mode: sut::BetaCompensationMode::Automatic,
        factor: 0,
    };

    assert_eq!(computed, expected);
}

#[test]
fn set_ets_bcf() {
    let mut vbd = create_emc2101();
    let val = sut::BetaCompensation {
        mode: sut::BetaCompensationMode::Manual,
        factor: 0b0000_0101,
    };

    sut::set_ets_bcf(&mut vbd, val);

    let computed = sut::get_ets_bcf(&mut vbd);
    let expected = sut::BetaCompensation {
        mode: sut::BetaCompensationMode::Manual,
        factor: 0b0000_0101,
    };

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
    let val = 50;

    sut::set_ets_dif(&mut vbd, val);

    let computed = sut::get_ets_dif(&mut vbd);
    let expected = 50;

    assert_eq!(computed, expected);
}

#[test]
fn get_ets_critical_limit() {
    let mut vbd = create_emc2101();

    let computed = sut::get_ets_critical_limit(&mut vbd);
    let expected = (85, 10);

    assert_eq!(computed, expected);
}

#[test]
fn set_ets_critical_limit() {
    let mut vbd = create_emc2101();

    sut::set_ets_critical_limit(&mut vbd, (11, 22));

    let computed = sut::get_ets_critical_limit(&mut vbd);
    let expected = (11, 22);

    assert_eq!(computed, expected);
}

#[test]
fn get_external_temperature() {
    let mut vbd = create_emc2101();

    let computed = sut::get_external_temperature(&mut vbd);
    let expected = (0.0f32, sut::ExternalDiodeStatus::Operational);

    assert_eq!(computed, expected);
}

#[test]
fn get_external_temperature_low_limit() {
    let mut vbd = create_emc2101();

    let computed = sut::get_external_temperature_low_limit(&mut vbd);
    let expected = 0.0f32;

    assert_eq!(computed, expected);
}

#[test]
fn set_external_temperature_low_limit() {
    let mut vbd = create_emc2101();

    sut::set_external_temperature_low_limit(&mut vbd, 10.0f32);

    let computed = sut::get_external_temperature_low_limit(&mut vbd);
    let expected = 10.0f32;

    assert_eq!(computed, expected);
}

#[test]
fn get_external_temperature_high_limit() {
    let mut vbd = create_emc2101();

    let computed = sut::get_external_temperature_high_limit(&mut vbd);
    let expected = 70.0f32;

    assert_eq!(computed, expected);
}

#[test]
fn set_external_temperature_high_limit() {
    let mut vbd = create_emc2101();

    sut::set_external_temperature_high_limit(&mut vbd, 75.50f32);

    let computed = sut::get_external_temperature_high_limit(&mut vbd);
    let expected = 75.50f32;

    assert_eq!(computed, expected);
}

#[test]
fn set_external_temperature_override_pass() {
    let mut vbd = create_emc2101();

    let computed = sut::set_external_temperature_override(&mut vbd, 75.5);
    let expected = true;

    assert_eq!(computed, expected);
}

#[test]
fn set_external_temperature_override_fail() {
    let mut vbd = create_emc2101();

    let computed = sut::set_external_temperature_override(&mut vbd, 125.0);
    let expected = false;

    assert_eq!(computed, expected);
}

#[test]
fn get_fan_config() {
    let mut vbd = create_emc2101();

    let computed = sut::get_fan_config(&mut vbd);
    let expected = sut::FanConfig {
        force: false,
        prog: true,
        polarity: false,
        clk_sel: false,
        clk_ovr: false,
        tach_mode: false,
    };

    assert_eq!(computed, expected);
}

#[test]
fn set_fan_config() {
    let mut vbd = create_emc2101();
    let fc = sut::FanConfig {
        force: false,
        prog: false,
        polarity: false,
        clk_sel: false,
        clk_ovr: false,
        tach_mode: false,
    };

    sut::set_fan_config(&mut vbd, fc);

    let computed = sut::get_fan_config(&mut vbd);
    let expected = sut::FanConfig {
        force: false,
        prog: false,
        polarity: false,
        clk_sel: false,
        clk_ovr: false,
        tach_mode: false,
    };

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
fn get_hardware_details_emc2101() {
    let mut vbd = create_emc2101();

    let computed = sut::get_hardware_details(&mut vbd);
    let expected = sut::HardwareDetails {
        mid: 0x5D,
        manufacturer: "SMSC",
        pid: 0x16,
        product: "EMC2101",
        revision: 1,
    };

    assert_eq!(computed, expected);
}

#[test]
fn get_hardware_details_emc2101r() {
    let mut vbd = create_emc2101r();

    let computed = sut::get_hardware_details(&mut vbd);
    let expected = sut::HardwareDetails {
        mid: 0x5D,
        manufacturer: "SMSC",
        pid: 0x28,
        product: "EMC2101-R",
        revision: 1,
    };

    assert_eq!(computed, expected);
}

#[test]
fn get_hardware_details_unknown() {
    let mut vbd = create_emc2101();
    vbd.registers[0xFD] = (0x17, false);
    vbd.registers[0xFE] = (0x5E, false);

    let computed = sut::get_hardware_details(&mut vbd);
    let expected = sut::HardwareDetails {
        mid: 0x5E,
        manufacturer: "<unknown>",
        pid: 0x17,
        product: "<unknown>",
        revision: 1,
    };

    assert_eq!(computed, expected);
}

#[test]
fn get_internal_temperature() {
    let mut vbd = create_emc2101();
    let val = (create_random_value::<u8>() / 3).clamp(0, 85);
    vbd.registers[0x00].0 = val;

    let computed = sut::get_internal_temperature(&mut vbd);
    let expected = val as f32;

    assert_eq!(computed, expected);
}

#[test]
fn get_internal_temperature_high_limit() {
    let mut vbd = create_emc2101();

    let computed = sut::get_internal_temperature_high_limit(&mut vbd);
    let expected = 70.0f32;

    assert_eq!(computed, expected);
}

#[test]
fn set_internal_temperature_high_limit_pass() {
    let mut vbd = create_emc2101();
    let val = (create_random_value::<u8>() / 3).clamp(0, 85);

    // value is rejected if out of range (0 ≤ x ≤ 85)
    sut::set_internal_temperature_high_limit(&mut vbd, val as f32);

    let computed = sut::get_internal_temperature_high_limit(&mut vbd);
    let expected = val as f32;

    assert_eq!(computed, expected);
}

#[test]
fn set_internal_temperature_high_limit_fail1() {
    let mut vbd = create_emc2101();

    // value is rejected if out of range (0 ≤ x ≤ 85)
    sut::set_internal_temperature_high_limit(&mut vbd, -1.0); // to low

    let computed = sut::get_internal_temperature_high_limit(&mut vbd);
    let expected = 70.0;

    assert_eq!(computed, expected);
}

#[test]
fn set_internal_temperature_high_limit_fail2() {
    let mut vbd = create_emc2101();

    // value is rejected if out of range (0 ≤ x ≤ 85)
    sut::set_internal_temperature_high_limit(&mut vbd, 127.0); // to high

    let computed = sut::get_internal_temperature_high_limit(&mut vbd);
    let expected = 70.0;

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
    let val = 8;

    sut::set_lookup_table_hysteresis(&mut vbd, val);

    let computed = sut::get_lookup_table_hysteresis(&mut vbd);
    let expected = 8;

    assert_eq!(computed, expected);
}

#[test]
fn get_pwm_settings() {
    let mut vbd = create_emc2101();

    let computed = sut::get_pwm_settings(&mut vbd);
    let expected = sut::PwmSettings {
        frequency: 23,
        divider: 1,
    };

    assert_eq!(computed, expected);
}

#[test]
fn set_pwm_settings() {
    let mut vbd = create_emc2101();
    let val = sut::PwmSettings {
        frequency: 2,
        divider: 3,
    };

    sut::set_pwm_settings(&mut vbd, val);

    let computed = sut::get_pwm_settings(&mut vbd);
    let expected = sut::PwmSettings {
        frequency: 2,
        divider: 3,
    };

    assert_eq!(computed, expected);
}

#[test]
fn get_status_register() {
    let mut vbd = create_emc2101();

    let computed = sut::get_status_register(&mut vbd);
    let expected = sut::StatusRegister {
        busy: false,
        temp_int_hi: false,
        eeprom: false,
        temp_ext_hi: false,
        temp_ext_lo: false,
        diode_fault: false,
        temp_crit: false,
        rpm_low: false,
    };

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
    let expected = sut::SpinUpBehavior {
        fast_mode: true,
        strength: sut::SpinUpStrength::Full,
        duration: sut::SpinUpDuration::Ms3200,
    };

    assert_eq!(computed, expected);
}

#[test]
fn set_spin_up_behavior() {
    let mut vbd = create_emc2101();
    let val = sut::SpinUpBehavior {
        fast_mode: false,
        strength: sut::SpinUpStrength::Full,
        duration: sut::SpinUpDuration::Ms0100,
    };

    sut::set_spin_up_behavior(&mut vbd, val);

    let computed = sut::get_spin_up_behavior(&mut vbd);
    let expected = sut::SpinUpBehavior {
        fast_mode: false,
        strength: sut::SpinUpStrength::Full,
        duration: sut::SpinUpDuration::Ms0100,
    };

    assert_eq!(computed, expected);
}

#[test]
fn get_minimum_rpm() {
    let mut vbd = create_emc2101();

    // minimum possible value: 82 RPM
    let computed = sut::get_minimum_rpm(&mut vbd);
    let expected = 82;

    assert_eq!(computed, expected);
}

#[test]
fn set_minimum_rpm() {
    let mut vbd = create_emc2101();

    sut::set_minimum_rpm(&mut vbd, 1000);

    let computed = sut::get_minimum_rpm(&mut vbd);
    let expected = 1000;

    assert_eq!(computed, expected);
}

#[test]
fn get_rpm() {
    let mut vbd = create_emc2101();

    // minimum possible value: 82 RPM
    let computed = sut::get_rpm(&mut vbd);
    let expected = 82;

    assert_eq!(computed, expected);
}

#[test]
fn reset_device_registers() {
    let mut vbd = create_emc2101();
    let registers_orig = vbd.registers;

    // randomize all writeable registers
    for x in sut::hw::defaults::DEFAULTS {
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
    for tuple in sut::hw::defaults::DEFAULTS {
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
    for tuple in sut::hw::defaults::DEFAULTS {
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
