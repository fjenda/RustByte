// https://www.nesdev.org/wiki/PPU_registers#PPUMASK_-_Rendering_settings_($2001_write)

use crate::byte_status::ByteStatus;
use crate::flags::Mask;

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

    fn add(&mut self, status: Mask) {
        self.value |= status.as_u8();
    }

    fn remove(&mut self, status: Mask) {
        self.value &= !status.as_u8()
    }

    fn is_set(&self, status: Mask) -> bool {
        self.value & status.as_u8() != 0
    }

    fn reset(&mut self) {
        self.value = 0;
    }

    fn set_bits(&mut self, bits: u8) {
        self.value = bits;
    }
}