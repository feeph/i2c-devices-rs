/*
    various functions for configuring the hardware
*/

use crate::emc2101::hw;

#[allow(unused_imports)]
use log::{debug, error, info, warn};

/// reset the device register to their default values
pub fn reset_device_registers<Ibd>(ibd: &mut Ibd)
where
    Ibd: crate::traits::I2cBusDevice,
{
    hw::reset_device_registers(ibd);
}

/// compare currently stored values to default values
/// (use after reset_device_registers())
pub fn validate_device_registers<Ibd>(ibd: &mut Ibd) -> bool
where
    Ibd: crate::traits::I2cBusDevice,
{
    hw::validate_device_registers(ibd)
}

/// a representation of the EMC2101's config register (0x03)
///
/// this is not the entire configuration, there are additional registers
/// which configure different aspects of this chip, e.g. fan configuration
/// register (0x4A)
///
/// for an exhaustive description refer to the data sheet (section 6.5)
#[derive(Debug, PartialEq)]
pub struct ConfigRegister {
    // the comment describes what happens if the value is set to True
    pub mask: bool,        // disable ALERT/TACH when in interrupt mode
    pub standby: bool,     // enable low power standby mode
    pub fan_standby: bool, // disable fan output while in standby
    pub dac: bool,         // enable DAC output on FAN pin
    pub dis_to: bool,      // disable I²C bus timeout
    pub alt_tach: bool,    // configure pin six as tacho input
    pub tcrit_ovrd: bool,  // unlock tcrit limit and allow one-time write
    pub queue: bool,       // alert after 3 critical temperature readings
}

/// <function not documented>
// TODO document get_config_register()
pub fn get_config_register<Ibd>(ibd: &mut Ibd) -> ConfigRegister
where
    Ibd: crate::traits::I2cBusDevice,
{
    let cfg = hw::get_config_register(ibd);

    // implicit return
    ConfigRegister {
        mask: (cfg & 0b1000_0000) != 0,
        standby: (cfg & 0b0100_0000) != 0,
        fan_standby: (cfg & 0b0010_0000) != 0,
        dac: (cfg & 0b0001_0000) != 0,
        dis_to: (cfg & 0b0000_1000) != 0,
        alt_tach: (cfg & 0b0000_0100) != 0,
        tcrit_ovrd: (cfg & 0b0000_0010) != 0,
        queue: (cfg & 0b0000_0001) != 0,
    }
}

/// <function not documented>
// TODO document set_config_register()
pub fn set_config_register<Ibd>(ibd: &mut Ibd, cr: ConfigRegister)
where
    Ibd: crate::traits::I2cBusDevice,
{
    let mut byte = 0u8;
    if cr.mask {
        byte |= 0b1000_0000;
    }
    if cr.standby {
        byte |= 0b0100_0000;
    }
    if cr.fan_standby {
        byte |= 0b0010_0000;
    }
    if cr.dac {
        byte |= 0b0001_0000;
    }
    if cr.dis_to {
        byte |= 0b0000_1000;
    }
    if cr.alt_tach {
        byte |= 0b0000_0100;
    }
    if cr.tcrit_ovrd {
        byte |= 0b0000_0010;
    }
    if cr.queue {
        byte |= 0b0000_0001;
    }

    hw::set_config_register(ibd, byte);
}

#[derive(Debug, PartialEq)]
pub struct AlertMask {
    // internal diode
    pub int_mask: bool,
    // external diode
    pub hi_mask: bool,
    pub lo_mask: bool,
    pub tcrit_mask: bool,
    // fan
    pub tach_mask: bool,
}

/// read the alert mask
///
/// (see data sheet section 6.11 for details)
pub fn get_alert_mask<Ibd>(ibd: &mut Ibd) -> AlertMask
where
    Ibd: crate::traits::I2cBusDevice,
{
    let byte = hw::get_alert_mask(ibd);

    // implicit return
    AlertMask {
        int_mask: byte & 0b0100_0000 != 0,
        hi_mask: byte & 0b0001_0000 != 0,
        lo_mask: byte & 0b0000_1000 != 0,
        tcrit_mask: byte & 0b0000_0010 != 0,
        tach_mask: byte & 0b0000_0001 != 0,
    }
}

/// change the alert mask
///
/// (see data sheet section 6.11 for details)
pub fn set_alert_mask<Ibd>(ibd: &mut Ibd, am: AlertMask)
where
    Ibd: crate::traits::I2cBusDevice,
{
    let mut byte = 0b1010_0100; // always set
    if am.int_mask {
        byte |= 0b0100_0000;
    }
    if am.hi_mask {
        byte |= 0b0001_0000;
    }
    if am.lo_mask {
        byte |= 0b0000_1000;
    }
    if am.tcrit_mask {
        byte |= 0b0000_0010;
    }
    if am.tach_mask {
        byte |= 0b0000_0001;
    }

    hw::set_alert_mask(ibd, byte);
}

#[derive(Debug, PartialEq)]
pub struct SpinUpBehavior {
    pub fast_mode: bool,
    pub strength: SpinUpStrength,
    pub duration: SpinUpDuration,
}

#[derive(Debug, PartialEq)]
pub enum SpinUpStrength {
    Bypass = 0b0000_0000,
    Half = 0b0000_1000,
    ThreeQuarter = 0b0001_0000,
    Full = 0b0001_1000,
}

