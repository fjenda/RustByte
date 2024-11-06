use crate::cpu::mirroring::Mirroring;

/// Class representing the PPU
/// https://www.nesdev.org/wiki/PPU
/// https://www.nesdev.org/wiki/PPU_registers
pub struct PPU {
    /// PPU Memory
    /// 2kB of RAM dedicated to PPU
    ram: [u8; 2048],

    /// Palette tables
    /// 32 bytes of palette data
    palette: [u8; 32],

    /// Visuals of the cartridge
    chr: Vec<u8>,

    /// Internal memory storing sprites
    /// max. 64 sprites (4 bytes each) = 256 bytes
    /// https://www.nesdev.org/wiki/PPU_OAM
    oam: [u8; 256],

    /// Mirroring mode
    /// https://www.nesdev.org/wiki/Mirroring
    mirroring: Mirroring,
}

impl PPU {
    /// Create a new PPU
    pub fn new(chr: Vec<u8>, mirroring: Mirroring) -> Self {
        PPU {
            ram: [0; 2048],
            palette: [0; 32],
            chr,
            oam: [0; 256],
            mirroring,
        }
    }

    // dummy read/write operations
    // PPU can't access rom and ram directly
}