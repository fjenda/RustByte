// https://www.nesdev.org/wiki/PPU_registers#PPUCTRL_-_Miscellaneous_settings_($2000_write)

use crate::byte_status::ByteStatus;
use crate::flags::Settings;

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
        if self.is_set(Settings::VRAM) {
            32
        } else {
            1
        }
    }

    pub fn sprite_pattern_table(&self) -> u16 {
        if self.is_set(Settings::Sprite) {
            0x1000
        } else {
            0
        }
    }

    pub fn background_pattern_table(&self) -> u16 {
        if self.is_set(Settings::Background) {
            0x1000
        } else {
            0
        }
    }

    pub fn sprite_size(&self) -> u8 {
        if self.is_set(Settings::SpriteSize) {
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
        self.is_set(Settings::Vblank)
    }

    pub fn master_slave(&self) -> bool {
        self.is_set(Settings::MasterSlave)
    }
}

impl ByteStatus for ControllerRegister {
    /// Add a flag to the Controller Register
    fn add(&mut self, status: Settings) {
        self.value |= status.as_u8();
    }

    /// Remove a flag from the Controller Register
    fn remove(&mut self, status: Settings) {
        self.value &= !status.as_u8()
    }

    /// Check if a flag is set in the Controller Register
    fn is_set(&self, status: Settings) -> bool {
        self.value & status.as_u8() != 0
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
