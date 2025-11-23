/*
    EMC2101's spin up behavior (0x4B)
*/

#[allow(unused_imports)]
use log::{debug, error, info, warn};

// ------------------------------------------------------------------------
// Spin Up Behavior
// ------------------------------------------------------------------------

/// a representation of the EMC2101's spin up behavior register (0x4B)
#[derive(Debug, PartialEq)]
pub struct SpinUpBehavior {
    pub fast_mode: bool,
    pub strength: SpinUpStrength,
    pub duration: SpinUpDuration,
}

// ------------------------------------------------------------------------
// Spin Up Strength
// ------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub enum SpinUpStrength {
    Bypass = 0b0000_0000,
    Half = 0b0000_1000,
    ThreeQuarter = 0b0001_0000,
    Full = 0b0001_1000,
}

impl From<u8> for SpinUpStrength {
    fn from(value: u8) -> Self {
        match value & 0b0001_1000 {
            0b0000_1000 => SpinUpStrength::Half,
            0b0001_0000 => SpinUpStrength::ThreeQuarter,
            0b0001_1000 => SpinUpStrength::Full,
            _ => SpinUpStrength::Bypass,
        }
    }
}

#[test]
fn convert_value_to_spin_up_strength() {
    assert_eq!(SpinUpStrength::from(0), SpinUpStrength::Bypass);
    assert_eq!(SpinUpStrength::from(8), SpinUpStrength::Half);
    assert_eq!(SpinUpStrength::from(16), SpinUpStrength::ThreeQuarter);
    assert_eq!(SpinUpStrength::from(24), SpinUpStrength::Full);
}

// ------------------------------------------------------------------------
// Spin Up Duration
// ------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub enum SpinUpDuration {
    Bypass = 0b0000_0000,
    Ms0050 = 0b0000_0001,
    Ms0100 = 0b0000_0010,
    Ms0200 = 0b0000_0011,
    Ms0400 = 0b0000_0100,
    Ms0800 = 0b0000_0101,
    Ms1600 = 0b0000_0110,
    Ms3200 = 0b0000_0111,
}

impl From<u8> for SpinUpDuration {
    fn from(value: u8) -> Self {
        match value & 0b0000_0111 {
            0b0000_0001 => SpinUpDuration::Ms0050,
            0b0000_0010 => SpinUpDuration::Ms0100,
            0b0000_0011 => SpinUpDuration::Ms0200,
            0b0000_0100 => SpinUpDuration::Ms0400,
            0b0000_0101 => SpinUpDuration::Ms0800,
            0b0000_0110 => SpinUpDuration::Ms1600,
            0b0000_0111 => SpinUpDuration::Ms3200,
            _ => SpinUpDuration::Bypass,
        }
    }
}

#[test]
fn convert_value_to_spin_up_duration() {
    assert_eq!(SpinUpDuration::from(0), SpinUpDuration::Bypass);
    assert_eq!(SpinUpDuration::from(1), SpinUpDuration::Ms0050);
    assert_eq!(SpinUpDuration::from(2), SpinUpDuration::Ms0100);
    assert_eq!(SpinUpDuration::from(3), SpinUpDuration::Ms0200);
    assert_eq!(SpinUpDuration::from(4), SpinUpDuration::Ms0400);
    assert_eq!(SpinUpDuration::from(5), SpinUpDuration::Ms0800);
    assert_eq!(SpinUpDuration::from(6), SpinUpDuration::Ms1600);
    assert_eq!(SpinUpDuration::from(7), SpinUpDuration::Ms3200);
}
