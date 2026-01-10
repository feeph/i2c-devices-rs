/*
    public traits
*/

use core::result::Result;

// TODO provide device-agnostic error types (enum?)

/// read data from and write data to an I²C device
pub trait I2cBusDevice {
    /// read the specified number of bytes from the I²C device
    ///
    /// returns the specified number of bytes if the read succeeded or a
    /// null value if the write failed
    fn read_bytes<const N: usize>(&mut self, _da: u8) -> Option<[u8; N]>;

    /// write the specified number of bytes to the I²C device
    ///
    /// returns true if the write succeeded and false if the write fails
    fn write_bytes<const N: usize>(&mut self, da: u8, bytes: &[u8; N]) -> bool;

    // --------------------------------------------------------------------
    // to refactor
    // --------------------------------------------------------------------

    /// read a single byte
    fn read_byte(&mut self, da: u8) -> Result<u8, &'static str>;

    /// send a single byte
    ///
    /// this function is useful for devices that combine the 'register' and
    /// the 'data' into a single byte, e.g. Holtek HT16K33
    fn write_byte(&mut self, da: u8, byte: u8);

    /// read a single byte from device register 'dr'
    /// TODO deprecate method 'write_register_as_byte(read_register_as_byte)'
    fn read_register_as_byte(&mut self, da: u8, dr: u8) -> u8;

    /// write a single byte to device register 'dr'
    /// TODO deprecate method 'write_register_as_byte()'
    fn write_register_as_byte(&mut self, da: u8, dr: u8, byte: u8);

    /// read multiple independent registers in the exact order provided
    ///
    /// returns the register's values in exactly the same order
    fn read_multibyte_register_as_u8<const N: usize>(&mut self, da: u8, dr: [u8; N]) -> [u8; N];

    /// write two independent registers in the exact order provided
    fn write_multibyte_register_as_u8<const N: usize>(&mut self, da: u8, values: [[u8; 2]; N]);

    // some functions require a little time to pass
    // the sleep function is hardware-dependent and must be provided by
    // the caller
    // TODO deprecate method 'sleep_ms' (let the calling code decide how to spend the time)
    fn sleep_ms(&mut self, milliseconds: u32);
}
