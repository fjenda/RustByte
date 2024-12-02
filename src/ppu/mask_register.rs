// https://www.nesdev.org/wiki/PPU_registers#PPUMASK_-_Rendering_settings_($2001_write)

use crate::byte_status::ByteStatus;

#[derive(Debug)]
pub struct MaskRegister {
    pub value: u8,
}

impl MaskRegister {
    pub fn new() -> Self {
        MaskRegister {
            value: 0,
        }
    }
}

impl ByteStatus for MaskRegister {

    fn add(&mut self, flag: u8) {
        self.value |= flag;
    }

    fn remove(&mut self, flag: u8) {
        self.value &= !flag
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