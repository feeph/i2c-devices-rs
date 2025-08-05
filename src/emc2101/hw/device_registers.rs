//
// device registers
// (described in section 5 of the data sheet)
//

// TODO remove this pragma once the implementation is complete and all values are used
#[allow(dead_code)]
pub enum DR {
    Its = 0x00,      // internal sensor - temperature
    EtsMsb = 0x01,   // external diode - temperature (high byte)
    Status = 0x02,   // status
    Cfg = 0x03,      // device config register
    Conv = 0x04,     // conversion rate
    ItsHi = 0x05,    // internal sensor - temperature high limit
    EtsHiMsb = 0x07, // external diode - temperature high limit (high byte)
    EtsLoMsb = 0x08, // external diode - temperature low limit (high byte)
    EtsFrc = 0x0C,   // force external temperature reading
    OneShot = 0x0F,  // one shot
    EtsLsb = 0x10,   // external temperature sensor (low byte)
    Scratch1 = 0x11, // scratchpad #1
    Scratch2 = 0x12, // scratchpad #2
    EtsHiLsb = 0x13, // external diode - temperature high limit (low byte)
    EtsLoLsb = 0x14, // external diode - temperature low limit (low byte)
    AlrtMsk = 0x16,  // alert mask
    EtsDif = 0x17,   // external diode - ideality factor
    EtsBcf = 0x18,   // external diode - beta compensation factor
    CritTemp = 0x19, // critical temperature limit
    CritHyst = 0x21, // critical temperature hysteresis
    TachLsb = 0x46,  // tach reading (low byte)
    TachMsb = 0x47,  // tach reading (high byte)
    RpmLoLsb = 0x48, // rpm low limit (low byte)
    RpmLoMsb = 0x49, // rpm low limit (high byte)
    FanCfg = 0x4A,   // fan configuration
    FanSpin = 0x4B,  // fan spin-up strength
    FanSett = 0x4C,  // fan setting
    PwmFrq = 0x4D,   // pwm frequency
    PwmDiv = 0x4E,   // pwm frequency divider
    LtHyst = 0x4F,   // lookup table hysteresis
    LtTemp1 = 0x50,  // lookup table temperature #1
    LtStep1 = 0x51,  // lookup table speed step #1
    LtTemp2 = 0x52,  // lookup table temperature #2
    LtStep2 = 0x53,  // lookup table speed step #2
    LtTemp3 = 0x54,  // lookup table temperature #3
    LtStep3 = 0x55,  // lookup table speed step #3
    LtTemp4 = 0x56,  // lookup table temperature #4
    LtStep4 = 0x57,  // lookup table speed step #4
    LtTemp5 = 0x58,  // lookup table temperature #5
    LtStep5 = 0x59,  // lookup table speed step #5
    LtTemp6 = 0x5A,  // lookup table temperature #6
    LtStep6 = 0x5B,  // lookup table speed step #6
    LtTemp7 = 0x5C,  // lookup table temperature #7
    LtStep7 = 0x5D,  // lookup table speed step #7
    LtTemp8 = 0x5E,  // lookup table temperature #8
    LtStep8 = 0x5F,  // lookup table speed step #8
    AvgFlt = 0xBF,   // averaging filter
    Pid = 0xFD,      // product id
    Mid = 0xFE,      // manufacturer idSag a
    Rev = 0xFF,      // product revision
}
