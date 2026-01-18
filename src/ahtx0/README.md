# ASAIR AHTx0 Humidity and Temperature Sensor

## Initialization procedure

1. Provide power to the sensor.
2. Wait for at least 100ms for the device to become ready.
3. Send the "trigger measurement" command.
4. Wait at least 80ms for the measurement to complete.
5. Get the measured data.

### device specific

Create a hardware-agnostic I²C bus device:

```RUST
// configure the I²C bus (device specific)
let mut i2c_bus0 = esp_hal::i2c::master::I2c::new(peripherals.I2C0, i2c_config)
    .unwrap()
    .with_scl(pin_scl)
    .with_sda(pin_sda);

// create an I²C bus device
let mut ibd = I2cBusDevice {
    i2c_bus: &mut i2c_bus,
};
```

This requires the implementation of trait 'i2c_devices::I2cBusDevice':

```RUST
struct I2cBusDevice<'a, Dm: esp_hal::DriverMode> {
    i2c_bus: &'a mut esp_hal::i2c::master::I2c<'a, Dm>,
}

impl<'a, Dm: esp_hal::DriverMode> i2c_devices::I2cBusDevice for I2cBusDevice<'a, Dm> {
    /// read the specified number of bytes from the I²C device
    fn read_bytes<const N: usize>(&mut self, da: u8) -> Option<[u8; N]> {
        let mut rb = [0u8; N];

        let res = self.i2c_bus.read(da, &mut rb);
        match res {
            Ok(_) => {
                debug!("read {} bytes from device {}", N, da);
                Some(rb)
            }
            Err(_) => {
                error!("Failed to read {} bytes from device {}!", N, da);
                None
            }
        }
    }

    /// read the specified number of bytes to the I²C device
    ///
    /// returns true if the write succeeded and false if the write fails
    fn write_bytes<const N: usize>(&mut self, da: u8, bytes: &[u8; N]) -> bool {
        let res = self.i2c_bus.write(da, bytes);
        match res {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    <...>
}
```

### device-agnostic

Now that we have a hardware-agnostic I²C bus device we can use this library
to access an AHT10/AHT20 sensor:

```RUST
// initialize a device handle
// and wait for at least 100ms (sensor state switches to 'Idle')
let mut aht20 = i2c_devices::ahtx0::create_aht20();

... wait ...

// trigger a measurement (sensor state switches to 'Busy')
// and wait at least 80ms (sensor state switches to 'Idle')
let sensor_state = aht20.trigger_measurement(&mut ibd);

... wait ...

// get the measured values
let result = aht20.get_sensor_data(&mut ibd);
match result {
    Ok(x) => {
        info!("temperature: {:1.2}°C", x.temperature);
        info!("humidity:    {:1.2}% rH", x.humidity);
    },
    Err(x) => {
        error!("measurement failed: {:?}", x)
    },
}
```
