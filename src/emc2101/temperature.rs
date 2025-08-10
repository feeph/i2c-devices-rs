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

pub enum BetaCompensationMode {
    Automatic,
    Manual,
    Disabled,
}

/// Please note:
/// - The mode must be set to 'Disabled' when using a thermal terminal
///   diode or a diode-connected transistor (e.g. 2N3904 / 2N3906).
/// - In modes 'Automatic' and 'Disabled' the factor's value is ignored.
///
/// See data sheet section 5.5 for details.
pub struct BetaCompensation {
    pub mode: BetaCompensationMode,
    pub factor: u8,
}

/// read the external sensor's beta compensation factor
pub fn get_ets_bcf<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> BetaCompensation
where
    Dm: esp_hal::DriverMode,
{
    let byte = hw::get_ets_bcf(i2c_bus);

    // implicit return
    match byte.clamp(0x00, 0x08) {
        0x08 => BetaCompensation {
            mode: BetaCompensationMode::Automatic,
            factor: 0b0000_0000,
        },
        0x07 => BetaCompensation {
            mode: BetaCompensationMode::Disabled,
            factor: 0b0000_0000,
        },
        _ => BetaCompensation {
            mode: BetaCompensationMode::Manual,
            factor: byte & 0b0000_0111,
        },
    }
}

/// change the external sensor's beta compensation factor
pub fn set_ets_bcf<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>, bcf: BetaCompensation)
where
    Dm: esp_hal::DriverMode,
{
    let byte = match bcf.mode {
        BetaCompensationMode::Automatic => 0x08,
        BetaCompensationMode::Disabled => 0x07,
        BetaCompensationMode::Manual => bcf.factor.clamp(0x00, 0x06),
    };

    hw::set_ets_bcf(i2c_bus, byte);
}

/// read the external sensor's diode ideality factor
/// - the value represents a specific ideality factor
/// - expected range: 0x08 to 0x37
///
/// (see data sheet section 6.12 for details)
pub fn get_ets_dif<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> u8
where
    Dm: esp_hal::DriverMode,
{
    // implicit return
    hw::get_ets_dif(i2c_bus)
}

/// change the external sensor's diode ideality factor
/// - the value represents a specific ideality factor
/// - expected range: 0x08 to 0x37
/// - the provided value is clamped to this range
///
/// (see data sheet section 6.12 for details)
pub fn set_ets_dif<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>, value: u8)
where
    Dm: esp_hal::DriverMode,
{
    let value_clamped = value.clamp(0x08, 0x37);
    hw::set_ets_dif(i2c_bus, value_clamped);

    // wait a little bit to ensure the temperature measurement register was
    // updated using the newly set DIF value
    esp_hal::delay::Delay::new().delay_millis(100u32);
}

/// read the external sensor's critical temperature threshold and hysteresis
/// - expected range: 0°C ≤ x ≤ 85.0°C
/// - default: 85°C threshold, 10°C hysteresis
///
/// (see data sheet section 6.12 for details)
pub fn get_ets_critical_limit<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> (u8, u8)
where
    Dm: esp_hal::DriverMode,
{
    let threshold = hw::get_ets_tcrit_threshold(i2c_bus);
    let hysteresis = hw::get_ets_tcrit_hysteresis(i2c_bus);

    // implicit return
    (threshold, hysteresis)
}

/// change the external sensor's critical temperature threshold and hysteresis
/// - expected range: 0°C ≤ x ≤ 85.0°C
/// - default: 85°C threshold, 10°C hysteresis
///
/// (see data sheet section 6.12 for details)
pub fn set_ets_critical_limit<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>, tcrit: (u8, u8))
where
    Dm: esp_hal::DriverMode,
{
    let threshold_clamped = tcrit.0.clamp(0, 85);
    let hysteresis_clamped = tcrit.1.clamp(0, 85);
    hw::set_ets_tcrit_threshold(i2c_bus, threshold_clamped);
    hw::set_ets_tcrit_hysteresis(i2c_bus, hysteresis_clamped);
}

/// read the temperature measured by the external sensor (in °C)
/// - the data sheet guarantees a precision of ±1°C
///
/// expected range: -64.0°C ≤ x ≤ 127.0°C
pub fn get_external_temperature<Dm>(
    i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>,
) -> (f32, ExternalDiodeStatus)
where
    Dm: esp_hal::DriverMode,
{
    // need to check the configuration register to check if continuous
    // conversion mode is enabled
    let cfg = hw::get_config_register(i2c_bus);
    if cfg & 0b0100_0000 != 0 {
        // The device is in low power (standby) mode and the temperature
        // measurement registers aren't continuously updated.
        debug!("Standby mode. Need to trigger a temperature conversion.");
        hw::trigger_one_shot(i2c_bus);

        // wait a little bit for the measurement to be completed
        //
        // 32 conversions per second (31.25ms per conversion) is the highest
        // possible sample rate in continuous conversion mode
        esp_hal::delay::Delay::new().delay_millis(50u32);
    }

    let bytes = hw::get_external_temperature(i2c_bus);
    debug!("get_external_temperature():");
    debug!("  MSB: {0:#04X}", bytes.0);
    debug!("  LSB: {0:#010b}", bytes.1);

    // TODO validate result

    // implicit return
    convert_bytes2temperature(bytes)
}

/// read the "low temperature" alerting limit in °C
///
/// expected range: -64.0°C ≤ x ≤ 127.0°C
pub fn get_external_temperature_low_limit<Dm>(
    i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>,
) -> f32
where
    Dm: esp_hal::DriverMode,
{
    let bytes = hw::get_external_temperature_low_limit(i2c_bus);

    // implicit return
    convert_bytes2temperature(bytes).0
}