#[derive(Debug, PartialEq)]
pub enum SpinUpDuration {
    Bypass = 0b0000_0000,
    Ms0050 = 0b0000_0001,
    Ms0100 = 0b0000_0010,
    Ms0200 = 0b0000_0011,
    Ms0400 = 0b0000_0100,
    Ms0800 = 0b0000_0101,
    Ms1600 = 0b0000_0110,
    Ms3200 = 0b0000_0111,
}

/// read the fan spin up behavior
pub fn get_spin_up_behavior<Ibd>(ibd: &mut Ibd) -> SpinUpBehavior
where
    Ibd: crate::traits::I2cBusDevice,
{
    let value = hw::get_spin_up_behavior(ibd);

    let fast_mode = match (value & 0b0010_0000) >> 5 {
        0b1 => true,
        0b0 => false,
        _ => panic!("internal error - validate match condition"),
    };

    let strength = match (value & 0b0001_1000) >> 3 {
        0b11 => SpinUpStrength::Full,
        0b10 => SpinUpStrength::ThreeQuarter,
        0b01 => SpinUpStrength::Half,
        0b00 => SpinUpStrength::Bypass,
        _ => panic!("internal error - validate match condition"),
    };

    let duration = match value & 0b0000_0111 {
        0b111 => SpinUpDuration::Ms3200,
        0b110 => SpinUpDuration::Ms1600,
        0b101 => SpinUpDuration::Ms0800,
        0b100 => SpinUpDuration::Ms0400,
        0b011 => SpinUpDuration::Ms0200,
        0b010 => SpinUpDuration::Ms0100,
        0b001 => SpinUpDuration::Ms0050,
        0b000 => SpinUpDuration::Bypass,
        _ => panic!("internal error - validate match condition"),
    };

    // implicit return
    SpinUpBehavior {
        fast_mode,
        strength,
        duration,
    }
}

/// change the fan spin up behavior
pub fn set_spin_up_behavior<Ibd>(ibd: &mut Ibd, sub: SpinUpBehavior)
where
    Ibd: crate::traits::I2cBusDevice,
{
    let mut value: u8 = 0x00;
    if sub.fast_mode {
        value |= 0b0010_0000;
    }
    value |= sub.strength as u8;
    value |= sub.duration as u8;

    hw::set_spin_up_behavior(ibd, value);
}

/// see data sheet (section 6.16) for details
/// TODO improve this struct and make it self-documenting
#[derive(Debug, PartialEq)]
pub struct FanConfig {
    // bit 7 is unused
    pub force: bool,     // enable the external temperature force register
    pub prog: bool,      // enable lookup table
    pub polarity: bool,  // duty cycle polarity: 0..63 = 0..100% or 0..63 = 100..0%
    pub clk_sel: bool,   // pwm base clock selection
    pub clk_ovr: bool,   // clock selection override
    pub tach_mode: bool, // select value to indicate "rpm low"
}

/// read the fan config register
pub fn get_fan_config<Ibd>(ibd: &mut Ibd) -> FanConfig
where
    Ibd: crate::traits::I2cBusDevice,
{
    let value = hw::get_fan_config(ibd);

    FanConfig {
        force: (value & 0b0100_0000) != 0,
        prog: (value & 0b0010_0000) != 0,
        polarity: (value & 0b0001_0000) != 0,
        clk_sel: (value & 0b0000_1000) != 0,
        clk_ovr: (value & 0b0000_0100) != 0,
        // 0b01, 0b10 and 0b11 all have the same meaning
        tach_mode: (value & 0b0000_0011) != 0,
    }
}

/// change the fan config register
pub fn set_fan_config<Ibd>(ibd: &mut Ibd, fan_config: FanConfig)
where
    Ibd: crate::traits::I2cBusDevice,
{
    let mut value = 0x00;
    if fan_config.force {
        value |= 0b0100_0000;
    }
    if fan_config.prog {
        value |= 0b0010_0000;
    }
    if fan_config.polarity {
        value |= 0b0001_0000;
    }
    if fan_config.clk_sel {
        value |= 0b0000_1000;
    }
    if fan_config.clk_ovr {
        value |= 0b0000_0100;
    }
    if fan_config.tach_mode {
        value |= 0b0000_0011;
    }

    hw::set_fan_config(ibd, value);
}

// ------------------------------------------------------------------------
// PWM related settings
// ------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub struct PwmSettings {
    pub frequency: u8, // range: 0..32
    pub divider: u8,   // range: 0..256
}

/// read the fan's PWM settings (frequency + divider)
pub fn get_pwm_settings<Ibd>(ibd: &mut Ibd) -> PwmSettings
where
    Ibd: crate::traits::I2cBusDevice,
{
    let pwm_f = hw::get_pwm_frequency(ibd);
    let pwm_d = hw::get_pwm_frequency_divider(ibd);

    // implicit return
    PwmSettings {
        frequency: pwm_f,
        divider: pwm_d,
    }
}

/// change the fan's PWM settings (frequency + divider)
/// (the values determine the available steps for setting the fan speed)
pub fn set_pwm_settings<Ibd>(ibd: &mut Ibd, pwm: PwmSettings)
where
    Ibd: crate::traits::I2cBusDevice,
{
    // TODO validate that PWM control is being used (refuse if configured for DAC)
    // TODO PWM settings could be temporarily incompatible
    //      (old divider incompatible with new frequency)
    //      may need to disable CLK_OVR, update PWM and reenable CLK_OVR?
    hw::set_pwm_frequency(ibd, pwm.frequency);
    hw::set_pwm_frequency_divider(ibd, pwm.divider);
}
