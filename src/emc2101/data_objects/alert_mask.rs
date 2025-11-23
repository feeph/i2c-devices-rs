/*
    EMC2101's alert mask (0x16)
*/

#[allow(unused_imports)]
use log::{debug, error, info, warn};

/// a representation of the EMC2101's alert mask register (0x16)
#[derive(Clone, Debug, PartialEq)]
pub struct AlertMask {
    // internal diode
    pub int_mask: bool,
    // external diode
    pub hi_mask: bool,
    pub lo_mask: bool,
    pub tcrit_mask: bool,
    // fan
    pub tach_mask: bool,
}

impl From<u8> for AlertMask {
    fn from(value: u8) -> Self {
        AlertMask {
            int_mask: (value & 0b0100_0000) != 0,
            hi_mask: (value & 0b0001_0000) != 0,
            lo_mask: (value & 0b0000_1000) != 0,
            tcrit_mask: (value & 0b0000_0010) != 0,
            tach_mask: (value & 0b0000_0001) != 0,
        }
    }
}

#[test]
fn convert_value_to_alert_mask() {
    let mut am = AlertMask {
        int_mask: false,
        hi_mask: false,
        lo_mask: false,
        tcrit_mask: false,
        tach_mask: false,
    };

    // initial state - no bits set
    assert_eq!(AlertMask::from(0b0000_0000), am);
    // toggle each available setting and validate the bit
    am.tach_mask = true;
    assert_eq!(AlertMask::from(0b0000_0001), am);
    am.tcrit_mask = true;
    assert_eq!(AlertMask::from(0b0000_0011), am);
    am.lo_mask = true;
    assert_eq!(AlertMask::from(0b0000_1011), am);
    am.hi_mask = true;
    assert_eq!(AlertMask::from(0b0001_1011), am);
    am.int_mask = true;
    assert_eq!(AlertMask::from(0b0101_1011), am);
}

impl From<AlertMask> for u8 {
    fn from(am: AlertMask) -> Self {
        let mut byte = 0b1010_0100; // always set

        if am.int_mask {
            byte |= 0b0100_0000;
        }
        if am.hi_mask {
            byte |= 0b0001_0000;
        }
        if am.lo_mask {
            byte |= 0b0000_1000;
        }
        if am.tcrit_mask {
            byte |= 0b0000_0010;
        }
        if am.tach_mask {
            byte |= 0b0000_0001;
        }

        byte
    }
}

#[test]
fn convert_alert_mask_to_value() {
    let mut am = AlertMask {
        int_mask: false,
        hi_mask: false,
        lo_mask: false,
        tcrit_mask: false,
        tach_mask: false,
    };

    // validate initial state
    assert_eq!(u8::from(am.clone()), 0b1010_0100);
    // toggle each available setting and validate the bit
    am.tach_mask = true;
    assert_eq!(u8::from(am.clone()), 0b1010_0101);
    am.tcrit_mask = true;
    assert_eq!(u8::from(am.clone()), 0b1010_0111);
    am.lo_mask = true;
    assert_eq!(u8::from(am.clone()), 0b1010_1111);
    am.hi_mask = true;
    assert_eq!(u8::from(am.clone()), 0b1011_1111);
    am.int_mask = true;
    assert_eq!(u8::from(am.clone()), 0b1111_1111);
}
