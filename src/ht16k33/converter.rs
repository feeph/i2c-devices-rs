/*
    convert the provided character into a 2-byte tuple,
    suitable for displaying on 7- or 14-segment displays

    Customized glyphs can be provided by duplicating this function and
    adjusting the bit-patterns or add/remove specific mappings as needed.
*/
// TODO the 4-digit/7-segment backpack is actually 5 digits and needs different handling
// (the colon between the 2nd and 3rd digit is addressed as a digit)

use numtoa::NumToA;

#[allow(unused_imports)]
use log::{debug, error, info, warn};

/// convert the provided character into a 2-byte tuple,
/// suitable for displaying on a 7-segment display
///
/// - make sure to always use upper case characters
/// - do not use this converter for 14-segment displays
pub fn convert_7(c: char) -> (u8, u8) {
    match c {
        '0' => (0b0011_1111, 0b0000_0000),
        '1' => (0b0000_0110, 0b0000_0000),
        '2' => (0b0101_1011, 0b0000_0000),
        '3' => (0b0100_1111, 0b0000_0000),
        '4' => (0b0110_0110, 0b0000_0000),
        '5' => (0b0110_1101, 0b0000_0000),
        '6' => (0b0111_1101, 0b0000_0000),
        '7' => (0b0000_0111, 0b0000_0000),
        '8' => (0b0111_1111, 0b0000_0000),
        '9' => (0b0110_1111, 0b0000_0000),
        'A' => (0b0111_0111, 0b0000_0000),
        'B' => (0b0111_1100, 0b0000_0000),
        'C' => (0b0011_1001, 0b0000_0000),
        'D' => (0b0101_1110, 0b0000_0000),
        'E' => (0b0111_1001, 0b0000_0000),
        'F' => (0b0111_0001, 0b0000_0000),
        'G' => (0b0011_1101, 0b0000_0000),
        'H' => (0b0111_0110, 0b0000_0000),
        'I' => (0b0011_0000, 0b0000_0000),
        'J' => (0b0001_1110, 0b0000_0000),
        // 'K' => <unable to display>
        'L' => (0b0011_1000, 0b0000_0000),
        // 'M' => <unable to display>
        'N' => (0b0101_0100, 0b0000_0000),
        'O' => (0b0101_1100, 0b0000_0000),
        'P' => (0b0111_0011, 0b0000_0000),
        'Q' => (0b0110_0111, 0b0000_0000),
        'R' => (0b0101_0000, 0b0000_0000),
        'S' => (0b0110_1101, 0b0000_0000),
        'T' => (0b0111_1000, 0b0000_0000),
        'U' => (0b0011_1110, 0b0000_0000),
        'V' => (0b0001_1100, 0b0000_0000),
        // 'W' => <unable to display>
        // 'X' => <unable to display>
        'Y' => (0b0110_1110, 0b0000_0000),
        // 'Z' => <unable to display>
        '.' => (0b1000_0000, 0b0000_0000),
        ':' => (0b1111_1111, 0b0000_0000),
        // encountered an unknown character
        _ => (0b0000_0000, 0b0000_0000),
    }
}

