#!/usr/bin/env python3
"""
low-level interface to the EMC2101 chip

datasheet: https://ww1.microchip.com/downloads/en/DeviceDoc/2101.pdf

You probably don't want to use this one. Use Emc2101_DAC / Emc2101_PWM instead.
"""

import logging
import math
from enum import Enum

# module busio provides no type hints
import busio  # type: ignore
from feeph.i2c import BurstHandle, BurstHandler

from feeph.emc2101.config_register import ConfigRegister, parse_config_register
from feeph.emc2101.conversions import convert_bytes2temperature, convert_temperature2bytes
from feeph.emc2101.ets_config import ExternalTemperatureSensorConfig

LH = logging.getLogger('feeph.emc2101')


DEFAULTS = {
    #     value           purpose                             section
    # -----------------------------------------------------------------
    0x03: 0b0000_0000,  # configuration register               6.5
    0x04: 0b0000_1000,  # conversion register                  6.6
    # temperature limit registers                              6.7
    0x05: 0b0100_0110,  # internal sensor
    0x07: 0b0100_0110,  # external diode, high limit, MSB
    0x08: 0b0000_0000,  # external diode, low limit,  MSB
    0x13: 0b0000_0000,  # external diode, high limit, LSB
    0x14: 0b0000_0000,  # external diode, low limit,  LSB
    0x19: 0b0101_0101,  # critical temperature threshold
    0x21: 0b0000_1010,  # critical temperature hysteresis
    # -------------------------
    0x0C: 0b0000_0000,  # external temperature (forced)        6.8
    0x11: 0b0000_0000,  # scratchpad #1                        6.10
    0x12: 0b0000_0000,  # scratchpad #2                        6.10
    0x16: 0b1010_0100,  # alert mask                           6.11
    0x17: 0b0001_0010,  # external ideality                    6.12
    0x18: 0b0000_1000,  # beta compensation                    6.13
    0x48: 0b1111_1111,  # tach limit, LSB                      6.15
    0x49: 0b1111_1111,  # tach limit, MSB                      6.15
    0x4A: 0b0010_0000,  # fan configuration                    6.16
    0x4B: 0b0011_1111,  # fan spinup configuration             6.17
    0x4C: 0b0000_0000,  # fan setting                          6.18
    0x4D: 0b0001_0111,  # pwm frequency                        6.19
    0x4E: 0b0000_0001,  # pwm frequency divide                 6.20
    0x4F: 0b0000_0100,  # fan control lookup table hysteresis  6.21
    # fan control lookup table (0x50..0x5F)                    6.22
    # -------------------------
    0xBF: 0b0000_0000,  # averaging filter                     6.23
}


# number of temperature conversions per second
CONVERSIONS_PER_SECOND = {
    "1/16": 0b0000,
    "1/8":  0b0001,
    "1/4":  0b0010,
    "1/2":  0b0011,
    "1":    0b0100,
    "2":    0b0101,
    "4":    0b0110,
    "8":    0b0111,
    "16":   0b1000,
    "32":   0b1001,  # and all unlisted values
}


class ExternalSensorStatus(Enum):
    OK = "all good"
    FAULT1 = "open circuit or short to VDD"
    FAULT2 = "short circuit or short to GND"


class SpinUpStrength(Enum):
    BYPASS       = 0b0000_0000  # bypass spin-up cycle
    STRENGTH_50  = 0b0000_1000  # drive at 50% speed
    STRENGTH_75  = 0b0001_0000  # drive at 75% speed
    STRENGTH_100 = 0b0001_1000  # drive at 100% speed (default)


class SpinUpDuration(Enum):
    TIME_0_00 = 0b0000_0000  # bypass spin-up cycle
    TIME_0_05 = 0b0000_0001  # 0.05s
    TIME_0_10 = 0b0000_0010  # 0.10s
    TIME_0_20 = 0b0000_0011  # 0.20s
    TIME_0_40 = 0b0000_0100  # 0.40s
    TIME_0_80 = 0b0000_0101  # 0.80s
    TIME_1_60 = 0b0000_0110  # 1.60s
    TIME_3_20 = 0b0000_0111  # 3.20s (default)


