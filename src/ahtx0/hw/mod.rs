/*
    ASAIR AHTx0 Humidity and Temperature Sensor

    The AHTx0 sensors are very simple devices. You can either
    - send a command (up to three bytes)
    - read data (up to 6 bytes)

    The sensor always returns the same set of data:
    [ <status>, <data0>, <data1>, <data2>, <data3>, <data4>, <data5>, <crc>]
*/

#[allow(unused_imports)]
use log::{debug, error, info, warn};

pub fn trigger_reset<Ibd>(ibd: &mut Ibd, da: u8) -> bool
where
    Ibd: crate::traits::I2cBusDevice,
{
    ibd.write_bytes(da, &[0xBA])
}

pub fn trigger_calibration<Ibd>(ibd: &mut Ibd, da: u8, dr: u8, command: &[u8; 2]) -> bool
where
    Ibd: crate::traits::I2cBusDevice,
{
    debug!("Sending command {:#04X} to {:#04X}.", dr, da);
    ibd.write_bytes(da, &[dr, command[0], command[1]])
}

pub fn trigger_measurement<Ibd>(ibd: &mut Ibd, da: u8)
where
    Ibd: crate::traits::I2cBusDevice,
{
    let _ = ibd.write_bytes(da, &[0xAC, 0x33, 0x00]);
}

pub fn get_sensor_status<Ibd>(ibd: &mut Ibd, da: u8) -> u8
where
    Ibd: crate::traits::I2cBusDevice,
{
    ibd.read_bytes::<1>(da).unwrap()[0]
}

pub fn get_sensor_data<Ibd, const N: usize>(ibd: &mut Ibd, da: u8) -> [u8; N]
where
    Ibd: crate::traits::I2cBusDevice,
{
    ibd.read_bytes::<N>(da).unwrap()
}

pub fn request_measurement<Ibd>(ibd: &mut Ibd, da: u8) -> bool
where
    Ibd: crate::traits::I2cBusDevice,
{
    ibd.write_bytes(da, &[0xAC, 0x33, 0x00])
}
