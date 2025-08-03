#!/usr/bin/env python3

import logging
import math

LH = logging.getLogger('feeph.emc2101')


def calculate_pwm_factors(pwm_frequency: int) -> tuple[int, int]:
    """
    calculate PWM_D and PWM_F for provided frequency
     - this function minimizes PWM_D to allow for maximum resolution (PWM_F)
     - PWM_F maxes out at 31 (0x1F)
    """
    if 0 <= pwm_frequency <= 180000:
        value1 = 360000 / (2 * pwm_frequency)
        pwm_d = math.ceil(value1 / 31)
        pwm_f = round(value1 / pwm_d)
        return (pwm_d, pwm_f)
    else:
        raise ValueError("provided frequency is out of range")
