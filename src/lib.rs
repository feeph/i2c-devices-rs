/*
    various I²C bus devices

    This code is written by a Rust beginner. Please be gentle.
*/

// TODO use device-agnostic trait to get rid of no-std
// (we should try to use the same approach as serde/smoltp to support
// 'std' and 'no_std')
#![no_std]

pub mod ads1x1x;
pub mod aht20;
pub mod at24c0xd;
pub mod emc2101;
pub mod tca953x;
pub mod tca9548a;

mod i2c_helpers;
mod traits;

pub use i2c_helpers::scan_i2c_bus;
pub use traits::I2cBusDevice;
