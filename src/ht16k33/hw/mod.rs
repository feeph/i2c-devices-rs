/*
    Holtek HT16K33 matrix driver
    RAM mapping 16×8 LED controller driver with keyscan

    The maximum number of devices per I²C bus depends on the package type:
    - 20SOP-A: 1 (0x70)
    - 24SOP-A: 4 (0x70..0x73)
    - 28SOP-A: 8 (0x70..0x77)
*/

#[allow(unused_imports)]
use log::{debug, error, info, warn};

// ------------------------------------------------------------------------
// display data address pointer (0x00)
// ------------------------------------------------------------------------

pub fn set_display_data<Dm>(
    i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>,
    da: u8,
    values: [u8; 16],
) where
    Dm: esp_hal::DriverMode,
{
    let mut buffer: [u8; 17] = [0x00; 17];
    for (idx, value) in values.iter().enumerate() {
        buffer[idx + 1] = *value;
    }

    let _ = i2c_bus.write(da, &buffer);
}

// ------------------------------------------------------------------------
// system setup (0x20)
// ------------------------------------------------------------------------

// enable/disable the internal system oscillator
// - 0: turn off system oscillator (standby mode)
// - 1: turn on system oscillator (normal operation mode)
pub fn set_oscillator_mode<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>, da: u8, value: u8)
where
    Dm: esp_hal::DriverMode,
{
    if value > 1 {
        panic!("Oscillator mode must be in range 0 ≤ x ≤ 1");
    }

    let value = 0x20 | value;
    let _ = i2c_bus.write(da, &[value]);
}

// ------------------------------------------------------------------------
// key data address pointer (0x40)
// ------------------------------------------------------------------------

// 0b0100_0###
// -----------
// - three bits of immediate data, bits K0 to K2, are transferred to the
//   data pointer to define one of six key data RAM addresses
// - it is strongly recommended that the key data RAM of address 0x40~0x45
//   should be read continuously and in one operation, so the key data RAM
//   of address should be started at 0x40 only
// - if the key data register address (An) is 0x40~0x45, after reaching the
//   memory location 0x45, the pointer will reset to 0x40

// ------------------------------------------------------------------------
// INT flag address pointer (0x60)
// ------------------------------------------------------------------------

/// read the interrupt flag signal output (unclear how this works)
///
/// When any key matrix key is pressed, after the completion of two key
/// scan cycles, this int flag bit goes to a high level and remains at a
/// high level until all key data has been read.
pub fn get_int_flag<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>, da: u8) -> [u8; 1]
where
    Dm: esp_hal::DriverMode,
{
    let mut rb: [u8; 1] = [0x00];
    let _ = i2c_bus.write_read(da, &[0x60], &mut rb);

    // implicit return
    rb
}

// ------------------------------------------------------------------------
// display setup (0x80)
// ------------------------------------------------------------------------

/// set the display's blink rate
/// - 1: no blinking
/// - 3: 2Hz
/// - 5: 1Hz
/// - 7: 0.5Hz
///
/// _(the even values 0, 2, 4 and 6 are not used)_
pub fn set_blink_rate<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>, da: u8, value: u8)
where
    Dm: esp_hal::DriverMode,
{
    let value = 0x80 | value;
    let _ = i2c_bus.write(da, &[value]);
}

// ------------------------------------------------------------------------
// ROW/INT set (0xA0)
// ------------------------------------------------------------------------

/// defines INT/ROW output pin select and INT pin output active level status
/// - 0: INT/ROW output pin is set to ROW driver output
/// - 1: INT/ROW output pin is set to INT output (active low)
/// - 3: INT/ROW output pin is set to INT output (active high)
///
/// _(the value 2 is treated as 0)_
pub fn set_output_select<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>, da: u8, value: u8)
where
    Dm: esp_hal::DriverMode,
{
    if value > 3 {
        panic!("Output pin select must be in range 0 ≤ x ≤ 3");
    }

    let value = 0xA0 | value;
    let _ = i2c_bus.write(da, &[value]);
}

// ------------------------------------------------------------------------
// test mode (0xD0)
// ------------------------------------------------------------------------

// undocumented (HOLTEK use only)
// default: 0xD9

// ------------------------------------------------------------------------
// dimming set (0xE0)
// ------------------------------------------------------------------------

/// set the display's brightness level
///
/// brightness level is graduated from 0 (6%) to 15 (100%)
pub fn set_brightness_level<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>, da: u8, value: u8)
where
    Dm: esp_hal::DriverMode,
{
    let value = 0xE0 | value;
    let _ = i2c_bus.write(da, &[value]);
}
