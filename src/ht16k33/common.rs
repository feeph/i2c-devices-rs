// ========================================================================
// generic trait for segmented display
// ========================================================================

#[derive(Copy, Clone)]
pub enum DisplayMode {
    Off = 0b0000,         // display disabled
    On = 0b0001,          // display enabled (no blinking)
    BlinkFast = 0b0011,   // blinking, 2Hz
    BlinkMedium = 0b0101, // blinking, 1Hz
    BlinkSlow = 0b0111,   // blinking, 0.5Hz
}

pub trait SegmentedDisplay {
    /// enter standby mode
    /// - display will be turned off
    /// - configuration is reset
    /// - device must be woken up before doing anything
    ///
    /// This is the most energy-efficient mode. Use DisplayMode
    /// 'Off' to disable the output but keep it running.
    fn disable<Ibd>(&self, ibd: &mut Ibd)
    where
        Ibd: crate::traits::I2cBusDevice;

    fn set_display_mode<Ibd>(&mut self, ibd: &mut Ibd, display_mode: DisplayMode)
    where
        Ibd: crate::traits::I2cBusDevice;

    /// set the display's brightness level
    /// (implicitly enables the display)
    /// - brightness level is graduated from 0 (6%) to 15 (100%)
    /// - use the disable() function to turn off the display entirely
    fn set_brightness_level<Ibd>(&mut self, ibd: &mut Ibd, brightness_level: u8)
    where
        Ibd: crate::traits::I2cBusDevice;

    /// display the provided data buffer
    /// (implicitly enables the display)
    /// - this is the most flexible way to address the display but the caller
    ///   needs to know which bit controls which matrix dot / segment
    fn show_buffer<Ibd>(&self, ibd: &mut Ibd, buffer: &[u8; 16])
    where
        Ibd: crate::traits::I2cBusDevice;

    fn show_number<Ibd>(&self, ibd: &mut Ibd, number: f32) -> bool
    where
        Ibd: crate::traits::I2cBusDevice;

    fn show_string<Ibd>(&self, ibd: &mut Ibd, string: &str) -> bool
    where
        Ibd: crate::traits::I2cBusDevice;
}
