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

use core::iter::Iterator;

mod common;
mod converter;
pub mod did;
pub mod hw;

pub use common::{DisplayMode, SegmentedDisplay};
pub use converter::{convert_14, convert_7, convert_to_4_digits};

#[allow(unused_imports)]
use log::{debug, error, info, warn};

// /// compute minimum and maximum displayable value for the given digit count
// /// (digits must be in range 1 ≤ x ≤ 9)
// fn calculate_digit_range(digits: u8) -> (i32, i32) {
//     // "2 digits" = "-9"
//     // "4 digits" = "-999"
//     let min_value = 1 - 10i32.pow((digits - 1).into());
//     // "2 digits" = "99"
//     // "4 digits" = "9999"
//     let max_value = 10i32.pow(digits.into()) - 1;

//     (min_value, max_value)
// }

// ========================================================================
// 7-segment, 4 digit display
// (addressed as 5 digits: digit, digit, colon, digit, digit)
// ========================================================================

pub struct Segment7x4 {
    pub convert: fn(char) -> (u8, u8),
    pub did: u8,
    pub display_mode: DisplayMode,
    pub brightness_level: u8,
}

impl Segment7x4 {
    fn configure_display<Ibd>(&self, ibd: &mut Ibd)
    where
        Ibd: crate::traits::I2cBusDevice,
    {
        let da = did::convert_did_to_address(self.did);
        // order is important
        // make sure the oscillator is enabled or this becomes a no-op
        hw::set_oscillator_mode(ibd, da, 1);
        // configure display
        hw::set_blink_rate(ibd, da, self.display_mode as u8);
        hw::set_brightness_level(ibd, da, self.brightness_level);
    }
}

impl SegmentedDisplay for Segment7x4 {
    /// enter standby mode
    /// - display will be turned off
    /// - configuration is reset
    /// - device must be woken up before doing anything
    ///
    /// This is the most energy-efficient mode. Use DisplayMode
    /// 'Off' to disable the output but keep it running.
    fn disable<Ibd>(&self, ibd: &mut Ibd)
    where
        Ibd: crate::traits::I2cBusDevice,
    {
        let da = did::convert_did_to_address(self.did);
        hw::set_oscillator_mode(ibd, da, 0);
    }

    /// set the display's blink rate
    /// (implicitly enables the display)
    fn set_display_mode<Ibd>(&mut self, ibd: &mut Ibd, display_mode: DisplayMode)
    where
        Ibd: crate::traits::I2cBusDevice,
    {
        // update internal value and apply configuration
        self.display_mode = display_mode;
        self.configure_display(ibd);
    }

    /// set the display's brightness level
    /// (implicitly enables the display)
    /// - brightness level is graduated from 0 (6%) to 15 (100%)
    /// - use the disable() function to turn off the display entirely
    fn set_brightness_level<Ibd>(&mut self, ibd: &mut Ibd, brightness_level: u8) -> bool
    where
        Ibd: crate::traits::I2cBusDevice,
    {
        const MAX: u8 = 15;
        if brightness_level <= MAX {
            // update internal value and apply configuration
            self.brightness_level = brightness_level;
            self.configure_display(ibd);
            true
        } else {
            error!("Brightness level must be in range 0 ≤ x ≤ {MAX}");
            false
        }
    }

    /// display the provided data buffer
    /// (implicitly enables the display)
    /// - this is the most flexible way to address the display but the caller
    ///   needs to know which bit controls which matrix dot / segment
    fn show_buffer<Ibd>(&self, ibd: &mut Ibd, buffer: &[u8; 16])
    where
        Ibd: crate::traits::I2cBusDevice,
    {
        self.configure_display(ibd);

        // send data
        let da = did::convert_did_to_address(self.did);
        hw::set_display_data(ibd, da, buffer);
    }

    fn show_number<Ibd>(&self, ibd: &mut Ibd, number: f32) -> bool
    where
        Ibd: crate::traits::I2cBusDevice,
    {
        let mut buffer: [u8; 16] = convert_to_4_digits(number, self.convert);

        // clear the third digit (colon)
        buffer[9] = buffer[7];
        buffer[8] = buffer[6];
        buffer[7] = buffer[5];
        buffer[6] = buffer[4];
        buffer[5] = 0b0000_0000; // colon
        buffer[4] = 0b0000_0000; // colon

        self.show_buffer(ibd, &buffer);

        true
    }

    fn show_string<Ibd>(&self, ibd: &mut Ibd, string: &str) -> bool
    where
        Ibd: crate::traits::I2cBusDevice,
    {
        let mut buffer = [0b0000_0000; 16];

        for (idx, c) in string.chars().enumerate() {
            let offset1 = 2 * idx;
            let offset2 = 2 * idx + 1;

            let (byte1, byte2) = (self.convert)(c);

            info!("offset byte1: {:02} -> {:#010b}", offset1, byte1);
            info!("offset byte2: {:02} -> {:#010b}", offset2, byte2);
            buffer[offset1] = byte1;
            buffer[offset2] = byte2;
        }

        self.show_buffer(ibd, &buffer);

        true
    }
}

// ========================================================================
// 14-segment, 4 digit display
// (addressed as 4 digits: digit, digit, digit, digit)
// ========================================================================

