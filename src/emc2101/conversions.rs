//
// helper functions to convert values to their internal representation
//

#[allow(unused_imports)]
use log::{debug, error, info, warn};

// ------------------------------------------------------------------------
// rpm <-> [msb, lsb]
// ------------------------------------------------------------------------
// convert between an RPM value and tach reading (msb, lsb)

// defined in data sheet (section 6.14)
static TACH_DIV: u32 = 5_400_000;

// calculate minimum possible RPM value
// - going below this value would result in an 'out of range' error
// - value will be calculated at compile time (avoid magic number)
static RPM_MIN: u32 = (TACH_DIV / u16::MAX as u32) + 1;

/// convert RPM value into tach reading
pub fn convert_rpm2tach(rpm: u32) -> u16 {
    let rpm_clamped = rpm.clamp(RPM_MIN, u32::MAX);
    info!("rpm:  {rpm} (clamped: {rpm_clamped})");

    let tach = (TACH_DIV / rpm_clamped) as u16;
    info!("tach: {tach}");

    // implicit return
    tach // 0..65_535
}

/// convert tach reading to RPM value
///
/// (the tach reading must not be zero)
pub fn convert_tach2rpm(tach: u16) -> u32 {
    let tach_clamped = tach.clamp(1, u16::MAX);
    info!("tach: {tach} (clamped: {tach_clamped})");

    let rpm = TACH_DIV / tach_clamped as u32;
    info!("rpm:  {rpm}");

    // implicit return
    rpm // 83..5_400_000
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
