//
// device registers
// (described in section 5 of the data sheet)
//

pub enum DR {
    Its = 0x00,       // internal sensor - temperature
    EtsMsb = 0x01,    // external diode - temperature (high byte)
    Status = 0x02,    // status
    Cfg = 0x03,       // device config register
    ConvRate = 0x04,  // conversion rate
    ItsHi = 0x05,     // internal sensor - temperature high limit
    EtsHiMsb = 0x07,  // external diode - temperature high limit (high byte)
    EtsLoMsb = 0x08,  // external diode - temperature low limit (high byte)
    EtsFrc = 0x0C,    // force external temperature reading
    OneShot = 0x0F,   // one shot
    EtsLsb = 0x10,    // external temperature sensor (low byte)
    Scratch1 = 0x11,  // scratchpad #1
    Scratch2 = 0x12,  // scratchpad #2
    EtsHiLsb = 0x13,  // external diode - temperature high limit (low byte)
    EtsLoLsb = 0x14,  // external diode - temperature low limit (low byte)
    AlrtMsk = 0x16,   // alert mask
    EtsDif = 0x17,    // external diode - ideality factor
    EtsBcf = 0x18,    // external diode - beta compensation factor
    CritTemp = 0x19,  // critical temperature limit
    CritHyst = 0x21,  // critical temperature hysteresis
    TachLsb = 0x46,   // tach reading (low byte)
    TachMsb = 0x47,   // tach reading (high byte)
    TachLoLsb = 0x48, // rpm low limit (low byte)
    TachLoMsb = 0x49, // rpm low limit (high byte)
    FanCfg = 0x4A,    // fan configuration
    FanSpinUp = 0x4B, // fan spin-up configuration
    FanSpeed = 0x4C,  // fan speed setting
    PwmFrq = 0x4D,    // pwm frequency
    PwmFrqDiv = 0x4E, // pwm frequency divider
    LutHyst = 0x4F,   // lookup table hysteresis
    LutBase = 0x50,   // lookup table base address (0x50..0x5F)
    AvgFlt = 0xBF,    // averaging filter
    Pid = 0xFD,       // product id
    Mid = 0xFE,       // manufacturer idSag a
    Rev = 0xFF,       // product revision
}
