/*
    EMC2101's averaging filter (0xBF)
*/

#[allow(unused_imports)]
use log::{debug, error, info, warn};

// ------------------------------------------------------------------------
// Averaging Filter
// ------------------------------------------------------------------------

/// a representation of the EMC2101's averaging filter register (0xBF)
#[derive(Debug, PartialEq)]
pub struct AveragingFilter {
    pub filter_mode: AveragingFilterMode,
    pub pin_mode: AlertPinMode,
}

impl From<u8> for AveragingFilter {
    fn from(value: u8) -> Self {
        AveragingFilter {
            filter_mode: AveragingFilterMode::from(value),
            pin_mode: AlertPinMode::from(value),
        }
    }
}

impl From<AveragingFilter> for u8 {
    fn from(af: AveragingFilter) -> Self {
        let mut bit_mask = 0b0000_0000;
        bit_mask += af.filter_mode as u8;
        bit_mask += af.pin_mode as u8;

        bit_mask
    }
}

// ------------------------------------------------------------------------
// Alert Pin Mode
// ------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub enum AlertPinMode {
    Interrupt = 0b0000_0000,
    Comparator = 0b0000_0001,
}

impl From<u8> for AlertPinMode {
    fn from(value: u8) -> Self {
        if value & 0b0000_0001 != 0 {
            AlertPinMode::Comparator
        } else {
            AlertPinMode::Interrupt
        }
    }
}

#[test]
fn convert_value_to_alert_pin_mode() {
    assert_eq!(AlertPinMode::from(0b0000_0000), AlertPinMode::Interrupt);
    assert_eq!(AlertPinMode::from(0b0000_0001), AlertPinMode::Comparator);
}

// ------------------------------------------------------------------------
// Averaging Filter Mode
// ------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub enum AveragingFilterMode {
    Disabled = 0b0000_0000, // (default)
    Level1 = 0b0000_0010,
    Level2 = 0b0000_0100,
    Level3 = 0b0000_0110,
}

impl From<u8> for AveragingFilterMode {
    fn from(value: u8) -> Self {
        match value & 0b0000_0110 {
            0b0000_0010 => AveragingFilterMode::Level1,
            0b0000_0100 => AveragingFilterMode::Level2,
            0b0000_0110 => AveragingFilterMode::Level3,
            _ => AveragingFilterMode::Disabled,
        }
    }
}

#[test]
fn convert_value_to_averaging_filter() {
    assert_eq!(
        AveragingFilterMode::from(0b0000_0000),
        AveragingFilterMode::Disabled
    );
    assert_eq!(
        AveragingFilterMode::from(0b0000_0010),
        AveragingFilterMode::Level1
    );
    assert_eq!(
        AveragingFilterMode::from(0b0000_0100),
        AveragingFilterMode::Level2
    );
    assert_eq!(
        AveragingFilterMode::from(0b0000_0110),
        AveragingFilterMode::Level3
    );
}

// impl From<u8> for AlertMask {
//     fn from(value: u8) -> Self {
//         AlertMask {
//             int_mask: (value & 0b0100_0000) != 0,
//             hi_mask: (value & 0b0001_0000) != 0,
//             lo_mask: (value & 0b0000_1000) != 0,
//             tcrit_mask: (value & 0b0000_0010) != 0,
//             tach_mask: (value & 0b0000_0001) != 0,
//         }
//     }
// }

// #[test]
// fn convert_value_to_alert_mask() {
//     let mut am = AlertMask {
//             int_mask: false,
//             hi_mask: false,
//             lo_mask: false,
//             tcrit_mask: false,
//             tach_mask: false,
//     };

//     // initial state - no bits set
//     assert_eq!(AlertMask::from(0b0000_0000), am);
//     // toggle each available setting and validate the bit
//     am.tach_mask = true;
//     assert_eq!(AlertMask::from(0b0000_0001), am);
//     am.tcrit_mask = true;
//     assert_eq!(AlertMask::from(0b0000_0011), am);
//     am.lo_mask = true;
//     assert_eq!(AlertMask::from(0b0000_1011), am);
//     am.hi_mask = true;
//     assert_eq!(AlertMask::from(0b0001_1011), am);
//     am.int_mask = true;
//     assert_eq!(AlertMask::from(0b0101_1011), am);
// }

// impl From<AlertMask> for u8 {
//     fn from(am: AlertMask) -> Self {
//         let mut byte = 0b1010_0100; // always set

//         if am.int_mask {
//             byte |= 0b0100_0000;
//         }
//         if am.hi_mask {
//             byte |= 0b0001_0000;
//         }
//         if am.lo_mask {
//             byte |= 0b0000_1000;
//         }
//         if am.tcrit_mask {
//             byte |= 0b0000_0010;
//         }
//         if am.tach_mask {
//             byte |= 0b0000_0001;
//         }

//         byte
//     }
// }

// #[test]
// fn convert_alert_mask_to_value() {
//     let mut am = AlertMask {
//             int_mask: false,
//             hi_mask: false,
//             lo_mask: false,
//             tcrit_mask: false,
//             tach_mask: false,
//     };

//     // validate initial state
//     assert_eq!(u8::from(am.clone()), 0b1010_0100);
//     // toggle each available setting and validate the bit
//     am.tach_mask = true;
//     assert_eq!(u8::from(am.clone()), 0b1010_0101);
//     am.tcrit_mask = true;
//     assert_eq!(u8::from(am.clone()), 0b1010_0111);
//     am.lo_mask = true;
//     assert_eq!(u8::from(am.clone()), 0b1010_1111);
//     am.hi_mask = true;
//     assert_eq!(u8::from(am.clone()), 0b1011_1111);
//     am.int_mask = true;
//     assert_eq!(u8::from(am.clone()), 0b1111_1111);
// }
