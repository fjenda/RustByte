// https://www.nesdev.org/wiki/Status_flags

use crate::byte_status::ByteStatus;
use crate::flags::Status;

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
    fn add(&mut self, status: Status) {
        self.value |= status.as_u8();
    }

    /// Remove a flag from the CPU Status
    fn remove(&mut self, status: Status) {
        self.value &= !status.as_u8()
    }

    /// Check if a flag is set in the CPU Status
    fn is_set(&self, status: Status) -> bool {
        self.value & status.as_u8() != 0
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
