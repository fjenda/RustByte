// https://www.nesdev.org/wiki/Status_flags

/// Actual Flags and their values
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Status {
    Carry                   = 0b0000_0001,
    Zero                    = 0b0000_0010,
    InterruptDisable        = 0b0000_0100,
    Decimal                 = 0b0000_1000,
    Break                   = 0b0001_0000,
    Overflow                = 0b0100_0000,
    Negative                = 0b1000_0000,
}
impl Status {
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

/// Class representing a CPU Status
pub struct CPUStatus {
    pub value: u8,
}

impl CPUStatus {
    /// Create a new CPU Status
    pub fn new() -> Self {
        CPUStatus {
            value: 0b0010_0000,
        }
    }

    /// Add a flag to the CPU Status
    pub fn add(&mut self, status: Status) {
        self.value |= status.as_u8();
    }

    /// Remove a flag from the CPU Status
    pub fn remove(&mut self, status: Status) {
        self.value &= !status.as_u8()
    }

    /// Check if a flag is set in the CPU Status
    pub fn is_set(&self, status: Status) -> bool {
        self.value & status.as_u8() != 0
    }

    /// Function that resets the status to 0
    pub fn reset(&mut self) {
        self.value = 0;
    }

    /// Function that sets the status to a specific value
    pub fn set_bits(&mut self, bits: u8) {
        self.value = bits;
    }
}
