// https://www.nesdev.org/wiki/Status_flags

use crate::byte_status::ByteStatus;
use std::fmt::{Display, Formatter};

/// Class representing a CPU Status
#[derive(Debug, Clone)]
pub struct CPUStatus {
    pub value: u8,
}

impl Default for CPUStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl CPUStatus {
    /// Create a new CPU Status
    pub fn new() -> Self {
        CPUStatus {
            value: 0b0010_0100,
        }
    }

    pub fn set(&mut self, flag: u8, cond: bool) {
        if cond {
            self.add(flag);
        } else {
            self.remove(flag);
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
        self.value = 0b0010_0100;
    }

    /// Function that sets the status to a specific value
    fn set_bits(&mut self, bits: u8) {
        self.value = bits;
    }
}

impl Display for CPUStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08b}", self.value)
    }
}
