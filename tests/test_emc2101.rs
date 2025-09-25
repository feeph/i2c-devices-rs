/*
    low-level integration tests for EMC2101
    (using 'i2c_devices::emc2101::hw')
*/

mod common;

use common::VirtualI2cBusDevice;
use i2c_devices::emc2101::hw as sut;

#[test]
fn test_hw_manufacturer() {
    let mut vbd = VirtualI2cBusDevice {};

    let computed = sut::get_manufacturer_id(&mut vbd);
    let expected = 0x5Du8;

    assert_eq!(computed, expected);
}
