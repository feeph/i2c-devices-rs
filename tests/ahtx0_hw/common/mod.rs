/*
    a virtual I²C block device (used for testing without hardware)
*/

pub struct VirtualAHTx0 {
    pub data: [u8; 7], // internal data
    pub lwb: [u8; 3],  // last written bytes
}

impl i2c_devices::I2cBusDevice for VirtualAHTx0 {
    fn read_bytes<const N: usize>(&mut self, da: u8) -> Option<[u8; N]> {
        validate_device_address(da);

        let mut bytes = [0x00; N];
        bytes[0..N].clone_from_slice(&self.data[0..N]);

        Some(bytes)
    }

    fn write_bytes<const N: usize>(&mut self, da: u8, bytes: &[u8; N]) -> bool {
        validate_device_address(da);

        if bytes.len() <= self.lwb.len() {
            // memcpy(self.lwb, bytes, bytes.len())
            self.lwb[0..bytes.len()].clone_from_slice(&bytes[..]);
            true
        } else {
            // input too long
            false
        }
    }

    fn write_and_read_bytes<const N: usize>(&mut self, da: u8, _bytes: &[u8]) -> Option<[u8; N]> {
        validate_device_address(da);

        panic!("read_byte(): function not implemented")
    }

    // --------------------------------------------------------------------
    // to refactor
    // --------------------------------------------------------------------

    // some hardware functions require a little time to pass
    // - functions that sleep mention this fact in their documentation
    // - sleeping is hardware-dependent, no_std provides no abstraction
    fn sleep_ms(&mut self, _milliseconds: u32) {
        panic!("sleep_ms(): function not implemented")
    }
}

fn validate_device_address(da: u8) {
    if da != 0x38 {
        panic!("device address must be 0x38")
    }
}
