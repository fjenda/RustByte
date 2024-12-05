// https://www.nesdev.org/wiki/PPU_registers#PPUADDR_-_VRAM_address_($2006_write)

/// Class representing a PPU Address Register $2006
#[derive(Debug)]
pub struct AddressRegister {
    /// 2-byte regiter (u8, u8)
    /// order: high byte, low byte
    value: (u8, u8),

    /// Flag for high byte
    high_byte: bool,
}

impl AddressRegister {
    pub fn new() -> Self {
        AddressRegister {
            value: (0, 0),
            high_byte: true,
        }
    }

    /// Reads the value of the register
    pub fn get(&self) -> u16 {
        u16::from_be_bytes([self.value.0, self.value.1])
    }

    /// Internal function that sets the value of the register
    fn internal_set(&mut self, val: u16) {
        let bytes = val.to_be_bytes();
        self.value = (bytes[0], bytes[1]);
    }

    /// Resets the high byte flag
    pub fn reset_high_byte(&mut self) {
        self.high_byte = true;
    }

    /// Sets the value of the register
    pub fn set(&mut self, val: u8) {
        // check if the high byte is set
        if self.high_byte {
            self.value.0 = val;
        } else {
            self.value.1 = val;
        }

        // check if the address needs to be mirrored down
        // 0x3FFF
        if self.get() > 0x3FFF {
            self.internal_set(self.get() & 0x3FFF);
        }

        // flip the high byte flag
        self.high_byte = !self.high_byte;
    }

    /// Increments the value of the register by value
    pub fn add(&mut self, val: u8) {
        // get low part
        let lo = self.value.1;

        // wrapping add the value
        self.value.1 = lo.wrapping_add(val);

        // check if the high byte needs to be incremented
        if self.value.1 < lo {
            self.value.0 = self.value.0.wrapping_add(1);
        }

        // check if the address needs to be mirrored down
        if self.get() > 0x3FFF {
            self.internal_set(self.get() & 0x3FFF);
        }
    }
}