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
/// this is a convenience function so the user doesn't need to know the
/// device's base address
pub fn convert_did_to_address(did: u8) -> u8 {
    const MAX: u8 = 7;
    if did > MAX {
        panic!("Device ID must be in range 0 ≤ x ≤ {MAX}!");
    }

    ADR + did
}

#[test]
fn convert_did_to_address_pass() {
    let computed = convert_did_to_address(0);
    let expected = 0x70;

    assert_eq!(computed, expected);
}

#[test]
#[should_panic(expected = "Device ID must be in range 0 ≤ x ≤ 7!")]
fn convert_did_to_address_fail() {
    let _ = convert_did_to_address(8);
}

/// convert I²C address to device ID
///
/// this is a convenience function so the user doesn't need to know the
/// device's base address
pub fn convert_address_to_did(address: u8) -> u8 {
    const MIN: u8 = ADR;
    const MAX: u8 = ADR + 7;
    if !(MIN..=MAX).contains(&address) {
        panic!("I²C address ID must be in range {MIN:#04x} ≤ x ≤ {MAX:#04x}!");
    }

    address - ADR
}

#[test]
fn convert_address_to_did_pass() {
    let computed = convert_address_to_did(0x70);
    let expected = 0;

    assert_eq!(computed, expected);
}

#[test]
#[should_panic(expected = "I²C address ID must be in range 0x70 ≤ x ≤ 0x77!")]
fn convert_address_to_did_fail() {
    let _ = convert_address_to_did(0x78);
}
