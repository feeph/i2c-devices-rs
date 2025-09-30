/*
    Espressif ESP32-C6 example

    This application demonstrates how to talk to I²C devices with an ESP32-C6
    using this library and the embassy framework.
*/

// TODO update the example to use esp-hal v1.0.0-rc.0 instead of v1.0.0-beta.1
// (using 1.0.0-rc.0 generates 4 errors, some details must have changed)

#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use embassy_executor::Spawner;
use esp_hal::clock::CpuClock;

#[allow(unused_imports)]
use log::{debug, error, info, warn};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // --------------------------------------------------------------------
    // device-specific setup (using embassy)
    // --------------------------------------------------------------------

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 72 * 1024);

    let timer = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer.timer0);

    info!("Embassy initialized!");

    // --------------------------------------------------------------------
    // configure I²C bus and timer
    // --------------------------------------------------------------------
    // documentation:
    // - https://github.com/esp-rs/esp-hal/blob/main/esp-hal/src/i2c/master/mod.rs

    // use the same pins that would be used in ESP32-C6's "Low Power" mode
    // to make circuit layouts compatible with both modes
    let pin_sda = peripherals.GPIO6;
    let pin_scl = peripherals.GPIO7;

    // set the bus frequency
    // - I²C standard mode: 100kHz
    // - I²C fast mode:     400kHz
    let i2c_config =
        esp_hal::i2c::master::Config::default().with_frequency(esp_hal::time::Rate::from_khz(400));

    #[allow(unused_mut)]
    let mut i2c_bus0 = esp_hal::i2c::master::I2c::new(peripherals.I2C0, i2c_config)
        .unwrap()
        .with_scl(pin_scl)
        .with_sda(pin_sda);

    // --------------------------------------------------------------------
    // spawn an embassy task and use the library
    // --------------------------------------------------------------------

    spawner.must_spawn(i2c_task(i2c_bus0));
}

// ------------------------------------------------------------------------
// create an embassy task
// ------------------------------------------------------------------------

#[embassy_executor::task]
pub async fn i2c_task(mut i2c_bus: esp_hal::i2c::master::I2c<'static, esp_hal::Blocking>) {
    // create an I²C bus device
    let mut ibd = I2cBusDevice {
        i2c_bus: &mut i2c_bus,
    };

    // use the I²C bus device to do something
    i2c_devices::emc2101::reset_device_registers(&mut ibd);
}

// ------------------------------------------------------------------------
// implement the trait 'i2c_devices::I2cBusDevice'
// ------------------------------------------------------------------------

struct I2cBusDevice<'a, Dm: esp_hal::DriverMode> {
    i2c_bus: &'a mut esp_hal::i2c::master::I2c<'a, Dm>,
}

impl<'a, Dm: esp_hal::DriverMode> i2c_devices::I2cBusDevice for I2cBusDevice<'a, Dm> {
    fn read_byte(&mut self, da: u8) -> Result<u8, &'static str> {
        let mut buf = [0, 1];

        let res = self.i2c_bus.read(da, &mut buf);
        match res {
            Ok(_) => Ok(buf[0]),
            Err(_) => Err(""),
        }
    }

    fn write_byte(&mut self, da: u8, byte: u8) {
        let _ = self.i2c_bus.write(da, &[byte]);
    }

    fn write_bytes(&mut self, da: u8, bytes: &[u8]) {
        let _ = self.i2c_bus.write(da, bytes);
    }

    fn read_register_as_byte(&mut self, da: u8, dr: u8) -> u8 {
        let mut rb = [0u8; 1];

        // TODO add error handling for read_register_as_u8()
        let _ = self.i2c_bus.write_read(da, &[dr], &mut rb);

        // implicit return
        rb[0]
    }

    fn write_register_as_byte(&mut self, da: u8, dr: u8, byte: u8) {
        // TODO add error handling for write_register_as_u8()
        let _ = self.i2c_bus.write(da, &[dr, byte]);
    }

    fn read_multibyte_register_as_u8<const N: usize>(&mut self, da: u8, dr: [u8; N]) -> [u8; N] {
        let mut rb = [0u8; N];

        // it's a bit overkill to use a loop for two iterations but that way we
        // avoid code duplication and it opens up the possibility of reading an
        // arbitrary number of values
        for (i, register) in dr.iter().enumerate() {
            let mut v = [0; 1];
            match self.i2c_bus.write_read(da, &[*register], &mut v) {
                Ok(_) => {
                    debug!(
                        "Successfully read register '{0:#04X}' (value: {1:#04X}).",
                        dr[i], rb[i]
                    );
                    rb[i] = v[0];
                }
                Err(reason) => warn!("Failed to read register '{0:#04X}': {reason}", dr[i]),
            }
        }

        // implicit return
        rb
    }

    fn write_multibyte_register_as_u8<const N: usize>(&mut self, da: u8, values: [[u8; 2]; N]) {
        for x in values.iter() {
            match self.i2c_bus.write(da, x) {
                Ok(_) => {
                    debug!(
                        "Successfully wrote register '{0:#04X}' (value: {1:#04X}).",
                        x[0], x[1]
                    );
                }
                Err(reason) => warn!("Failed to read register '{0:#04X}': {reason}", x[0]),
            }
        }
    }

    // some functions require a little time to pass
    // the sleep function is hardware-dependent and must be provided by
    // the caller
    fn sleep_ms(&mut self, milliseconds: u32) {
        esp_hal::delay::Delay::new().delay_millis(milliseconds);
    }
}
