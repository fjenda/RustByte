use crate::byte_status::ByteStatus;

/// Class representing the button status
#[derive(Debug)]
pub struct ButtonStatus {
    pub value: u8,
}

impl Default for ButtonStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl ButtonStatus {
    pub fn new() -> Self {
        ButtonStatus {
            value: 0b0000_0000,
        }
    }
}

impl ByteStatus for ButtonStatus {
    fn add(&mut self, flag: u8) {
        self.value |= flag;
    }

    fn remove(&mut self, flag: u8) {
        self.value &= !flag;
    }

    fn is_set(&self, status: u8) -> bool {
        self.value & status != 0
    }

    fn reset(&mut self) {
        self.value = 0b0000_0000;
    }

    fn set_bits(&mut self, bits: u8) {
        self.value = bits;
    }
}

