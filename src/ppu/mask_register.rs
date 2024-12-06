// https://www.nesdev.org/wiki/PPU_registers#PPUMASK_-_Rendering_settings_($2001_write)

use crate::byte_status::ByteStatus;
use crate::flags::Mask;

#[derive(Debug)]
pub struct MaskRegister {
    pub value: u8,
}

pub enum Color {
    Red,
    Green,
    Blue,
}

impl MaskRegister {
    pub fn new() -> Self {
        MaskRegister {
            value: 0b0000_0000,
        }
    }
    
    pub fn color_emphasis(&self) -> Vec<Color> {
        let mut result = Vec::<Color>::new();
        if self.is_set(Mask::Red.as_u8()) {
            result.push(Color::Red);
        }
        
        if self.is_set(Mask::Green.as_u8()) {
            result.push(Color::Green);
        }
        
        if self.is_set(Mask::Blue.as_u8()) {
            result.push(Color::Blue);
        }
        
        result
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