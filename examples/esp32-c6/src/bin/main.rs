/*
    Espressif ESP32-C6 example

    This application demonstrates how to talk to I²C devices with an ESP32-C6
    using this library and the embassy framework.

    Use `esp-generate` to generate a new project:

    ```BASH
    esp-generate --chip esp32c6 hello_world
    cargo add i2c_devices --no-default-features
    ```
*/

#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use core::option::Option;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;

#[allow(unused_imports)]
use log::{debug, error, info, warn};

// import required traits
use i2c_devices::ahtx0::AHTx0;
use i2c_devices::ht16k33::SegmentedDisplay;

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    // --------------------------------------------------------------------
    // device-specific setup (using embassy)
    // --------------------------------------------------------------------

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 65536);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    info!("Embassy initialized!");

    let radio_init = esp_radio::init().expect("Failed to initialize Wi-Fi/BLE controller");
    let (mut _wifi_controller, _interfaces) =
        esp_radio::wifi::new(&radio_init, peripherals.WIFI, Default::default())
            .expect("Failed to initialize Wi-Fi controller");

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

    // TODO: Spawn some tasks
    // let _ = spawner;

    // --------------------------------------------------------------------
    // spawn an embassy task and use the library
    // --------------------------------------------------------------------

    spawner.must_spawn(i2c_task(i2c_bus0));

    loop {
        info!("Hello world!");
        Timer::after(Duration::from_secs(1)).await;
    }
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

    // EMC2101
    // -------

    // use the I²C bus device to do something
    i2c_devices::emc2101::reset_device_registers(&mut ibd);

    // AHT20
    // -----

    let mut aht20 = i2c_devices::ahtx0::create_aht20();

    // need to wait at least 100ms after initial power up
    // (sensor state switches to 'Idle')
    Timer::after(Duration::from_millis(110)).await;

    // trigger a measurement
    // (sensor state switches from 'Idle' to 'Busy')
    let _ = aht20.trigger_measurement(&mut ibd);

    // need to wait at least 80ms for the measurement to complete
    // (sensor state switches from 'Busy' to 'Idle')
    Timer::after(Duration::from_millis(90)).await;

    // get the measured humidity and temperature values
    let result = aht20.get_sensor_data(&mut ibd);
    match result {
        Ok(x) => {
            info!("temperature: {:1.2}°C", x.temperature);
            info!("humidity:    {:1.2}% rH", x.humidity);
        }
        Err(x) => {
            error!("measurement failed: {:?}", x)
        }
    }

    // HT16K33
    // -----

    // mutable allows us to change blink rate and brightness later on
    let mut sd0 = i2c_devices::ht16k33::Segment14x4 {
        convert: i2c_devices::ht16k33::convert_14,
        did: 0,
        display_mode: i2c_devices::ht16k33::DisplayMode::BlinkFast,
        brightness_level: 8,
    };

    // configure display #0
    sd0.set_display_mode(&mut ibd, i2c_devices::ht16k33::DisplayMode::BlinkSlow);
    sd0.set_brightness_level(&mut ibd, 1);
    sd0.show_string(&mut ibd, "1234");

    // non-mutable is sufficient if we don't want to change display settings
    let sd1 = i2c_devices::ht16k33::Segment14x4 {
        convert: i2c_devices::ht16k33::convert_14,
        did: 1,
        display_mode: i2c_devices::ht16k33::DisplayMode::On,
        brightness_level: 1,
    };

    // configure display #1
    sd1.show_buffer(
        &mut ibd,
        &[
            0b11110111,
            0b0000_0000,
            0b10001111,
            0b0001_0010,
            0b00111001,
            0b0000_0000,
            0b00001111,
            0b0001_0010,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
        ],
    );

    // show different values on display #1
    sd1.show_string(&mut ibd, "ABCD");
    Timer::after(Duration::from_secs(2)).await;
    sd1.show_string(&mut ibd, "EFGH");
    Timer::after(Duration::from_secs(2)).await;
    sd1.show_string(&mut ibd, "IJKL");

    // temporarily disable display #0
    Timer::after(Duration::from_secs(5)).await;
    sd0.set_display_mode(&mut ibd, i2c_devices::ht16k33::DisplayMode::Off);
    Timer::after(Duration::from_secs(2)).await;
    sd0.set_display_mode(&mut ibd, i2c_devices::ht16k33::DisplayMode::On);

    // show some numbers on both displays
    sd0.show_number(&mut ibd, 1.234);
    sd1.show_number(&mut ibd, -12.34);
}

// ------------------------------------------------------------------------
// implement the trait 'i2c_devices::I2cBusDevice'
// ------------------------------------------------------------------------

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

    fn write_and_read_bytes<const N: usize>(&mut self, da: u8, bytes: &[u8]) -> Option<[u8; N]> {
        let mut rb = [0u8; N];

        // TODO add error handling for read_register_as_u8()
        let _ = self.i2c_bus.write_read(da, bytes, &mut rb);

        // implicit return
        Some(rb)
    }

    // --------------------------------------------------------------------
    // to refactor
    // --------------------------------------------------------------------

    fn write_register_as_byte(&mut self, da: u8, dr: u8, byte: u8) {
        // TODO add error handling for write_register_as_u8()
        let _ = self.i2c_bus.write(da, &[dr, byte]);
    }

    // TODO rename function: write_multiple_registers_as_u8()
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
