/*
    a virtual I²C block device (used for testing without hardware)
*/

pub struct VirtualHt16K33 {
    pub dda: [u8; 16], // 0x00
    pub osc: u8,       // system setup register (0x20) (oscillator mode)
    pub dis: u8,       // display setup register (0x80)
    pub ris: u8,       // ROW/INT set register (0xA0)
    pub dim: u8,       // dimming set (0xE0)
}

impl i2c_devices::I2cBusDevice for VirtualHt16K33 {
    fn read_byte(&mut self, da: u8) -> Result<u8, &'static str> {
        validate_device_address(da);

        panic!("function not implemented")
    }

    fn write_byte(&mut self, da: u8, byte: u8) {
        validate_device_address(da);

        let register = byte & 0xF0;
        let value = byte & 0x0F;
        match register {
            0x20 => self.osc = value,
            0x80 => self.dis = value,
            0xA0 => self.ris = value,
            0xE0 => self.dim = value,
            _ => panic!("invalid register"),
        }
    }

    fn write_bytes(&mut self, da: u8, bytes: &[u8]) {
        validate_device_address(da);

        // validate the first byte, copy the remaining 16
        if bytes[0] == 0x00 {
            self.dda[0] = bytes[1];
            self.dda[1] = bytes[2];
            self.dda[2] = bytes[3];
            self.dda[3] = bytes[4];
            self.dda[4] = bytes[5];
            self.dda[5] = bytes[6];
            self.dda[6] = bytes[7];
            self.dda[7] = bytes[8];
            self.dda[8] = bytes[9];
            self.dda[9] = bytes[10];
            self.dda[10] = bytes[11];
            self.dda[11] = bytes[12];
            self.dda[12] = bytes[13];
            self.dda[13] = bytes[14];
            self.dda[14] = bytes[15];
            self.dda[15] = bytes[16];
        } else {
            panic!("invalid write")
        }
    }

    fn read_register_as_byte(&mut self, da: u8, _dr: u8) -> u8 {
        validate_device_address(da);

        panic!("function not implemented")
    }

    fn write_register_as_byte(&mut self, da: u8, _dr: u8, _byte: u8) {
        validate_device_address(da);

        panic!("function not implemented")
    }

    fn read_multibyte_register_as_u8<const N: usize>(&mut self, da: u8, _dr: [u8; N]) -> [u8; N] {
        validate_device_address(da);

        panic!("function not implemented")
    }

    fn write_multibyte_register_as_u8<const N: usize>(&mut self, da: u8, _values: [[u8; 2]; N]) {
        validate_device_address(da);

        panic!("function not implemented")
        // for x in values.iter() {
        //     let dr = x[0];
        //     let dv = x[1];

        //     if self.registers[dr as usize].1 {
        //         self.registers[dr as usize].0 = dv;
        //     } else {
        //         panic!("attempted write to read-only register {dr:#02X}")
        //     }
        // }
    }

    // some hardware functions require a little time to pass
    // - functions that sleep mention this fact in their documentation
    // - sleeping is hardware-dependent, no_std provides no abstraction
    fn sleep_ms(&mut self, _milliseconds: u32) {
        panic!("function not implemented")
    }
}

fn validate_device_address(da: u8) {
    if !(0x70..0x77).contains(&da) {
        panic!("device address must be in range 0x70 ≤ x ≤ 0x77")
    }
}
