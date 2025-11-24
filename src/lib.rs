/*
    various I²C bus devices

    This code is written by a Rust beginner. Please be gentle.
*/

// allow using i2c-devices-rs without depending on the standard library
// ('no_std' is required for running on micro-controllers without an OS)
#![cfg_attr(not(feature = "std"), no_std)]

pub mod ads1x1x;
pub mod aht20;
pub mod at24c0xd;
pub mod emc2101;
pub mod ht16k33;
pub mod tca953x;
pub mod tca9548a;

mod i2c_helpers;
mod traits;

pub use i2c_helpers::scan_i2c_bus;
pub use traits::I2cBusDevice;
