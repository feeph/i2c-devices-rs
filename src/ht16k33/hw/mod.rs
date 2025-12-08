/*
    Holtek HT16K33 matrix driver
    RAM mapping 16×8 LED controller driver with keyscan

    The maximum number of devices per I²C bus depends on the package type:
    - 20SOP-A: 1 (0x70)
    - 24SOP-A: 4 (0x70..0x73)
    - 28SOP-A: 8 (0x70..0x77)
*/

use core::iter::Iterator;

#[allow(unused_imports)]
use log::{debug, error, info, warn};

// ------------------------------------------------------------------------
// display data address pointer (0x00)
// ------------------------------------------------------------------------

pub fn set_display_data<Ibd>(ibd: &mut Ibd, da: u8, values: &[u8; 16])
where
    Ibd: crate::traits::I2cBusDevice,
{
    let mut buffer: [u8; 17] = [0x00; 17];
    for (idx, value) in values.iter().enumerate() {
        buffer[idx + 1] = *value;
    }

    debug!("Setting data on {0:#04X}.", da);
    ibd.write_bytes(da, &buffer);
}

// ------------------------------------------------------------------------
// system setup register (0x20)
// ------------------------------------------------------------------------
// while the system oscillator is disabled the device is in standby mode
//
// while in standby mode the HT16K33 can not
// - accept input commands
// - write data to any register except the system setup register

// enable/disable the internal system oscillator
// - 0: turn off system oscillator (standby mode)
// - 1: turn on system oscillator (normal operation mode)
pub fn set_oscillator_mode<Ibd>(ibd: &mut Ibd, da: u8, value: u8) -> bool
where
    Ibd: crate::traits::I2cBusDevice,
{
    if value <= 1 {
        let value = 0x20 | value;
        debug!(
            "Setting oscillator mode on {0:#04X} to {1:#04X}.",
            da, value
        );
        ibd.write_byte(da, value);
        true
    } else {
        error!("Oscillator mode must be in range 0 ≤ x ≤ 1");
        false
    }
}

// ------------------------------------------------------------------------
// key data address pointer (0x40)
// ------------------------------------------------------------------------
// - three bits of immediate data, bits K0 to K2, are transferred to the
//   data pointer to define one of six key data RAM addresses
// - it is strongly recommended that the key data RAM of address 0x40~0x45
//   should be read continuously and in one operation, so the key data RAM
//   of address should be started at 0x40 only
// - if the key data register address (An) is 0x40~0x45, after reaching the
//   memory location 0x45, the pointer will reset to 0x40

// <not implemented>

// ------------------------------------------------------------------------
// INT flag address pointer (0x60)
// ------------------------------------------------------------------------
// When any key matrix key is pressed, after the completion of two key
// scan cycles, this int flag bit goes to a high level and remains at a
// high level until all key data has been read.

// <not implemented>

// ------------------------------------------------------------------------
// display setup register (0x80)
// ------------------------------------------------------------------------

/// set the display's blink rate
/// - 1: no blinking
/// - 3: 2Hz
/// - 5: 1Hz
/// - 7: 0.5Hz
///
/// _(the even values 0, 2, 4 and 6 are not used)_
pub fn set_blink_rate<Ibd>(ibd: &mut Ibd, da: u8, value: u8) -> bool
where
    Ibd: crate::traits::I2cBusDevice,
{
    if value <= 15 {
        let value = 0x80 | value;
        debug!("Setting blink rate on {0:#04X} to {1:#04X}.", da, value);
        ibd.write_byte(da, value);
        true
    } else {
        error!("Blink rate must be in range 0 ≤ x ≤ 15");
        false
    }
}

// ------------------------------------------------------------------------
// ROW/INT set register (0xA0)
// ------------------------------------------------------------------------

/// defines INT/ROW output pin select and INT pin output active level status
/// - 0: INT/ROW output pin is set to ROW driver output
/// - 1: INT/ROW output pin is set to INT output (active low)
/// - 3: INT/ROW output pin is set to INT output (active high)
///
/// _(the value 2 is treated as 0)_
pub fn set_output_select<Ibd>(ibd: &mut Ibd, da: u8, value: u8) -> bool
where
    Ibd: crate::traits::I2cBusDevice,
{
    if value <= 3 {
        ibd.write_byte(da, 0xA0 | value);
        true
    } else {
        error!("Output pin select must be in range 0 ≤ x ≤ 3");
        false
    }
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
pub fn set_brightness_level<Ibd>(ibd: &mut Ibd, da: u8, value: u8) -> bool
where
    Ibd: crate::traits::I2cBusDevice,
{
    if value <= 15 {
        let value = 0xE0 | value;
        debug!(
            "Setting brightness level on {0:#04X} to {1:#04X}.",
            da, value
        );
        ibd.write_byte(da, value);
        true
    } else {
        error!("Brightness level must be in range 0 ≤ x ≤ 15");
        false
    }
}
