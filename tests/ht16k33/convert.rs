use i2c_devices::ht16k33 as sut;

#[test]
fn convert7_digits_pass() {
    for value in '0'..='9' {
        assert_ne!(sut::convert_7(value), (0b0000_0000, 0b0000_0000));
    }
}

#[test]
fn convert7_uppercase_pass() {
    for value in 'A'..='Z' {
        if ['K', 'M', 'W', 'X', 'Z'].contains(&value) {
            // unable to represent with 7-segment displays
            continue;
        }
        assert_ne!(sut::convert_7(value), (0b0000_0000, 0b0000_0000));
    }
}

#[test]
fn convert7_symbols_pass() {
    let values = ['.', ':'];

    for value in values {
        assert_ne!(sut::convert_7(value), (0b0000_0000, 0b0000_0000));
    }
}

#[test]
fn convert7_fail() {
    let computed = sut::convert_7('²'); // unable to display with 7 segments
    let no_value = (0b0000_0000, 0b0000_0000);

    assert_eq!(computed, no_value);
}

// ------------------------------------------------------------------------

#[test]
fn convert14_digits_pass() {
    for value in '0'..='9' {
        assert_ne!(sut::convert_14(value), (0b0000_0000, 0b0000_0000));
    }
}

#[test]
fn convert14_lowercase_pass() {
    for value in 'a'..='z' {
        assert_ne!(sut::convert_14(value), (0b0000_0000, 0b0000_0000));
    }
}

#[test]
fn convert14_uppercase_pass() {
    for value in 'A'..='Z' {
        assert_ne!(sut::convert_14(value), (0b0000_0000, 0b0000_0000));
    }
}

#[test]
fn convert14_symbols_pass() {
    // ensure all representable characters are covered
    let values = [
        '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<',
        '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~',
    ];

    for value in values {
        println!("{}", value);
        assert_ne!(sut::convert_14(value), (0b0000_0000, 0b0000_0000));
    }
}

#[test]
fn convert14_fail() {
    let computed = sut::convert_14('²'); // unable to display with 14 segments
    let no_value = (0b0000_0000, 0b0000_0000);

    assert_eq!(computed, no_value);
}

// ------------------------------------------------------------------------

#[test]
fn convert_number_pos1() {
    let computed = sut::convert_to_4_digits(1234.0, convert_14);
    #[rustfmt::skip]
    let expected = [
        0b0000_0001, 0b0000_0000, // 1
        0b0000_0010, 0b0000_0000, // 2
        0b0000_0011, 0b0000_0000, // 3
        0b0000_0100, 0b0000_0000, // 4
        0b0000_0000, 0b0000_0000,
        0b0000_0000, 0b0000_0000,
        0b0000_0000, 0b0000_0000,
        0b0000_0000, 0b0000_0000,
    ];

    assert_eq!(computed, expected);
}

#[test]
fn convert_number_pos2() {
    let computed = sut::convert_to_4_digits(123.4, convert_14);
    #[rustfmt::skip]
    let expected = [
        0b0000_0001, 0b0000_0000, // 1
        0b0000_0010, 0b0000_0000, // 2
        0b0000_0011, 0b0100_0000, // 3 with dot
        0b0000_0100, 0b0000_0000, // 4
        0b0000_0000, 0b0000_0000,
        0b0000_0000, 0b0000_0000,
        0b0000_0000, 0b0000_0000,
        0b0000_0000, 0b0000_0000,
    ];

    assert_eq!(computed, expected);
}

#[test]
fn convert_number_pos3() {
    let computed = sut::convert_to_4_digits(12.34, convert_14);
    #[rustfmt::skip]
    let expected = [
        0b0000_0001, 0b0000_0000, // 1
        0b0000_0010, 0b0100_0000, // 2 with dot
        0b0000_0011, 0b0000_0000, // 3
        0b0000_0100, 0b0000_0000, // 4
        0b0000_0000, 0b0000_0000,
        0b0000_0000, 0b0000_0000,
        0b0000_0000, 0b0000_0000,
        0b0000_0000, 0b0000_0000,
    ];

    assert_eq!(computed, expected);
}

#[test]
fn convert_number_pos4() {
    let computed = sut::convert_to_4_digits(1.234, convert_14);
    #[rustfmt::skip]
    let expected = [
        0b0000_0001, 0b0100_0000, // 1 with dot
        0b0000_0010, 0b0000_0000, // 2
        0b0000_0011, 0b0000_0000, // 3
        0b0000_0100, 0b0000_0000, // 4
        0b0000_0000, 0b0000_0000,
        0b0000_0000, 0b0000_0000,
        0b0000_0000, 0b0000_0000,
        0b0000_0000, 0b0000_0000,
    ];

    assert_eq!(computed, expected);
}

