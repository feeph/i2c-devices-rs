/*
    The EMC2101 and EMC2101-R provide two scratch registers that are never
    used by the hardware itself.

    These registers are non-volatile and can be used to store arbitrary
    data.
*/

use crate::emc2101::hw;

/// get value of scratch register #1
///
/// (this register is not used by the chip in any way)
pub fn get_scratch_register1<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> u8
where
    Dm: esp_hal::DriverMode,
{
    hw::get_scratch_register1(i2c_bus)
}

/// set value of scratch register #1
///
/// (this register is not used by the chip in any way)
pub fn set_scratch_register1<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>, value: u8)
where
    Dm: esp_hal::DriverMode,
{
    hw::set_scratch_register1(i2c_bus, value)
}

/// get value of scratch register #2
///
/// (this register is not used by the chip in any way)
pub fn get_scratch_register2<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> u8
where
    Dm: esp_hal::DriverMode,
{
    hw::get_scratch_register2(i2c_bus)
}

/// set value of scratch register #2
///
/// (this register is not used by the chip in any way)
pub fn set_scratch_register2<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>, value: u8)
where
    Dm: esp_hal::DriverMode,
{
    hw::set_scratch_register2(i2c_bus, value)
}
