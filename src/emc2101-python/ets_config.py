#!/usr/bin/env python3


class ExternalTemperatureSensorConfig:

    def __init__(self, ideality_factor: int, beta_factor: int):
        """
        configure hardware-specific settings

        These settings depend on the external temperature sensor's characteristics.
        """
        self.diode_ideality_factor    = ideality_factor   # datasheet section 6.12
        self.beta_compensation_factor = beta_factor       # datasheet section 6.13


# temperature sensitive transistors
ets_2n3904 = ExternalTemperatureSensorConfig(ideality_factor=0x12, beta_factor=0x08)  # 2N3904 (NPN)
ets_2n3906 = ExternalTemperatureSensorConfig(ideality_factor=0x12, beta_factor=0x08)  # 2N3906 (PNP)
