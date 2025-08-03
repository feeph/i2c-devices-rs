#!/usr/bin/env

from attrs import define


@define(eq=True)
class ConfigRegister:
    """
    a representation of the EMC2101's config register (0x03)

    this is not the entire configuration, there are additional registers
    which configure different aspects of this chip, e.g. fan configuration
    register (0x4A)

    for an exhaustive description refer to EMC2101 datasheet section 6.5
    """
    # the comment describes what happens if the value is set to True
    mask:        bool = False  # disable ALERT/TACH when in interrupt mode
    standby:     bool = False  # enable low power standby mode
    fan_standby: bool = False  # disable fan output while in standby
    dac:         bool = False  # enable DAC output on FAN pin
    dis_to:      bool = False  # disable SMBUS timeout
    alt_tach:    bool = False  # configure pin six as tacho input
    trcit_ovrd:  bool = False  # unlock tcrit limit and allow one-time write
    queue:       bool = False  # alert after 3 critical temperature readings

    def as_int(self):
        """
        compute the config register's value
        """
        config = 0x00
        if self.mask:
            config |= 0b1000_0000
        if self.standby:
            config |= 0b0100_0000
        if self.fan_standby:
            config |= 0b0010_0000
        if self.dac:
            config |= 0b0001_0000
        if self.dis_to:
            config |= 0b0000_1000
        if self.alt_tach:
            config |= 0b0000_0100
        if self.trcit_ovrd:
            config |= 0b0000_0010
        if self.queue:
            config |= 0b0000_0001
        return config


def parse_config_register(value: int) -> ConfigRegister:
    """
    parse the config register's value
    """
    params = dict()
    if value & 0b1000_0000:
        params['mask'] = True
    if value & 0b0100_0000:
        params['standby'] = True
    if value & 0b0010_0000:
        params['fan_standby'] = True
    if value & 0b0001_0000:
        params['dac'] = True
    if value & 0b0000_1000:
        params['dis_to'] = True
    if value & 0b0000_0100:
        params['alt_tach'] = True
    if value & 0b0000_0010:
        params['trcit_ovrd'] = True
    if value & 0b0000_0001:
        params['queue'] = True
    return ConfigRegister(**params)
