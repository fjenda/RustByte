// https://www.nesdev.org/wiki/PPU_registers#PPUADDR_-_VRAM_address_($2006_write)
// https://www.nesdev.org/wiki/PPU_registers#PPUDATA_-_VRAM_data_($2007_read/write)

/// Class representing a PPU Register
/// $2006, $2007, $4014
pub struct PPURegister {
    /// 2-byte regiter (u8, u8)
    /// order: high byte, low byte
    value: (u8, u8),
}