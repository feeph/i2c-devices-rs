/*
    raw, low-level access (as implemented by hardware)
*/
// TODO convert code from Python to Rust
// TODO consider using async writes for I²C operations
//      https://docs.espressif.com/projects/rust/esp-hal/1.0.0-rc.0/esp32c6/esp_hal/i2c/master/index.html#usage

pub mod defaults;
mod device_registers;

use core::cmp::Ord;
use core::iter::Iterator;
use device_registers::DR;

#[allow(unused_imports)]
use log::{debug, error, info, warn};

use crate::emc2101::hw::defaults::DEFAULTS;

// ------------------------------------------------------------------------
// constants
// ------------------------------------------------------------------------

// the device's I²C bus address is always 0x4C
// you must use an I²C bus multiplexer (e.g. TCA9548A) to connect multiple
// EMC2101's to the same I²C bus
static DEVICE_ADDRESS: u8 = 0x4C;

// ------------------------------------------------------------------------
// hardware details
// ------------------------------------------------------------------------

/// read the manufacturer ID
///
/// expected values:
/// - 0x5d for SMSC
pub fn get_manufacturer_id<Ibd>(ibd: &mut Ibd) -> u8
where
    Ibd: crate::traits::I2cBusDevice,
{
    // implicit return
    ibd.read_register_as_byte(DEVICE_ADDRESS, DR::Mid as u8)
}

/// read the product ID
///
/// expected values:
/// - 0x16 for EMC2101
/// - 0x28 for EMC2101-R
pub fn get_product_id<Ibd>(ibd: &mut Ibd) -> u8
where
    Ibd: crate::traits::I2cBusDevice,
{
    // implicit return
    ibd.read_register_as_byte(DEVICE_ADDRESS, DR::Pid as u8)
}

/// read the product's revision
///
/// expected values:
/// - 0x01
pub fn get_product_revision<Ibd>(ibd: &mut Ibd) -> u8
where
    Ibd: crate::traits::I2cBusDevice,
{
    // implicit return
    ibd.read_register_as_byte(DEVICE_ADDRESS, DR::Rev as u8)
}

/// reset all R/W registers to their default values
pub fn reset_device_registers<Ibd>(ibd: &mut Ibd)
where
    Ibd: crate::traits::I2cBusDevice,
{
    // TODO perform a single write transaction
    for data in DEFAULTS.iter() {
        let register = data[0];
        let default = data[1];
        ibd.write_register_as_byte(DEVICE_ADDRESS, register, default);
    }
}

/// validate that the R/W registers are set to their default values
/// (this function can be used to verify the hardware is working)
pub fn validate_device_registers<Ibd>(ibd: &mut Ibd) -> bool
where
    Ibd: crate::traits::I2cBusDevice,
{
    let mut is_ok = true;
    for data in DEFAULTS.iter() {
        let register = data[0];
        let default = data[1];

        let value = ibd.read_register_as_byte(DEVICE_ADDRESS, register);
        if default != value {
            warn!("Currently stored and default value for register '{register:#04X}' do not match: {default:#04X} != {value:#04X}");
            is_ok = false;
        }
    }

    // implicit return
    is_ok
}

/// get the device's status register
///
/// default: 0b0000_0000
pub fn get_status_register<Ibd>(ibd: &mut Ibd) -> u8
where
    Ibd: crate::traits::I2cBusDevice,
{
    // implicit return
    ibd.read_register_as_byte(DEVICE_ADDRESS, DR::Status as u8)
}

pub fn get_scratch_register1<Ibd>(ibd: &mut Ibd) -> u8
where
    Ibd: crate::traits::I2cBusDevice,
{
    // implicit return
    ibd.read_register_as_byte(DEVICE_ADDRESS, DR::Scratch1 as u8)
}

pub fn set_scratch_register1<Ibd>(ibd: &mut Ibd, value: u8)
where
    Ibd: crate::traits::I2cBusDevice,
{
    ibd.write_register_as_byte(DEVICE_ADDRESS, DR::Scratch1 as u8, value);
}

