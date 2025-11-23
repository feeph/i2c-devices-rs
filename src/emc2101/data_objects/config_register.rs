/*
    EMC2101's config register (0x03)
*/

#[allow(unused_imports)]
use log::{debug, error, info, warn};

/// a representation of the EMC2101's config register (0x03)
///
/// this is not the entire configuration, there are additional registers
/// which configure different aspects of this chip, e.g. fan configuration
/// register (0x4A)
///
/// for an exhaustive description refer to the data sheet (section 6.5)
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigRegister {
    // the comment describes what happens if the value is set to True
    pub mask: bool,        // disable ALERT/TACH when in interrupt mode
    pub standby: bool,     // enable low power standby mode
    pub fan_standby: bool, // disable fan output while in standby
    pub dac: bool,         // enable DAC output on FAN pin
    pub dis_to: bool,      // disable I²C bus timeout
    pub alt_tach: bool,    // configure pin six as tacho input
    pub tcrit_ovrd: bool,  // unlock tcrit limit and allow one-time write
    pub queue: bool,       // alert after 3 critical temperature readings
}

impl From<u8> for ConfigRegister {
    fn from(value: u8) -> Self {
        ConfigRegister {
            mask: (value & 0b1000_0000) != 0,
            standby: (value & 0b0100_0000) != 0,
            fan_standby: (value & 0b0010_0000) != 0,
            dac: (value & 0b0001_0000) != 0,
            dis_to: (value & 0b0000_1000) != 0,
            alt_tach: (value & 0b0000_0100) != 0,
            tcrit_ovrd: (value & 0b0000_0010) != 0,
            queue: (value & 0b0000_0001) != 0,
        }
    }
}

#[test]
fn convert_value_to_config_register() {
    let mut cr = ConfigRegister {
        mask: false,
        standby: false,
        fan_standby: false,
        dac: false,
        dis_to: false,
        alt_tach: false,
        tcrit_ovrd: false,
        queue: false,
    };

    // validate initial state
    assert_eq!(ConfigRegister::from(0b0000_0000), cr);
    // toggle each available setting and validate the bit
    cr.queue = true;
    assert_eq!(ConfigRegister::from(0b0000_0001), cr);
    cr.tcrit_ovrd = true;
    assert_eq!(ConfigRegister::from(0b0000_0011), cr);
    cr.alt_tach = true;
    assert_eq!(ConfigRegister::from(0b0000_0111), cr);
    cr.dis_to = true;
    assert_eq!(ConfigRegister::from(0b0000_1111), cr);
    cr.dac = true;
    assert_eq!(ConfigRegister::from(0b0001_1111), cr);
    cr.fan_standby = true;
    assert_eq!(ConfigRegister::from(0b0011_1111), cr);
    cr.standby = true;
    assert_eq!(ConfigRegister::from(0b0111_1111), cr);
    cr.mask = true;
    assert_eq!(ConfigRegister::from(0b1111_1111), cr);
}

impl From<ConfigRegister> for u8 {
    fn from(cr: ConfigRegister) -> Self {
        let mut byte = 0u8;
        if cr.mask {
            byte |= 0b1000_0000;
        }
        if cr.standby {
            byte |= 0b0100_0000;
        }
        if cr.fan_standby {
            byte |= 0b0010_0000;
        }
        if cr.dac {
            byte |= 0b0001_0000;
        }
        if cr.dis_to {
            byte |= 0b0000_1000;
        }
        if cr.alt_tach {
            byte |= 0b0000_0100;
        }
        if cr.tcrit_ovrd {
            byte |= 0b0000_0010;
        }
        if cr.queue {
            byte |= 0b0000_0001;
        }

        byte
    }
}

#[test]
fn convert_config_register_to_value() {
    let mut cr = ConfigRegister {
        mask: false,
        standby: false,
        fan_standby: false,
        dac: false,
        dis_to: false,
        alt_tach: false,
        tcrit_ovrd: false,
        queue: false,
    };

    // initial state - no bits set
    assert_eq!(u8::from(cr.clone()), 0b0000_0000);
    // toggle each available setting and validate the bit
    cr.queue = true;
    assert_eq!(u8::from(cr.clone()), 0b0000_0001);
    cr.tcrit_ovrd = true;
    assert_eq!(u8::from(cr.clone()), 0b0000_0011);
    cr.alt_tach = true;
    assert_eq!(u8::from(cr.clone()), 0b0000_0111);
    cr.dis_to = true;
    assert_eq!(u8::from(cr.clone()), 0b0000_1111);
    cr.dac = true;
    assert_eq!(u8::from(cr.clone()), 0b0001_1111);
    cr.fan_standby = true;
    assert_eq!(u8::from(cr.clone()), 0b0011_1111);
    cr.standby = true;
    assert_eq!(u8::from(cr.clone()), 0b0111_1111);
    cr.mask = true;
    assert_eq!(u8::from(cr.clone()), 0b1111_1111);
}
