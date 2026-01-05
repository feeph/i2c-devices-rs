# ASAIR AHT10 & AHT20 Humidity and Temperature Sensor

Important notes from data sheet.

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