pub fn get_scratch_register2<Ibd>(ibd: &mut Ibd) -> u8
where
    Ibd: crate::traits::I2cBusDevice,
{
    // implicit return
    ibd.read_register_as_byte(DEVICE_ADDRESS, DR::Scratch2 as u8)
}

pub fn set_scratch_register2<Ibd>(ibd: &mut Ibd, value: u8)
where
    Ibd: crate::traits::I2cBusDevice,
{
    ibd.write_register_as_byte(DEVICE_ADDRESS, DR::Scratch2 as u8, value);
}

// ------------------------------------------------------------------------
// fan speed control
// ------------------------------------------------------------------------

/// get the device's config register
///
/// default: 0b0000_0000
pub fn get_config_register<Ibd>(ibd: &mut Ibd) -> u8
where
    Ibd: crate::traits::I2cBusDevice,
{
    // implicit return
    ibd.read_register_as_byte(DEVICE_ADDRESS, DR::Cfg as u8)
}

/// set the device's config register
///
/// default: 0b0000_0000
pub fn set_config_register<Ibd>(ibd: &mut Ibd, byte: u8)
where
    Ibd: crate::traits::I2cBusDevice,
{
    // implicit return
    ibd.write_register_as_byte(DEVICE_ADDRESS, DR::Cfg as u8, byte);
}

//     def configure_spinup_behaviour(self, spinup_strength: SpinUpStrength, spinup_duration: SpinUpDuration, fast_mode: bool) -> bool:
//         """
//         configure the spin-up behavior for the attached fan (duration and
//         strength). This helps to ensure the fan has sufficient power
//         available to be able to start spinning the rotor.
//          - EMC2101 enters the spin-up routine any time it transitions
//            from a minimum fan setting (00h) to a higher fan setting
//          - EMC2101 does not invoke the spin-up routine upon power up
//          - setting a strength of 0% or duration of 0s disables spin-up entirely

//         Once spin-up has completed the fan speed is reduced to the programmed setting.

//         Please note: Fast_mode is ignored if pin 6 is in alert mode.
//         """
//         with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
//             config = _get_config_register(bh)
//             if config.alt_tach:
//                 # pin 6 is configured as tacho pin
//                 value = 0x00
//                 # configure spin up time
//                 value |= spinup_duration.value
//                 # configure spin up strength (dutycycle)
//                 value |= spinup_strength.value
//                 if fast_mode:
//                     value |= 0b0010_0000
//                 bh.write_register(0x4B, value)
//                 return True
//             else:
//                 # pin 6 is configured as alert pin
//                 LH.warning("Pin 6 is in alert mode. Can't configure spinup behavior.")
//                 return False

/// read the fan's current speed (expressed as "tach reading")
/// - see section 6.14 of data sheet for details
///
/// expected range: 512 (0x0200) .. 5104 (0x13F0)
pub fn get_tach_reading<Ibd>(ibd: &mut Ibd) -> u16
where
    Ibd: crate::traits::I2cBusDevice,
{
    let adr = [
        DR::TachLsb as u8, // low byte, must be read first!
        DR::TachMsb as u8, // high byte
    ];
    let values = ibd.read_multibyte_register_as_u8(DEVICE_ADDRESS, adr);
    debug!("tach (bytes): {0:#04X} {1:#04X}", values[0], values[1]);

    // implicit return
    u16::from_le_bytes(values)
}

/// read the fan's speed limit (expressed as "tach reading")
pub fn get_tach_limit<Ibd>(ibd: &mut Ibd) -> u16
where
    Ibd: crate::traits::I2cBusDevice,
{
    let adr = [
        DR::TachLoLsb as u8, // low byte, must be read first!
        DR::TachLoMsb as u8, // high byte
    ];
    let values = ibd.read_multibyte_register_as_u8(DEVICE_ADDRESS, adr);

    // implicit return
    u16::from_le_bytes(values)
}

