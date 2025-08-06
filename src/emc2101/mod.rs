//
// interface for SMSC's EMC2101 and EMC2101-R fan controller chips
//

pub mod hw;

mod conversions;

#[allow(unused_imports)]
use log::{debug, error, info, warn};

use conversions::{
    convert_bytes2temperature, convert_rpm2tach, convert_tach2rpm, convert_temperature2bytes,
};

static UNKNOWN: &str = "<unknown>";

// ------------------------------------------------------------------------
// hardware details
// ------------------------------------------------------------------------

pub struct HardwareDetails {
    pub mid: u8,
    pub manufacturer: &'static str,
    pub pid: u8,
    pub product: &'static str,
    pub revision: u8,
}

/// read the hardware details
///
/// usage:
/// ```rust
/// // <initialize an I²C bus object>
///
/// let hwd = i2c_devices::emc2101::get_hardware_details(&mut i2c_bus0);
/// // concise, e.g. "SMSC EMC2101 (rev: 1)"
/// info!("{0} {1} (rev: {2})", hwd.manufacturer, hwd.product, hwd.prv);
/// // detailed
/// info!("Manufacturer: {0} ({1:#04X})", hwd.manufacturer, hwd.mid);
/// info!("Product:      {0} ({1:#04X})", hwd.product, hwd.pid);
/// info!("Revision:     {0:#04X}", hwd.revision);
/// ```
pub fn get_hardware_details<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> HardwareDetails
where
    Dm: esp_hal::DriverMode,
{
    let mid = hw::get_manufacturer_id(i2c_bus);
    let pid = hw::get_product_id(i2c_bus);
    let rev = hw::get_product_revision(i2c_bus);

    let man = identify_manufacturer(mid);
    let prd = identify_product(pid);

    // implicit return
    HardwareDetails {
        mid,
        manufacturer: man,
        pid,
        product: prd,
        revision: rev,
    }
}

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

/// a representation of the EMC2101's status register (0x02)
///
/// for an exhaustive description refer to the data sheet (section 6.4)
pub struct StatusRegister {
    // the comment describes what happens if the value is set to True
    pub busy: bool,        // ADC is converting
    pub temp_int_hi: bool, // internal temperature has met or exceeded the high limit
    pub eeprom: bool,      // EEPROM  could  not  be  found (EMC2101-R)
    pub temp_ext_hi: bool, // external diode temperature has exceeded the high limit
    pub temp_ext_lo: bool, // external diode temperature has fallen below the low limit
    pub diode_fault: bool, // fault has occurred on the External Diode
    pub temp_crit: bool,   // external diode temperature has met or exceeded the TCRIT limit
    pub rpm_low: bool,     // tach count has exceeded the tach limit (RPM too low)
}

pub fn get_status_register<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> StatusRegister
where
    Dm: esp_hal::DriverMode,
{
    let cfg = hw::get_status_register(i2c_bus);

    // implicit return
    StatusRegister {
        busy: (cfg & 0b1000_0000) != 0,
        temp_int_hi: (cfg & 0b0100_0000) != 0,
        eeprom: (cfg & 0b0010_0000) != 0,
        temp_ext_hi: (cfg & 0b0001_0000) != 0,
        temp_ext_lo: (cfg & 0b0000_1000) != 0,
        diode_fault: (cfg & 0b0000_0100) != 0,
        temp_crit: (cfg & 0b0000_0010) != 0,
        rpm_low: (cfg & 0b0000_0001) != 0,
    }
}

// ------------------------------------------------------------------------
// fan speed control
// ------------------------------------------------------------------------

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

/// read the fan's current RPM
///
/// expected range: 83 to 5_400_000
pub fn get_rpm<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> u32
where
    Dm: esp_hal::DriverMode,
{
    let tach = hw::get_tach_reading(i2c_bus);

    convert_tach2rpm(tach)
}

/// read the fan's minimum RPM
/// - if the measured RPM is below this RPM the fan is considered to be not
///   spinning and the TACH bit is set in the status register (sr.rpm_low)
/// - (depending on the config register) the ALERT/TACH pin will be pulled high
///
/// expected range: 83 to 5_400_000
pub fn get_minimum_rpm<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> u32
where
    Dm: esp_hal::DriverMode,
{
    let tach = hw::get_tach_limit(i2c_bus);

    // implicit return
    convert_tach2rpm(tach)
}

/// change the fan's minimum RPM
/// - if the measured RPM is below this RPM the fan is considered to be not
///   spinning and the TACH bit is set in the status register (sr.rpm_low)
/// - (depending on the config register) the ALERT/TACH pin will be pulled high
///
/// expected range: 83 to 5_400_000
pub fn set_minimum_rpm<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>, rpm: u32)
where
    Dm: esp_hal::DriverMode,
{
    let tach = convert_rpm2tach(rpm);
    hw::set_tach_limit(i2c_bus, tach);
}

pub struct PwmSettings {
    pub frequency: u8, // range: 0..32
    pub divider: u8,   // range: 0..256
}

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

