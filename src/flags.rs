// https://www.nesdev.org/wiki/Status_flags
/// Actual CPU Flags and their values
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

// https://www.nesdev.org/wiki/PPU_registers#PPUCTRL_-_Miscellaneous_settings_($2000_write)

/// Enum representing the controller settings for the PPU
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Settings {
    Nametable1      = 0b0000_0001,
    Nametable2      = 0b0000_0010,
    VRAM            = 0b0000_0100,
    Sprite          = 0b0000_1000,
    Background      = 0b0001_0000,
    SpriteSize      = 0b0010_0000,
    MasterSlave     = 0b0100_0000,
    Vblank          = 0b1000_0000,
}

impl Settings {
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

// https://www.nesdev.org/wiki/PPU_registers#PPUMASK_-_Rendering_settings_($2001_write)

/// Enum representing the mask settings for the PPU
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Mask {
    Greyscale       = 0b0000_0001,
    BackgroundLeft  = 0b0000_0010,
    SpriteLeft      = 0b0000_0100,
    Background      = 0b0000_1000,
    Sprite          = 0b0001_0000,
    Red             = 0b0010_0000,
    Green           = 0b0100_0000,
    Blue            = 0b1000_0000,
}

impl Mask {
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

// https://www.nesdev.org/wiki/PPU_registers#PPUSTATUS_-_Rendering_events_($2002_read)

/// Enum representing the status settings for the PPU
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum PPUStatus {
    SpriteOverflow  = 0b0100_0000,
    Sprite0Hit      = 0b0001_0000,
    Vblank          = 0b1000_0000,
}

impl PPUStatus {
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}