/// change the fan's speed limit (expressed as "tach reading")
pub fn set_tach_limit<Ibd>(ibd: &mut Ibd, tach: u16)
where
    Ibd: crate::traits::I2cBusDevice,
{
    let lsb = (tach & 0b1111_1111) as u8;
    let msb = ((tach >> 8) & 0b1111_1111) as u8;

    let values = [
        [DR::TachLoLsb as u8, lsb], // low byte
        [DR::TachLoMsb as u8, msb], // high byte
    ];
    ibd.write_multibyte_register_as_u8(DEVICE_ADDRESS, values);
}

/// read the fan config register
pub fn get_fan_config<Ibd>(ibd: &mut Ibd) -> u8
where
    Ibd: crate::traits::I2cBusDevice,
{
    // implicit return
    ibd.read_register_as_byte(DEVICE_ADDRESS, DR::FanCfg as u8)
}

/// change the fan config register
pub fn set_fan_config<Ibd>(ibd: &mut Ibd, value: u8)
where
    Ibd: crate::traits::I2cBusDevice,
{
    let value_clamped = value.clamp(0, 32);
    ibd.write_register_as_byte(DEVICE_ADDRESS, DR::FanCfg as u8, value_clamped);
}

/// read the fan spin up behavior register
pub fn get_spin_up_behavior<Ibd>(ibd: &mut Ibd) -> u8
where
    Ibd: crate::traits::I2cBusDevice,
{
    // implicit return
    ibd.read_register_as_byte(DEVICE_ADDRESS, DR::FanSpinUp as u8)
}

/// change the fan spin up behavior register
pub fn set_spin_up_behavior<Ibd>(ibd: &mut Ibd, value: u8)
where
    Ibd: crate::traits::I2cBusDevice,
{
    let value_clamped = value.clamp(0, 32);
    ibd.write_register_as_byte(DEVICE_ADDRESS, DR::FanSpinUp as u8, value_clamped);
}

/// read the fan speed register
/// - the fan speed is expressed as a decimal number
/// - the granularity of this value depends on the chosen PWM setting
/// - this value has no effect if a lookup table is used
///
/// expected range: 0..63 (maximum value is PWM dependent)
pub fn get_fan_speed<Ibd>(ibd: &mut Ibd) -> u8
where
    Ibd: crate::traits::I2cBusDevice,
{
    // implicit return
    ibd.read_register_as_byte(DEVICE_ADDRESS, DR::FanSpeed as u8)
}

/// change the fan speed register
/// - the fan speed is expressed as a decimal number
/// - the granularity of this value depends on the chosen PWM setting
/// - this value has no effect if a lookup table is used
///
/// expected range: 0..63 (maximum value is PWM dependent)
// TODO clamp to minimum/maximum as defined by the fan configuration
pub fn set_fan_speed<Ibd>(ibd: &mut Ibd, value: u8)
where
    Ibd: crate::traits::I2cBusDevice,
{
    let value_clamped = value.clamp(0, 32);
    ibd.write_register_as_byte(DEVICE_ADDRESS, DR::FanSpeed as u8, value_clamped);
}

/// read the PWM frequency register
///
/// expected range: 0..32
pub fn get_pwm_frequency<Ibd>(ibd: &mut Ibd) -> u8
where
    Ibd: crate::traits::I2cBusDevice,
{
    // implicit return
    ibd.read_register_as_byte(DEVICE_ADDRESS, DR::PwmFrq as u8)
}

/// change the PWM frequency register
///
/// expected range: 0..32
pub fn set_pwm_frequency<Ibd>(ibd: &mut Ibd, value: u8)
where
    Ibd: crate::traits::I2cBusDevice,
{
    let value_clamped = value.clamp(0, 32);
    ibd.write_register_as_byte(DEVICE_ADDRESS, DR::PwmFrq as u8, value_clamped);
}

/// read the PWM frequency divider register
///
/// expected range: 0..256
pub fn get_pwm_frequency_divider<Ibd>(ibd: &mut Ibd) -> u8
where
    Ibd: crate::traits::I2cBusDevice,
{
    // implicit return
    ibd.read_register_as_byte(DEVICE_ADDRESS, DR::PwmFrqDiv as u8)
}

