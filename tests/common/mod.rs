/*
    a virtual I²C block device (used for testing without hardware)
*/

pub struct VirtualI2cBusDevice {
    // 'device register' is an 8 bit value, resulting in 256 registers
    // false -> read-only
    // true  -> read-write
    pub registers: [(u8, bool); 256],
}

impl i2c_devices::I2cBusDevice for VirtualI2cBusDevice {
    fn read_byte(&mut self, da: u8) -> Result<u8, &'static str> {
        validate_device_address(da);

        panic!("function not implemented")
    }

    fn write_byte(&mut self, da: u8, _byte: u8) {
        validate_device_address(da);

        panic!("function not implemented")
    }

    fn write_bytes(&mut self, da: u8, _bytes: &[u8]) {
        validate_device_address(da);

        panic!("function not implemented")
    }

    fn read_register_as_byte(&mut self, da: u8, dr: u8) -> u8 {
        validate_device_address(da);

        self.registers[dr as usize].0
    }

    fn write_register_as_byte(&mut self, da: u8, dr: u8, byte: u8) {
        validate_device_address(da);

        if self.registers[dr as usize].1 {
            self.registers[dr as usize].0 = byte;
        } else {
            panic!("attempted write to read-only register {dr:#02X}")
        }
    }

    fn read_multibyte_register_as_u8<const N: usize>(&mut self, da: u8, dr: [u8; N]) -> [u8; N] {
        validate_device_address(da);

        let mut rb = [0u8; N];

        for (i, register) in dr.iter().enumerate() {
            rb[i] = self.registers[*register as usize].0;
        }

        // implicit return
        rb
    }

    fn write_multibyte_register_as_u8<const N: usize>(&mut self, da: u8, values: [[u8; 2]; N]) {
        validate_device_address(da);

        for x in values.iter() {
            let dr = x[0];
            let dv = x[1];

            if self.registers[dr as usize].1 {
                self.registers[dr as usize].0 = dv;
            } else {
                panic!("attempted write to read-only register {dr:#02X}")
            }
        }
    }

    // some hardware functions require a little time to pass
    // - functions that sleep mention this fact in their documentation
    // - sleeping is hardware-dependent, no_std provides no abstraction
    fn sleep_ms(&mut self, _milliseconds: u32) {
        panic!("function not implemented")
    }
}

fn validate_device_address(da: u8) {
    if da != 0x4C {
        panic!("device address must be 0x4C")
    }
}
