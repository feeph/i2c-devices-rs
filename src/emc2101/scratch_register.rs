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
pub fn get_scratch_register1<Ibd>(ibd: &mut Ibd) -> u8
where
    Ibd: crate::traits::I2cBusDevice,
{
    hw::get_scratch_register1(ibd)
}

/// set value of scratch register #1
///
/// (this register is not used by the chip in any way)
pub fn set_scratch_register1<Ibd>(ibd: &mut Ibd, value: u8)
where
    Ibd: crate::traits::I2cBusDevice,
{
    hw::set_scratch_register1(ibd, value)
}

/// get value of scratch register #2
///
/// (this register is not used by the chip in any way)
pub fn get_scratch_register2<Ibd>(ibd: &mut Ibd) -> u8
where
    Ibd: crate::traits::I2cBusDevice,
{
    hw::get_scratch_register2(ibd)
}

/// set value of scratch register #2
///
/// (this register is not used by the chip in any way)
pub fn set_scratch_register2<Ibd>(ibd: &mut Ibd, value: u8)
where
    Ibd: crate::traits::I2cBusDevice,
{
    hw::set_scratch_register2(ibd, value)
}
