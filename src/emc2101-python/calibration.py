#!/usr/bin/env python3

import logging
import time

# module busio provides no type hints
import busio  # type: ignore

import feeph.emc2101.core
import feeph.emc2101.utilities
from feeph.emc2101.fan_configs import FanConfig, RpmControlMode, Steps

LH = logging.getLogger('feeph.emc2101')

SLEEP_TIME1 = 5.0
SLEEP_TIME2 = 0.5


# This function has limited code coverage since it depends on active
# feedback from the underlying device. We will need a mock to be able
# to test the missing lines of code.
def calibrate_pwm_fan(i2c_bus: busio.I2C, model: str, pwm_frequency: int = 22500) -> FanConfig | None:
    """
    walk through various settings and determine the fan's configuration
    parameters
    """
    LH.info("Calibrating fan parameters.")
    pwm_d, pwm_f = feeph.emc2101.utilities.calculate_pwm_factors(pwm_frequency=pwm_frequency)
    steps_list = list(range(pwm_f * 2))
    # tacho signal on pin 6, device uses PWM control
    config = feeph.emc2101.core.ConfigRegister(alt_tach=True, dac=False)
    emc2101 = feeph.emc2101.core.Emc2101(i2c_bus=i2c_bus, config=config)
    emc2101.configure_pwm_control(pwm_d=pwm_d, pwm_f=pwm_f, step_max=max(steps_list))
    # -----------------------------------------------------------------
    LH.debug("Disabling gradual speed rampup.")
    # TODO disable gradual rampup
    # TODO set initial driver strength to 100%
    # -----------------------------------------------------------------
    LH.info("Testing if fan responds to PWM signal:")
    LH.debug("speed control steps: %s", steps_list)
    if len(steps_list) <= 2:
        LH.warning("Fan does not have enough steps to calibrate!")
        return None
    step1 = steps_list[int(len(steps_list) / 2)]  # pick something in the middle
    step2 = steps_list[-2]                        # pick the second highest possible setting
    emc2101.set_driver_strength(step1)
    time.sleep(SLEEP_TIME1)
    dutycycle1 = int(step1 * 100 / len(steps_list))
    rpm1 = emc2101.get_rpm()
    LH.debug("dutycycle: %i%% -> RPM: %i", dutycycle1, rpm1)
    emc2101.set_driver_strength(step2)
    time.sleep(SLEEP_TIME1)
    dutycycle2 = int(step2 * 100 / len(steps_list))
    rpm2 = emc2101.get_rpm()
    LH.debug("dutycycle: %i%% -> RPM: %i", dutycycle2, rpm2)
    if rpm1 is None or rpm2 is None:
        LH.error("Unable to get a reliable RPM reading. Aborting.")
        return None
    if rpm1 * 100 / rpm2 < 96:
        LH.info("Yes, it does. Observed an RPM change in response to PWM signal. (%i%%: %i -> %i%%: %i RPM)", dutycycle1, rpm1, dutycycle2, rpm2)
    else:
        LH.warning("Failed to observe a significant speed change in response to PWM signal! Aborting.")
        LH.warning("Please verify wiring and configuration.")
        return None
    # -----------------------------------------------------------------
    LH.info("Mapping PWM dutycycle to RPM. Please wait.")
    mappings = list()
    for step in steps_list:
        dutycycle = int(step * 100 / len(steps_list))
        # set fan speed and wait for the speed to settle
        emc2101.set_driver_strength(step)
        time.sleep(SLEEP_TIME2)
        readings = [99999, 99999, 99999]
        for i in range(24):
            cursor = i % len(readings)
            rpm_cur = emc2101.get_rpm()
            if rpm_cur is not None:
                # order is important! (update readings before calculating the average)
                readings[cursor] = rpm_cur
                rpm_avg = sum(readings) / len(readings)
                # calculate deviation from average
                deviation = rpm_cur / rpm_avg
                LH.debug("step: %2i i: %2i -> rpm: %4i deviation: %3.2f", step, cursor, rpm_cur, deviation)
                if 0.99 <= deviation <= 1.01:
                    # RPM will never be exact and fluctuates slightly
                    # -> round to nearest factor of 5
                    rpm = round(rpm_avg / 5) * 5
                    LH.debug("Fan has settled: (step: %i -> dutycycle: %3i%%, rpm: %i)", step, dutycycle, rpm)
                    mappings.append((step, dutycycle, rpm))
                    break
                else:
                    time.sleep(SLEEP_TIME2)
            else:
                LH.error("Unable to get a reliable RPM reading. Aborting.")
                return None
        else:
            LH.warning("Fan never settled! (step: %i -> dutycycle: %3i%%, rpm: <n/a>)", step, dutycycle)
            mappings.append((step, dutycycle, rpm))

    # determine maximum RPM
    rpm_max = max([rpm for (_, _, rpm) in mappings])
    LH.info("Maximum RPM: %i", rpm_max)

    # prune steps
    #  - multiple steps may result in the same RPM (e.g. minimum RPM)
    #  - ensure each step is significantly different from the previous
    #  - ensure each step increases RPM
    prune = list()
    rpm_delta_min = rpm_max * 0.011
    for i in range(len(mappings) - 1):
        step, _, rpm_this = mappings[i]
        _, _, rpm_next = mappings[i + 1]
        if rpm_this + rpm_delta_min <= rpm_next:
            # significantly different from next element -> keep it
            pass
        else:
            # within range of next element -> prune it
            prune.append(step)

    steps: Steps = dict()
    for step, dutycycle, rpm in mappings:
        rpm_percent = rpm * 100 / rpm_max
        LH.info("step: %2i dutycycle: %3i%% -> RPM: %5i (%3.0f%%)", step, dutycycle, rpm, rpm_percent)
        if step not in prune:
            steps[step] = (dutycycle, rpm)

    fan_profile = FanConfig(
        model=model,
        rpm_control_mode=RpmControlMode.PWM,
        pwm_frequency=pwm_frequency,
        minimum_duty_cycle=min([dutycycle for (_, (dutycycle, _)) in steps.items()]),  # e.g. 20%
        maximum_duty_cycle=max([dutycycle for (_, (dutycycle, _)) in steps.items()]),  # typically 100%
        minimum_rpm=min([rpm for (_, (_, rpm)) in steps.items() if rpm is not None]),
        maximum_rpm=max([rpm for (_, (_, rpm)) in steps.items() if rpm is not None]),
        steps=steps,
    )
    return fan_profile
