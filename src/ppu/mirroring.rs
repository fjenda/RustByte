// https://www.nesdev.org/wiki/Mirroring

/// Mirroring modes for the PPU
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Mirroring {
    Horizontal,
    Vertical,
    FourScreen,
    // SingleScreen - only certain mappers
}