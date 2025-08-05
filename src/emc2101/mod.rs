//
// interface for SMSC's EMC2101 and EMC2101-R fan controller chips
//

pub mod hw;

#[allow(unused_imports)]
use log::{debug, error, info, warn};

static UNKNOWN: &str = "<unknown>";

// ------------------------------------------------------------------------
// hardware details
// ------------------------------------------------------------------------

pub struct HardwareDetails {
    pub mid: u8,
    pub manufacturer: &'static str,
    pub pid: u8,
    pub product: &'static str,
    pub revision: u8,
}

/// read the hardware details
///
/// usage:
/// ```rust
/// // <initialize an I²C bus object>
///
/// let hwd = i2c_devices::emc2101::get_hardware_details(&mut i2c_bus0);
/// // concise, e.g. "SMSC EMC2101 (rev: 1)"
/// info!("{0} {1} (rev: {2})", hwd.manufacturer, hwd.product, hwd.prv);
/// // detailed
/// info!("Manufacturer: {0} ({1:#04X})", hwd.manufacturer, hwd.mid);
/// info!("Product:      {0} ({1:#04X})", hwd.product, hwd.pid);
/// info!("Revision:     {0:#04X}", hwd.revision);
/// ```
pub fn get_hardware_details<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> HardwareDetails
where
    Dm: esp_hal::DriverMode,
{
    let mid = hw::get_manufacturer_id(i2c_bus);
    let pid = hw::get_product_id(i2c_bus);
    let rev = hw::get_product_revision(i2c_bus);

    let man = identify_manufacturer(mid);
    let prd = identify_product(pid);

    // implicit return
    HardwareDetails {
        mid,
        manufacturer: man,
        pid,
        product: prd,
        revision: rev,
    }
}

/// reset the device register to their default values
pub fn reset_device_registers<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>)
where
    Dm: esp_hal::DriverMode,
{
    hw::reset_device_registers(i2c_bus);
}

// ------------------------------------------------------------------------
// fan speed control
// ------------------------------------------------------------------------

/// a representation of the EMC2101's config register (0x03)
///
/// this is not the entire configuration, there are additional registers
/// which configure different aspects of this chip, e.g. fan configuration
/// register (0x4A)
///
/// for an exhaustive description refer to the data sheet (section 6.5)
pub struct ConfigRegister {
    // the comment describes what happens if the value is set to True
    pub mask: bool,        // disable ALERT/TACH when in interrupt mode
    pub standby: bool,     // enable low power standby mode
    pub fan_standby: bool, // disable fan output while in standby
    pub dac: bool,         // enable DAC output on FAN pin
    pub dis_to: bool,      // disable I²C bus timeout
    pub alt_tach: bool,    // configure pin six as tacho input
    pub tcrit_ovrd: bool,  // unlock tcrit limit and allow one-time write
    pub queue: bool,       // alert after 3 critical temperature readings
}

pub fn get_config_register<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> ConfigRegister
where
    Dm: esp_hal::DriverMode,
{
    let cfg = hw::get_config_register(i2c_bus);

    // implicit return
    ConfigRegister {
        mask: (cfg & 0b1000_0000) != 0,
        standby: (cfg & 0b0100_0000) != 0,
        fan_standby: (cfg & 0b0010_0000) != 0,
        dac: (cfg & 0b0001_0000) != 0,
        dis_to: (cfg & 0b0000_1000) != 0,
        alt_tach: (cfg & 0b0000_0100) != 0,
        tcrit_ovrd: (cfg & 0b0000_0010) != 0,
        queue: (cfg & 0b0000_0001) != 0,
    }
}

//     # ---------------------------------------------------------------------
//     # temperature measurements - internal temperature sensor
//     # ---------------------------------------------------------------------

/// read the temperature measured by the internal sensor (in °C)
/// - the data sheet guarantees a precision of ±2°C
/// - expected range: 0º.00C to 85.00ºC
pub fn get_internal_temperature<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> f32
where
    Dm: esp_hal::DriverMode,
{
    // implicit return
    hw::get_internal_temperature(i2c_bus) as f32
}

/// read the "high temperature" alerting limit
/// - expected range: 0.00ºC to 85.00ºC
/// - default: 70.00°C
pub fn get_internal_temperature_high_limit<Dm>(
    i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>,
) -> f32
where
    Dm: esp_hal::DriverMode,
{
    // implicit return
    hw::get_internal_temperature_high_limit(i2c_bus) as f32
}

/// set the "high temperature" alerting limit
/// - expected range: 0.00ºC to 85.00ºC
/// - decimal points are truncated (not rounded)
pub fn set_internal_temperature_high_limit<Dm>(
    i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>,
    value: f32,
) -> bool
where
    Dm: esp_hal::DriverMode,
{
    if (0.0..=85.0).contains(&value) {
        // implicit return
        hw::set_internal_temperature_high_limit(i2c_bus, value as u8)
    } else {
        warn!("Provided value for internal temperature limit must be in range 0.0°C <= x <= 85°C!");
        // implicit return
        false
    }
}

/// read the temperature measured by the external sensor (in °C)
/// - the data sheet guarantees a precision of ±1°C
/// - expected range: 0.00ºC to 85.00ºC
pub fn get_external_temperature<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> f32
where
    Dm: esp_hal::DriverMode,
{
    let bytes = hw::get_external_temperature(i2c_bus);
    debug!("get_external_temperature():");
    debug!("  MSB: {0:#04X}", bytes[1]);
    debug!("  LSB: {0:#010b}", bytes[1]);

    // TODO validate result

    // implicit return
    convert_bytes2temperature(bytes)
}

/// read the "low temperature" alerting limit
/// - expected range: 0.00ºC to 85.00ºC
/// - default: 0.00°C
pub fn get_external_temperature_low_limit<Dm>(
    i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>,
) -> f32
where
    Dm: esp_hal::DriverMode,
{
    let bytes = hw::get_external_temperature_low_limit(i2c_bus);

    // implicit return
    convert_bytes2temperature(bytes)
}

/// read the "high temperature" alerting limit
/// - expected range: 0.00ºC to 85.00ºC
/// - default: 70.00°C
pub fn get_external_temperature_high_limit<Dm>(
    i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>,
) -> f32
where
    Dm: esp_hal::DriverMode,
{
    let bytes = hw::get_external_temperature_high_limit(i2c_bus);

    // implicit return
    convert_bytes2temperature(bytes)
}

// ------------------------------------------------------------------------
// helper functions
// ------------------------------------------------------------------------

// convert the provided temperature from internal value to float
// e.g.: 0x0C + 0xE0 -> 13.9 (13 + 0.50 + 0.25 + 0.15)
fn convert_bytes2temperature(bytes: [u8; 2]) -> f32 {
    let msb = bytes[0];
    let lsb = bytes[1];

    let mut temp = msb as f32;
    if (lsb & 0b1000_0000) != 0 {
        temp += 0.50;
    }
    if (lsb & 0b0100_0000) != 0 {
        temp += 0.25;
    }
    if (lsb & 0b0010_0000) != 0 {
        temp += 0.15;
    }

    // implicit return
    temp
}

fn identify_manufacturer(mid: u8) -> &'static str {
    let smsc: &'static str = "SMSC";

    // implicit return
    match mid {
        0x5D => smsc,
        _ => UNKNOWN,
    }
}

fn identify_product(pid: u8) -> &'static str {
    let emc2101: &'static str = "EMC2101";
    let emc2101r: &'static str = "EMC2101-R";

    // implicit return
    match pid {
        0x16 => emc2101,
        0x28 => emc2101r,
        _ => UNKNOWN,
    }
}
