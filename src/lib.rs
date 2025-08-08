/*
    various I²C bus devices

    This code is written by a Rust beginner. Please be gentle.
*/

#![no_std]

pub mod ads1x1x;
pub mod aht20;
pub mod at24c0xd;
pub mod emc2101;
pub mod tca953x;
pub mod tca9548a;

mod i2c_helpers;

pub use i2c_helpers::scan_i2c_bus;