/// change the PWM frequency divider register
///
/// expected range: 0..256
pub fn set_pwm_frequency_divider<Ibd>(ibd: &mut Ibd, value: u8)
where
    Ibd: crate::traits::I2cBusDevice,
{
    ibd.write_register_as_byte(DEVICE_ADDRESS, DR::PwmFrqDiv as u8, value);
}

//     def enable_lookup_table(self) -> bool:
//         """
//         The Fan Setting register (0x4C) and Fan Control Look-Up Table
//         registers (0x50-0x5F) are writeable and the Fan Setting
//         register will be used.

//         An external temperature sensor must be connected to use this feature.
//         """
//         with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
//             value = bh.read_register(0x4A)
//             bh.write_register(0x4A, value & 0b1101_1111)
//         return True

//     def disable_lookup_table(self):
//         """
//         the Fan Setting register (0x4C) and Fan Control Look-Up Table
//         registers (0x50-0x5F) are read-only and the Fan Control Look-Up
//         Table registers will be used.
//         """
//         with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
//             value = bh.read_register(0x4A)
//             bh.write_register(0x4A, value | 0b0010_0000)

//     def is_lookup_table_enabled(self) -> bool:
//         with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
//             return not bh.read_register(0x4A) & 0b0010_0000

//     def update_lookup_table(self, values: dict[int, int]) -> bool:
//         """
//         populate the lookup table with the provided values and
//         sets all unused values to zero

//         returns 'True' if the lookup table was updated and 'False' if it wasn't.
//         """
//         if len(values) > 8:
//             raise ValueError("too many entries in lookup table (max: 8)")
//         for temp, step in values.items():
//             if not self._temp_min <= temp <= self._temp_max:
//                 raise ValueError("temperature is out of range")
//             if not self._step_min <= step <= self._step_max:
//                 raise ValueError("step is out of range")
//         # -------------------------------------------------------------
//         # must disable lookup table to make it writeable
//         if self.is_lookup_table_enabled():
//             LH.error("Lookup table is enabled. Disabling.")
//             self.disable_lookup_table()
//             reenable_lut = True
//         else:
//             LH.error("Lookup table is not enabled. Good.")
//             reenable_lut = False
//         # 0x50..0x5f (8 x 2 registers; temp->step)
//         with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
//             offset = 0
//             # set provided value
//             for temp, step in values.items():
//                 bh.write_register(0x50 + offset, temp)
//                 bh.write_register(0x51 + offset, step)
//                 offset += 2
//             # fill remaining slots
//             for offset in range(offset, 16, 2):
//                 bh.write_register(0x50 + offset, 0x00)
//                 bh.write_register(0x51 + offset, 0x00)
//         # reenable lookup table if it was previously enabled
//         if reenable_lut:
//             self.enable_lookup_table()
//         return True

//     def reset_lookup_table(self):
//         # must disable lookup table to make it writeable
//         self.disable_lookup_table()
//         # set all slots to zero
//         with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
//             for offset in range(0, 16, 2):
//                 bh.write_register(0x50 + offset, 0x00)
//                 bh.write_register(0x51 + offset, 0x00)

// ------------------------------------------------------------------------
// temperature measurements
// ------------------------------------------------------------------------

/// read the temperature conversion rate register
///
/// expected range: 0..16
pub fn get_conversion_rate<Ibd>(ibd: &mut Ibd) -> u8
where
    Ibd: crate::traits::I2cBusDevice,
{
    // implicit return
    ibd.read_register_as_byte(DEVICE_ADDRESS, DR::ConvRate as u8)
}

/// change the temperature conversion rate register
///
/// expected range: 0..16
pub fn set_conversion_rate<Ibd>(ibd: &mut Ibd, value: u8)
where
    Ibd: crate::traits::I2cBusDevice,
{
    ibd.write_register_as_byte(DEVICE_ADDRESS, DR::ConvRate as u8, value);
}

// ------------------------------------------------------------------------
// temperature measurements - internal temperature sensor
// ------------------------------------------------------------------------

