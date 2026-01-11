# ASAIR AHT10 & AHT20 Humidity and Temperature Sensor

Important notes from data sheet.

documentation:

- https://asairsensors.com/wp-content/uploads/2021/09/Data-Sheet-AHT20-Humidity-and-Temperature-Sensor-ASAIR-V1.0.03.pdf
- https://github.com/enjoyneering/AHTxx/blob/main/src/AHTxx.cpp

## Initialization delay

After power up sensor takes up to 20ms to become ready

## Sensor Reading Process

1. After power-on, wait for ≥100ms Before reading the temperature and
   humidity value, get a byte of status word by sending 0x71. If the status
   word and 0x18 are not equal to 0x18, initialize the 0x1B, 0x1C, 0x1E
   registers, details.
   Please refer to our official website routine for the initialization
   process; if they are equal, proceed to the next step.
2. Wait 10ms to send the 0xAC command (trigger measurement).
   This command parameter has two bytes, the first byte is 0x33, and the
   second byte is 0x00.
3. Wait 80ms for the measurement to be completed, if the read status word
   Bit 7 is 0, it means the measurement is completed, and then six bytes
   can be read continuously; otherwise, continue to wait.
4. After receiving six bytes, the next byte is CRC check data, which the
   user can read as needed. If the receiver needs CRC check, it will send
   an ACK reply after receiving the sixth byte, otherwise it will send a
   NACK reply. The initial value of CRC is 0xFF, and the CRC8 check
   polynomial is: CRC = 1 + X4 + X5 + X8
5. Calculate the temperature and humidity value

Note: The calibration status check in the first step only needs to be
checked when the power is turned on. No operation is required during
the acquisition process.

## data sheet

### section 7.3: Send Command

  After sending the measurement command 0xAC, the MCU must wait until the
  measurement is completed.

### section 7.4: Sensor Reading Process

1. After power-on, wait for ≥100ms Before reading the temperature and
   humidity value, get a byte of status word by sending 0x71. If the
   status word and 0x18 are not equal to 0x18, initialize the 0x1B, 0x1C,
   0x1E registers, details
   Please refer to our official website routine for the initialization
   process; if they are equal, proceed to the next step.
2. Wait 10ms to send the 0xAC command (trigger measurement). This command
   parameter has two bytes, the first byte is 0x33, and the second byte is
   0x00.
3. Wait 80ms for the measurement to be completed, if the read status word
   Bit[7] is 0, it means the measurement is completed, and then six bytes
   can be read continuously; otherwise, continue to wait.
4. After receiving six bytes, the next byte is CRC check data, which the
   user can read as needed. If the receiver needs CRC check, it will send
   an ACK reply after receiving the sixth byte, otherwise it will send a
   NACK reply. The initial value of CRC is 0xFF, and the CRC8 check
   polynomial is: CRC [7:0] = 1+X4+X5+X8
5. Calculate the temperature and humidity value

Note: The calibration status check in the first step only needs to be
checked when the power is turned on.No operation is required during the
acquisition process.

### initialization sequence (after power on)

1. wait for ≥100ms
2. send [0x71], read 1 byte of data
2.1. if `(byte & 0b0001_1111) == 0x18` -> all good
2.2. if `(byte & 0b0001_1111) != 0x18` -> initialize 0x1B, 0x1C & 0x1E
3. wait for ≥10ms

### take a measurement

1. send [0xAC, 0x33, 0x00]
2. wait for ≥80ms
3. send [0x71], read 1 byte of data
3.1. if `(byte & 0b1000_0000) != 0b1000_0000` -> measurement in progress, wait & repeat 2.
4. read 6 bytes of data
5. if CRC check is needed
5.1. send an ACK reply
5.2. read 1 byte
6. calculate temperature and humidity value

try to reduce complexity:

- status check:       read [u8; 1] from 0x71
- values without CRC: read [u8; 7] from 0x71
- values with CRC:    read [u8; 8] from 0x71

status bits

```TEXT
0b0000_000x - retain
0b0000_00x0 - retain
0b0000_0x00 - retain
0b0000_x000 - 1: calibrated, 0: not calibrated
0b000x_0000 - retain
0b00x0_0000 - retain
0b0x00_0000 - retain
0bx000_0000 - 1: busy, 0: standby
```
