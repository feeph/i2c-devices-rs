//
// device registers
// (described in section 5 of the data sheet)
//
pub enum DR {
    Its = 0x00,      // internal sensor - temperature
    EtsMsb = 0x01,   // external diode - temperature (high byte)
    Cfg = 0x03,      // device config register
    ItsHi = 0x05,    // internal sensor - temperature high limit
    EtsHiMsb = 0x07, // external diode - temperature high limit (high byte)
    EtsLoMsb = 0x08, // external diode - temperature low limit (high byte)
    EtsLsb = 0x10,   // external temperature sensor (low byte)
    EtsHiLsb = 0x13, // external diode - temperature high limit (low byte)
    EtsLoLsb = 0x14, // external diode - temperature low limit (low byte)
    Pid = 0xFD,      // product ID
    Mid = 0xFE,      // manufacturer ID
    Rev = 0xFF,      // product revision
}