/// read the temperature measured by the internal sensor (in °C)
///  - the data sheet guarantees a precision of ±2°C
///
/// expected range: 0x00 (0°C) to 0x55 (85°C)
pub fn get_internal_temperature<Ibd>(ibd: &mut Ibd) -> u8
where
    Ibd: crate::traits::I2cBusDevice,
{
    // implicit return
    ibd.read_register_as_byte(DEVICE_ADDRESS, DR::Its as u8)
}

/// read the "high temperature" alerting limit
///
/// expected range: 0x00 (0.0°C) to 0x55 (85.0°C)
/// default: 0x46 (70.0°C)
pub fn get_internal_temperature_high_limit<Ibd>(ibd: &mut Ibd) -> u8
where
    Ibd: crate::traits::I2cBusDevice,
{
    // implicit return
    ibd.read_register_as_byte(DEVICE_ADDRESS, DR::ItsHi as u8)
}

/// set the "high temperature" alerting limit
///
/// expected range: 0x00 (0.0°C) to 0x55 (85.0°C)
pub fn set_internal_temperature_high_limit<Ibd>(ibd: &mut Ibd, limit: u8) -> bool
where
    Ibd: crate::traits::I2cBusDevice,
{
    if limit <= 85 {
        ibd.write_register_as_byte(DEVICE_ADDRESS, DR::ItsHi as u8, limit);
        // implicit return
        true
    } else {
        // implicit return
        false
    }
}

/// read the alert mask
pub fn get_alert_mask<Ibd>(ibd: &mut Ibd) -> u8
where
    Ibd: crate::traits::I2cBusDevice,
{
    // implicit return
    ibd.read_register_as_byte(DEVICE_ADDRESS, DR::AlrtMsk as u8)
}

/// change the alert mask
pub fn set_alert_mask<Ibd>(ibd: &mut Ibd, byte: u8)
where
    Ibd: crate::traits::I2cBusDevice,
{
    ibd.write_register_as_byte(DEVICE_ADDRESS, DR::AlrtMsk as u8, byte);
}

// ------------------------------------------------------------------------
// temperature measurements - external temperature sensor
// ------------------------------------------------------------------------

/// read the external sensor's beta compensation factor
pub fn get_ets_bcf<Ibd>(ibd: &mut Ibd) -> u8
where
    Ibd: crate::traits::I2cBusDevice,
{
    // implicit return
    ibd.read_register_as_byte(DEVICE_ADDRESS, DR::EtsBcf as u8)
}

/// change the external sensor's beta compensation factor
pub fn set_ets_bcf<Ibd>(ibd: &mut Ibd, byte: u8)
where
    Ibd: crate::traits::I2cBusDevice,
{
    ibd.write_register_as_byte(DEVICE_ADDRESS, DR::EtsBcf as u8, byte);
}

/// read the external sensor's diode ideality factor
pub fn get_ets_dif<Ibd>(ibd: &mut Ibd) -> u8
where
    Ibd: crate::traits::I2cBusDevice,
{
    // implicit return
    ibd.read_register_as_byte(DEVICE_ADDRESS, DR::EtsDif as u8)
}

/// change the external sensor's diode ideality factor
pub fn set_ets_dif<Ibd>(ibd: &mut Ibd, byte: u8)
where
    Ibd: crate::traits::I2cBusDevice,
{
    ibd.write_register_as_byte(DEVICE_ADDRESS, DR::EtsDif as u8, byte);
}

/// read the external sensor's critical temperature threshold
pub fn get_ets_tcrit_threshold<Ibd>(ibd: &mut Ibd) -> u8
where
    Ibd: crate::traits::I2cBusDevice,
{
    // implicit return
    ibd.read_register_as_byte(DEVICE_ADDRESS, DR::CritTemp as u8)
}

/// change the external sensor's critical temperature threshold
pub fn set_ets_tcrit_threshold<Ibd>(ibd: &mut Ibd, byte: u8)
where
    Ibd: crate::traits::I2cBusDevice,
{
    ibd.write_register_as_byte(DEVICE_ADDRESS, DR::CritTemp as u8, byte);
}

