//
// raw, low-level access (as implemented by hardware)
//
// TODO convert code from Python to Rust
// TODO consider using async writes for I²C operations
//      https://docs.espressif.com/projects/rust/esp-hal/1.0.0-rc.0/esp32c6/esp_hal/i2c/master/index.html#usage

mod defaults;
mod device_registers;
mod i2c_helpers;

use device_registers::DR;
use i2c_helpers::{
    read_multibyte_register_as_u8, read_register_as_u8, write_multibyte_register_as_u8,
    write_register_as_u8,
};

#[allow(unused_imports)]
use log::{debug, error, info, warn};

use crate::emc2101::hw::defaults::DEFAULTS;

// ------------------------------------------------------------------------
// hardware details
// ------------------------------------------------------------------------

/// read the manufacturer ID
///
/// expected values:
/// - 0x5d for SMSC
pub fn get_manufacturer_id<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> u8
where
    Dm: esp_hal::DriverMode,
{
    // implicit return
    read_register_as_u8(i2c_bus, DR::Mid as u8)
}

/// read the product ID
///
/// expected values:
/// - 0x16 for EMC2101
/// - 0x28 for EMC2101-R
pub fn get_product_id<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> u8
where
    Dm: esp_hal::DriverMode,
{
    // implicit return
    read_register_as_u8(i2c_bus, DR::Pid as u8)
}

/// read the product's revision
///
/// expected values:
/// - 0x01
pub fn get_product_revision<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> u8
where
    Dm: esp_hal::DriverMode,
{
    // implicit return
    read_register_as_u8(i2c_bus, DR::Rev as u8)
}

/// reset all R/W registers to their default values
pub fn reset_device_registers<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>)
where
    Dm: esp_hal::DriverMode,
{
    for data in DEFAULTS.iter() {
        let register = data[0];
        let default = data[1];
        write_register_as_u8(i2c_bus, register, default);
    }
}

/// validate that the R/W registers are set to their default values
/// (this function can be used to verify the hardware is working)
pub fn validate_device_registers<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> bool
where
    Dm: esp_hal::DriverMode,
{
    let mut is_ok = true;
    for data in DEFAULTS.iter() {
        let register = data[0];
        let default = data[1];

        let value = read_register_as_u8(i2c_bus, register);
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
pub fn get_status_register<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> u8
where
    Dm: esp_hal::DriverMode,
{
    // implicit return
    read_register_as_u8(i2c_bus, DR::Status as u8)
}

pub fn get_scratch_register1<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> u8
where
    Dm: esp_hal::DriverMode,
{
    // implicit return
    read_register_as_u8(i2c_bus, DR::Scratch1 as u8)
}

pub fn set_scratch_register1<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>, value: u8)
where
    Dm: esp_hal::DriverMode,
{
    write_register_as_u8(i2c_bus, DR::Scratch1 as u8, value);
}

pub fn get_scratch_register2<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> u8
where
    Dm: esp_hal::DriverMode,
{
    // implicit return
    read_register_as_u8(i2c_bus, DR::Scratch2 as u8)
}

pub fn set_scratch_register2<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>, value: u8)
where
    Dm: esp_hal::DriverMode,
{
    write_register_as_u8(i2c_bus, DR::Scratch2 as u8, value);
}

// ------------------------------------------------------------------------
// fan speed control
// ------------------------------------------------------------------------

/// get the device's config register
///
/// default: 0b0000_0000
pub fn get_config_register<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> u8
where
    Dm: esp_hal::DriverMode,
{
    // implicit return
    read_register_as_u8(i2c_bus, DR::Cfg as u8)
}

/// set the device's config register
///
/// default: 0b0000_0000
pub fn set_config_register<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>, byte: u8)
where
    Dm: esp_hal::DriverMode,
{
    // implicit return
    write_register_as_u8(i2c_bus, DR::Cfg as u8, byte);
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
pub fn get_tach_reading<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> u16
where
    Dm: esp_hal::DriverMode,
{
    let adr = [
        DR::TachLsb as u8, // low byte, must be read first!
        DR::TachMsb as u8, // high byte
    ];
    let values = read_multibyte_register_as_u8(i2c_bus, adr);
    debug!("tach (bytes): {0:#04X} {1:#04X}", values[0], values[1]);

    // implicit return
    u16::from_le_bytes(values)
}

/// read the fan's speed limit (expressed as "tach reading")
pub fn get_tach_limit<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> u16
where
    Dm: esp_hal::DriverMode,
{
    let adr = [
        DR::TachLoLsb as u8, // low byte, must be read first!
        DR::TachLoMsb as u8, // high byte
    ];
    let values = read_multibyte_register_as_u8(i2c_bus, adr);

    // implicit return
    u16::from_le_bytes(values)
}

/// change the fan's speed limit (expressed as "tach reading")
pub fn set_tach_limit<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>, tach: u16)
where
    Dm: esp_hal::DriverMode,
{
    let lsb = (tach & 0b1111_1111) as u8;
    let msb = ((tach >> 8) & 0b1111_1111) as u8;

    let values = [
        [DR::TachLsb as u8, lsb], // low byte
        [DR::TachMsb as u8, msb], // high byte
    ];
    write_multibyte_register_as_u8(i2c_bus, values);
}

