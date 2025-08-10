/*
    various functions for configuring the hardware
*/

use crate::emc2101::hw;

#[allow(unused_imports)]
use log::{debug, error, info, warn};

/// reset the device register to their default values
pub fn reset_device_registers<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>)
where
    Dm: esp_hal::DriverMode,
{
    hw::reset_device_registers(i2c_bus);
}

/// compare currently stored values to default values
/// (use after reset_device_registers())
pub fn validate_device_registers<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> bool
where
    Dm: esp_hal::DriverMode,
{
    hw::validate_device_registers(i2c_bus)
}

/// a representation of the EMC2101's config register (0x03)
///
/// this is not the entire configuration, there are additional registers
/// which configure different aspects of this chip, e.g. fan configuration
/// register (0x4A)
///
/// for an exhaustive description refer to the data sheet (section 6.5)
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
pub fn get_config_register<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> ConfigRegister
where
    Dm: esp_hal::DriverMode,
{
    let cfg = hw::get_config_register(i2c_bus);

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
pub fn set_config_register<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>, cr: ConfigRegister)
where
    Dm: esp_hal::DriverMode,
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

    hw::set_config_register(i2c_bus, byte);
}

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
pub fn get_alert_mask<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> AlertMask
where
    Dm: esp_hal::DriverMode,
{
    let byte = hw::get_alert_mask(i2c_bus);

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
pub fn set_alert_mask<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>, am: AlertMask)
where
    Dm: esp_hal::DriverMode,
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

    hw::set_alert_mask(i2c_bus, byte);
}

pub struct SpinUpBehavior {
    pub fast_mode: bool,
    pub strength: SpinUpStrength,
    pub duration: SpinUpDuration,
}

pub enum SpinUpStrength {
    Bypass = 0b0000_0000,
    Half = 0b0000_1000,
    ThreeQuarter = 0b0001_0000,
    Full = 0b0001_1000,
}

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
pub fn get_spin_up_behavior<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> SpinUpBehavior
where
    Dm: esp_hal::DriverMode,
{
    let value = hw::get_spin_up_behavior(i2c_bus);

    let mut fast_mode = false;
    if (value & 0b0010_0000) != 0 {
        fast_mode = true;
    }

    // slightly smelly code
    let strength: SpinUpStrength;
    if (value & 0b0001_1000) != 0 {
        strength = SpinUpStrength::Full;
    } else if (value & 0b0001_0000) != 0 {
        strength = SpinUpStrength::ThreeQuarter;
    } else if (value & 0b0000_1000) != 0 {
        strength = SpinUpStrength::Half;
    } else {
        strength = SpinUpStrength::Bypass;
    }

    // really smelly code
    let duration: SpinUpDuration;
    if (value & 0b0000_0111) != 0 {
        duration = SpinUpDuration::Ms3200;
    } else if (value & 0b0000_0110) != 0 {
        duration = SpinUpDuration::Ms1600;
    } else if (value & 0b0000_0101) != 0 {
        duration = SpinUpDuration::Ms0800;
    } else if (value & 0b0000_0100) != 0 {
        duration = SpinUpDuration::Ms0400;
    } else if (value & 0b0000_0011) != 0 {
        duration = SpinUpDuration::Ms0200;
    } else if (value & 0b0000_0010) != 0 {
        duration = SpinUpDuration::Ms0100;
    } else if (value & 0b0000_0001) != 0 {
        duration = SpinUpDuration::Ms0050;
    } else {
        duration = SpinUpDuration::Bypass;
    }

    // implicit return
    SpinUpBehavior {
        fast_mode,
        strength,
        duration,
    }
}

/// change the fan spin up behavior
pub fn set_spin_up_behavior<Dm>(
    i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>,
    sub: SpinUpBehavior,
) where
    Dm: esp_hal::DriverMode,
{
    let mut value: u8 = 0x00;
    if sub.fast_mode {
        value |= 0b0010_0000;
    }
    value |= sub.strength as u8;
    value |= sub.duration as u8;

    hw::set_spin_up_behavior(i2c_bus, value);
}

/// see data sheet (section 6.16) for details
/// TODO improve this struct and make it self-documenting
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
pub fn get_fan_config<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> FanConfig
where
    Dm: esp_hal::DriverMode,
{
    let value = hw::get_fan_config(i2c_bus);

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
pub fn set_fan_config<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>, fan_config: FanConfig)
where
    Dm: esp_hal::DriverMode,
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

    hw::set_fan_config(i2c_bus, value);
}

// ------------------------------------------------------------------------
// PWM related settings
// ------------------------------------------------------------------------

pub struct PwmSettings {
    pub frequency: u8, // range: 0..32
    pub divider: u8,   // range: 0..256
}

/// read the fan's PWM settings (frequency + divider)
pub fn get_pwm_settings<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> PwmSettings
where
    Dm: esp_hal::DriverMode,
{
    let pwm_f = hw::get_pwm_frequency(i2c_bus);
    let pwm_d = hw::get_pwm_frequency_divider(i2c_bus);

    // implicit return
    PwmSettings {
        frequency: pwm_f,
        divider: pwm_d,
    }
}

/// change the fan's PWM settings (frequency + divider)
/// (the values determine the available steps for setting the fan speed)
pub fn set_pwm_settings<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>, pwm: PwmSettings)
where
    Dm: esp_hal::DriverMode,
{
    // TODO validate that PWM control is being used (refuse if configured for DAC)
    // TODO PWM settings could be temporarily incompatible
    //      (old divider incompatible with new frequency)
    //      may need to disable CLK_OVR, update PWM and reenable CLK_OVR?
    hw::set_pwm_frequency(i2c_bus, pwm.frequency);
    hw::set_pwm_frequency_divider(i2c_bus, pwm.divider);
}