/// read the external sensor's critical temperature hysteresis
pub fn get_ets_tcrit_hysteresis<Ibd>(ibd: &mut Ibd) -> u8
where
    Ibd: crate::traits::I2cBusDevice,
{
    // implicit return
    ibd.read_register_as_byte(DEVICE_ADDRESS, DR::CritHyst as u8)
}

/// change the external sensor's critical temperature hysteresis
pub fn set_ets_tcrit_hysteresis<Ibd>(ibd: &mut Ibd, byte: u8)
where
    Ibd: crate::traits::I2cBusDevice,
{
    ibd.write_register_as_byte(DEVICE_ADDRESS, DR::CritHyst as u8, byte);
}

/// read the temperature measured by the external sensor
/// - the data sheet guarantees a precision of ±1°C
/// - negative values are represented using two's-complement
///
/// expected range: [0xBF, 0x00] (-64.0°C) to [0x7F, 0xE0] (127.875°C)
pub fn get_external_temperature<Ibd>(ibd: &mut Ibd) -> (u8, u8)
where
    Ibd: crate::traits::I2cBusDevice,
{
    let adr = [
        DR::EtsMsb as u8, // high byte, must be read first!
        DR::EtsLsb as u8, // low byte
    ];

    let [msb, lsb] = ibd.read_multibyte_register_as_u8(DEVICE_ADDRESS, adr);

    // implicit return
    (msb, lsb)
}

/// override the temperature measured by the external temperature sensor
/// (see section 6.8 and 6.16 for details)
/// - this setting can be used to test the lookup table or use a
///   temperature measured by an independent sensor
/// - requires the FORCE and PROG bit in the fan configuration register
///
/// The external diode temperature registers are updated normally with
/// the measured temperature and compared against the THIGH and TCRIT
/// limits but not used to determine the fan speed.
///
/// expected range: 0x00 (0°C) to 0x55 (85°C)
pub fn set_external_temperature_override<Ibd>(ibd: &mut Ibd, value: u8)
where
    Ibd: crate::traits::I2cBusDevice,
{
    ibd.write_register_as_byte(DEVICE_ADDRESS, DR::EtsFrc as u8, value);
}

/// read the "low temperature" alerting limit
///
/// expected range: [0x00, 0x00] (0.0°C) to [0x55, 0x00] (85.0°C)
/// default: [0x00, 0x00] (0.0°C)
pub fn get_external_temperature_low_limit<Ibd>(ibd: &mut Ibd) -> (u8, u8)
where
    Ibd: crate::traits::I2cBusDevice,
{
    let adr = [
        DR::EtsLoMsb as u8, // high byte, must be read first!
        DR::EtsLoLsb as u8, // low byte
    ];

    let [msb, lsb] = ibd.read_multibyte_register_as_u8(DEVICE_ADDRESS, adr);

    // implicit return
    (msb, lsb)
}

/// change the "low temperature" alerting limit
///
/// expected range: [0x00, 0x00] (0.0°C) to [0x55, 0x00] (85.0°C)
pub fn set_external_temperature_low_limit<Ibd>(ibd: &mut Ibd, bytes: (u8, u8))
where
    Ibd: crate::traits::I2cBusDevice,
{
    let values = [
        [DR::EtsLoMsb as u8, bytes.0], // high byte
        [DR::EtsLoLsb as u8, bytes.1], // low byte
    ];
    ibd.write_multibyte_register_as_u8(DEVICE_ADDRESS, values);
}

/// read the "high temperature" alerting limit
///
/// expected range: [0x00, 0x00] (0.0°C) to [0x55, 0x00] (85.0°C)
/// default: [0x46, 0x00] (70.0°C)
pub fn get_external_temperature_high_limit<Ibd>(ibd: &mut Ibd) -> (u8, u8)
where
    Ibd: crate::traits::I2cBusDevice,
{
    let adr = [
        DR::EtsHiMsb as u8, // high byte, must be read first!
        DR::EtsHiLsb as u8, // low byte
    ];

    let [msb, lsb] = ibd.read_multibyte_register_as_u8(DEVICE_ADDRESS, adr);

    // implicit return
    (msb, lsb)
}

