/*
    various functions for configuring the hardware
*/

use crate::emc2101::data_objects::{
    AlertMask, ConfigRegister, SpinUpBehavior, SpinUpDuration, SpinUpStrength,
};
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

/// <function not documented>
// TODO document get_config_register()
pub fn get_config_register<Ibd>(ibd: &mut Ibd) -> ConfigRegister
where
    Ibd: crate::traits::I2cBusDevice,
{
    // implicit return
    ConfigRegister::from(hw::get_config_register(ibd))
}

/// <function not documented>
// TODO document set_config_register()
pub fn set_config_register<Ibd>(ibd: &mut Ibd, cr: ConfigRegister)
where
    Ibd: crate::traits::I2cBusDevice,
{
    hw::set_config_register(ibd, cr.into());
}

/// read the alert mask
///
/// (see data sheet section 6.11 for details)
pub fn get_alert_mask<Ibd>(ibd: &mut Ibd) -> AlertMask
where
    Ibd: crate::traits::I2cBusDevice,
{
    // implicit return
    AlertMask::from(hw::get_alert_mask(ibd))
}

/// change the alert mask
///
/// (see data sheet section 6.11 for details)
pub fn set_alert_mask<Ibd>(ibd: &mut Ibd, am: AlertMask)
where
    Ibd: crate::traits::I2cBusDevice,
{
    hw::set_alert_mask(ibd, am.into());
}

/// read the fan spin up behavior
pub fn get_spin_up_behavior<Ibd>(ibd: &mut Ibd) -> SpinUpBehavior
where
    Ibd: crate::traits::I2cBusDevice,
{
    let value = hw::get_spin_up_behavior(ibd);

    // implicit return
    SpinUpBehavior {
        fast_mode: (value & 0b0010_0000) != 0,
        strength: SpinUpStrength::from(value),
        duration: SpinUpDuration::from(value),
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

// ------------------------------------------------------------------------
// Fan Config
// ------------------------------------------------------------------------

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
