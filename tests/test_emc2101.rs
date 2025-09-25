/*
    low-level integration tests for EMC2101
    (using 'i2c_devices::emc2101::hw')
*/

mod common;

use common::VirtualI2cBusDevice;
use i2c_devices::emc2101::hw as sut;

// ------------------------------------------------------------------------

#[test]
fn hw_pid_emc2101() {
    let mut vbd = create_emc2101();

    let computed = sut::get_product_id(&mut vbd);
    let expected = 0x16u8;

    assert_eq!(computed, expected);
}

#[test]
fn hw_pid_emc2101r() {
    let mut vbd = create_emc2101r();

    let computed = sut::get_product_id(&mut vbd);
    let expected = 0x28u8;

    assert_eq!(computed, expected);
}

#[test]
fn hw_mid_smsc() {
    let mut vbd = create_emc2101();

    let computed = sut::get_manufacturer_id(&mut vbd);
    let expected = 0x5Du8;

    assert_eq!(computed, expected);
}

#[test]
fn hw_rev() {
    let mut vbd = create_emc2101();

    let computed = sut::get_product_revision(&mut vbd);
    let expected = 0x01u8;

    assert_eq!(computed, expected);
}

// ------------------------------------------------------------------------

fn create_emc2101() -> VirtualI2cBusDevice {
    let mut registers = [(0u8, false); 256];

    // set read-only registers
    registers[0xFD] = (0x16, false);
    registers[0xFE] = (0x5D, false);
    registers[0xFF] = (0x01, false);

    // set read-write registers
    for tuple in sut::defaults::DEFAULTS {
        let dr = tuple[0];
        let dv = tuple[1];

        registers[dr as usize] = (dv, true);
    }

    VirtualI2cBusDevice { registers }
}

fn create_emc2101r() -> VirtualI2cBusDevice {
    let mut registers = [(0u8, false); 256];

    // set read-only registers
    registers[0xFD] = (0x28, false);
    registers[0xFE] = (0x5D, false);
    registers[0xFF] = (0x01, false);

    // set read-write registers
    for tuple in sut::defaults::DEFAULTS {
        let dr = tuple[0];
        let dv = tuple[1];

        registers[dr as usize] = (dv, true);
    }

    VirtualI2cBusDevice { registers }
}
