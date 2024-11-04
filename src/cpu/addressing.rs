// https://www.nesdev.org/wiki/CPU_addressing_modes
// https://www.nesdev.org/obelisk-6502-guide/addressing.html


/// Addressing modes for the CPU
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Addressing {
    None,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
}
