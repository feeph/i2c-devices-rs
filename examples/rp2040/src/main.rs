/*
    Raspberry Pi Pico example

    This application demonstrates how to talk to I²C devices with an RP2040
    or RP2350 using this library.

    original code:
    https://github.com/rp-rs/rp-hal/blob/main/rp2040-hal-examples/src/bin/i2c.rs
*/

#![no_std]
#![no_main]

// ensure we halt the program on panic
// (if we don't mention this crate it won't be linked)
use panic_halt as _;

#[allow(unused_imports)]
use log::{debug, error, info, warn};

// #[allow(unused_imports)]
// use defmt::{debug, error, info, warn};

// create an alias for our HAL crate
// (use either 'rp2040_hal' or 'rp2350_hal')
use rp2040_hal as hal;

use i2c_devices::ht16k33::SegmentedDisplay;

// this trait is required for '400.kHz()'
use hal::fugit::RateExtU32;

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
/// Note: This boot block is not necessary when using a rp-hal based BSP
/// as the BSPs already perform this step.
#[unsafe(link_section = ".boot2")]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

/// External high-speed crystal on the Raspberry Pi Pico board is 12 MHz.
/// Adjust if your board has a different frequency
const XTAL_FREQ_HZ: u32 = 12_000_000;

/// Entry point to our bare-metal application.
#[hal::entry]
fn main() -> ! {
    info!("program start");

    // --------------------------------------------------------------------
    // device-specific setup
    // --------------------------------------------------------------------

    let mut pac = hal::pac::Peripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .unwrap();

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins to their default state
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // --------------------------------------------------------------------
    // configure I²C bus and timer
    // --------------------------------------------------------------------

    let sda_pin: hal::gpio::Pin<_, hal::gpio::FunctionI2C, _> = pins.gpio18.reconfigure();
    let scl_pin: hal::gpio::Pin<_, hal::gpio::FunctionI2C, _> = pins.gpio19.reconfigure();

    let mut i2c_bus = hal::I2C::i2c1(
        pac.I2C1,
        sda_pin,
        scl_pin,
        400.kHz(),
        &mut pac.RESETS,
        &clocks.system_clock,
    );

    let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    // --------------------------------------------------------------------
    // use library
    // --------------------------------------------------------------------

    // create an I²C bus device
    let mut ibd = I2cBusDevice {
        i2c_bus: &mut i2c_bus,
        timer,
    };

    // use the I²C bus device to do something
    // i2c_devices::emc2101::reset_device_registers(&mut ibd);

    // mutable allows us to change blink rate and brightness later on
    let mut sd1 = i2c_devices::ht16k33::Segment7x4 {
        convert: i2c_devices::ht16k33::convert_7,
        did: 1,
        display_mode: i2c_devices::ht16k33::DisplayMode::On,
        brightness_level: 1,
    };

    // change blink rate and brightness
    sd1.set_display_mode(&mut ibd, i2c_devices::ht16k33::DisplayMode::On);
    sd1.set_brightness_level(&mut ibd, 1);
    // write data
    sd1.show_string(&mut ibd, "12:34");

    // non-mutable is sufficient if we don't want to change display settings
    let sd2 = i2c_devices::ht16k33::Segment7x4 {
        convert: i2c_devices::ht16k33::convert_7,
        did: 2,
        display_mode: i2c_devices::ht16k33::DisplayMode::BlinkSlow,
        brightness_level: 8,
    };

    // write data
    sd2.show_number(&mut ibd, 3.456);

    // sd1.disable(&mut ibd);
    // sd1.enable(&mut ibd);
    // sd1.show_number(&mut ibd, 1.235);

    // --------------------------------------------------------------------
    // demo has finished - just loop until reset
    // --------------------------------------------------------------------

    loop {
        cortex_m::asm::wfi();
    }
}

// --------------------------------------------------------------------
// implement the trait 'i2c_devices::I2cBusDevice'
// --------------------------------------------------------------------

struct I2cBusDevice<'a, I2c: embedded_hal::i2c::I2c, Timer: embedded_hal::delay::DelayNs> {
    i2c_bus: &'a mut I2c,
    timer: Timer,
}

impl<'a, I2c, Timer> i2c_devices::I2cBusDevice for I2cBusDevice<'a, I2c, Timer>
where
    I2c: embedded_hal::i2c::I2c,
    Timer: embedded_hal::delay::DelayNs,
{
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
                Err(reason) => warn!("Failed to read register '{0:#04X}': {reason:?}", dr[i]),
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
                Err(reason) => warn!("Failed to read register '{0:#04X}': {reason:?}", x[0]),
            }
        }
    }

    // some hardware functions require a little time to pass
    // - functions that sleep mention this fact in their documentation
    // - sleeping is hardware-dependent, no_std provides no abstraction
    fn sleep_ms(&mut self, milliseconds: u32) {
        self.timer.delay_ms(milliseconds);
    }
}
