// https://www.nesdev.org/obelisk-6502-guide/reference.html

use crate::cpu::addressing::Addressing;
use crate::cpu::register::Register;
use crate::cpu::cpu_status::{CPUStatus, Status};
use crate::cpu::instructions::{Instruction, INSTRUCTION_MAP};
use crate::cpu::memory::Memory;

/// This class represents the CPU
pub struct CPU {
    /// 3x 8-bit registers A (accumulator), X, Y (indexes)
    pub a: Register,
    pub x: Register,
    pub y: Register,

    /// CPU Status with Flags
    pub status: CPUStatus,

    /// This is also a register, but it's simpler to represent it using u16
    pub prog_counter: u16,

    /// CPU Memory
    pub memory: Memory,
}

impl CPU {
    /// Creates an instance of CPU
    pub fn new() -> Self {
        CPU {
            a: Register::new(),
            x: Register::new(),
            y: Register::new(),
            status: CPUStatus::new(),
            prog_counter: 0,
            memory: Memory::new(),
        }
    }

    /// Function that loads the Program ROM into memory and resets the CPU
    pub fn load_program(&mut self, program: Vec<u8>) -> Result<(), &'static str> {
        // load new program into the memory
        self.memory.load(program)?;

        // reset the cpu
        self.reset();

        // start of the Program ROM
        self.prog_counter = 0x8000;