/// change the "low temperature" alerting limit (in °C)
/// - provided value will be clamped to allowed range (0.0°C ≤ x ≤ 85.0°C)
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
    convert_bytes2temperature(bytes).0
}

/// read the "high temperature" alerting limit (in °C)
///
/// expected range: 0.0°C ≤ x ≤ 85.0°C
pub fn get_external_temperature_high_limit<Dm>(
    i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>,
) -> f32
where
    Dm: esp_hal::DriverMode,
{
    let bytes = hw::get_external_temperature_high_limit(i2c_bus);

    // implicit return
    convert_bytes2temperature(bytes).0
}

/// change the "high temperature" alerting limit (in °C)
/// - provided value will be clamped to allowed range (0.0°C ≤ x ≤ 85.0°C)
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
    convert_bytes2temperature(bytes).0
}

pub enum AlertFilterMode {
    Disabled = 0b0000_0000,
    Level1 = 0b0000_0010,
    Level2 = 0b0000_0100,
    Level3 = 0b0000_0110,
}

pub enum AlertPinMode {
    Interrupt = 0b0000_0000,
    Comparator = 0b0000_0001,
}

pub struct AveragingFilter {
    pub filter_mode: AlertFilterMode,
    pub pin_mode: AlertPinMode,
}

/// get the level of digital averaging used for the external diode
/// temperature measurements
pub fn get_ets_averaging_filter<Dm>(
    i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>,
) -> AveragingFilter
where
    Dm: esp_hal::DriverMode,
{
    let bytes = hw::get_ets_averaging_filter(i2c_bus);

    let fm = match bytes & 0b0000_0110 {
        0b0000_0000 => AlertFilterMode::Disabled,
        0b0000_0010 => AlertFilterMode::Level1,
        0b0000_0100 => AlertFilterMode::Level2,
        0b0000_0110 => AlertFilterMode::Level3,
        // no other value can occur
        _ => panic!("Internal error: Check bit mask."),
    };
    let pm = match bytes & 0b0000_0001 {
        0b0000_0000 => AlertPinMode::Interrupt,
        0b0000_0001 => AlertPinMode::Comparator,
        // no other value can occur
        _ => panic!("Internal error: Check bit mask."),
    };

    // implicit return
    AveragingFilter {
        filter_mode: fm,
        pin_mode: pm,
    }
}

/// set the level of digital averaging used for the external diode
/// temperature measurements
pub fn set_ets_averaging_filter<Dm>(
    i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>,
    af: AveragingFilter,
) where
    Dm: esp_hal::DriverMode,
{
    let mut bytes = 0x00;
    bytes += af.filter_mode as u8;
    bytes += af.pin_mode as u8;

    hw::set_ets_averaging_filter(i2c_bus, bytes);
}

// ------------------------------------------------------------------------
// temperature <-> [msb, lsb]
// ------------------------------------------------------------------------
// convert between a temperature value (f32) and its internal
// representation (msb, lsb)
// - expected range: -64.0°C ≤ x ≤ 127.75°C
// - the internal representation has limited granularity,
//   external temperatures are graduated in 0.125°C steps

pub enum ExternalDiodeStatus {
    Operational,
    OpenCircuit,
    ShortCircuit,
}

/// convert the provided temperature from internal value to float
/// e.g.: [0x0D, 0b1110_000] -> 13.875 (13 + 7*0.125)
pub fn convert_bytes2temperature(bytes: (u8, u8)) -> (f32, ExternalDiodeStatus) {
    let msb = bytes.0 as i8;
    let lsb = bytes.1;

    // test for external sensor error (data sheet section 5.5.1)
    //   (0x7F, 0x00) -> open circuit
    //   (0x7F, 0xE0) -> short circuit
    // Warning: (0x7F, 0x00) could be an actual measurement (127.0°C)
    // TODO find out if 127.125°C .. 127.750°C are allowed values
    //      according to the data sheet all measured values exceeding
    //      127.750°C are clamped to 127.750°C which would imply the
    //      answer is yes
    if msb == 0x7F {
        if lsb == 0b0000_0000 {
            return (f32::NAN, ExternalDiodeStatus::OpenCircuit);
        } else if lsb == 0b1110_0000 {
            return (f32::NAN, ExternalDiodeStatus::ShortCircuit);
        }
    }

    let temp = msb as f32 + ((lsb >> 5) as f32 * 0.125);
    debug!("convert_bytes2temperature(): [{msb:#04X}, {lsb:#04X}] => {temp:.3}°C");

    // implicit return
    (temp, ExternalDiodeStatus::Operational)
}

/// convert the provided temperature from float to internal value
/// e.g.: 13.875 (13 + 7*0.125) -> [0x0D, 0b1110_000]
///
/// the internal representation has limited granularity,
/// temperatures are graduated in 0.125°C steps
pub fn convert_temperature2bytes(value: f32) -> (u8, u8) {
    let value_clamped = value.clamp(-64.0, 127.75);

    let msb = value_clamped as i8;
    let lsb = ((value_clamped % 1.0 / 0.125) as u8) << 5;

    // implicit return
    if msb == 0x75 && lsb == 0b0000_0000 {
        // avoid this value (indicating 'diode fault: open circuit')
        (msb as u8, 0b0010_0000)
    } else {
        (msb as u8, lsb)
    }
}
