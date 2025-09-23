/*
    The conversion rate register defines the sampling rate for the internal
    and external temperature sensor.

    unit: samples per seconds
*/

use crate::emc2101::hw;

/// conversion rate (measured in 'samples per seconds')
pub enum ConversionRate {
    Sps1o16 = 0b0000, // 1 sample every 16 seconds (1/16)
    Sps1o8 = 0b0001,  // 1 sample every 8 seconds (1/8)
    Sps1o4 = 0b0010,  // 1 sample every 4 seconds (1/4)
    Sps1o2 = 0b0011,  // 1 sample every 2 seconds (1/2)
    Sps1 = 0b0100,    // 1 samples per second
    Sps2 = 0b0101,    // 2 samples per second
    Sps4 = 0b0110,    // 4 samples per second
    Sps8 = 0b0111,    // 8 samples per second
    Sps16 = 0b1000,   // 16 samples per second (default)
    Sps32 = 0b1001,   // 32 samples per second
                      // all remaining values map to 32 samples per second
}

/// read the temperature conversion rate register
///
/// expected range: 0..16
pub fn get_conversion_rate<Ibd>(ibd: &mut Ibd) -> ConversionRate
where
    Ibd: crate::traits::I2cBusDevice,
{
    // implicit return
    let value = hw::get_conversion_rate(ibd);
    match value {
        0b0000 => ConversionRate::Sps1o16,
        0b0001 => ConversionRate::Sps1o8,
        0b0010 => ConversionRate::Sps1o4,
        0b0011 => ConversionRate::Sps1o2,
        0b0100 => ConversionRate::Sps1,
        0b0101 => ConversionRate::Sps2,
        0b0110 => ConversionRate::Sps1o4,
        0b0111 => ConversionRate::Sps1o8,
        0b1000 => ConversionRate::Sps1o8,
        // all remaining values map to 32 samples per second
        _ => ConversionRate::Sps32,
    }
}

/// change the temperature conversion rate register
///
/// expected range: 0..16
pub fn set_conversion_rate<Ibd>(ibd: &mut Ibd, value: ConversionRate)
where
    Ibd: crate::traits::I2cBusDevice,
{
    hw::set_conversion_rate(ibd, value as u8);
}