        Ok(())
    }

    /// Function that loads the Program ROM into the memory and runs the program
    // pub fn run_program(&mut self, program: Vec<u8>) -> Result<(), &'static str> {
    //     // load new program into the memory
    //     self.memory.load(program)?;
    //
    //     // reset the cpu
    //     // self.reset();
    //
    //     // start of the Program ROM
    //     self.prog_counter = 0x8000;
    //
    //     // interpret the program
    //     self.interpret();
    //
    //     Ok(())
    // }

    /// Function that resets the CPU
    pub fn reset(&mut self) {
        // reset the registers
        self.a.reset();
        self.x.reset();
        self.y.reset();

        // reset the status
        self.status.reset();

        // set prog_counter to address at 0xFFFC
        self.prog_counter = self.memory.read_u16(0xFFFC);
    }

    /// Function that handles the logic of setting Zero and Negative flags
    fn zero_negative(&mut self, res: u8) {
        // Zero Flag
        match res {
            0 => self.status.add(Status::Zero),
            _ => self.status.remove(Status::Zero),
        }

        // Negative Flag
        match res & Status::Negative.as_u8() {
            0 => self.status.remove(Status::Negative),
            _ => self.status.add(Status::Negative),
        }
    }

    // https://www.nesdev.org/obelisk-6502-guide/addressing.html
    /// Function that gets the parameter address for a function using its addressing mode
    fn get_param_address(&mut self, mode: &Addressing) -> u16 {
        match mode {
            // Immediate
            Addressing::Immediate => self.prog_counter,

            // Zero Page
            Addressing::ZeroPage => self.memory.read(self.prog_counter) as u16,
            Addressing::ZeroPageX => {
                // u8 value from memory
                let val = self.memory.read(self.prog_counter);

                // add register x value to it (wrap around if needed)
                let param = val.wrapping_add(self.x.value()) as u16;
                param
            },
            Addressing::ZeroPageY => {
                // u8 value from memory
                let val = self.memory.read(self.prog_counter);

                // add register y value to it (wrap around if needed)
                let param = val.wrapping_add(self.y.value()) as u16;
                param
            },

            // Absolute
            Addressing::Absolute => self.memory.read_u16(self.prog_counter),
            Addressing::AbsoluteX => {
                // u16 value from memory
                let val = self.memory.read_u16(self.prog_counter);

                // add register x value to it (wrap around if needed)
                let param = val.wrapping_add(self.x.value() as u16);
                param
            },
            Addressing::AbsoluteY => {
                // u16 value from memory
                let val = self.memory.read_u16(self.prog_counter);

                // add register y value to it (wrap around if needed)
                let param = val.wrapping_add(self.y.value() as u16);
                param
            },

            // Indirect
            Addressing::IndirectX => {
                // u8 value from memory
                let val = self.memory.read(self.prog_counter);

                // index into the memory
                let index: u8 = val.wrapping_add(self.x.value());

                // high and low
                let low = self.memory.read(index as u16);
                let high = self.memory.read(index.wrapping_add(1) as u16);
                u16::from_le_bytes([low, high])
            },
            Addressing::IndirectY => {
                // u8 value from memory
                let val = self.memory.read(self.prog_counter);

                // high and low
                let low = self.memory.read(val as u16);
                let high = self.memory.read(val.wrapping_add(1) as u16);

                // param
                let tmp = u16::from_le_bytes([low, high]);
                let param = tmp.wrapping_add(self.y.value() as u16);
                param
            },

            // None
            Addressing::None => {
                panic!("mode {:?} not supported", mode);
            }
        }
    }

    fn adc(&mut self, mode: &Addressing) {
        todo!()
    }

    fn and(&mut self, mode: &Addressing) {
        todo!()
    }

    fn asl(&mut self) {
        todo!()
    }

    fn bcc(&mut self) {
        todo!()
    }

    fn bcs(&mut self) {
        todo!()
    }

    fn beq(&mut self) {
        todo!()
    }

    fn bit(&mut self) {
        todo!()
    }

    fn bmi(&mut self) {
        todo!()
    }

    fn bne(&mut self) {
        todo!()
    }

    fn bpl(&mut self) {
        todo!()
    }

    // fn brk(&mut self) {
    //     return;
    // }

    fn bvc(&mut self) {
        todo!()
    }

    fn bvs(&mut self) {
        todo!()
    }

    fn clc(&mut self) {
        self.status.remove(Status::Carry);
    }

    fn cld(&mut self) {
        self.status.remove(Status::Decimal);
    }

    fn cli(&mut self) {
        self.status.remove(Status::InterruptDisable);
    }

    fn clv(&mut self) {
        self.status.remove(Status::Overflow)
    }

    fn cmp(&mut self) {
        todo!()
    }

    fn cpx(&mut self) {
        todo!()
    }

    fn cpy(&mut self) {
        todo!()
    }

    fn dec(&mut self) {
        todo!()
    }

    fn dex(&mut self) {
        self.x.subtract(1);
        self.zero_negative(self.x.value());
    }

    fn dey(&mut self) {
        self.y.subtract(1);
        self.zero_negative(self.y.value())
    }

    fn eor(&mut self) {
        todo!()
    }

    fn inc(&mut self) {
        todo!()
    }

    fn inx(&mut self) {
        self.x.add(1);
        self.zero_negative(self.x.value());
    }

    fn iny(&mut self) {
        self.y.add(1);
        self.zero_negative(self.y.value());
    }

    fn jmp(&mut self) {
        todo!()
    }

    fn jsr(&mut self) {
        todo!()
    }

    fn lda(&mut self, mode: &Addressing) {
        // get param from memory
        let address = self.get_param_address(mode);
        let param = self.memory.read(address);

        // set param
        self.a.set(param);
        self.zero_negative(self.a.value());
    }

    fn ldx(&mut self) {
        todo!()
    }

    fn ldy(&mut self) {
        todo!()
    }

    fn lsr(&mut self) {
        todo!()
    }

    // fn nop(&mut self) {
    //     return;
    // }

    fn ora(&mut self) {
        todo!()
    }

    fn pha(&mut self) {
        todo!()
    }

    fn php(&mut self) {
        todo!()
    }

    fn pla(&mut self) {
        todo!()
    }

    fn plp(&mut self) {
        todo!()
    }

    fn rol(&mut self) {
        todo!()
    }

    fn ror(&mut self) {
        todo!()
    }

    fn rti(&mut self) {
        todo!()
    }

    fn rts(&mut self) {
        todo!()
    }

    fn sbc(&mut self) {
        todo!()
    }

    fn sec(&mut self) {
        self.status.add(Status::Carry);
    }

    fn sed(&mut self) {
        self.status.add(Status::Decimal);
    }

    fn sei(&mut self) {
        self.status.add(Status::InterruptDisable);
    }

    fn sta(&mut self) {
        todo!()
    }

    fn stx(&mut self) {
        todo!()
    }

    fn sty(&mut self) {
        todo!()
    }

    fn tax(&mut self) {
        self.x.set(self.a.value());
        self.zero_negative(self.x.value());
    }

    fn tay(&mut self) {
        self.y.set(self.a.value());
        self.zero_negative(self.y.value());
    }

    fn tsx(&mut self) {
        todo!()
    }

    fn txa(&mut self) {
        todo!()
    }

    fn txs(&mut self) {
        todo!()
    }

    fn tya(&mut self) {
        todo!()
    }



    /// Function that interprets the given program
    pub fn interpret(&mut self) {
        loop {
            let ins_code = self.memory.read(self.prog_counter);
            self.prog_counter += 1;

            let ins: &Instruction = INSTRUCTION_MAP.get(&ins_code).expect("Code not recognized");

            match ins_code {
                // ADC
                0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => {
                    // TODO
                    self.adc(&ins.mode);
                },
                // AND
                0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => {
                    // TODO
                    self.and(&ins.mode);
                },
                // ASL
                0x0A | 0x06 | 0x16 | 0x0E | 0x1E => {
                    // TODO
                    self.asl();
                },
                // BCC
                0x90 => {
                    // TODO
                    self.bcc();
                },
                // BCS
                0xB0 => {
                    // TODO
                    self.bcs();
                },
                // BEQ
                0xF0 => {
                    // TODO
                    self.beq();
                },
                // BIT
                0x24 | 0x2C => {
                    // TODO
                    self.bit();
                },
                // BMI
                0x30 => {
                    // TODO
                    self.bmi();
                },
                // BNE
                0xD0 => {
                    // TODO
                    self.bne();
                },
                // BPL
                0x10 => {
                    // TODO
                    self.bpl();
                },
                // BRK
                0x00 => { return; },
                // BVC
                0x50 => {
                    // TODO
                    self.bvc();
                },
                // BVS
                0x70 => {
                    // TODO
                    self.bvs();
                }
                // CLC
                0x18 => self.clc(),
                // CLD
                0xD8 => self.cld(),
                // CLI
                0x58 => self.cli(),
                // CLV
                0xb8 => self.clv(),
                // CMP
                0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => {
                    // TODO
                    self.cmp();
                },
                // CPX
                0xE0 | 0xE4 | 0xEC => {
                    // TODO
                    self.cpx();
                },
                // CPY
                0xC0 | 0xC4 | 0xCC => {
                    // TODO
                    self.cpy();
                },
                // DEC
                0xC6 | 0xD6 | 0xCE | 0xDE => {
                    // TODO
                    self.dec()
                },
                // DEX
                0xCA => self.dex(),
                // DEY
                0x88 => self.dey(),
                // EOR
                0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => {
                    // TODO
                    self.eor();
                },
                // INC
                0xE6 | 0xF6 | 0xEE | 0xFF => {
                    // TODO
                    self.inc();
                },
                // INX
                0xE8 => self.inx(),
                // INY
                0xC8 => self.iny(),
                // JMP
                0x4C | 0x6C => {
                    // TODO
                    self.jmp();
                },
                // JSR
                0x20 => {
                    // TODO
                    self.jsr();
                },
                // LDA
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                    self.lda(&ins.mode);
                },
                // LDX
                0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => {
                    // TODO
                    self.ldx();
                },
                // LDY
                0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => {
                    // TODO
                    self.ldy();
                },
                // LSR
                0x4A | 0x46 | 0x56 | 0x4E | 0x5E => {
                    // TODO
                    self.lsr();
                },
                // NOP
                0xEA => {
                    // self.nop();
                    // no change
                },
                // ORA
                0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => {
                    // TODO
                    self.ora();
                },
                // PHA
                0x48 => {
                    // TODO
                    self.pha();
                },
                // PHP
                0x08 => {
                  // TODO
                    self.php();
                },
                // PLA
                0x68 => {
                    // TODO
                    self.pla();
                },
                // PLP
                0x28 => {
                    // TODO
                    self.plp();
                },
                // ROL
                0x2A | 0x26 | 0x36 | 0x2E | 0x3E => {
                    // TODO
                    self.rol();
                }
                // ROR
                0x6A | 0x66 | 0x76 | 0x6E | 0x7E => {
                    // TODO
                    self.ror();
                },
                // RTI
                0x40 => {
                    // TODO
                    self.rti();
                },
                // RTS
                0x60 => {
                    // TODO
                    self.rts();
                },
                // SBC
                0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => {
                    // TODO
                    self.sbc();
                },
                // SEC
                0x38 => {
                    self.sec();
                },
                // SED
                0xF8 => {
                    self.sed();
                },
                // SEI
                0x78 => {
                    self.sei();
                },
                // STA
                0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
                    // TODO
                    self.sta();
                },
                // STX
                0x86 | 0x96 | 0x8E => {
                    // TODO
                    self.stx();
                },
                // STY
                0x84 | 0x94 | 0x8C => {
                    // TODO
                    self.sty();
                },
                // TAX
                0xAA => self.tax(),
                // TAY
                0xA8 => self.tay(),
                // TSX
                0xBA => {
                    // TODO
                    self.tsx();
                },
                // TXA
                0x8A => {
                    // TODO
                    self.txa();
                },
                // TXS
                0x9A => {
                    // TODO
                    self.txs();
                },
                // TYA
                0x98 => {
                    // TODO
                    self.tya();
                }
                _ => panic!("Unknown code {:?}!", ins_code),
            }

            // increase prog_counter
            self.prog_counter += (ins.bytes - 1) as u16;

        }
    }
}