/// change the "high temperature" alerting limit
///
/// expected range: [0x00, 0x00] (0.0°C) to [0x55, 0x00] (85.0°C)
pub fn set_external_temperature_high_limit<Ibd>(ibd: &mut Ibd, bytes: (u8, u8))
where
    Ibd: crate::traits::I2cBusDevice,
{
    let values = [
        [DR::EtsHiMsb as u8, bytes.0], // high byte
        [DR::EtsHiLsb as u8, bytes.1], // low byte
    ];
    ibd.write_multibyte_register_as_u8(DEVICE_ADDRESS, values);
}

/// trigger a temperature conversion ('one shot')
/// - device must be in standby mode
/// - does nothing in 'continuous conversion' mode)
pub fn trigger_one_shot<Ibd>(ibd: &mut Ibd)
where
    Ibd: crate::traits::I2cBusDevice,
{
    // the write operation is the important part
    // (the data value is irrelevant and ignored)

    // implicit return
    ibd.write_register_as_byte(DEVICE_ADDRESS, DR::OneShot as u8, 0x00)
}

/// get the level of digital averaging used for the external diode
/// temperature measurements
///
/// (see data sheet section 6.23 for details)
pub fn get_ets_averaging_filter<Ibd>(ibd: &mut Ibd) -> u8
where
    Ibd: crate::traits::I2cBusDevice,
{
    // implicit return
    ibd.read_register_as_byte(DEVICE_ADDRESS, DR::AvgFlt as u8)
}

/// set the level of digital averaging used for the external diode
/// temperature measurements
///
/// (see data sheet section 6.23 for details)
pub fn set_ets_averaging_filter<Ibd>(ibd: &mut Ibd, byte: u8)
where
    Ibd: crate::traits::I2cBusDevice,
{
    ibd.write_register_as_byte(DEVICE_ADDRESS, DR::AvgFlt as u8, byte);
}

// ------------------------------------------------------------------------
// lookup table
// ------------------------------------------------------------------------

/// read the lookup table hysteresis register
pub fn get_lookup_table_hysteresis<Ibd>(ibd: &mut Ibd) -> u8
where
    Ibd: crate::traits::I2cBusDevice,
{
    // implicit return
    ibd.read_register_as_byte(DEVICE_ADDRESS, DR::LutHyst as u8)
}

/// change the lookup table hysteresis register
pub fn set_lookup_table_hysteresis<Ibd>(ibd: &mut Ibd, byte: u8)
where
    Ibd: crate::traits::I2cBusDevice,
{
    ibd.write_register_as_byte(DEVICE_ADDRESS, DR::LutHyst as u8, byte);
}

/// read the lookup table registers
///
/// (see data sheet section 6.22 for details)
pub fn get_lookup_table<Ibd>(ibd: &mut Ibd) -> [(u8, u8); 8]
where
    Ibd: crate::traits::I2cBusDevice,
{
    let mut lut = [(0x00, 0x00); 8];

    // convert the tuple pairs into consecutive read requests
    let adr = DR::LutBase as u8;
    for (i, value) in lut.iter_mut().enumerate() {
        let offset = (i as u8) * 2; // 0, 2, 4, .. 14
        value.0 = ibd.read_register_as_byte(DEVICE_ADDRESS, adr + offset);
        value.1 = ibd.read_register_as_byte(DEVICE_ADDRESS, adr + offset + 1);
    }

    // implicit return
    lut
}

/// change the lookup table registers
///
/// (see data sheet section 6.22 for details)
pub fn set_lookup_table<Ibd>(ibd: &mut Ibd, lut: [(u8, u8); 8])
where
    Ibd: crate::traits::I2cBusDevice,
{
    // convert consecutive read requests into the tuple pairs
    let adr = DR::LutBase as u8;
    for (i, value) in lut.iter().enumerate() {
        let offset = (i as u8) * 2; // 0, 2, 4, .. 14
        ibd.write_register_as_byte(DEVICE_ADDRESS, adr + offset, value.0);
        ibd.write_register_as_byte(DEVICE_ADDRESS, adr + offset + 1, value.1);
    }
}
