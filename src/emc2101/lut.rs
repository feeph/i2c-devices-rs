/*
    lookup table
*/

use crate::emc2101::hw;

/// read the lookup table hysteresis register
/// - expected range: 0°C ≤ x ≤ 31°C
/// - default: 4°C
pub fn get_lookup_table_hysteresis<Ibd>(ibd: &mut Ibd) -> u8
where
    Ibd: crate::traits::I2cBusDevice,
{
    // implicit return
    hw::get_lookup_table_hysteresis(ibd)
}

/// change the lookup table hysteresis register
/// - expected range: 0°C ≤ x ≤ 31°C
/// - default: 4°C
pub fn set_lookup_table_hysteresis<Ibd>(ibd: &mut Ibd, value: u8)
where
    Ibd: crate::traits::I2cBusDevice,
{
    let value_clamped = value.clamp(0, 31);
    hw::set_lookup_table_hysteresis(ibd, value_clamped);
}

/// read the lookup table
/// - expected temperature range: 0°C ≤ x ≤ 85°C
/// - expected fan speed range: 0x00 ≤ x ≤ 0x63
pub fn get_lookup_table<Ibd>(ibd: &mut Ibd) -> [(u8, u8); 8]
where
    Ibd: crate::traits::I2cBusDevice,
{
    // implicit return
    hw::get_lookup_table(ibd)
}

/// change the lookup table
/// - expected temperature range: 0°C ≤ x ≤ 85°C
/// - expected fan speed range: 0x00 ≤ x ≤ 0x63
pub fn set_lookup_table<Ibd>(ibd: &mut Ibd, lut: [(u8, u8); 8])
where
    Ibd: crate::traits::I2cBusDevice,
{
    let mut lut_clamped = [(0x00, 0x00); 8];
    for (i, value) in lut.iter().enumerate() {
        lut_clamped[i].0 = value.0.clamp(0, 85);
        lut_clamped[i].1 = value.1.clamp(0x00, 0x63);
    }

    hw::set_lookup_table(ibd, lut_clamped);
}
