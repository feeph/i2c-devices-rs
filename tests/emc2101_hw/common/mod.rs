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
    fn read_bytes<const N: usize>(&mut self, da: u8) -> Option<[u8; N]> {
        validate_device_address(da);

        panic!("function not implemented")
    }

    fn write_bytes<const N: usize>(&mut self, da: u8, bytes: &[u8; N]) -> bool {
        validate_device_address(da);

        if bytes.len() == 2 {
            let dr = bytes[0]; // device register
            let rv = bytes[1]; // register value
            if self.registers[dr as usize].1 {
                self.registers[dr as usize].0 = rv;
                true
            } else {
                panic!("attempted write to read-only register {dr:#02X}")
            }
        } else {
            // too many bytes (expect exactly 2 for all EMC2101 writes)
            false
        }
    }

    // --------------------------------------------------------------------
    // to refactor
    // --------------------------------------------------------------------

    fn read_byte(&mut self, da: u8) -> Result<u8, &'static str> {
        validate_device_address(da);

        panic!("function not implemented")
    }

    fn read_register_as_byte(&mut self, da: u8, dr: u8) -> u8 {
        validate_device_address(da);

        self.registers[dr as usize].0
    }

    fn write_register_as_byte(&mut self, da: u8, dr: u8, byte: u8) {
        self.write_bytes(da, &[dr, byte]);
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
        for bytes in values.iter() {
            self.write_bytes(da, bytes);
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
