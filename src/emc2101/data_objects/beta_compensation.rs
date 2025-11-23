/*
    EMC2101's beta compensation register (0x18)
*/

#[allow(unused_imports)]
use log::{debug, error, info, warn};

// ------------------------------------------------------------------------
// Beta Compensation
// ------------------------------------------------------------------------

/// Please note:
/// - The mode must be set to 'Disabled' when using a thermal terminal
///   diode or a diode-connected transistor (e.g. 2N3904 / 2N3906).
/// - In modes 'Automatic' and 'Disabled' the factor's value is ignored.
///
/// See data sheet section 5.5 for details.
#[derive(Debug, PartialEq)]
pub struct BetaCompensation {
    pub mode: BetaCompensationMode,
    pub factor: u8,
}

impl From<u8> for BetaCompensation {
    fn from(value: u8) -> Self {
        // implicit return
        match value.clamp(0x00, 0x08) {
            0x08 => BetaCompensation {
                mode: BetaCompensationMode::Automatic,
                factor: 0b0000_0000,
            },
            0x07 => BetaCompensation {
                mode: BetaCompensationMode::Disabled,
                factor: 0b0000_0000,
            },
            _ => BetaCompensation {
                mode: BetaCompensationMode::Manual,
                factor: value & 0b0000_0111,
            },
        }
    }
}

#[test]
fn convert_value_to_beta_compensation() {
    assert_eq!(
        BetaCompensation::from(0x04),
        BetaCompensation {
            mode: BetaCompensationMode::Manual,
            factor: 4
        }
    );
    assert_eq!(
        BetaCompensation::from(0x07),
        BetaCompensation {
            mode: BetaCompensationMode::Disabled,
            factor: 0b0000_0000
        }
    );
    assert_eq!(
        BetaCompensation::from(0x08),
        BetaCompensation {
            mode: BetaCompensationMode::Automatic,
            factor: 0b0000_0000
        }
    );
}

impl From<BetaCompensation> for u8 {
    fn from(bcf: BetaCompensation) -> Self {
        match bcf.mode {
            BetaCompensationMode::Automatic => 0x08,
            BetaCompensationMode::Disabled => 0x07,
            BetaCompensationMode::Manual => bcf.factor.clamp(0x00, 0x06),
        }
    }
}

#[test]
fn convert_beta_compensation_to_value() {
    assert_eq!(
        u8::from(BetaCompensation {
            mode: BetaCompensationMode::Automatic,
            factor: 4
        }),
        0b0000_1000
    );
    assert_eq!(
        u8::from(BetaCompensation {
            mode: BetaCompensationMode::Disabled,
            factor: 4
        }),
        0b0000_0111
    );
    assert_eq!(
        u8::from(BetaCompensation {
            mode: BetaCompensationMode::Manual,
            factor: 4
        }),
        0b0000_0100
    );
}

// ------------------------------------------------------------------------
// Beta Compensation Mode
// ------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub enum BetaCompensationMode {
    Automatic,
    Manual,
    Disabled,
}
