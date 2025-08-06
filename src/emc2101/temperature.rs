//
// helper functions to convert between a temperature value (f32) and the
// internal representation (msb, lsb)
//
// the internal representation has limited granularity,
// representable fractions are: 0.15, 0.25, 0.40, 0.50, 0.65, 0.75, 0.90

/// convert the provided temperature from internal value to float
/// e.g.: 0x0C + 0xE0 -> 13.9 (13 + 0.50 + 0.25 + 0.15)
///
/// the internal value has limited granularity, any remaining fraction is
/// lost
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
/// the internal value has limited granularity, any remaining fraction is
/// lost
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
