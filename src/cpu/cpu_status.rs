// https://www.nesdev.org/wiki/Status_flags

use crate::byte_status::ByteStatus;

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
}

impl ByteStatus for CPUStatus {

    /// Add a flag to the CPU Status
    fn add(&mut self, flag: u8) {
        self.value |= flag;
    }

    /// Remove a flag from the CPU Status
    fn remove(&mut self, flag: u8) {
        self.value &= !flag;
    }

    /// Check if a flag is set in the CPU Status
    fn is_set(&self, status: u8) -> bool {
        self.value & status != 0
    }

    /// Function that resets the status to 0
    fn reset(&mut self) {
        self.value = 0;
    }

    /// Function that sets the status to a specific value
    fn set_bits(&mut self, bits: u8) {
        self.value = bits;
    }
}
