#!/usr/bin/env python3
"""
a reimplementation of https://github.com/adafruit/Adafruit_CircuitPython_EMC2101

The main design goal is to hide as many low-level details as possible while
making the code comprehensible. Additionally we want to leverage as much
tool-based development support as possible.
"""

# typical usage scenarios
# =======================

# calibrate an unknown PWM fan
# -> automatically identify the fan's parameters and create a fan profile
# -------------------------------------------------------------------------
# import board
# import busio
# import yaml
#
# from feeph.emc2101 import calibrate_pwm_fan, export_fan_config
#
# i2c_bus = busio.I2C(scl=board.SCL, sda=board.SDA)
# fan_profile = calibrate_pwm_fan(i2c_bus=i2c_bus, model="My PWM Fan")
#
# with open('my_pwm_fan.yaml', 'w') as fh:
#   yaml.dump(export_fan_config(fan_profile), fh)
# -------------------------------------------------------------------------

# control a PWM fan
# -> load a fan profile and manage the fan
# -------------------------------------------------------------------------
# import board
# import busio
# import yaml
#
# from feeph.emc2101 import DeviceConfig, Emc2101_PWM, PinSixMode, RpmControlMode
#
# with open('my_pwm_fan.yaml', 'r') as fh:
#   fan_profile = import_fan_config(yaml.load(fh))
#
# i2c_bus = busio.I2C(scl=board.SCL, sda=board.SDA)
# device_config = DeviceConfig(pin_six_mode=PinSixMode.TACHO, rpm_control_mode=RpmControlMode.PWM)
# emc2101 = Emc2101_PWM(i2c_bus=i2c_bus, device_config=device_config, fan_config=fan_profile)
# print("RPM:", emc2101.get_rpm()
# -------------------------------------------------------------------------

# the following imports are provided for user convenience
# flake8: noqa: F401
from feeph.emc2101.calibration import calibrate_pwm_fan
from feeph.emc2101.config_register import ConfigRegister
from feeph.emc2101.core import CONVERSIONS_PER_SECOND, DEFAULTS, ExternalSensorStatus, SpinUpDuration, SpinUpStrength
from feeph.emc2101.ets_config import ExternalTemperatureSensorConfig, ets_2n3904, ets_2n3906
from feeph.emc2101.fan_configs import FanConfig, RpmControlMode, Steps, export_fan_config, generic_pwm_fan
from feeph.emc2101.pwm import DeviceConfig, Emc2101_PWM, FanSpeedUnit, PinSixMode, emc2101_default_config