pub struct Segment14x4 {
    pub convert: fn(char) -> (u8, u8),
    pub did: u8,
    pub display_mode: DisplayMode,
    pub brightness_level: u8,
}

impl Segment14x4 {
    fn configure_display<Ibd>(&self, ibd: &mut Ibd)
    where
        Ibd: crate::traits::I2cBusDevice,
    {
        let da = did::convert_did_to_address(self.did);
        // order is important
        // make sure the oscillator is enabled or this becomes a no-op
        hw::set_oscillator_mode(ibd, da, 1);
        // configure display
        hw::set_blink_rate(ibd, da, self.display_mode as u8);
        hw::set_brightness_level(ibd, da, self.brightness_level);
    }
}

impl SegmentedDisplay for Segment14x4 {
    /// enter standby mode
    /// - display will be turned off
    /// - configuration is reset
    /// - device must be woken up before doing anything
    ///
    /// This is the most energy-efficient mode. Use DisplayMode
    /// 'Off' to disable the output but keep it running.
    fn disable<Ibd>(&self, ibd: &mut Ibd)
    where
        Ibd: crate::traits::I2cBusDevice,
    {
        let da = did::convert_did_to_address(self.did);
        hw::set_oscillator_mode(ibd, da, 0);
    }

    /// set the display's blink rate
    /// (implicitly enables the display)
    fn set_display_mode<Ibd>(&mut self, ibd: &mut Ibd, display_mode: DisplayMode)
    where
        Ibd: crate::traits::I2cBusDevice,
    {
        // update internal value and apply configuration
        self.display_mode = display_mode;
        self.configure_display(ibd);
    }

    /// set the display's brightness level
    /// (implicitly enables the display)
    /// - brightness level is graduated from 0 (6%) to 15 (100%)
    /// - use the disable() function to turn off the display entirely
    fn set_brightness_level<Ibd>(&mut self, ibd: &mut Ibd, brightness_level: u8) -> bool
    where
        Ibd: crate::traits::I2cBusDevice,
    {
        const MAX: u8 = 15;
        if brightness_level <= MAX {
            // update internal value and apply configuration
            self.brightness_level = brightness_level;
            self.configure_display(ibd);
            true
        } else {
            error!("Brightness level must be in range 0 ≤ x ≤ {MAX}");
            false
        }
    }

    /// display the provided data buffer
    /// (implicitly enables the display)
    /// - this is the most flexible way to address the display but the caller
    ///   needs to know which bit controls which matrix dot / segment
    fn show_buffer<Ibd>(&self, ibd: &mut Ibd, buffer: &[u8; 16])
    where
        Ibd: crate::traits::I2cBusDevice,
    {
        self.configure_display(ibd);

        // send data
        let da = did::convert_did_to_address(self.did);
        hw::set_display_data(ibd, da, buffer);
    }

    fn show_number<Ibd>(&self, ibd: &mut Ibd, number: f32) -> bool
    where
        Ibd: crate::traits::I2cBusDevice,
    {
        let buffer: [u8; 16] = convert_to_4_digits(number, self.convert);

        self.show_buffer(ibd, &buffer);

        true
    }

    fn show_string<Ibd>(&self, ibd: &mut Ibd, string: &str) -> bool
    where
        Ibd: crate::traits::I2cBusDevice,
    {
        let mut buffer = [0b0000_0000; 16];

        for (idx, c) in string.chars().enumerate() {
            let offset1 = 2 * idx;
            let offset2 = 2 * idx + 1;

            let (byte1, byte2) = (self.convert)(c);

            info!("offset byte1: {:02} -> {:#010b}", offset1, byte1);
            info!("offset byte2: {:02} -> {:#010b}", offset2, byte2);
            buffer[offset1] = byte1;
            buffer[offset2] = byte2;
        }

        self.show_buffer(ibd, &buffer);

        true
    }
}

// /// convert an integer number into a sequence of digits
// ///
// /// ```TEXT
// /// 1234 -> [ 1,  2,  3,  4]
// /// -123 -> [-2,  1,  2,  3]  (minus => -2)
// ///   12 -> [-1, -1,  1,  2]  (empty => -1)
// /// ```
// fn int2seq(value: i32, digits: u8) -> [i8; 8] {
//     let mut buf: [i8; 8] = [-1; 8];

//     // (there's probably a more elegant way to do this)

//     if value == 0 {
//         // show a zero in the last digit and return
//         buf[(digits - 1) as usize] = 0;
//         return buf;
//     }

//     if value < 0 {
//         // show a negative sign in first digit
//         // TODO cuddle up the minus sign to first indicated digit? (' -12' instead of '- 12')
//         buf[0] = -2;
//     }

//     let mut tmp = value.unsigned_abs();
//     for i in 1..=digits {
//         // deconstruct from left to right (thousands, hundreds, tenths, ones)
//         let exp = digits - i;
//         debug!(" -> exp {exp}");
//         // TODO consider using 'exp.try_into()'
//         let div = 10_u32.pow(exp.into());
//         // skip leading zeros
//         if value.unsigned_abs() >= div {
//             debug!(" -> div {div}");
//             let digit = tmp / div;
//             debug!(" -> digit {i} = {digit}");
//             buf[(i - 1) as usize] = digit as i8;
//             tmp -= digit * div;
//         }
//     }

//     buf
// }
