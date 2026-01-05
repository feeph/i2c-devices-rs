/*
    ASAIR AHTx0 Humidity and Temperature Sensor
*/

#[allow(unused_imports)]
use log::{debug, error, info, warn};

// ------------------------------------------------------------------------
// send a command (e.g. for calibration)
// ------------------------------------------------------------------------

pub fn send_command<Ibd>(ibd: &mut Ibd, da: u8, command: &[u8; 3]) -> bool
where
    Ibd: crate::traits::I2cBusDevice,
{
    debug!("Sending command {:#04X} to {:#04X}.", command[0], da);
    ibd.write_bytes(da, command);
    true
}

// AHTX0_CMD_TRIGGER: int = const(0xAC)  # Trigger reading command
// AHTX0_CMD_SOFTRESET: int = const(0xBA)  # Soft reset command
// AHTX0_STATUS_BUSY: int = const(0x80)  # Status bit for busy
// AHTX0_STATUS_CALIBRATED: int = const(0x08)  # Status bit for calibrated

//             temperature = aht.temperature
//             relative_humidity = aht.relative_humidity
//     """
//     def __init__(self, i2c_bus: busio.I2C, address: int = AHTX0_I2CADDR_DEFAULT) -> None:
//         time.sleep(0.02)  # 20ms delay to wake up
//         self.i2c_device = I2CDevice(i2c_bus, address)
//         self._buf = bytearray(6)
//         self.reset()
//         if not self.calibrate():
//             raise RuntimeError("Could not calibrate")
//         self._temp = None
//         self._humidity = None
//     def reset(self) -> None:
//         """Perform a soft-reset of the AHT"""
//         self._buf[0] = AHTX0_CMD_SOFTRESET
//         with self.i2c_device as i2c:
//             i2c.write(self._buf, start=0, end=1)
//         time.sleep(0.02)  # 20ms delay to wake up
//     def calibrate(self) -> bool:
//         """Ask the sensor to self-calibrate. Returns True on success, False otherwise"""
//         self._buf[0] = AHT10_CMD_CALIBRATE
//         self._buf[1] = 0x08
//         self._buf[2] = 0x00
//         calibration_failed = False
//         with self.i2c_device as i2c:
//             try:
//                 # Newer AHT20's may not succeed with old command, so wrapping in try/except
//                 i2c.write(self._buf, start=0, end=3)
//             except (RuntimeError, OSError):
//                 calibration_failed = True
//         if calibration_failed:
//             # try another calibration command for newer AHT20's
//             # print("Calibration failed, trying AH20 command")
//             time.sleep(0.01)
//             self._buf[0] = AHT20_CMD_CALIBRATE
//             with self.i2c_device as i2c:
//                 try:
//                     i2c.write(self._buf, start=0, end=3)
//                 except (RuntimeError, OSError):
//                     pass
//         start_busy_time = time.monotonic()
//         while self.status & AHTX0_STATUS_BUSY:
//             if time.monotonic() - start_busy_time > 3.0:
//                 raise RuntimeError("Sensor remained busy 3 seconds. Could not be calibrated")
//             time.sleep(0.01)
//         if not self.status & AHTX0_STATUS_CALIBRATED:
//             return False
//         return True
//     @property
//     def status(self) -> int:
//         """The status byte initially returned from the sensor, see datasheet for details"""
//         with self.i2c_device as i2c:
//             i2c.readinto(self._buf, start=0, end=1)
//         # print("status: "+hex(self._buf[0]))
//         return self._buf[0]
//     @property
//     def relative_humidity(self) -> int:
//         """The measured relative humidity in percent."""
//         self._readdata()
//         return self._humidity
//     @property
//     def temperature(self) -> int:
//         """The measured temperature in degrees Celsius."""
//         self._readdata()
//         return self._temp
//     def _readdata(self) -> None:
//         """Internal function for triggering the AHT to read temp/humidity"""
//         self._buf[0] = AHTX0_CMD_TRIGGER
//         self._buf[1] = 0x33
//         self._buf[2] = 0x00
//         with self.i2c_device as i2c:
//             i2c.write(self._buf, start=0, end=3)
//         while self.status & AHTX0_STATUS_BUSY:
//             time.sleep(0.01)
//         with self.i2c_device as i2c:
//             i2c.readinto(self._buf, start=0, end=6)
//         self._humidity = (self._buf[1] << 12) | (self._buf[2] << 4) | (self._buf[3] >> 4)
//         self._humidity = (self._humidity * 100) / 0x100000
//         self._temp = ((self._buf[3] & 0xF) << 16) | (self._buf[4] << 8) | self._buf[5]
//         self._temp = ((self._temp * 200.0) / 0x100000) - 50
