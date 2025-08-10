/*
    interface for SMSC's EMC2101 and EMC2101-R fan controller chips
*/

pub mod hw;

// pull in all abstractions and re-export for user convenience
// (there are dozens of functions + their accompanying structs and enums)

mod config;
mod conversion_rate;
mod fan;
mod hardware_details;
mod lut;
mod scratch_register;
mod temperature;

pub use config::*;
pub use conversion_rate::*;
pub use fan::*;
pub use hardware_details::*;
pub use lut::*;
pub use scratch_register::*;
pub use temperature::*;