/// read the fan config register
pub fn get_fan_config<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> u8
where
    Dm: esp_hal::DriverMode,
{
    // implicit return
    read_register_as_u8(i2c_bus, DR::FanCfg as u8)
}

/// change the fan config register
pub fn set_fan_config<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>, value: u8)
where
    Dm: esp_hal::DriverMode,
{
    let value_clamped = value.clamp(0, 32);
    write_register_as_u8(i2c_bus, DR::FanCfg as u8, value_clamped);
}

/// read the fan spin up behavior register
pub fn get_spin_up_behavior<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> u8
where
    Dm: esp_hal::DriverMode,
{
    // implicit return
    read_register_as_u8(i2c_bus, DR::FanSpinUp as u8)
}

/// change the fan spin up behavior register
pub fn set_spin_up_behavior<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>, value: u8)
where
    Dm: esp_hal::DriverMode,
{
    let value_clamped = value.clamp(0, 32);
    write_register_as_u8(i2c_bus, DR::FanSpinUp as u8, value_clamped);
}

/// read the fan speed register
/// - the fan speed is expressed as a decimal number
/// - the granularity of this value depends on the chosen PWM setting
/// - this value has no effect if a lookup table is used
///
/// expected range: 0..63 (maximum value is PWM dependent)
pub fn get_fan_speed<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> u8
where
    Dm: esp_hal::DriverMode,
{
    // implicit return
    read_register_as_u8(i2c_bus, DR::FanSpeed as u8)
}

/// change the fan speed register
/// - the fan speed is expressed as a decimal number
/// - the granularity of this value depends on the chosen PWM setting
/// - this value has no effect if a lookup table is used
///
/// expected range: 0..63 (maximum value is PWM dependent)
pub fn set_fan_speed<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>, value: u8)
where
    Dm: esp_hal::DriverMode,
{
    let value_clamped = value.clamp(0, 32);
    write_register_as_u8(i2c_bus, DR::FanSpeed as u8, value_clamped);
}

//     def get_driver_strength(self) -> int:
//         """
//         get the configured fan speed (raw value)
//         """
//         with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
//             return bh.read_register(0x4C)

//     def set_driver_strength(self, step: int, disable_lut: bool = False) -> bool:
//         """
//         set the configured fan speed (raw value)
//          - clamp to minimum/maximum as defined by the fan configuration
//         """
//         with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
//             if self._step_min <= step <= self._step_max:
//                 bh.write_register(0x4C, step)
//             # confirm the register was set to desired value
//             return step == bh.read_register(0x4C)

/// read the PWM  frequency register
///
/// expected range: 0..32
pub fn get_pwm_frequency<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> u8
where
    Dm: esp_hal::DriverMode,
{
    // implicit return
    read_register_as_u8(i2c_bus, DR::PwmFrq as u8)
}

/// change the PWM  frequency register
///
/// expected range: 0..32
pub fn set_pwm_frequency<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>, value: u8)
where
    Dm: esp_hal::DriverMode,
{
    let value_clamped = value.clamp(0, 32);
    write_register_as_u8(i2c_bus, DR::PwmFrq as u8, value_clamped);
}

/// read the PWM  frequency divider register
///
/// expected range: 0..256
pub fn get_pwm_frequency_divider<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> u8
where
    Dm: esp_hal::DriverMode,
{
    // implicit return
    read_register_as_u8(i2c_bus, DR::PwmFrqDiv as u8)
}

