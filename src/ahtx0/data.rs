/*
    parse provided sensor data (with or without CRC)
*/

use core::option::Option;

#[derive(Debug, PartialEq)]
pub struct SensorData {
    pub temperature: f32,
    pub humidity: f32,
}

// for AHT10
pub fn parse_data_without_crc(data: &[u8; 6]) -> SensorData {
    let h1 = ((data[1] as i32) << 12) | ((data[2] as i32) << 4) | ((data[3] as i32) >> 4);
    let h2 = calculate_humidity(h1);

    let t1 = (((data[3] & 0xF) as i32) << 16) | ((data[4] as i32) << 8) | data[5] as i32;
    let t2 = calculate_temperature(t1);

    SensorData {
        temperature: t2,
        humidity: h2,
    }
}

#[test]
fn parse_data_without_crc_pass() {
    let computed = parse_data_without_crc(&[0x1C, 0x44, 0xF2, 0xF5, 0xF9, 0xD6]);
    let expected = SensorData {
        humidity: 26.933193,
        temperature: 24.69902,
    };

    assert_eq!(computed, expected);
}

// ------------------------------------------------------------------------

// for AHT20
pub fn parse_data_with_crc(data: &[u8; 7]) -> Option<SensorData> {
    if validate_crc(data, 0x31) {
        let h1 = ((data[1] as i32) << 12) | ((data[2] as i32) << 4) | ((data[3] as i32) >> 4);
        let h2 = calculate_humidity(h1);

        let t1 = (((data[3] & 0xF) as i32) << 16) | ((data[4] as i32) << 8) | data[5] as i32;
        let t2 = calculate_temperature(t1);

        Some(SensorData {
            temperature: t2,
            humidity: h2,
        })
    } else {
        None
    }
}

#[test]
fn parse_data_with_crc_pass() {
    let computed = parse_data_with_crc(&[0x1C, 0x44, 0xF2, 0xF5, 0xF9, 0xD6, 0x34]).unwrap();
    let expected = SensorData {
        humidity: 26.933193,
        temperature: 24.69902,
    };

    assert_eq!(computed, expected);
}

#[test]
fn parse_data_with_crc_fail() {
    let computed = parse_data_with_crc(&[0x1C, 0x44, 0xF2, 0xF5, 0xF9, 0xD6, 0x00]);
    let expected = None;

    assert_eq!(computed, expected);
}

// ------------------------------------------------------------------------

fn validate_crc(data: &[u8; 7], p: u8) -> bool {
    let mut crc: u8 = 0xFF;

    for byte in data.iter() {
        crc ^= byte;

        for _bitindex in (0..=7).rev() {
            if crc & 0x80 == 0x80 {
                crc = (crc << 1) ^ p;
            } else {
                crc <<= 1;
            }
        }
    }

    crc == 0
}

#[test]
fn validate_crc_pass() {
    let valid_crcs = [
        // sample data from actual measurements
        ([0x1C, 0x44, 0xF2, 0xF5, 0xF9, 0xD6, 0x34], 0x31),
        ([0x1C, 0x45, 0x1F, 0x55, 0xF8, 0x44, 0x0C], 0x31),
        ([0x1C, 0x44, 0x8C, 0x15, 0xF9, 0xDF, 0x5F], 0x31),
        ([0x1C, 0x44, 0xF2, 0xF5, 0xF9, 0xD6, 0x34], 0x31),
    ];

    for (data, p) in valid_crcs.iter() {
        let computed = validate_crc(data, *p);
        let expected = true;

        assert_eq!(computed, expected);
    }
}

#[test]
fn validate_crc_fail() {
    let invalid_crcs = [
        ([0x1C, 0x44, 0xF2, 0xF5, 0xF9, 0xD6, 0x00], 0x31), // data mangled
        ([0x1C, 0x44, 0xF2, 0xF5, 0xF9, 0xD6, 0x34], 0x30), // wrong polynomial
    ];

    for (data, p) in invalid_crcs.iter() {
        let computed = validate_crc(data, *p);
        let expected = false;

        assert_eq!(computed, expected);
    }
}

// ------------------------------------------------------------------------

fn calculate_humidity(value: i32) -> f32 {
    let d = 0x100000 as f32; // 2^20
    (value as f32 * 100.0) / d
}

#[test]
fn calculate_humidity_pass() {
    let computed = calculate_humidity(302567);
    let expected = 28.855038;

    assert_eq!(computed, expected);
}

// ------------------------------------------------------------------------

fn calculate_temperature(value: i32) -> f32 {
    let d = 0x100000 as f32; // 2^20
    ((value as f32 * 200.0) / d) - 50.0
}

#[test]
fn calculate_temperature_pass() {
    let computed = calculate_temperature(390987);
    let expected = 24.574852;

    assert_eq!(computed, expected);
}
