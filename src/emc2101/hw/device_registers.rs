//
// device registers
// (described in section 5 of the data sheet)
//
pub enum DR {
    Cfg = 0x03, // device config register
    Pid = 0xFD, // product ID
    Mid = 0xFE, // manufacturer ID
    Rev = 0xFF, // product revision
}
