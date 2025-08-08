/*
    internal and external temperature sensor
*/

use crate::emc2101::hw;

#[allow(unused_imports)]
use log::{debug, error, info, warn};

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

// ------------------------------------------------------------------------
// temperature measurements - external temperature sensor (diode)
// ------------------------------------------------------------------------

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
// temperature <-> [msb, lsb]
// ------------------------------------------------------------------------
// convert between a temperature value (f32) and its internal
// representation (msb, lsb)
//
// the internal representation has limited granularity,
// representable fractions are: 0.15, 0.25, 0.40, 0.50, 0.65, 0.75, 0.90

/// convert the provided temperature from internal value to float
/// e.g.: 0x0D + 0b1110_000 -> 13.9 (13 + 0.50 + 0.25 + 0.15)
pub fn convert_bytes2temperature(bytes: [u8; 2]) -> f32 {
    let msb = bytes[0];
    let lsb = bytes[1];

    let mut temp = msb as f32;
    if (lsb & 0b1000_0000) != 0 {
        temp += 0.50;
    }
    if (lsb & 0b0100_0000) != 0 {
        temp += 0.25;
    }
    if (lsb & 0b0010_0000) != 0 {
        temp += 0.15;
    }

    // implicit return
    temp
}

/// convert the provided temperature from float to internal value
/// e.g.: 13.95= 0x0D (13) + 0b1110_000 (0.5 + 0.25 + 0.1)
///
/// the internal value has limited granularity,
/// any remaining fraction is lost
pub fn convert_temperature2bytes(value: f32) -> [u8; 2] {
    let msb = value as u8;
    let mut lsb = 0x00;

    let mut fraction = value % 1.0;
    // use slightly lower values to allow rounding, e.g.
    // - "0.45" should round up to 0.5 instead of down to 0.4 (0.25+0.15)
    // - using '> 0.44' instead of '>= 0.45' because the latter
    //   rounded 84.45 down to 84.40 for some reason
    // TODO validate the rounding logic and test for edge cases
    if fraction > 0.44 {
        lsb |= 0b1000_0000;
        fraction -= 0.5;
    }
    if fraction > 0.19 {
        lsb |= 0b0100_0000;
        fraction -= 0.25;
    }
    if fraction > 0.07 {
        lsb |= 0b0010_0000;
    }

    // implicit return
    [msb, lsb]
}