/// change the PWM  frequency divider register
///
/// expected range: 0..256
pub fn set_pwm_frequency_divider<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>, value: u8)
where
    Dm: esp_hal::DriverMode,
{
    write_register_as_u8(i2c_bus, DR::PwmFrq as u8, value);
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

//     def get_temperature_conversion_rate(self) -> str:
//         """
//         get the number of temperature conversions per second
//         """
//         with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
//             value = bh.read_register(0x04)
//         value = min(value, 0b1001)  # all values larger than 0b1001 map to 0b1001
//         return [k for k, v in CONVERSIONS_PER_SECOND.items() if v == value][0]

//     def get_temperature_conversion_rates(self) -> list[str]:
//         """
//         returns all available temperature conversion rates
//         """
//         return list(CONVERSIONS_PER_SECOND.keys())

//     def set_temperature_conversion_rate(self, conversion_rate: str) -> bool:
//         """
//         set the number of temperature conversions per second
//         """
//         value = CONVERSIONS_PER_SECOND.get(conversion_rate)
//         if value is not None:
//             with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
//                 bh.write_register(0x04, value)
//             return True
//         else:
//             return False

// ------------------------------------------------------------------------
// temperature measurements - internal temperature sensor
// ------------------------------------------------------------------------

/// read the temperature measured by the internal sensor (in °C)
///  - the data sheet guarantees a precision of ±2°C
///
/// expected range: 0x00 (0ºC) to 0x55 (85ºC)
pub fn get_internal_temperature<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> u8
where
    Dm: esp_hal::DriverMode,
{
    // implicit return
    read_register_as_u8(i2c_bus, DR::Its as u8)
}

/// read the "high temperature" alerting limit
///
/// expected range: 0x00 (0.0ºC) to 0x55 (85.0ºC)
/// default: 0x46 (70.0°C)
pub fn get_internal_temperature_high_limit<Dm>(
    i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>,
) -> u8
where
    Dm: esp_hal::DriverMode,
{
    // implicit return
    read_register_as_u8(i2c_bus, DR::ItsHi as u8)
}

/// set the "high temperature" alerting limit
///
/// expected range: 0x00 (0.0ºC) to 0x55 (85.0ºC)
pub fn set_internal_temperature_high_limit<Dm>(
    i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>,
    limit: u8,
) -> bool
where
    Dm: esp_hal::DriverMode,
{
    if limit <= 85 {
        write_register_as_u8(i2c_bus, DR::ItsHi as u8, limit);
        // implicit return
        true
    } else {
        // implicit return
        false
    }
}

// ------------------------------------------------------------------------
// temperature measurements - external temperature sensor
// ------------------------------------------------------------------------

//     def configure_ets(self, ets_config: ExternalTemperatureSensorConfig) -> bool:
//         """
//         configure diode_ideality_factor and beta_compensation_factor of
//         the external temperature sensor
//         """
//         dif = ets_config.diode_ideality_factor
//         bcf = ets_config.beta_compensation_factor
//         with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
//             dev_status = bh.read_register(0x02)
//             if not dev_status & 0b0000_0100:
//                 LH.debug("The diode fault bit is clear.")
//                 bh.write_register(0x17, dif)
//                 bh.write_register(0x18, bcf)
//                 return True
//             else:
//                 LH.error("The diode fault bit is set: Sensor is faulty or missing.")
//                 return False

//     def get_ets_state(self) -> ExternalSensorStatus:
//         # The status register 0x02 has a diode fault bit but that bit is
//         # set only if there is an open circuit between DP-DN.
//         # (It is NOT set if there is a short circuit between DP-DN.)
//         with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
//             msb = bh.read_register(0x01)  # high byte, must be read first!
//             lsb = bh.read_register(0x10)  # low byte
//         if msb != 0b0111_1111:
//             return ExternalSensorStatus.OK
//         else:
//             if lsb == 0b0000_0000:
//                 return ExternalSensorStatus.FAULT1
//             elif lsb == 0b1110_0000:
//                 return ExternalSensorStatus.FAULT2
//             else:
//                 raise RuntimeError(f"unexpected external sensor state (msb: 0x{msb:02X} lsb:0x{lsb:02X})")

//     def has_ets(self) -> bool:
//         # The EMC2101 has a fault bit in the status register (0x02) but
//         # that bit is set only if there is an open circuit between DP-DN
//         # or if it's shorted to VDD. The bit is not set if there is a
//         # short circuit between DP-DN or to ground.
//         # -> read the temperature MSB instead
//         with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
//             return bh.read_register(0x01) != 0b0111_1111

/// read the temperature measured by the external sensor
//  - the data sheet guarantees a precision of ±1°C
///
/// expected range: [0x00, 0x00] (0.0ºC) to [0x55, 0x00] (85.0ºC)
pub fn get_external_temperature<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>) -> [u8; 2]
where
    Dm: esp_hal::DriverMode,
{
    let adr = [
        DR::EtsMsb as u8, // high byte, must be read first!
        DR::EtsLsb as u8, // low byte
    ];

    // implicit return
    read_multibyte_register_as_u8(i2c_bus, adr)
}

/// read the "low temperature" alerting limit
///
/// expected range: [0x00, 0x00] (0.0ºC) to [0x55, 0x00] (85.0ºC)
/// default: [0x00, 0x00] (0.0°C)
pub fn get_external_temperature_low_limit<Dm>(
    i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>,
) -> [u8; 2]
where
    Dm: esp_hal::DriverMode,
{
    let adr = [
        DR::EtsLoMsb as u8, // high byte, must be read first!
        DR::EtsLoLsb as u8, // low byte
    ];

    // implicit return
    read_multibyte_register_as_u8(i2c_bus, adr)
}

