/*
    low-level integration tests for EMC2101
    (using 'i2c_devices::emc2101::hw')
*/

mod common;
mod convert;

use common::VirtualHt16K33;

use i2c_devices::ht16k33 as sut;
use i2c_devices::ht16k33::SegmentedDisplay; // import trait

// ------------------------------------------------------------------------
// character conversion logic
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
fn convert_number_too_large() {
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

// ------------------------------------------------------------------------
// 7-segment display
// ------------------------------------------------------------------------

#[test]
fn configure7_and_show_number() {
    let mut vbd = create_ht16k33();

    let sd = sut::Segment7x4 {
        convert: sut::convert_7,
        did: 0,
        display_mode: sut::DisplayMode::On,
        brightness_level: 8,
    };
    sd.show_number(&mut vbd, 1234.0);

    assert_eq!(
        vbd.dda,
        [6, 0, 91, 0, 0, 0, 79, 0, 102, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(vbd.dis, 1);
    assert_eq!(vbd.dim, 8);
    assert_eq!(vbd.ris, 0);
    assert_eq!(vbd.osc, 1);
}

#[test]
fn configure7_and_show_string() {
    let mut vbd = create_ht16k33();

    let sd = sut::Segment7x4 {
        convert: sut::convert_7,
        did: 0,
        display_mode: sut::DisplayMode::On,
        brightness_level: 8,
    };
    sd.show_string(&mut vbd, "ABCD");

    assert_eq!(
        vbd.dda,
        [119, 0, 124, 0, 57, 0, 94, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(vbd.dis, 1);
    assert_eq!(vbd.dim, 8);
    assert_eq!(vbd.ris, 0);
    assert_eq!(vbd.osc, 1);
}

#[test]
fn update7_brightness_pass() {
    let mut vbd = create_ht16k33();

    let mut sd = sut::Segment7x4 {
        convert: sut::convert_7,
        did: 0,
        display_mode: sut::DisplayMode::On,
        brightness_level: 8,
    };

    let computed = sd.set_brightness_level(&mut vbd, 8);
    let expected = true;

    assert_eq!(computed, expected);
}

#[test]
fn update7_brightness_fail() {
    let mut vbd = create_ht16k33();

    let mut sd = sut::Segment7x4 {
        convert: sut::convert_7,
        did: 0,
        display_mode: sut::DisplayMode::On,
        brightness_level: 8,
    };

    let computed = sd.set_brightness_level(&mut vbd, 16);
    let expected = false;

    assert_eq!(computed, expected);
}

#[test]
fn update7_brightness_and_show() {
    let mut vbd = create_ht16k33();

    let mut sd = sut::Segment7x4 {
        convert: sut::convert_7,
        did: 0,
        display_mode: sut::DisplayMode::On,
        brightness_level: 8,
    };
    sd.show_string(&mut vbd, "ABCD");

    sd.set_brightness_level(&mut vbd, 9);

    assert_eq!(
        vbd.dda,
        [119, 0, 124, 0, 57, 0, 94, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(vbd.dis, 1);
    assert_eq!(vbd.dim, 9);
    assert_eq!(vbd.ris, 0);
    assert_eq!(vbd.osc, 1);
}

#[test]
fn update7_display_mode_and_show() {
    let mut vbd = create_ht16k33();

    let mut sd = sut::Segment7x4 {
        convert: sut::convert_7,
        did: 0,
        display_mode: sut::DisplayMode::On,
        brightness_level: 8,
    };
    sd.show_string(&mut vbd, "ABCD");

    sd.set_display_mode(&mut vbd, sut::DisplayMode::BlinkSlow);

    assert_eq!(
        vbd.dda,
        [119, 0, 124, 0, 57, 0, 94, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(vbd.dis, 7);
    assert_eq!(vbd.dim, 8);
    assert_eq!(vbd.ris, 0);
    assert_eq!(vbd.osc, 1);
}

#[test]
fn disable7() {
    let mut vbd = create_ht16k33();

    // configure the display and ensure the device was updated
    let sd = sut::Segment7x4 {
        convert: sut::convert_7,
        did: 0,
        display_mode: sut::DisplayMode::On,
        brightness_level: 8,
    };
    sd.show_number(&mut vbd, 1234.0);

    // put in standby (disable oscillator)
    sd.disable(&mut vbd);

    assert_eq!(vbd.osc, 0);
}

// ------------------------------------------------------------------------
// 14-segment display
// ------------------------------------------------------------------------

#[test]
fn configure14_and_show_number() {
    let mut vbd = create_ht16k33();

    let sd = sut::Segment14x4 {
        convert: sut::convert_14,
        did: 0,
        display_mode: sut::DisplayMode::On,
        brightness_level: 8,
    };
    sd.show_number(&mut vbd, 1234.0);

    assert_eq!(
        vbd.dda,
        [6, 0, 219, 0, 143, 0, 230, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(vbd.dis, 1);
    assert_eq!(vbd.dim, 8);
    assert_eq!(vbd.ris, 0);
    assert_eq!(vbd.osc, 1);
}

#[test]
fn configure14_and_show_string() {
    let mut vbd = create_ht16k33();

    let sd = sut::Segment14x4 {
        convert: sut::convert_14,
        did: 0,
        display_mode: sut::DisplayMode::On,
        brightness_level: 8,
    };
    sd.show_string(&mut vbd, "ABCD");

    assert_eq!(
        vbd.dda,
        [247, 0, 143, 18, 57, 0, 15, 18, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(vbd.dis, 1);
    assert_eq!(vbd.dim, 8);
    assert_eq!(vbd.ris, 0);
    assert_eq!(vbd.osc, 1);
}

#[test]
fn update14_brightness_pass() {
    let mut vbd = create_ht16k33();

    let mut sd = sut::Segment14x4 {
        convert: sut::convert_14,
        did: 0,
        display_mode: sut::DisplayMode::On,
        brightness_level: 8,
    };

    let computed = sd.set_brightness_level(&mut vbd, 8);
    let expected = true;

    assert_eq!(computed, expected);
}

#[test]
fn update14_brightness_fail() {
    let mut vbd = create_ht16k33();

    let mut sd = sut::Segment14x4 {
        convert: sut::convert_14,
        did: 0,
        display_mode: sut::DisplayMode::On,
        brightness_level: 8,
    };

    let computed = sd.set_brightness_level(&mut vbd, 16);
    let expected = false;

    assert_eq!(computed, expected);
}

#[test]
fn update14_brightness_and_show() {
    let mut vbd = create_ht16k33();

    let mut sd = sut::Segment14x4 {
        convert: sut::convert_14,
        did: 0,
        display_mode: sut::DisplayMode::On,
        brightness_level: 8,
    };
    sd.show_string(&mut vbd, "ABCD");

    sd.set_brightness_level(&mut vbd, 9);

    assert_eq!(
        vbd.dda,
        [247, 0, 143, 18, 57, 0, 15, 18, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(vbd.dis, 1);
    assert_eq!(vbd.dim, 9);
    assert_eq!(vbd.ris, 0);
    assert_eq!(vbd.osc, 1);
}

#[test]
fn update14_display_mode_and_show() {
    let mut vbd = create_ht16k33();

    let mut sd = sut::Segment14x4 {
        convert: sut::convert_14,
        did: 0,
        display_mode: sut::DisplayMode::On,
        brightness_level: 8,
    };
    sd.show_string(&mut vbd, "ABCD");

    sd.set_display_mode(&mut vbd, sut::DisplayMode::BlinkSlow);

    assert_eq!(
        vbd.dda,
        [247, 0, 143, 18, 57, 0, 15, 18, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(vbd.dis, 7);
    assert_eq!(vbd.dim, 8);
    assert_eq!(vbd.ris, 0);
    assert_eq!(vbd.osc, 1);
}

#[test]
fn disable14() {
    let mut vbd = create_ht16k33();

    // configure the display and ensure the device was updated
    let sd = sut::Segment14x4 {
        convert: sut::convert_14,
        did: 0,
        display_mode: sut::DisplayMode::On,
        brightness_level: 8,
    };
    sd.show_number(&mut vbd, 1234.0);

    // put in standby (disable oscillator)
    sd.disable(&mut vbd);

    assert_eq!(vbd.osc, 0);
}

// ------------------------------------------------------------------------

fn create_ht16k33() -> VirtualHt16K33 {
    VirtualHt16K33 {
        dda: [0u8; 16],
        osc: 0x00,
        dis: 0x00,
        ris: 0x00,
        dim: 0x00,
    }
}
