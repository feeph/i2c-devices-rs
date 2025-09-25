//
// default register values as defined in data sheet (section 6)
// (R/W registers only)

pub static DEFAULTS: [[u8; 2]; 41] = [
    [0x03, 0x00], // Configuration
    [0x04, 0x08], // Conversion Rate
    [0x05, 0x46], // Internal Temp Limit
    [0x07, 0x46], // External Temp High Limit High Byte
    [0x08, 0x00], // External Temp Low Limit High Byte
    [0x0C, 0x00], // External Temperature Force
    [0x0F, 0x00], // One Shot
    [0x11, 0x00], // Scratchpad #1
    [0x12, 0x00], // Scratchpad #2
    [0x13, 0x00], // External Diode High Limit Low Byte
    [0x14, 0x00], // External Diode Low Limit Low Byte
    [0x16, 0xA4], // Alert Mask
    [0x17, 0x12], // External Diode Ideality Factor
    [0x18, 0x08], // Beta Compensation Factor
    [0x19, 0x55], // TCRIT Temp Limit
    [0x21, 0x0A], // TCRIT Hysteresis
    [0x48, 0xFF], // TACH Limit Low Byte
    [0x49, 0xFF], // TACH Limit High Byte
    [0x4A, 0x20], // FAN Configuration
    [0x4B, 0x3F], // Fan Spin-up
    [0x4C, 0x00], // Fan Setting
    [0x4D, 0x17], // PWM Frequency
    [0x4E, 0x01], // PWM Frequency Divider
    [0x4F, 0x04], // Lookup Table Hysteresis
    [0x50, 0x7F], // Lookup Table Temp Setting 1
    [0x51, 0x3F], // Lookup Table Fan Setting 1
    [0x52, 0x7F], // Lookup Table Temp Setting 2
    [0x53, 0x3F], // Lookup Table Fan Setting 2
    [0x54, 0x7F], // Lookup Table Temp Setting 3
    [0x55, 0x3F], // Lookup Table Fan Setting 3
    [0x56, 0x7F], // Lookup Table Temp Setting 4
    [0x57, 0x3F], // Lookup Table Fan Setting 4
    [0x58, 0x7F], // Lookup Table Temp Setting 5
    [0x59, 0x3F], // Lookup Table Fan Setting 5
    [0x5A, 0x7F], // Lookup Table Temp Setting 6
    [0x5B, 0x3F], // Lookup Table Fan Setting 6
    [0x5C, 0x7F], // Lookup Table Temp Setting 7
    [0x5D, 0x3F], // Lookup Table Fan Setting 7
    [0x5E, 0x7F], // Lookup Table Temp Setting 8
    [0x5F, 0x3F], // Lookup Table Fan Setting 8
    [0xBF, 0x00], // Averaging Filter
];
