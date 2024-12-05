// https://www.nesdev.org/wiki/PPU_registers#PPUSTATUS_-_Rendering_events_($2002_read)

use crate::byte_status::ByteStatus;

/// Class representing a PPU Status Register $2002

#[derive(Debug)]
pub struct StatusRegister {
    pub value: u8,
}

impl StatusRegister {
    pub fn new() -> Self {
        StatusRegister {
            value: 0b0000_0000,
        }
    }
}

impl ByteStatus for StatusRegister {

    fn add(&mut self, flag: u8) {
        self.value |= flag;
    }

    fn remove(&mut self, flag: u8) {
        self.value &= !flag;
    }

    fn is_set(&self, flag: u8) -> bool {
        self.value & flag != 0
    }

    fn reset(&mut self) {
        self.value = 0;
    }

    fn set_bits(&mut self, bits: u8) {
        self.value = bits;
    }
}