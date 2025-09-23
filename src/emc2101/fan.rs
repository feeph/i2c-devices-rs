/*
    fan speed measurement and control
*/

use crate::emc2101::hw;
use core::cmp::Ord;

#[allow(unused_imports)]
use log::{debug, error, info, warn};

/// read the fan's current RPM
///
/// expected range: 83 to 5_400_000
pub fn get_rpm<Ibd>(ibd: &mut Ibd) -> u32
where
    Ibd: crate::traits::I2cBusDevice,
{
    let tach = hw::get_tach_reading(ibd);

    convert_tach2rpm(tach)
}

/// read the fan speed register
/// - this value has no effect if a lookup table is used
///
/// expected range: 0..100%
pub fn get_fan_speed<Ibd>(ibd: &mut Ibd) -> u8
where
    Ibd: crate::traits::I2cBusDevice,
{
    // TODO ensure the lookup table is disabled
    // TODO use a percentage as input value
    //      (hide the internal value from the user)
    // TODO report the new value back to the user
    //      (may differ from requested value depending on granularity)

    // implicit return
    hw::get_fan_speed(ibd)
}

/// change the fan speed register
/// - this value has no effect if a lookup table is used
/// - remember that the change won't instantly change the actual RPM!
///
/// expected range: 0..100%
pub fn set_fan_speed<Ibd>(ibd: &mut Ibd, value: u8)
where
    Ibd: crate::traits::I2cBusDevice,
{
    // TODO ensure the lookup table is disabled
    // TODO use a percentage as input value
    //      (hide the internal value from the user)
    // TODO report the new value back to the user
    //      (may differ from requested value depending on granularity)

    let value_clamped = value.clamp(0, 32);
    hw::set_fan_speed(ibd, value_clamped);
}

/// read the fan's minimum RPM
/// - if the measured RPM is below this RPM the fan is considered to be not
///   spinning and the TACH bit is set in the status register (sr.rpm_low)
/// - (depending on the config register) the ALERT/TACH pin will be pulled high
///
/// expected range: 83 to 5_400_000
pub fn get_minimum_rpm<Ibd>(ibd: &mut Ibd) -> u32
where
    Ibd: crate::traits::I2cBusDevice,
{
    let tach = hw::get_tach_limit(ibd);

    // implicit return
    convert_tach2rpm(tach)
}

/// change the fan's minimum RPM
/// - if the measured RPM is below this RPM the fan is considered to be not
///   spinning and the TACH bit is set in the status register (sr.rpm_low)
/// - (depending on the config register) the ALERT/TACH pin will be pulled high
///
/// expected range: 83 to 5_400_000
pub fn set_minimum_rpm<Ibd>(ibd: &mut Ibd, rpm: u32)
where
    Ibd: crate::traits::I2cBusDevice,
{
    let tach = convert_rpm2tach(rpm);
    hw::set_tach_limit(ibd, tach);
}

// ------------------------------------------------------------------------
// helper functions
// ------------------------------------------------------------------------

// defined in data sheet (section 6.14)
static TACH_DIV: u32 = 5_400_000;

// calculate minimum possible RPM value
// - going below this value would result in an 'out of range' error
// - value will be calculated at compile time (avoid magic number)
static RPM_MIN: u32 = (TACH_DIV / u16::MAX as u32) + 1;

/// convert RPM value into tach reading
pub fn convert_rpm2tach(rpm: u32) -> u16 {
    let rpm_clamped = rpm.clamp(RPM_MIN, u32::MAX);
    debug!("rpm:  {rpm} (clamped: {rpm_clamped})");

    let tach = (TACH_DIV / rpm_clamped) as u16;
    debug!("tach: {tach}");

    // implicit return
    tach // 0..65_535
}

/// convert tach reading to RPM value
///
/// (the tach reading must not be zero)
pub fn convert_tach2rpm(tach: u16) -> u32 {
    let tach_clamped = tach.clamp(1, u16::MAX);
    debug!("tach: {tach} (clamped: {tach_clamped})");

    let rpm = TACH_DIV / tach_clamped as u32;
    debug!("rpm:  {rpm}");

    // implicit return
    rpm // 83..5_400_000
}
