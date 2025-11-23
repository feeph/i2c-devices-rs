/*
    data objects
*/

mod alert_mask;
mod averaging_filter;
mod beta_compensation;
mod config_register;
mod spin_up_behavior;

pub use alert_mask::AlertMask;
pub use averaging_filter::{AlertPinMode, AveragingFilter, AveragingFilterMode};
pub use beta_compensation::{BetaCompensation, BetaCompensationMode};
pub use config_register::ConfigRegister;
pub use spin_up_behavior::{SpinUpBehavior, SpinUpDuration, SpinUpStrength};
