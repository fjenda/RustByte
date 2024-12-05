// https://www.nesdev.org/obelisk-6502-guide/reference.html

use crate::cpu::addressing::Addressing;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

// Kindly borrowed and modified from https://github.com/bugzmanov/nes_ebook/blob/master/code/ch8/src/opcodes.rs
// I can't be bothered to write all this out
lazy_static! {
    pub static ref INSTRUCTION_MAP: HashMap<u8, &'static Instruction> = {
        let mut map = HashMap::new();
        for op in &*INSTRUCTIONS {
            map.insert(op.address, op);
        }
        map
    };

    pub static ref INSTRUCTIONS: Vec<Instruction> = vec![
        // Instruction::new(0x1a, OpName::NOP, 1, 2, Addressing::None),

        Instruction::new(0x00, OpName::BRK, 1, 7, Addressing::None),
        Instruction::new(0xea, OpName::NOP, 1, 2, Addressing::None),

        /* Arithmetic */
        Instruction::new(0x69, OpName::ADC, 2, 2, Addressing::Immediate),
        Instruction::new(0x65, OpName::ADC, 2, 3, Addressing::ZeroPage),
        Instruction::new(0x75, OpName::ADC, 2, 4, Addressing::ZeroPageX),
        Instruction::new(0x6d, OpName::ADC, 3, 4, Addressing::Absolute),
        Instruction::new(0x7d, OpName::ADC, 3, 4/*+1 if page crossed*/, Addressing::AbsoluteX),
        Instruction::new(0x79, OpName::ADC, 3, 4/*+1 if page crossed*/, Addressing::AbsoluteY),
        Instruction::new(0x61, OpName::ADC, 2, 6, Addressing::IndirectX),
        Instruction::new(0x71, OpName::ADC, 2, 5/*+1 if page crossed*/, Addressing::IndirectY),

        Instruction::new(0xe9, OpName::SBC, 2, 2, Addressing::Immediate),
        Instruction::new(0xe5, OpName::SBC, 2, 3, Addressing::ZeroPage),
        Instruction::new(0xf5, OpName::SBC, 2, 4, Addressing::ZeroPageX),
        Instruction::new(0xed, OpName::SBC, 3, 4, Addressing::Absolute),
        Instruction::new(0xfd, OpName::SBC, 3, 4/*+1 if page crossed*/, Addressing::AbsoluteX),
        Instruction::new(0xf9, OpName::SBC, 3, 4/*+1 if page crossed*/, Addressing::AbsoluteY),
        Instruction::new(0xe1, OpName::SBC, 2, 6, Addressing::IndirectX),
        Instruction::new(0xf1, OpName::SBC, 2, 5/*+1 if page crossed*/, Addressing::IndirectY),

        Instruction::new(0x29, OpName::AND, 2, 2, Addressing::Immediate),
        Instruction::new(0x25, OpName::AND, 2, 3, Addressing::ZeroPage),
        Instruction::new(0x35, OpName::AND, 2, 4, Addressing::ZeroPageX),
        Instruction::new(0x2d, OpName::AND, 3, 4, Addressing::Absolute),
        Instruction::new(0x3d, OpName::AND, 3, 4/*+1 if page crossed*/, Addressing::AbsoluteX),
        Instruction::new(0x39, OpName::AND, 3, 4/*+1 if page crossed*/, Addressing::AbsoluteY),
        Instruction::new(0x21, OpName::AND, 2, 6, Addressing::IndirectX),
        Instruction::new(0x31, OpName::AND, 2, 5/*+1 if page crossed*/, Addressing::IndirectY),

        Instruction::new(0x49, OpName::EOR, 2, 2, Addressing::Immediate),
        Instruction::new(0x45, OpName::EOR, 2, 3, Addressing::ZeroPage),
        Instruction::new(0x55, OpName::EOR, 2, 4, Addressing::ZeroPageX),
        Instruction::new(0x4d, OpName::EOR, 3, 4, Addressing::Absolute),
        Instruction::new(0x5d, OpName::EOR, 3, 4/*+1 if page crossed*/, Addressing::AbsoluteX),
        Instruction::new(0x59, OpName::EOR, 3, 4/*+1 if page crossed*/, Addressing::AbsoluteY),
        Instruction::new(0x41, OpName::EOR, 2, 6, Addressing::IndirectX),
        Instruction::new(0x51, OpName::EOR, 2, 5/*+1 if page crossed*/, Addressing::IndirectY),

        Instruction::new(0x09, OpName::ORA, 2, 2, Addressing::Immediate),
        Instruction::new(0x05, OpName::ORA, 2, 3, Addressing::ZeroPage),
        Instruction::new(0x15, OpName::ORA, 2, 4, Addressing::ZeroPageX),
        Instruction::new(0x0d, OpName::ORA, 3, 4, Addressing::Absolute),
        Instruction::new(0x1d, OpName::ORA, 3, 4/*+1 if page crossed*/, Addressing::AbsoluteX),
        Instruction::new(0x19, OpName::ORA, 3, 4/*+1 if page crossed*/, Addressing::AbsoluteY),
        Instruction::new(0x01, OpName::ORA, 2, 6, Addressing::IndirectX),
        Instruction::new(0x11, OpName::ORA, 2, 5/*+1 if page crossed*/, Addressing::IndirectY),

        /* Shifts */
        Instruction::new(0x0a, OpName::ASL_A, 1, 2, Addressing::None),
        Instruction::new(0x06, OpName::ASL, 2, 5, Addressing::ZeroPage),
        Instruction::new(0x16, OpName::ASL, 2, 6, Addressing::ZeroPageX),
        Instruction::new(0x0e, OpName::ASL, 3, 6, Addressing::Absolute),
        Instruction::new(0x1e, OpName::ASL, 3, 7, Addressing::AbsoluteX),

        Instruction::new(0x4a, OpName::LSR_A, 1, 2, Addressing::None),
        Instruction::new(0x46, OpName::LSR, 2, 5, Addressing::ZeroPage),
        Instruction::new(0x56, OpName::LSR, 2, 6, Addressing::ZeroPageX),
        Instruction::new(0x4e, OpName::LSR, 3, 6, Addressing::Absolute),
        Instruction::new(0x5e, OpName::LSR, 3, 7, Addressing::AbsoluteX),

        Instruction::new(0x2a, OpName::ROL_A, 1, 2, Addressing::None),
        Instruction::new(0x26, OpName::ROL, 2, 5, Addressing::ZeroPage),
        Instruction::new(0x36, OpName::ROL, 2, 6, Addressing::ZeroPageX),
        Instruction::new(0x2e, OpName::ROL, 3, 6, Addressing::Absolute),
        Instruction::new(0x3e, OpName::ROL, 3, 7, Addressing::AbsoluteX),

        Instruction::new(0x6a, OpName::ROR_A, 1, 2, Addressing::None),
        Instruction::new(0x66, OpName::ROR, 2, 5, Addressing::ZeroPage),
        Instruction::new(0x76, OpName::ROR, 2, 6, Addressing::ZeroPageX),
        Instruction::new(0x6e, OpName::ROR, 3, 6, Addressing::Absolute),
        Instruction::new(0x7e, OpName::ROR, 3, 7, Addressing::AbsoluteX),

        Instruction::new(0xe6, OpName::INC, 2, 5, Addressing::ZeroPage),
        Instruction::new(0xf6, OpName::INC, 2, 6, Addressing::ZeroPageX),
        Instruction::new(0xee, OpName::INC, 3, 6, Addressing::Absolute),
        Instruction::new(0xfe, OpName::INC, 3, 7, Addressing::AbsoluteX),

        Instruction::new(0xe8, OpName::INX, 1, 2, Addressing::None),
        Instruction::new(0xc8, OpName::INY, 1, 2, Addressing::None),

        Instruction::new(0xc6, OpName::DEC, 2, 5, Addressing::ZeroPage),
        Instruction::new(0xd6, OpName::DEC, 2, 6, Addressing::ZeroPageX),
        Instruction::new(0xce, OpName::DEC, 3, 6, Addressing::Absolute),
        Instruction::new(0xde, OpName::DEC, 3, 7, Addressing::AbsoluteX),

        Instruction::new(0xca, OpName::DEX, 1, 2, Addressing::None),
        Instruction::new(0x88, OpName::DEY, 1, 2, Addressing::None),

        Instruction::new(0xc9, OpName::CMP, 2, 2, Addressing::Immediate),
        Instruction::new(0xc5, OpName::CMP, 2, 3, Addressing::ZeroPage),
        Instruction::new(0xd5, OpName::CMP, 2, 4, Addressing::ZeroPageX),
        Instruction::new(0xcd, OpName::CMP, 3, 4, Addressing::Absolute),
        Instruction::new(0xdd, OpName::CMP, 3, 4/*+1 if page crossed*/, Addressing::AbsoluteX),
        Instruction::new(0xd9, OpName::CMP, 3, 4/*+1 if page crossed*/, Addressing::AbsoluteY),
        Instruction::new(0xc1, OpName::CMP, 2, 6, Addressing::IndirectX),
        Instruction::new(0xd1, OpName::CMP, 2, 5/*+1 if page crossed*/, Addressing::IndirectY),

        Instruction::new(0xc0, OpName::CPY, 2, 2, Addressing::Immediate),
        Instruction::new(0xc4, OpName::CPY, 2, 3, Addressing::ZeroPage),
        Instruction::new(0xcc, OpName::CPY, 3, 4, Addressing::Absolute),

        Instruction::new(0xe0, OpName::CPX, 2, 2, Addressing::Immediate),
        Instruction::new(0xe4, OpName::CPX, 2, 3, Addressing::ZeroPage),
        Instruction::new(0xec, OpName::CPX, 3, 4, Addressing::Absolute),


        /* Branching */
        Instruction::new(0x4c, OpName::JMP_ABS, 3, 3, Addressing::None), // Addressing that acts as Immediate
        Instruction::new(0x6c, OpName::JMP_IND, 3, 5, Addressing::None), // Addressing:Indirect with 6502 bug

        Instruction::new(0x20, OpName::JSR, 3, 6, Addressing::None),
        Instruction::new(0x60, OpName::RTS, 1, 6, Addressing::None),

        Instruction::new(0x40, OpName::RTI, 1, 6, Addressing::None),

        Instruction::new(0xd0, OpName::BNE, 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, Addressing::None),
        Instruction::new(0x70, OpName::BVS, 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, Addressing::None),
        Instruction::new(0x50, OpName::BVC, 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, Addressing::None),
        Instruction::new(0x30, OpName::BMI, 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, Addressing::None),
        Instruction::new(0xf0, OpName::BEQ, 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, Addressing::None),
        Instruction::new(0xb0, OpName::BCS, 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, Addressing::None),
        Instruction::new(0x90, OpName::BCC, 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, Addressing::None),
        Instruction::new(0x10, OpName::BPL, 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, Addressing::None),

        Instruction::new(0x24, OpName::BIT, 2, 3, Addressing::ZeroPage),
        Instruction::new(0x2c, OpName::BIT, 3, 4, Addressing::Absolute),


        /* Stores, Loads */
        Instruction::new(0xa9, OpName::LDA, 2, 2, Addressing::Immediate),
        Instruction::new(0xa5, OpName::LDA, 2, 3, Addressing::ZeroPage),
        Instruction::new(0xb5, OpName::LDA, 2, 4, Addressing::ZeroPageX),
        Instruction::new(0xad, OpName::LDA, 3, 4, Addressing::Absolute),
        Instruction::new(0xbd, OpName::LDA, 3, 4/*+1 if page crossed*/, Addressing::AbsoluteX),
        Instruction::new(0xb9, OpName::LDA, 3, 4/*+1 if page crossed*/, Addressing::AbsoluteY),
        Instruction::new(0xa1, OpName::LDA, 2, 6, Addressing::IndirectX),
        Instruction::new(0xb1, OpName::LDA, 2, 5/*+1 if page crossed*/, Addressing::IndirectY),

        Instruction::new(0xa2, OpName::LDX, 2, 2, Addressing::Immediate),
        Instruction::new(0xa6, OpName::LDX, 2, 3, Addressing::ZeroPage),
        Instruction::new(0xb6, OpName::LDX, 2, 4, Addressing::ZeroPageY),
        Instruction::new(0xae, OpName::LDX, 3, 4, Addressing::Absolute),
        Instruction::new(0xbe, OpName::LDX, 3, 4/*+1 if page crossed*/, Addressing::AbsoluteY),

        Instruction::new(0xa0, OpName::LDY, 2, 2, Addressing::Immediate),
        Instruction::new(0xa4, OpName::LDY, 2, 3, Addressing::ZeroPage),
        Instruction::new(0xb4, OpName::LDY, 2, 4, Addressing::ZeroPageX),
        Instruction::new(0xac, OpName::LDY, 3, 4, Addressing::Absolute),
        Instruction::new(0xbc, OpName::LDY, 3, 4/*+1 if page crossed*/, Addressing::AbsoluteX),


        Instruction::new(0x85, OpName::STA, 2, 3, Addressing::ZeroPage),
        Instruction::new(0x95, OpName::STA, 2, 4, Addressing::ZeroPageX),
        Instruction::new(0x8d, OpName::STA, 3, 4, Addressing::Absolute),
        Instruction::new(0x9d, OpName::STA, 3, 5, Addressing::AbsoluteX),
        Instruction::new(0x99, OpName::STA, 3, 5, Addressing::AbsoluteY),
        Instruction::new(0x81, OpName::STA, 2, 6, Addressing::IndirectX),
        Instruction::new(0x91, OpName::STA, 2, 6, Addressing::IndirectY),

        Instruction::new(0x86, OpName::STX, 2, 3, Addressing::ZeroPage),
        Instruction::new(0x96, OpName::STX, 2, 4, Addressing::ZeroPageY),
        Instruction::new(0x8e, OpName::STX, 3, 4, Addressing::Absolute),

        Instruction::new(0x84, OpName::STY, 2, 3, Addressing::ZeroPage),
        Instruction::new(0x94, OpName::STY, 2, 4, Addressing::ZeroPageX),
        Instruction::new(0x8c, OpName::STY, 3, 4, Addressing::Absolute),


        /* Flags clear */
        Instruction::new(0xD8, OpName::CLD, 1, 2, Addressing::None),
        Instruction::new(0x58, OpName::CLI, 1, 2, Addressing::None),
        Instruction::new(0xb8, OpName::CLV, 1, 2, Addressing::None),
        Instruction::new(0x18, OpName::CLC, 1, 2, Addressing::None),
        Instruction::new(0x38, OpName::SEC, 1, 2, Addressing::None),
        Instruction::new(0x78, OpName::SEI, 1, 2, Addressing::None),
        Instruction::new(0xf8, OpName::SED, 1, 2, Addressing::None),

        Instruction::new(0xaa, OpName::TAX, 1, 2, Addressing::None),
        Instruction::new(0xa8, OpName::TAY, 1, 2, Addressing::None),
        Instruction::new(0xba, OpName::TSX, 1, 2, Addressing::None),
        Instruction::new(0x8a, OpName::TXA, 1, 2, Addressing::None),
        Instruction::new(0x9a, OpName::TXS, 1, 2, Addressing::None),
        Instruction::new(0x98, OpName::TYA, 1, 2, Addressing::None),

        /* Stack */
        Instruction::new(0x48, OpName::PHA, 1, 3, Addressing::None),
        Instruction::new(0x68, OpName::PLA, 1, 4, Addressing::None),
        Instruction::new(0x08, OpName::PHP, 1, 3, Addressing::None),
        Instruction::new(0x28, OpName::PLP, 1, 4, Addressing::None),
    ];
}


#[derive(Debug)]
pub struct Instruction {
    pub address: u8,
    pub name: OpName,
    pub bytes: u8,
    pub cycles: u8,
    pub mode: Addressing,
}

impl Instruction {
    pub fn new(address: u8, name: OpName, bytes: u8, cycles: u8, mode: Addressing) -> Self {
        Instruction {
            address,
            name,
            bytes,
            cycles,
            mode,
        }
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum OpName {
    ADC,
    AND,
    ASL_A,
    ASL,
    BIT,
    BCS,
    BCC,
    BEQ,
    BNE,
    BMI,
    BPL,
    BVS,
    BVC,
    BRK,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP_ABS,
    JMP_IND,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR_A,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL_A,
    ROL,
    ROR_A,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
}

impl Display for OpName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = format!("{:?}", self);
        let first = name.split('_').next().unwrap_or(&name);
        write!(f, "{}", first)
    }
}