class Emc2101:
    """
    low-level interface to the EMC2101 chip

    You probably don't want to use this one. Use Emc2101_DAC / Emc2101_PWM instead.
    """

    def __init__(self, i2c_bus: busio.I2C, config: ConfigRegister):
        """
        initialize the object

        Make sure to configure pin 6 and the control mode before use:
          - config.alt_tach
          - config.dac

        These settings MUST match the electric circuit!If you don't set
        these values correctly you won't get sensible readings!
        """
        self._i2c_bus = i2c_bus
        self._i2c_adr = 0x4c  # the I²C bus address is hardcoded
        self.set_config_register(config)
        # allowed steps can be lower if PWM is used
        self._step_min = 0
        self._step_max = 63
        # minimum and maximum operation temperature
        # TODO reduce self._temp_max to 85
        self._temp_min = 0
        self._temp_max = 100

    def describe_device(self):
        with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
            manufacturer_id = bh.read_register(0xFE)
            product_id = bh.read_register(0xFD)
            product_revision = bh.read_register(0xFF)
        manufacturer_name = MANUFACTURER_IDS.get(manufacturer_id, "<unknown manufacturer>")
        product_name      = PRODUCT_IDS.get(product_id, "<unknown product>")
        return f"{manufacturer_name} (0x{manufacturer_id:02X}) {product_name} (0x{product_id:02X}) (rev: {product_revision})"

    # ---------------------------------------------------------------------
    # fan speed control
    # ---------------------------------------------------------------------

    def get_config_register(self) -> ConfigRegister:
        with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
            return _get_config_register(bh)

    def set_config_register(self, config: ConfigRegister):
        with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
            _set_config_register(bh, config)

    def configure_pwm_control(self, pwm_d: int, pwm_f: int, step_max: int) -> bool:
        with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
            # enable PWM control
            if _get_config_register(bh).dac:
                LH.warning("Unable to use PWM! Device is configured for direct current control.")
                return False
            # configure pwm frequency divider settings
            bh.write_register(0x4D, pwm_f)
            bh.write_register(0x4E, pwm_d)
            # configure maximum allowed step
            self._step_max = step_max
            return True

    def configure_spinup_behaviour(self, spinup_strength: SpinUpStrength, spinup_duration: SpinUpDuration, fast_mode: bool) -> bool:
        """
        configure the spin-up behavior for the attached fan (duration and
        strength). This helps to ensure the fan has sufficient power
        available to be able to start spinning the rotor.
         - EMC2101 enters the spin-up routine any time it transitions
           from a minimum fan setting (00h) to a higher fan setting
         - EMC2101 does not invoke the spin-up routine upon power up
         - setting a strength of 0% or duration of 0s disables spin-up entirely

        Once spin-up has completed the fan speed is reduced to the programmed setting.

        Please note: Fast_mode is ignored if pin 6 is in alert mode.
        """
        with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
            config = _get_config_register(bh)
            if config.alt_tach:
                # pin 6 is configured as tacho pin
                value = 0x00
                # configure spin up time
                value |= spinup_duration.value
                # configure spin up strength (dutycycle)
                value |= spinup_strength.value
                if fast_mode:
                    value |= 0b0010_0000
                bh.write_register(0x4B, value)
                return True
            else:
                # pin 6 is configured as alert pin
                LH.warning("Pin 6 is in alert mode. Can't configure spinup behavior.")
                return False

    def configure_minimum_rpm(self, minimum_rpm: int):
        """
        configure the expected minimum RPM value

        if the measured RPM is below this RPM the fan is considered to be
        not spinning and the TACH bit is set

        due to the way the RPM is measured the lowest possible value is 82 RPM
        """
        (msb, lsb) = _convert_rpm2tach(minimum_rpm)
        with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
            bh.write_register(0x48, lsb)  # TACH Limit Low Byte
            bh.write_register(0x49, msb)  # TACH Limit High Byte

    def get_rpm(self) -> int | None:
        """
        get current fan speed

        (pin 6 must be configured for tacho mode)
        """
        if self._uses_tacho_mode():
            # get tacho readings
            # (the order of is important; see datasheet section 6.1 for details)
            with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
                lsb = bh.read_register(0x46)  # TACH Reading Low Byte, must be read first!
                msb = bh.read_register(0x47)  # TACH Reading High Byte
            LH.debug("tach readings: LSB=0x%02X MSB=0x%02X", lsb, msb)
            return _convert_tach2rpm(msb=msb, lsb=lsb)
        else:
            LH.warning("Pin six is not configured for tacho mode. Please enable tacho mode.")
            return None

    def get_driver_strength(self) -> int:
        """
        get the configured fan speed (raw value)
        """
        with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
            return bh.read_register(0x4C)

    def set_driver_strength(self, step: int, disable_lut: bool = False) -> bool:
        """
        set the configured fan speed (raw value)
         - clamp to minimum/maximum as defined by the fan configuration
        """
        with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
            if self._step_min <= step <= self._step_max:
                bh.write_register(0x4C, step)
            # confirm the register was set to desired value
            return step == bh.read_register(0x4C)

    def enable_lookup_table(self) -> bool:
        """
        The Fan Setting register (0x4C) and Fan Control Look-Up Table
        registers (0x50-0x5F) are writeable and the Fan Setting
        register will be used.

        An external temperature sensor must be connected to use this feature.
        """
        with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
            value = bh.read_register(0x4A)
            bh.write_register(0x4A, value & 0b1101_1111)
        return True

    def disable_lookup_table(self):
        """
        the Fan Setting register (0x4C) and Fan Control Look-Up Table
        registers (0x50-0x5F) are read-only and the Fan Control Look-Up
        Table registers will be used.
        """
        with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
            value = bh.read_register(0x4A)
            bh.write_register(0x4A, value | 0b0010_0000)

    def is_lookup_table_enabled(self) -> bool:
        with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
            return not bh.read_register(0x4A) & 0b0010_0000

    def update_lookup_table(self, values: dict[int, int]) -> bool:
        """
        populate the lookup table with the provided values and
        sets all unused values to zero

        returns 'True' if the lookup table was updated and 'False' if it wasn't.
        """
        if len(values) > 8:
            raise ValueError("too many entries in lookup table (max: 8)")
        for temp, step in values.items():
            if not self._temp_min <= temp <= self._temp_max:
                raise ValueError("temperature is out of range")
            if not self._step_min <= step <= self._step_max:
                raise ValueError("step is out of range")
        # -------------------------------------------------------------
        # must disable lookup table to make it writeable
        if self.is_lookup_table_enabled():
            LH.error("Lookup table is enabled. Disabling.")
            self.disable_lookup_table()
            reenable_lut = True
        else:
            LH.error("Lookup table is not enabled. Good.")
            reenable_lut = False
        # 0x50..0x5f (8 x 2 registers; temp->step)
        with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
            offset = 0
            # set provided value
            for temp, step in values.items():
                bh.write_register(0x50 + offset, temp)
                bh.write_register(0x51 + offset, step)
                offset += 2
            # fill remaining slots
            for offset in range(offset, 16, 2):
                bh.write_register(0x50 + offset, 0x00)
                bh.write_register(0x51 + offset, 0x00)
        # reenable lookup table if it was previously enabled
        if reenable_lut:
            self.enable_lookup_table()
        return True

    def reset_lookup_table(self):
        # must disable lookup table to make it writeable
        self.disable_lookup_table()
        # set all slots to zero
        with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
            for offset in range(0, 16, 2):
                bh.write_register(0x50 + offset, 0x00)
                bh.write_register(0x51 + offset, 0x00)

    # ---------------------------------------------------------------------
    # temperature measurements
    # ---------------------------------------------------------------------

    def get_temperature_conversion_rate(self) -> str:
        """
        get the number of temperature conversions per second
        """
        with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
            value = bh.read_register(0x04)
        value = min(value, 0b1001)  # all values larger than 0b1001 map to 0b1001
        return [k for k, v in CONVERSIONS_PER_SECOND.items() if v == value][0]

    def get_temperature_conversion_rates(self) -> list[str]:
        """
        returns all available temperature conversion rates
        """
        return list(CONVERSIONS_PER_SECOND.keys())

    def set_temperature_conversion_rate(self, conversion_rate: str) -> bool:
        """
        set the number of temperature conversions per second
        """
        value = CONVERSIONS_PER_SECOND.get(conversion_rate)
        if value is not None:
            with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
                bh.write_register(0x04, value)
            return True
        else:
            return False

    # ---------------------------------------------------------------------
    # temperature measurements - internal temperature sensor
    # ---------------------------------------------------------------------

    def get_its_temperature(self) -> float:
        """
        get internal sensor temperature in °C

        the datasheet guarantees a precision of ±2°C
        """
        with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
            LH.error("get_its_temperature(): %0.1f", bh.read_register(0x00))
            return float(bh.read_register(0x00))

    def get_its_temperature_limit(self) -> float:
        with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
            return float(bh.read_register(0x05))

    def set_its_temperature_limit(self, value: float):
        with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
            bh.write_register(0x05, int(value))

    # ---------------------------------------------------------------------
    # temperature measurements - external temperature sensor
    # ---------------------------------------------------------------------

    def configure_ets(self, ets_config: ExternalTemperatureSensorConfig) -> bool:
        """
        configure diode_ideality_factor and beta_compensation_factor of
        the external temperature sensor
        """
        dif = ets_config.diode_ideality_factor
        bcf = ets_config.beta_compensation_factor
        with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
            dev_status = bh.read_register(0x02)
            if not dev_status & 0b0000_0100:
                LH.debug("The diode fault bit is clear.")
                bh.write_register(0x17, dif)
                bh.write_register(0x18, bcf)
                return True
            else:
                LH.error("The diode fault bit is set: Sensor is faulty or missing.")
                return False

    def get_ets_state(self) -> ExternalSensorStatus:
        # The status register 0x02 has a diode fault bit but that bit is
        # set only if there is an open circuit between DP-DN.
        # (It is NOT set if there is a short circuit between DP-DN.)
        with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
            msb = bh.read_register(0x01)  # high byte, must be read first!
            lsb = bh.read_register(0x10)  # low byte
        if msb != 0b0111_1111:
            return ExternalSensorStatus.OK
        else:
            if lsb == 0b0000_0000:
                return ExternalSensorStatus.FAULT1
            elif lsb == 0b1110_0000:
                return ExternalSensorStatus.FAULT2
            else:
                raise RuntimeError(f"unexpected external sensor state (msb: 0x{msb:02X} lsb:0x{lsb:02X})")

    def has_ets(self) -> bool:
        # The EMC2101 has a fault bit in the status register (0x02) but
        # that bit is set only if there is an open circuit between DP-DN
        # or if it's shorted to VDD. The bit is not set if there is a
        # short circuit between DP-DN or to ground.
        # -> read the temperature MSB instead
        with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
            return bh.read_register(0x01) != 0b0111_1111

    def get_ets_temperature(self) -> float:
        """
        get external sensor temperature in °C

        the datasheet guarantees a precision of ±1°C
        """
        with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
            msb = bh.read_register(0x01)  # high byte, must be read first!
            lsb = bh.read_register(0x10)  # low byte
        if msb != 0b0111_1111:
            return convert_bytes2temperature(msb, lsb)
        else:
            return math.nan

    def get_ets_low_temperature_limit(self) -> float:
        """
        get upper/lower temperature alerting limit in °C
        """
        with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
            msb = bh.read_register(0x08)  # high byte, must be read first!
            lsb = bh.read_register(0x14)  # low byte
        return convert_bytes2temperature(msb, lsb)

    def set_ets_low_temperature_limit(self, value: float) -> float:
        """
        set upper/lower temperature alerting limit in °C

        The fractional part has limited precision and will be clamped to the
        nearest available step. The clamped value is returned to the caller.
        """
        if self._temp_min <= value <= self._temp_max:
            (msb, lsb) = convert_temperature2bytes(value)
            with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
                bh.write_register(0x08, msb)
                bh.write_register(0x14, lsb)
            return convert_bytes2temperature(msb, lsb)
        else:
            raise ValueError(f"temperature limit out of range ({self._temp_min} ≤ x ≤ {self._temp_max}°C)")

    def get_ets_high_temperature_limit(self) -> float:
        """
        get upper/lower temperature alerting limit in °C
        """
        with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
            msb = bh.read_register(0x07)  # high byte, must be read first!
            lsb = bh.read_register(0x13)  # low byte
        return convert_bytes2temperature(msb, lsb)

    def set_ets_high_temperature_limit(self, value: float) -> float:
        """
        set upper/lower temperature alerting limit in °C

        The fractional part has limited precision and will be clamped to the
        nearest available step. The clamped value is returned to the caller.
        """
        if self._temp_min <= value <= self._temp_max:
            (msb, lsb) = convert_temperature2bytes(value)
            with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
                bh.write_register(0x07, msb)
                bh.write_register(0x13, lsb)
            return convert_bytes2temperature(msb, lsb)
        else:
            raise ValueError("temperature limit out of range (0 ≤ x ≤ 85°C)")

    def force_temperature_conversion(self):
        """
        performs a one-shot conversion
        """
        with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
            bh.write_register(0x0F, 0x00)

    def force_temperature(self, temperature: float):
        """
        force external sensor to read a specific temperature

        (this is useful to debug the lookup table)
        """
        with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
            # write to register
            bh.write_register(0x0C, round(temperature))
            # force chip take readings from register instead of sensor
            fan_config = bh.read_register(0x4A)
            bh.write_register(0x4A, fan_config | 0b0100_0000)

    def clear_temperature(self):
        """
        clear a previously forced temperature reading
        """
        with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
            # stop reading from register
            fan_config = bh.read_register(0x4A)
            bh.write_register(0x4A, fan_config & 0b1011_1111)
            # reset register to default state
            bh.write_register(0x0C, 0x00)

    # ---------------------------------------------------------------------
    # convenience functions
    # ---------------------------------------------------------------------

    def read_fancfg_register(self) -> int:
        # described in datasheet section 6.16 "Fan Configuration Register"
        # 0b00000000
        #         ^^-- tachometer input mode
        #        ^---- clock frequency override
        #       ^----- clock select
        #      ^------ polarity (0 = 100->0, 1 = 0->100)
        #     ^------- configure lookup table (0 = on, 1 = off)
        with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
            return bh.read_register(0x4A)

    def write_fancfg_register(self, value: int):
        # described in datasheet section 6.16 "Fan Configuration Register"
        # 0b00000000
        #         ^^-- tachometer input mode
        #        ^---- clock frequency override
        #       ^----- clock select
        #      ^------ polarity (0 = 100->0, 1 = 0->100)
        #     ^------- configure lookup table (0 = on, 1 = off)
        with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
            bh.write_register(0x4A, value & 0xFF)

    def read_device_registers(self) -> dict[int, int]:
        registers = {}
        with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
            for register in DEFAULTS:
                registers[register] = bh.read_register(register)
        return registers

    def reset_device_registers(self):
        LH.debug("Resetting all device registers to their default values.")
        with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
            for register, value in DEFAULTS.items():
                bh.write_register(register, value)

    def _uses_tacho_mode(self) -> bool:
        with BurstHandler(i2c_bus=self._i2c_bus, i2c_adr=self._i2c_adr) as bh:
            status_register = bh.read_register(0x03)
        return bool(status_register & 0b0000_0100)


def _convert_rpm2tach(rpm: int) -> tuple[int, int]:
    # due to the way the conversion works the RPM can never
    # be less than 82
    if rpm < 82:
        raise ValueError("RPM can't be lower than 82")
    tach = int(5_400_000 / rpm)
    tach = 4096
    msb = (tach & 0xFF00) >> 8
    lsb = tach & 0x00FF
    return (msb, lsb)


def _convert_tach2rpm(msb: int, lsb: int) -> int | None:
    """
    convert the raw values to an RPM value

    returns 'None' if the reading is invalid
    """
    tach = (msb << 8) + lsb
    # 0xFFFF = invalid value
    if tach < 0xFFFF:
        rpm = int(5_400_000 / tach)
        return rpm
    else:
        return None


def _get_config_register(bh: BurstHandle) -> ConfigRegister:
    return parse_config_register(bh.read_register(0x03))


def _set_config_register(bh: BurstHandle, config: ConfigRegister):
    bh.write_register(0x03, config.as_int())
