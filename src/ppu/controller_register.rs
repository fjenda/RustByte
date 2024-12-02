// https://www.nesdev.org/wiki/PPU_registers#PPUCTRL_-_Miscellaneous_settings_($2000_write)

use crate::byte_status::ByteStatus;
use crate::flags::{Settings};

/// Class representing a PPU Controller Register $2000
#[derive(Debug)]
pub struct ControllerRegister {
    pub value: u8,
}

impl ControllerRegister {
    /// Function that creates a new Controller Register
    pub fn new() -> Self {
        ControllerRegister {
            value: 0b0010_0000,
        }
    }

    /// Increment value of VRAM address after accessing memory
    pub fn vram_increment(&self) -> u8 {
        if self.is_set(Settings::VRAM.as_u8()) {
            32
        } else {
            1
        }
    }

    pub fn sprite_pattern_table(&self) -> u16 {
        if self.is_set(Settings::Sprite.as_u8()) {
            0x1000
        } else {
            0
        }
    }

    pub fn background_pattern_table(&self) -> u16 {
        if self.is_set(Settings::Background.as_u8()) {
            0x1000
        } else {
            0
        }
    }

    pub fn sprite_size(&self) -> u8 {
        if self.is_set(Settings::SpriteSize.as_u8()) {
            16
        } else {
            8
        }
    }

    pub fn nametable(&self) -> u16 {
        match self.value & 0b11 {
            0 => 0x2000,
            1 => 0x2400,
            2 => 0x2800,
            3 => 0x2C00,
            _ => panic!("Invalid nametable value"),
        }
    }

    pub fn vblank(&self) -> bool {
        self.is_set(Settings::Vblank.as_u8())
    }

    pub fn master_slave(&self) -> bool {
        self.is_set(Settings::MasterSlave.as_u8())
    }
}

impl ByteStatus for ControllerRegister {
    /// Add a flag to the Controller Register
    fn add(&mut self, flag: u8) {
        self.value |= flag;
    }

    /// Remove a flag from the Controller Register
    fn remove(&mut self, flag: u8) {
        self.value &= !flag
    }

    /// Check if a flag is set in the Controller Register
    fn is_set(&self, flag: u8) -> bool {
        self.value & flag != 0
    }

    /// Function that resets the register to 0
    fn reset(&mut self) {
        self.value = 0;
    }

    /// Function that sets the register to a specific value
    fn set_bits(&mut self, bits: u8) {
        self.value = bits;
    }
}