// ------------------------------------------------------------------------
// temperature measurements - internal temperature sensor
// ------------------------------------------------------------------------

/// read the temperature measured by the internal sensor (in °C)
/// - the data sheet guarantees a precision of ±2°C
/// - expected range: 0º.00C to 85.00ºC
pub fn get_internal_temperature<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> f32
where
    Dm: esp_hal::DriverMode,
{
    // implicit return
    hw::get_internal_temperature(i2c_bus) as f32
}

/// read the "high temperature" alerting limit
/// - expected range: 0.00ºC to 85.00ºC
/// - default: 70.00°C
pub fn get_internal_temperature_high_limit<Dm>(
    i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>,
) -> f32
where
    Dm: esp_hal::DriverMode,
{
    // implicit return
    hw::get_internal_temperature_high_limit(i2c_bus) as f32
}

/// set the "high temperature" alerting limit
/// - expected range: 0.00ºC to 85.00ºC
/// - decimal points are truncated (not rounded)
pub fn set_internal_temperature_high_limit<Dm>(
    i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>,
    value: f32,
) -> bool
where
    Dm: esp_hal::DriverMode,
{
    if (0.0..=85.0).contains(&value) {
        // implicit return
        hw::set_internal_temperature_high_limit(i2c_bus, value as u8)
    } else {
        warn!("Provided value for internal temperature limit must be in range 0.0°C <= x <= 85°C!");
        // implicit return
        false
    }
}

/// read the temperature measured by the external sensor (in °C)
/// - the data sheet guarantees a precision of ±1°C
/// - expected range: 0.00ºC to 85.00ºC
pub fn get_external_temperature<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> f32
where
    Dm: esp_hal::DriverMode,
{
    // TODO check if in 'standby' or 'continuous conversion' mode and act accordingly
    info!("Trigger a temperature conversion.");
    hw::trigger_one_shot(i2c_bus);

    let bytes = hw::get_external_temperature(i2c_bus);
    debug!("get_external_temperature():");
    debug!("  MSB: {0:#04X}", bytes[1]);
    debug!("  LSB: {0:#010b}", bytes[1]);

    // TODO validate result

    // implicit return
    convert_bytes2temperature(bytes)
}

/// read the "low temperature" alerting limit in °C
///
/// expected range: 0.0 ≤ x ≤ 85.0°C
pub fn get_external_temperature_low_limit<Dm>(
    i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>,
) -> f32
where
    Dm: esp_hal::DriverMode,
{
    let bytes = hw::get_external_temperature_low_limit(i2c_bus);

    // implicit return
    convert_bytes2temperature(bytes)
}

/// change the "low temperature" alerting limit (in °C)
/// - provided value will be clamped to allowed range (0.0 ≤ x ≤ 85.0°C)
/// - the fractional part has limited precision and will be clamped to the
///   nearest available step.
/// - The clamped value is returned to the caller.
///
/// default: 0.00°C
pub fn set_external_temperature_low_limit<Dm>(
    i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>,
    value: f32,
) -> f32
where
    Dm: esp_hal::DriverMode,
{
    let value_clamped = value.clamp(0.0, 85.0);

    let bytes = convert_temperature2bytes(value_clamped);
    hw::set_external_temperature_low_limit(i2c_bus, bytes);

    // implicit return
    convert_bytes2temperature(bytes)
}

/// read the "high temperature" alerting limit (in °C)
///
/// expected range: 0.0 ≤ x ≤ 85.0°C
pub fn get_external_temperature_high_limit<Dm>(
    i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>,
) -> f32
where
    Dm: esp_hal::DriverMode,
{
    let bytes = hw::get_external_temperature_high_limit(i2c_bus);

    // implicit return
    convert_bytes2temperature(bytes)
}

/// change the "high temperature" alerting limit (in °C)
/// - provided value will be clamped to allowed range (0.0 ≤ x ≤ 85.0°C)
/// - the fractional part has limited precision and will be clamped to the
///   nearest available step.
/// - The clamped value is returned to the caller.
///
/// default: 85.00°C
pub fn set_external_temperature_high_limit<Dm>(
    i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>,
    value: f32,
) -> f32
where
    Dm: esp_hal::DriverMode,
{
    let value_clamped = value.clamp(0.0, 85.0);

    let bytes = convert_temperature2bytes(value_clamped);
    hw::set_external_temperature_high_limit(i2c_bus, bytes);

    // implicit return
    convert_bytes2temperature(bytes)
}

// ------------------------------------------------------------------------
// helper functions
// ------------------------------------------------------------------------

fn identify_manufacturer(mid: u8) -> &'static str {
    let smsc: &'static str = "SMSC";

    // implicit return
    match mid {
        0x5D => smsc,
        _ => UNKNOWN,
    }
}

fn identify_product(pid: u8) -> &'static str {
    let emc2101: &'static str = "EMC2101";
    let emc2101r: &'static str = "EMC2101-R";

    // implicit return
    match pid {
        0x16 => emc2101,
        0x28 => emc2101r,
        _ => UNKNOWN,
    }
}
