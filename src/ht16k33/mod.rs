/*
    Holtek HT16K33 matrix driver
    RAM mapping 16×8 LED controller driver with keyscan

    The maximum number of devices per I²C bus depends on the package type:
    - 20SOP-A: 1
    - 24SOP-A: 4
    - 28SOP-A: 8

    The device is identified by the Device ID (did) parameter. The allowed
    range is 0 ≤ x ≤ 7 and it represents a binary encoding of A0, A1 & A2.

    |  A0     |  A1     |  A2     | did |
    |:-------:|:-------:|:-------:|:---:|
    | open    | open    | open    |  0  |
    | bridged | open    | open    |  1  |
    | open    | bridged | open    |  2  |
    | bridged | bridged | open    |  3  |
    | open    | open    | bridged |  4  |
    | bridged | open    | bridged |  5  |
    | open    | bridged | bridged |  6  |
    | bridged | bridged | bridged |  7  |
*/

pub mod did;
pub mod hw;

#[allow(unused_imports)]
use log::{debug, error, info, warn};

pub enum BlinkRate {
    Steady = 0b0001, // no blinking
    Fast = 0b0011,   // 2Hz
    Medium = 0b0101, // 1Hz
    Slow = 0b0111,   // 0.5Hz
}

/// set the display's blink rate
pub fn set_blink_rate<Dm>(
    i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>,
    did: u8,
    blink_rate: BlinkRate,
) where
    Dm: esp_hal::DriverMode,
{
    let da = did::convert_did_to_address(did);
    hw::set_blink_rate(i2c_bus, da, blink_rate as u8);
}

/// set the display's brightness level
/// - brightness level is graduated from 0 (6%) to 15 (100%)
/// - use the clear() function to turn off the display entirely
pub fn set_brightness_level<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>, did: u8, value: u8)
where
    Dm: esp_hal::DriverMode,
{
    const MAX: u8 = 15;
    if value > MAX {
        panic!("Brightness level must be in range 0 ≤ x ≤ {MAX}");
    }

    let da = did::convert_did_to_address(did);
    hw::set_brightness_level(i2c_bus, da, value);
}

/// clear the display buffer and change to standby mode
pub fn clear<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>, did: u8)
where
    Dm: esp_hal::DriverMode,
{
    let da = did::convert_did_to_address(did);
    // reset data
    hw::set_display_data(i2c_bus, da, [0x00; 16]);
    // disable display
    hw::set_oscillator_mode(i2c_bus, da, 0);
}

/// display the provided data buffer
/// - this is the most flexible way to address the display but the caller
///   needs to know which bit controls which matrix dot / segment
pub fn show_buffer<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>, did: u8, buffer: [u8; 16])
where
    Dm: esp_hal::DriverMode,
{
    let da = did::convert_did_to_address(did);
    // enable display
    hw::set_oscillator_mode(i2c_bus, da, 1);
    // send data
    hw::set_display_data(i2c_bus, da, buffer);
}

/// compute minimum and maximum displayable value for the given digit count
/// (digits must be in range 1 ≤ x ≤ 9)
fn calculate_digit_range(digits: u8) -> (i32, i32) {
    // "2 digits" = "-9"
    // "4 digits" = "-999"
    let min_value = 1 - 10i32.pow((digits - 1).into());
    // "2 digits" = "99"
    // "4 digits" = "9999"
    let max_value = 10i32.pow(digits.into()) - 1;

    (min_value, max_value)
}

// ------------------------------------------------------------------------
// 14-segment display
// ------------------------------------------------------------------------

/// convert an integer number into a sequence of digits
/// ```
/// 1234 -> [ 1,  2,  3,  4]
/// -123 -> [-2,  1,  2,  3]  (minus => -2)
///   12 -> [-1, -1,  1,  2]  (empty => -1)
/// ```
fn int2seq(value: i32, digits: u8) -> [i8; 8] {
    let mut buf: [i8; 8] = [-1; 8];

    // (there's probably a more elegant way to do this)

    if value == 0 {
        // show a zero in the last digit and return
        buf[(digits - 1) as usize] = 0;
        return buf;
    }

    if value < 0 {
        // show a negative sign in first digit
        // TODO cuddle up the minus sign to first indicated digit? (' -12' instead of '- 12')
        buf[0] = -2;
    }

    let mut tmp = value.unsigned_abs();
    for i in 1..=digits {
        // deconstruct from left to right (thousands, hundreds, tenths, ones)
        let exp = digits - i;
        debug!(" -> exp {exp}");
        let div = 10_u32.pow(exp.into());
        // skip leading zeros
        if value.unsigned_abs() >= div {
            debug!(" -> div {div}");
            let digit = tmp / div;
            debug!(" -> digit {i} = {digit}");
            buf[(i - 1) as usize] = digit as i8;
            tmp -= digit * div;
        }
    }

    buf
}

/// convert digits into a 2-byte bitmask
/// (0 -> segment is off, 1 -> segment is on)
fn convert_to_14segments(value: i32, digits: u8) -> [u8; 16] {
    let mut buf: [u8; 16] = [0b0000_0000; 16];

    const NUMBERS_14: [(u8, u8); 10] = [
        (0b00111111, 0b00001100), // 0
        (0b00000110, 0b00000000), // 1
        (0b11011011, 0b00000000), // 2
        (0b10001111, 0b00000000), // 3
        (0b11100110, 0b00000000), // 4
        (0b01101001, 0b00100000), // 5
        (0b11111101, 0b00000000), // 6
        (0b00000111, 0b00000000), // 7
        (0b11111111, 0b00000000), // 8
        (0b11101111, 0b00000000), // 9
    ];

    for (i, v) in int2seq(value, digits).iter().enumerate() {
        let idx_b0 = i * 2;
        let idx_b1 = i * 2 + 1;
        match v {
            -2 => {
                // negative sign
                (buf[idx_b0], buf[idx_b1]) = (0b1100_0000, 0b0000_0000);
            }
            -1 => {
                // empty
                (buf[idx_b0], buf[idx_b1]) = (0b0000_0000, 0b0000_0000);
            }
            _ => {
                // digit (0 to 9)
                let idx = v.unsigned_abs() as usize;
                (buf[idx_b0], buf[idx_b1]) = NUMBERS_14[idx];
            }
        }
    }

    buf
}

/// display the provided number (assumes a 14-segment display)
/// TODO define as a trait
pub fn show_number<Dm>(
    i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>,
    did: u8,
    value: i32,
    digits: u8,
) -> bool
where
    Dm: esp_hal::DriverMode,
{
    const MIN: u8 = 2;
    const MAX: u8 = 8;
    if !(MIN..=MAX).contains(&digits) {
        warn!("Provided digit count {digits} is out of range! ({MIN} ≤ x ≤ {MAX})");
        return false;
    }
    let (min_value, max_value) = calculate_digit_range(digits);
    if value < min_value || value > max_value {
        warn!("Provided value {value} is out of range! ({min_value} ≤ x ≤ {max_value})");
        return false;
    }

    let buffer = convert_to_14segments(value, digits);
    show_buffer(i2c_bus, did, buffer);

    true
}