/// convert the provided character into a 2-byte tuple,
/// suitable for displaying on a 14-segment display
///
/// - do not use this converter for 7-segment displays
pub fn convert_14(c: char) -> (u8, u8) {
    match c {
        '!' => (0b0000_0110, 0b0100_0000),
        '"' => (0b0010_0000, 0b0000_0010),
        '#' => (0b1100_1110, 0b0001_0010),
        '$' => (0b1110_1101, 0b0001_0010),
        '%' => (0b0010_0100, 0b0000_1100),
        '&' => (0b0101_1101, 0b0010_0011),
        '\'' => (0b0000_0000, 0b0000_0100),
        '(' => (0b0000_0000, 0b0010_0100),
        ')' => (0b0000_0000, 0b0000_1001),
        '*' => (0b1100_0000, 0b0011_1111),
        '+' => (0b1100_0000, 0b0001_0010),
        ',' => (0b0000_0000, 0b0000_1000),
        '-' => (0b1100_0000, 0b0000_0000),
        '.' => (0b0000_0000, 0b0100_0000),
        '/' => (0b0000_0000, 0b0000_1100),
        '0' => (0b0011_1111, 0b0000_1100),
        '1' => (0b0000_0110, 0b0000_0000),
        '2' => (0b1101_1011, 0b0000_0000),
        '3' => (0b1000_1111, 0b0000_0000),
        '4' => (0b1110_0110, 0b0000_0000),
        '5' => (0b0110_1001, 0b0010_0000),
        '6' => (0b1111_1101, 0b0000_0000),
        '7' => (0b0000_0111, 0b0000_0000),
        '8' => (0b1111_1111, 0b0000_0000),
        '9' => (0b1110_1111, 0b0000_0000),
        ':' => (0b0000_0000, 0b0001_0010),
        ';' => (0b0000_0000, 0b0000_1010),
        '<' => (0b0100_0000, 0b0010_0100),
        '=' => (0b1100_1000, 0b0000_0000),
        '>' => (0b1000_0000, 0b0000_1001),
        '?' => (0b1010_0011, 0b0110_0000),
        '@' => (0b1011_1011, 0b0000_0010),
        'A' => (0b1111_0111, 0b0000_0000),
        'B' => (0b1000_1111, 0b0001_0010),
        'C' => (0b0011_1001, 0b0000_0000),
        'D' => (0b0000_1111, 0b0001_0010),
        'E' => (0b1111_1001, 0b0000_0000),
        'F' => (0b0111_0001, 0b0000_0000),
        'G' => (0b1011_1101, 0b0000_0000),
        'H' => (0b1111_0110, 0b0000_0000),
        'I' => (0b0000_0000, 0b0001_0010),
        'J' => (0b0001_1110, 0b0000_0000),
        'K' => (0b0111_0000, 0b0010_0100),
        'L' => (0b0011_1000, 0b0000_0000),
        'M' => (0b0011_0110, 0b0000_0101),
        'N' => (0b0011_0110, 0b0010_0001),
        'O' => (0b0011_1111, 0b0000_0000),
        'P' => (0b1111_0011, 0b0000_0000),
        'Q' => (0b0011_1111, 0b0010_0000),
        'R' => (0b1111_0011, 0b0010_0000),
        'S' => (0b1110_1101, 0b0000_0000),
        'T' => (0b0000_0001, 0b0001_0010),
        'U' => (0b0011_1110, 0b0000_0000),
        'V' => (0b0011_0000, 0b0000_1100),
        'W' => (0b0011_0110, 0b0010_1000),
        'X' => (0b0000_0000, 0b0010_1101),
        'Y' => (0b0000_0000, 0b0001_0101),
        'Z' => (0b0000_1001, 0b0000_1100),
        '[' => (0b0011_1001, 0b0000_0000),
        '\\' => (0b0000_0000, 0b0010_0001),
        ']' => (0b0000_1111, 0b0000_0000),
        '^' => (0b0000_0011, 0b0000_1100),
        '_' => (0b0000_1000, 0b0000_0000),
        '`' => (0b0000_0000, 0b0000_0001),
        'a' => (0b0101_1000, 0b0001_0000),
        'b' => (0b0111_1000, 0b0010_0000),
        'c' => (0b1101_1000, 0b0000_0000),
        'd' => (0b1000_1110, 0b0000_1000),
        'e' => (0b0101_1000, 0b0000_1000),
        'f' => (0b0111_0001, 0b0000_0000),
        'g' => (0b1000_1110, 0b0000_0100),
        'h' => (0b0111_0000, 0b0001_0000),
        'i' => (0b0000_0000, 0b0001_0000),
        'j' => (0b0000_1110, 0b0000_0000),
        'k' => (0b0000_0000, 0b0011_0110),
        'l' => (0b0011_0000, 0b0000_0000),
        'm' => (0b1101_0100, 0b0001_0000),
        'n' => (0b0101_0000, 0b0001_0000),
        'o' => (0b1101_1100, 0b0000_0000),
        'p' => (0b0111_0000, 0b0000_0001),
        'q' => (0b1000_0110, 0b0000_0100),
        'r' => (0b0101_0000, 0b0000_0000),
        's' => (0b1000_1000, 0b0010_0000),
        't' => (0b0111_1000, 0b0000_0000),
        'u' => (0b0001_1100, 0b0000_0000),
        'v' => (0b0000_0100, 0b0010_0000),
        'w' => (0b0001_0100, 0b0010_1000),
        'x' => (0b1100_0000, 0b0010_1000),
        'y' => (0b0000_1100, 0b0010_0000),
        'z' => (0b0100_1000, 0b0000_1000),
        '{' => (0b0100_1001, 0b0000_1001),
        '|' => (0b0000_0000, 0b0001_0010),
        '}' => (0b1000_1001, 0b0010_0100),
        '~' => (0b0010_0000, 0b0000_0101),
        // encountered an unknown character
        _ => (0b0000_0000, 0b0000_0000),
    }
}

fn _make_buffer(number_str: &[char; 9], digits: usize, convert: fn(char) -> (u8, u8)) -> [u8; 16] {
    let mut buffer: [u8; 16] = [0b0000_0000; 16];

    let mut idx = 0;
    for c in number_str {
        let offset1 = 2 * idx;
        let offset2 = 2 * idx + 1;
        let (byte1, byte2) = convert(*c);
        if *c != '.' {
            buffer[offset1] = byte1;
            buffer[offset2] = byte2;
            // advance the index
            idx += 1;
        } else {
            // add dot to the previous digit
            buffer[offset1 - 2] |= byte1;
            buffer[offset2 - 2] |= byte2;
        }
        if idx >= digits {
            break;
        }
    }

    buffer
}

pub fn convert_to_4_digits(number: f32, convert: fn(char) -> (u8, u8)) -> [u8; 16] {
    let digits = 4;
    let mut number_str = [' '; 9];

    if number > 9999.0 {
        // too high
        number_str[0] = '-';
        number_str[1] = 'h';
        number_str[2] = 'i';
        number_str[3] = '-';
    } else if number >= -999.0 {
        let d = number as i32; // decimal part
        let f = (((number.abs() * 1000.0) + 0.5) % 1000.0) as i32; // fraction, up to 3 digits, rounded
        let mut number_buf = [0u8; 20];
        let mut idx = 0;
        for c in d.numtoa_str(10, &mut number_buf).chars() {
            number_str[idx] = c;
            idx += 1;
        }
        number_str[idx] = '.';
        idx += 1;
        for c in f.numtoa_str(10, &mut number_buf).chars() {
            number_str[idx] = c;
            idx += 1;
            if idx > (digits * 2) {
                break;
            }
        }
    } else {
        // too high
        number_str[0] = '-';
        number_str[1] = 'l';
        number_str[2] = 'o';
        number_str[3] = '-';
    }

    _make_buffer(&number_str, digits, convert)
}