#[test]
fn convert_number_neg1() {
    let computed = sut::convert_to_4_digits(-123.0, convert_14);
    #[rustfmt::skip]
    let expected = [
        0b0000_0000, 0b1000_0000, // -
        0b0000_0001, 0b0000_0000, // 1
        0b0000_0010, 0b0000_0000, // 2
        0b0000_0011, 0b0000_0000, // 3
        0b0000_0000, 0b0000_0000,
        0b0000_0000, 0b0000_0000,
        0b0000_0000, 0b0000_0000,
        0b0000_0000, 0b0000_0000,
    ];

    assert_eq!(computed, expected);
}

#[test]
fn convert_number_neg2() {
    let computed = sut::convert_to_4_digits(-12.3, convert_14);
    #[rustfmt::skip]
    let expected = [
        0b0000_0000, 0b1000_0000, // -
        0b0000_0001, 0b0000_0000, // 1
        0b0000_0010, 0b0100_0000, // 2 with dot
        0b0000_0011, 0b0000_0000, // 3
        0b0000_0000, 0b0000_0000,
        0b0000_0000, 0b0000_0000,
        0b0000_0000, 0b0000_0000,
        0b0000_0000, 0b0000_0000,
    ];

    assert_eq!(computed, expected);
}

#[test]
fn convert_number_neg3() {
    let computed = sut::convert_to_4_digits(-1.23, convert_14);
    #[rustfmt::skip]
    let expected = [
        0b0000_0000, 0b1000_0000, // -
        0b0000_0001, 0b0100_0000, // 1 with dot
        0b0000_0010, 0b0000_0000, // 2
        0b0000_0011, 0b0000_0000, // 3
        0b0000_0000, 0b0000_0000,
        0b0000_0000, 0b0000_0000,
        0b0000_0000, 0b0000_0000,
        0b0000_0000, 0b0000_0000,
    ];

    assert_eq!(computed, expected);
}

#[test]
fn convert_number_too_small() {
    let computed = sut::convert_to_4_digits(-1000.0, convert_14);
    #[rustfmt::skip]
    let expected = [
        0b0000_0000, 0b1000_0000, // -
        0b0000_0000, 0b0000_0001, // l
        0b0000_0000, 0b0000_0010, // o
        0b0000_0000, 0b1000_0000, // -
        0b0000_0000, 0b0000_0000,
        0b0000_0000, 0b0000_0000,
        0b0000_0000, 0b0000_0000,
        0b0000_0000, 0b0000_0000,
    ];

    assert_eq!(computed, expected);
}

#[test]
fn convert_number_too_large1() {
    // too large to display (but would fit into buffer)
    let computed = sut::convert_to_4_digits(10000.0, convert_14);
    #[rustfmt::skip]
    let expected = [
        0b0000_0000, 0b1000_0000, // -
        0b0000_0000, 0b0000_0100, // h
        0b0000_0000, 0b0000_1000, // i
        0b0000_0000, 0b1000_0000, // -
        0b0000_0000, 0b0000_0000,
        0b0000_0000, 0b0000_0000,
        0b0000_0000, 0b0000_0000,
        0b0000_0000, 0b0000_0000,
    ];

    assert_eq!(computed, expected);
}

#[test]
fn convert_number_too_large2() {
    // attempt a buffer overrun
    let computed = sut::convert_to_4_digits(1234567890.1234, convert_14);
    #[rustfmt::skip]
    let expected = [
        0b0000_0000, 0b1000_0000, // -
        0b0000_0000, 0b0000_0100, // h
        0b0000_0000, 0b0000_1000, // i
        0b0000_0000, 0b1000_0000, // -
        0b0000_0000, 0b0000_0000,
        0b0000_0000, 0b0000_0000,
        0b0000_0000, 0b0000_0000,
        0b0000_0000, 0b0000_0000,
    ];

    assert_eq!(computed, expected);
}

// ------------------------------------------------------------------------

// use binary pattern for easier debugging
fn convert_14(c: char) -> (u8, u8) {
    match c {
        '-' => (0b0000_0000, 0b1000_0000),
        '.' => (0b0000_0000, 0b0100_0000),
        '0' => (0b0000_0000, 0b0000_0000),
        '1' => (0b0000_0001, 0b0000_0000),
        '2' => (0b0000_0010, 0b0000_0000),
        '3' => (0b0000_0011, 0b0000_0000),
        '4' => (0b0000_0100, 0b0000_0000),
        '5' => (0b0000_0101, 0b0010_0000),
        '6' => (0b0000_0110, 0b0000_0000),
        '7' => (0b0000_0111, 0b0000_0000),
        '8' => (0b0000_1000, 0b0000_0000),
        '9' => (0b0000_1001, 0b0000_0000),
        'l' => (0b0000_0000, 0b0000_0001),
        'o' => (0b0000_0000, 0b0000_0010),
        'h' => (0b0000_0000, 0b0000_0100),
        'i' => (0b0000_0000, 0b0000_1000),
        // encountered an unknown character
        _ => (0b1111_1111, 0b1111_1111),
    }
}
