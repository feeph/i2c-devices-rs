//
// device registers
// (described in section 5 of the data sheet)
//
pub enum DR {
    CFG = 0x03,  // device config register
    PID = 0xFD,  // product ID
    MID = 0xFE,  // manufacturer ID
    PRV = 0xFF,  // product revision
}
