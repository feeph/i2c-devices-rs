/*
    map device ID to I²C address and vice versa
*/

use core::panic;

#[allow(unused_imports)]
use log::{debug, error, info, warn};

// I²C base address
static ADR: u8 = 0x70;

/// convert device ID to I²C address
///
/// this is a convenience function so the user doesn't need to know
pub fn convert_did_to_address(did: u8) -> u8 {
    const MAX: u8 = 7;
    if did > MAX {
        panic!("Device ID must be in range 0 ≤ x ≤ {MAX}");
    }

    ADR + did
}

/// convert I²C address to device ID
///
/// this is a convenience function so the user doesn't need to know
pub fn convert_address_to_did(address: u8) -> u8 {
    const MIN: u8 = ADR;
    const MAX: u8 = ADR + 7;
    if !(MIN..=MAX).contains(&address) {
        panic!("I²C address ID must be in range {MIN} ≤ x ≤ {MAX}");
    }

    address - ADR
}
