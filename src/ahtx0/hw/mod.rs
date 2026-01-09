/*
    ASAIR AHTx0 Humidity and Temperature Sensor

    The AHTx0 sensors are very simple devices. You can either
    - send a command (up to three bytes)
    - read data (up to 6 bytes)

    The sensor always returns the same set of data:
    [ <status>, <data0>, <data1>, <data2>, <data3>, <data4>, <data5>, <crc>]
*/

// buf2: bytearray(b'\x1cK\x15%\xf3\xa9')
// h: 307538 -> 29.3% rH
// t: 390057 -> 24.4°C
//
// buf2: bytearray(b'\x1cJ6e\xf7\x02\x00\x00')
// h: 303974 -> 29.0% rH
// t: 390914 -> 24.6°C
//
// buf2: bytearray(b'\x1cI\xdeu\xf7K\x00\x00')
// buf2: 0x1C 0x49 0xDE 0x75 0xF7 0x4B 0x0 0x0
// h: 302567 -> 28.9% rH
// t: 390987 -> 24.6°C

#[allow(unused_imports)]
use log::{debug, error, info, warn};

pub fn trigger_calibration<Ibd>(ibd: &mut Ibd, da: u8, dr: u8, command: &[u8; 2]) -> bool
where
    Ibd: crate::traits::I2cBusDevice,
{
    debug!("Sending command {:#04X} to {:#04X}.", dr, da);
    ibd.write_bytes_to_register(da, dr, command);
    true
}

// TODO validate this call is correct
// (Adafruit's device library indicates a simple read is sufficient)
pub fn get_sensor_status<Ibd>(ibd: &mut Ibd, da: u8) -> u8
where
    Ibd: crate::traits::I2cBusDevice,
{
    ibd.read_bytes_from_register::<1>(da, 0x71)[0]
}

// TODO validate this call is correct
// (Adafruit's device library indicates a simple read is sufficient)
pub fn get_sensor_data<Ibd>(ibd: &mut Ibd, da: u8) -> [u8; 8]
where
    Ibd: crate::traits::I2cBusDevice,
{
    // returned values:
    // [<status>, <data0>, <data1>, <data2>, <data3>, <data4>, <data5>, <crc>]
    ibd.read_bytes_from_register::<8>(da, 0x71)
}

pub fn request_measurement<Ibd>(ibd: &mut Ibd, da: u8)
where
    Ibd: crate::traits::I2cBusDevice,
{
    // send measurement command (always followed by 0x33, 0x00)
    ibd.write_bytes_to_register(da, 0xAC, &[0x33, 0x00]);
}
