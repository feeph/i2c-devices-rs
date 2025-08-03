#!/usr/bin/env python3
"""
conversion-related functions
"""


def convert_bytes2temperature(msb: int, lsb: int) -> float:
    """
    convert the provided temperature from internal value to float
    (0x0C + 0xE0 -> 13.9)
    """
    temp = float(msb)
    if lsb & 0b1000_0000:
        temp += 0.50
    if lsb & 0b0100_0000:
        temp += 0.25
    if lsb & 0b0010_0000:
        temp += 0.15
    return temp


def convert_temperature2bytes(value: float) -> tuple[int, int]:
    """
    convert the provided temperature from float to the internal value
    (13.9 -> 0x0C + 0xE0)
    """
    msb = int(value)                  # 0..99
    decimal = round((value % 1) * 100)  # 0..99
    lsb = 0x00
    if decimal >= 95:
        msb += 1
    else:
        if decimal >= 38:
            lsb |= 0b1000_0000
            decimal -= 50
        if decimal >= 20:
            lsb |= 0b0100_0000
            decimal -= 25
        if decimal >= 8:
            lsb |= 0b0010_0000
            decimal -= 15
    return (msb, lsb)
