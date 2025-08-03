//
// interface for SMSC's EMC2101 and EMC2101-R fan controller chips
//

pub mod hw;

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

// ------------------------------------------------------------------------
// helper functions
// ------------------------------------------------------------------------

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
