/*
    Get information about the hardware.
*/

use crate::emc2101::hw;

static UNKNOWN: &str = "<unknown>";

// ------------------------------------------------------------------------
// hardware details
// ------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
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
/// ```TEXT
///
/// // <initialize an I²C bus object>
///
/// let hwd = i2c_devices::emc2101::get_hardware_details(&mut ibd0);
/// // concise, e.g. "SMSC EMC2101 (rev: 1)"
/// info!("{0} {1} (rev: {2})", hwd.manufacturer, hwd.product, hwd.prv);
/// // detailed
/// info!("Manufacturer: {0} ({1:#04X})", hwd.manufacturer, hwd.mid);
/// info!("Product:      {0} ({1:#04X})", hwd.product, hwd.pid);
/// info!("Revision:     {0:#04X}", hwd.revision);
/// ```
pub fn get_hardware_details<Ibd>(ibd: &mut Ibd) -> HardwareDetails
where
    Ibd: crate::traits::I2cBusDevice,
{
    let mid = hw::get_manufacturer_id(ibd);
    let pid = hw::get_product_id(ibd);
    let rev = hw::get_product_revision(ibd);

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

/// a representation of the EMC2101's status register (0x02)
///
/// for an exhaustive description refer to the data sheet (section 6.4)
#[derive(Debug, PartialEq)]
pub struct StatusRegister {
    // the comment describes what happens if the value is set to True
    pub busy: bool,        // ADC is converting
    pub temp_int_hi: bool, // internal temperature has met or exceeded the high limit
    pub eeprom: bool,      // EEPROM  could  not  be  found (EMC2101-R)
    pub temp_ext_hi: bool, // external diode temperature has exceeded the high limit
    pub temp_ext_lo: bool, // external diode temperature has fallen below the low limit
    pub diode_fault: bool, // fault has occurred on the External Diode
    pub temp_crit: bool,   // external diode temperature has met or exceeded the TCRIT limit
    pub rpm_low: bool,     // tach count has exceeded the tach limit (RPM too low)
}

pub fn get_status_register<Ibd>(ibd: &mut Ibd) -> StatusRegister
where
    Ibd: crate::traits::I2cBusDevice,
{
    let cfg = hw::get_status_register(ibd);

    // implicit return
    StatusRegister {
        busy: (cfg & 0b1000_0000) != 0,
        temp_int_hi: (cfg & 0b0100_0000) != 0,
        eeprom: (cfg & 0b0010_0000) != 0,
        temp_ext_hi: (cfg & 0b0001_0000) != 0,
        temp_ext_lo: (cfg & 0b0000_1000) != 0,
        diode_fault: (cfg & 0b0000_0100) != 0,
        temp_crit: (cfg & 0b0000_0010) != 0,
        rpm_low: (cfg & 0b0000_0001) != 0,
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