/// change the "low temperature" alerting limit
///
/// expected range: [0x00, 0x00] (0.0ºC) to [0x55, 0x00] (85.0ºC)
pub fn set_external_temperature_low_limit<Dm>(
    i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>,
    bytes: [u8; 2],
) where
    Dm: esp_hal::DriverMode,
{
    let values = [
        [DR::EtsLoMsb as u8, bytes[0]], // high byte
        [DR::EtsLoLsb as u8, bytes[1]], // low byte
    ];
    write_multibyte_register_as_u8(i2c_bus, values);
}

/// read the "high temperature" alerting limit
///
/// expected range: [0x00, 0x00] (0.0ºC) to [0x55, 0x00] (85.0ºC)
/// default: [0x46, 0x00] (70.0°C)
pub fn get_external_temperature_high_limit<Dm>(
    i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>,
) -> [u8; 2]
where
    Dm: esp_hal::DriverMode,
{
    let adr = [
        DR::EtsHiMsb as u8, // high byte, must be read first!
        DR::EtsHiLsb as u8, // low byte
    ];

    // implicit return
    read_multibyte_register_as_u8(i2c_bus, adr)
}

/// change the "high temperature" alerting limit
///
/// expected range: [0x00, 0x00] (0.0ºC) to [0x55, 0x00] (85.0ºC)
pub fn set_external_temperature_high_limit<Dm>(
    i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>,
    bytes: [u8; 2],
) where
    Dm: esp_hal::DriverMode,
{
    let values = [
        [DR::EtsHiMsb as u8, bytes[0]], // high byte
        [DR::EtsHiLsb as u8, bytes[1]], // low byte
    ];
    write_multibyte_register_as_u8(i2c_bus, values);
}

/// trigger a temperature conversion ('one shot')
/// - device must be in standby mode
/// - does nothing in 'continuous conversion' mode)
pub fn trigger_one_shot<Dm>(i2c_bus: &mut esp_hal::i2c::master::I2c<'_, Dm>)
where
    Dm: esp_hal::DriverMode,
{
    // the write operation is the important part
    // (the data value is irrelevant and ignored)

    // implicit return
    write_register_as_u8(i2c_bus, DR::OneShot as u8, 0x00)
}

//     def force_temperature(self, temperature: float):
//         """
//         force external sensor to read a specific temperature

//         (this is useful to debug the lookup table)
//         """
//         with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
//             # write to register
//             bh.write_register(0x0C, round(temperature))
//             # force chip take readings from register instead of sensor
//             fan_config = bh.read_register(0x4A)
//             bh.write_register(0x4A, fan_config | 0b0100_0000)

//     def clear_temperature(self):
//         """
//         clear a previously forced temperature reading
//         """
//         with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
//             # stop reading from register
//             fan_config = bh.read_register(0x4A)
//             bh.write_register(0x4A, fan_config & 0b1011_1111)
//             # reset register to default state
//             bh.write_register(0x0C, 0x00)

//     # ---------------------------------------------------------------------
//     # convenience functions
//     # ---------------------------------------------------------------------

//     def read_fancfg_register(self) -> int:
//         # described in datasheet section 6.16 "Fan Configuration Register"
//         # 0b00000000
//         #         ^^-- tachometer input mode
//         #        ^---- clock frequency override
//         #       ^----- clock select
//         #      ^------ polarity (0 = 100->0, 1 = 0->100)
//         #     ^------- configure lookup table (0 = on, 1 = off)
//         with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
//             return bh.read_register(0x4A)

//     def write_fancfg_register(self, value: int):
//         # described in datasheet section 6.16 "Fan Configuration Register"
//         # 0b00000000
//         #         ^^-- tachometer input mode
//         #        ^---- clock frequency override
//         #       ^----- clock select
//         #      ^------ polarity (0 = 100->0, 1 = 0->100)
//         #     ^------- configure lookup table (0 = on, 1 = off)
//         with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
//             bh.write_register(0x4A, value & 0xFF)

//     def _uses_tacho_mode(self) -> bool:
//         with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
//             status_register = bh.read_register(0x03)
//         return bool(status_register & 0b0000_0100)

// def _convert_rpm2tach(rpm: int) -> tuple[int, int]:
//     # due to the way the conversion works the RPM can never
//     # be less than 82
//     if rpm < 82:
//         raise ValueError("RPM can't be lower than 82")
//     tach = int(5_400_000 / rpm)
//     tach = 4096
//     msb = (tach & 0xFF00) >> 8
//     lsb = tach & 0x00FF
//     return (msb, lsb)
