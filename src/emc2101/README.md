# EMC2101

The EMC2101 is an SMBus 2.0 compliant, integrated fan control solution
complete with two temperature monitors, one external and one internal.

_Please note:_ All temperature values are in °C.

The hardware does not support other units.

## Product IDs

- EMC2101-ACZL-TR
- EMC2101-R-ACZL-TR

## Documentation

[product page <https://www.microchip.com/en-us/product/EMC2101>](https://www.microchip.com/en-us/product/EMC2101)

[data sheet <https://ww1.microchip.com/downloads/aemDocuments/documents/MSLD/ProductDocuments/DataSheets/EMC2101-Data-Sheet-DS20006703.pdf>](https://ww1.microchip.com/downloads/aemDocuments/documents/MSLD/ProductDocuments/DataSheets/EMC2101-Data-Sheet-DS20006703.pdf)

## Device Registers

source: section 6 of the data sheet (revision 2.54)

| REGISTER | R/W | REGISTER NAME                        | FUNCTION                                                                                       | DEFAULT |
|:--------:|:---:| ------------------------------------ | ---------------------------------------------------------------------------------------------- |:-------:|
|   0x00   |  R  | Internal Temperature                 | Stores the Internal Temperature                                                                |    -    |
|   0x01   |  R  | External Diode Temperature High Byte | Stores the External Temperature High Byte                                                      |    -    |
|   0x02   |  R  | Status                               | Reports internal, external, and TCRIT alarms                                                   |    -    |
|   0x03   | R/W | Configuration                        | Alert Mask, STANDBY, TCRIT override, Alert Fault Queue                                         |   0x00  |
|   0x04   | R/W | Conversion Rate                      | Sets conversion rate                                                                           |   0x08  |
|   0x05   | R/W | Internal Temp Limit                  | ALERT / TACH asserted if measured temp above this value                                        |   0x46  |
|   0x07   | R/W | External Temp High Limit High Byte   | ALERT / TACH asserted if measured temp above this value                                        |   0x46  |
|   0x08   | R/W | External Temp Low Limit High Byte    | ALERT / TACH asserted if measured temp below this value                                        |   0x00  |
|   0x09   |  -  |       -                              |     _same as 0x03_                                                                             |    -    |
|   0x0A   |  -  |       -                              |     _same as 0x04_                                                                             |    -    |
|   0x0B   |  -  |       -                              |     _same as 0x05_                                                                             |    -    |
|   0x0C   | R/W | External Temperature Force           | Force the temperature for determining the next fan speed used in the Fan Control Look-Up Table |   0x00  |
|   0x0D   |  -  |       -                              |     _same as 0x07_                                                                             |    -    |
|   0x0E   |  -  |       -                              |     _same as 0x08_                                                                             |    -    |
|   0x0F   | R/W | One Shot                             | When written, performs a one-shot conversion.                                                  |   0x00  |
|   0x10   |  R  | External Diode Temperature Low Byte  | Stores the External Temperature Low Byte                                                       |    -    |
|   0x11   | R/W | Scratchpad #1                        | Scratchpad - This register is read/write but does nothing                                      |   0x00  |
|   0x12   | R/W | Scratchpad #2                        | Scratchpad - This register is read/write but does nothing                                      |   0x00  |
|   0x13   | R/W | External Diode High Limit Low Byte   | Fractional data of High Limit                                                                  |   0x00  |
|   0x14   | R/W | External Diode Low Limit Low Byte    | Fractional data of Low Limit                                                                   |   0x00  |
|   0x16   | R/W | Alert Mask                           | Disables alarms                                                                                |   0xA4  |
|   0x17   | R/W | External Diode Ideality Factor       | Sets ideality factor based on diode type                                                       |   0x12  |
|   0x18   | R/W | Beta Compensation Factor             | Compensates for transistors with various beta factors                                          |   0x08  |
|   0x19   | R/W | TCRIT Temp Limit                     | Fan will be set to full speed if external temp above this value                                |   0x55  |
|   0x21   | R/W | TCRIT Hysteresis                     | Amount of hysteresis applied to TCRIT Temp (1LSB = 1°C)                                        |   0x0A  |
|   0x46   |  R  | TACH Reading Low Byte                | Stores the lower 6 bits of the TACH count. and the TACH configuration bits                     |    -    |
|   0x47   |  R  | TACH Reading High Byte               | Stores the upper 8 bits of the TACH count.                                                     |    -    |
|   0x48   | R/W | TACH Limit Low Byte                  | Stores the lower 6 bits of the TACH Limit                                                      |   0xFF  |
|   0x49   | R/W | TACH Limit High Byte                 | Stores the upper 8 bits of the TACH Limit                                                      |   0xFF  |
|   0x4A   | R/W | FAN Configuration                    | defines polarity of PWM or DAC                                                                 |   0x20  |
|   0x4B   | R/W | Fan Spin-up                          | Sets Spin Up options                                                                           |   0x3F  |
|   0x4C   | R/W | Fan Setting                          | Sets PWM or DAC value                                                                          |   0x00  |
|   0x4D   | R/W | PWM Frequency                        | Sets the final PWM Frequency                                                                   |   0x17  |
|   0x4E   | R/W | PWM Frequency Divide                 | Sets the base PWM frequency                                                                    |   0x01  |
|   0x4F   | R/W | Lookup Table Hysteresis              | Amount of hysteresis applied to Lookup Table Temp (1LSB = 1°C)                                 |   0x04h |
|   0x50   | R/W | Lookup Table Temp Setting 1          | Look Up Table Temperature Setting 1                                                            |   0x7F  |
|   0x51   | R/W | Lookup Table Fan Setting 1           | Associated Fan Setting for Temp Setting 1                                                      |   0x3F  |
|   0x52   | R/W | Lookup Table Temp Setting 2          | Look Up Table Temperature Setting 2                                                            |   0x7F  |
|   0x53   | R/W | Lookup Table Fan Setting 2           | Associated Fan Setting for Temp Setting 2                                                      |   0x3F  |
|   0x54   | R/W | Lookup Table Temp Setting 3          | Look Up Table Temperature Setting 3                                                            |   0x7F  |
|   0x55   | R/W | Lookup Table Fan Setting 3           | Associated Fan Setting for Temp Setting 3                                                      |   0x3F  |
|   0x56   | R/W | Lookup Table Temp Setting 4          | Look Up Table Temperature Setting 4                                                            |   0x7F  |
|   0x57   | R/W | Lookup Table Fan Setting 4           | Associated Fan Setting for Temp Setting 4                                                      |   0x3F  |
|   0x58   | R/W | Lookup Table Temp Setting 5          | Look Up Table Temperature Setting 5                                                            |   0x7F  |
|   0x59   | R/W | Lookup Table Fan Setting 5           | Associated Fan Setting for Temp Setting 5                                                      |   0x3F  |
|   0x5A   | R/W | Lookup Table Temp Setting 6          | Look Up Table Temperature Setting 6                                                            |   0x7F  |
|   0x5B   | R/W | Lookup Table Fan Setting 6           | Associated Fan Setting for Temp Setting 6                                                      |   0x3F  |
|   0x5C   | R/W | Lookup Table Temp Setting 7          | Look Up Table Temperature Setting 7                                                            |   0x7F  |
|   0x5D   | R/W | Lookup Table Fan Setting 7           | Associated Fan Setting for Temp Setting 7                                                      |   0x3F  |
|   0x5E   | R/W | Lookup Table Temp Setting 8          | Look Up Table Temperature Setting 8                                                            |   0x7F  |
|   0x5F   | R/W | Lookup Table Fan Setting 8           | Associated Fan Setting for Temp Setting 8                                                      |   0x3F  |
|   0xBF   | R/W | Averaging Filter                     | Selects averaging function for external diode                                                  |   0x00  |
|   0xFD   |  R  | Product ID                           | Product ID (numeric code)                                                                      |    -    |
|   0xFE   |  R  | Manufacturer ID                      | Manufacturer ID (numeric code)                                                                 |    -    |
|   0xFF   |  R  | Revision Register                    | Product Revision                                                                               |    -    |

The Look Up Table Registers (0x4F..0x5F) are made read only if the
PWM Program bit (bit 5) in PWM Configuration Register (0x4A) is set.
