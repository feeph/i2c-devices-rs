//
// device registers
// (described in section 5 of the data sheet)
//
pub enum DR {
    Its = 0x00, // internal temperature sensor
    Cfg = 0x03, // device config register
    Pid = 0xFD, // product ID
    Mid = 0xFE, // manufacturer ID
    Rev = 0xFF, // product revision
}
