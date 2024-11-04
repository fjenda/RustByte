// https://www.nesdev.org/obelisk-6502-guide/reference.html

use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use crate::cpu::addressing::Addressing;
use crate::cpu::register::Register;
use crate::cpu::cpu_status::{CPUStatus, Status};
use crate::cpu::instructions::{Instruction, INSTRUCTION_MAP, OpName::*};
use crate::cpu::memory::Memory;
use crate::cpu::cpu_stack::CPUStack;

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

    // TODO: Stack Pointer
    // 0x0100 - 0x01FF
    pub stack: CPUStack
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
            stack: CPUStack::new(),
        }
    }

    /// Function that loads the Program ROM into memory and resets the CPU
    pub fn load_program(&mut self, program: Vec<u8>) -> Result<(), &'static str> {
        // load new program into the memory
        self.memory.load(program)?;

        // reset the cpu
        // self.reset();

        // start of the Program ROM
        // (actually it can be anything from 0x8000 to 0xFFFF)
        // self.prog_counter = 0x8000;
        // self.prog_counter = 0xC000;

        Ok(())
    }

    /// Function that resets the CPU
    pub fn reset(&mut self) {
        // reset the registers
        self.a.reset();
        self.x.reset();
        self.y.reset();

        // reset the status
        self.status.reset();

        // reset the stack
        self.stack.reset();

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
    /// TODO: page wrapping
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
                let addr = val.wrapping_add(self.x.value()) as u16;
                addr
            },
            Addressing::ZeroPageY => {
                // u8 value from memory
                let val = self.memory.read(self.prog_counter);

                // add register y value to it (wrap around if needed)
                let addr = val.wrapping_add(self.y.value()) as u16;
                addr
            },

            // Absolute
            Addressing::Absolute => self.memory.read_u16(self.prog_counter),
            Addressing::AbsoluteX => {
                // u16 value from memory
                let val = self.memory.read_u16(self.prog_counter);

                // add register x value to it (wrap around if needed)
                let addr = val.wrapping_add(self.x.value() as u16);
                addr
            },
            Addressing::AbsoluteY => {
                // u16 value from memory
                let val = self.memory.read_u16(self.prog_counter);

                // add register y value to it (wrap around if needed)
                let addr = val.wrapping_add(self.y.value() as u16);
                addr
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

                let tmp = u16::from_le_bytes([low, high]);
                let addr = tmp.wrapping_add(self.y.value() as u16);
                addr
            },

            // None
            Addressing::None => {
                panic!("mode {:?} not supported", mode);
            }
        }
    }

    fn adc(&mut self, mode: &Addressing) {
        let address = self.get_param_address(mode);
        let param = self.memory.read(address);

        let old_status = self.status.is_set(Status::Carry);
        let res = self.a.value().wrapping_add(param).wrapping_add(old_status as u8);

        // set carry flag
        match res < self.a.value() {
            true => self.status.add(Status::Carry),
            false => self.status.remove(Status::Carry),
        }

        // set overflow flag
        match (self.a.value() ^ res) & (param ^ res) & 0x80 {
            0 => self.status.remove(Status::Overflow),
            _ => self.status.add(Status::Overflow),
        }

        self.a.set(res);
        self.zero_negative(res);
    }

    fn and(&mut self, mode: &Addressing) {
        let address = self.get_param_address(mode);
        let param = self.memory.read(address);

        self.a.set(self.a.value() & param);
        self.zero_negative(self.a.value());
    }

    fn asl_a(&mut self) {
        let param = self.a.value();

        // set carry flag
        match param & 0x80 {
            0 => self.status.remove(Status::Carry),
            _ => self.status.add(Status::Carry),
        }

        // shift left
        let res = param << 1;
        self.a.set(res);
        self.zero_negative(res);
    }

    fn asl(&mut self, mode: &Addressing) {
        let address = self.get_param_address(mode);
        let param = self.memory.read(address);

        // set carry flag
        match param & 0x80 {
            0 => self.status.remove(Status::Carry),
            _ => self.status.add(Status::Carry),
        }

        // shift left
        let res = param << 1;
        self.memory.write(address, res);
        self.zero_negative(res);
    }

    fn branch(&mut self, condition: bool) {
        if condition {
            // get the offset
            let offset = self.memory.read(self.prog_counter) as i8;
            self.prog_counter = self.prog_counter.wrapping_add((1 + offset) as u16);
        }
    }

    fn clear_status(&mut self, status: Status) {
        self.status.remove(status);
    }

    fn set_status(&mut self, status: Status) {
        self.status.add(status);
    }

    fn bit(&mut self, mode: &Addressing) {
        let address = self.get_param_address(mode);
        let param = self.memory.read(address);

        match self.a.value() & param {
            0 => self.status.add(Status::Zero),
            _ => self.status.remove(Status::Zero),
        }

        match param & Status::Negative.as_u8() {
            0 => self.status.remove(Status::Negative),
            _ => self.status.add(Status::Negative),
        }

        match param & Status::Overflow.as_u8() {
            0 => self.status.remove(Status::Overflow),
            _ => self.status.add(Status::Overflow),
        }
    }

    fn compare(&mut self, reg_val: u8, mode: &Addressing) {
        let address = self.get_param_address(mode);
        let param = self.memory.read(address);

        match param <= reg_val {
            true => self.status.add(Status::Carry),
            false => self.status.remove(Status::Carry),
        }

        self.zero_negative(reg_val.wrapping_sub(param));
    }
    fn dec(&mut self, mode: &Addressing) {
        let address = self.get_param_address(mode);
        let param = self.memory.read(address);

        let res = param.wrapping_sub(1);
        self.memory.write(address, res);
        self.zero_negative(res);
    }

    fn dex(&mut self) {
        self.x.subtract(1);
        self.zero_negative(self.x.value());
    }

    fn dey(&mut self) {
        self.y.subtract(1);
        self.zero_negative(self.y.value())
    }

    fn eor(&mut self, mode: &Addressing) {
        let address = self.get_param_address(mode);
        let param = self.memory.read(address);

        self.a.set(self.a.value() ^ param);
        self.zero_negative(self.a.value());
    }

    fn inc(&mut self, mode: &Addressing) {
        let address = self.get_param_address(mode);
        let param = self.memory.read(address);

        let res = param.wrapping_add(1);
        self.memory.write(address, res);
        self.zero_negative(res);
    }

    fn inx(&mut self) {
        self.x.add(1);
        self.zero_negative(self.x.value());
    }

    fn iny(&mut self) {
        self.y.add(1);
        self.zero_negative(self.y.value());
    }

    fn jmp_abs(&mut self) {
        let address = self.memory.read_u16(self.prog_counter);
        self.prog_counter = address;
    }

    // An original 6502 has does not correctly fetch the target address if the indirect vector falls on a page boundary
    // (e.g. $xxFF where xx is any value from $00 to $FF).
    // In this case fetches the LSB from $xxFF as expected but takes the MSB from $xx00.
    // This is fixed in some later chips like the 65SC02 so for compatibility always ensure the indirect vector is not at the end of the page.
    fn jmp_ind(&mut self) {
        let address = self.memory.read_u16(self.prog_counter);

        let indirect_ref = if CPU::is_page_boundary(address) {
            self.read_indirect_address(address)
        } else {
            self.memory.read_u16(address)
        };

        self.prog_counter = indirect_ref;
    }


    // helper function for indirect jump
    fn is_page_boundary(address: u16) -> bool {
        address & 0x00FF == 0x00FF
    }

    // helper function for indirect jump
    fn read_indirect_address(&self, mem_address: u16) -> u16 {
        let lo = self.memory.read(mem_address);
        let hi = self.memory.read(mem_address & 0xFF00);
        u16::from_le_bytes([lo, hi])
    }

    fn jsr(&mut self) {
        let address = self.memory.read_u16(self.prog_counter);
        self.stack.push_u16(self.prog_counter + 2 - 1);
        self.prog_counter = address;
    }

    fn lda(&mut self, mode: &Addressing) {
        // get param from memory
        let address = self.get_param_address(mode);
        let param = self.memory.read(address);

        // set param
        self.a.set(param);
        self.zero_negative(self.a.value());
    }

    fn ldx(&mut self, mode: &Addressing) {
        // get param from memory
        let address = self.get_param_address(mode);
        let param = self.memory.read(address);

        // set param
        self.x.set(param);
        self.zero_negative(self.x.value());
    }

    fn ldy(&mut self, mode: &Addressing) {
        // get param from memory
        let address = self.get_param_address(mode);
        let param = self.memory.read(address);

        // set param
        self.y.set(param);
        self.zero_negative(self.y.value());
    }

    fn lsr_a(&mut self) {
        let param = self.a.value();

        // set carry flag
        match param & 0x01 {
            0 => self.status.remove(Status::Carry),
            _ => self.status.add(Status::Carry),
        }

        // shift right
        let res = param >> 1;
        self.a.set(res);
        self.zero_negative(res);
    }

    fn lsr(&mut self, mode: &Addressing) {
        let address = self.get_param_address(mode);
        let param = self.memory.read(address);

        // set carry flag
        match param & 0x01 {
            0 => self.status.remove(Status::Carry),
            _ => self.status.add(Status::Carry),
        }

        // shift right
        let res = param >> 1;
        self.memory.write(address, res);
        self.zero_negative(res);
    }

    fn ora(&mut self, mode: &Addressing) {
        let address = self.get_param_address(mode);
        let param = self.memory.read(address);

        self.a.set(self.a.value() | param);
        self.zero_negative(self.a.value());
    }

    fn pha(&mut self) {
        self.stack.push(self.a.value());
    }

    fn php(&mut self) {
        self.stack.push(self.status.value.clone());
    }

    fn pla(&mut self) {
        self.a.set(self.stack.pop());
        self.zero_negative(self.a.value());
    }

    fn plp(&mut self) {
        self.status.set_bits(self.stack.pop());
    }

    fn rol_a(&mut self) {
        let param = self.a.value();

        // set carry flag
        match param & 0x80 {
            0 => self.status.remove(Status::Carry),
            _ => self.status.add(Status::Carry),
        }

        // shift left
        let res = param << 1;
        self.a.set(res);
        self.zero_negative(res);
    }

    fn rol(&mut self, mode: &Addressing) {
        let address = self.get_param_address(mode);
        let param = self.memory.read(address);

        // set carry flag
        match param & 0x80 {
            0 => self.status.remove(Status::Carry),
            _ => self.status.add(Status::Carry),
        }

        // shift left
        let res = param << 1;
        self.memory.write(address, res);
        self.zero_negative(res);
    }

    fn ror_a(&mut self) {
        let param = self.a.value();
        let old_status = self.status.is_set(Status::Carry);

        // set carry flag
        match param & 0x01 {
            0 => self.status.remove(Status::Carry),
            _ => self.status.add(Status::Carry),
        }

        // shift right
        let mut res = param >> 1;
        if old_status {
            res |= 0x80;
        }
        self.a.set(res);
    }

    fn ror(&mut self, mode: &Addressing) {
        let address = self.get_param_address(mode);
        let param = self.memory.read(address);
        let old_status = self.status.is_set(Status::Carry);

        // set carry flag
        match param & 0x01 {
            0 => self.status.remove(Status::Carry),
            _ => self.status.add(Status::Carry),
        }

        // shift right
        let mut res = param >> 1;
        if old_status {
            res |= 0x80;
        }
        self.memory.write(address, res);

        match res >> 7 {
            1 => self.status.add(Status::Negative),
            _ => self.status.remove(Status::Negative),
        }
    }

    fn rti(&mut self) {
        self.status.set_bits(self.stack.pop());
        self.prog_counter = self.stack.pop_u16();

    }

    fn rts(&mut self) {
        self.prog_counter = self.stack.pop_u16() + 1;
    }

    fn sbc(&mut self, mode: &Addressing) {
        let address = self.get_param_address(mode);
        let param = self.memory.read(address);

        let old_status = self.status.is_set(Status::Carry);
        let res = self.a.value().wrapping_sub(param).wrapping_sub(!old_status as u8);

        // Set carry flag (no borrow occurred)
        match self.a.value() >= param + (!old_status as u8) {
            true => self.status.add(Status::Carry),
            false => self.status.remove(Status::Carry),
        }

        // Set overflow flag
        match (self.a.value() ^ res) & (param ^ res) & 0x80 {
            0 => self.status.remove(Status::Overflow),
            _ => self.status.add(Status::Overflow),
        }

        self.a.set(res);
        self.zero_negative(res);
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

    fn sta(&mut self, mode: &Addressing) {
        let address = self.get_param_address(mode);
        self.memory.write(address, self.a.value());
    }

    fn stx(&mut self, mode: &Addressing) {
        let address = self.get_param_address(mode);
        self.memory.write(address, self.x.value());
    }

    fn sty(&mut self, mode: &Addressing) {
        let address = self.get_param_address(mode);
        self.memory.write(address, self.y.value());
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
        self.x.set(self.stack.peek());
        self.zero_negative(self.x.value());
    }

    fn txa(&mut self, mode: &Addressing) {
        self.a.set(self.x.value());
        self.zero_negative(self.a.value());
    }

    fn txs(&mut self) {
        self.stack.push(self.x.value());
    }

    fn tya(&mut self) {
        self.a.set(self.y.value());
        self.zero_negative(self.a.value());
    }



    /// Function that interprets the given program
    pub fn interpret(&mut self) {
        self.interpret_callback(|_| {});
        // loop {
        //     let ins_code = self.memory.read(self.prog_counter);
        //     self.prog_counter += 1;
        //
        //     let ins: &Instruction = INSTRUCTION_MAP.get(&ins_code).expect("Code not recognized");
        //
        //     match ins.name {
        //         // TODO
        //         ADC => self.adc(&ins.mode),
        //         AND => self.and(&ins.mode),
        //         ASL_A => self.asl_a(),
        //         ASL => self.asl(&ins.mode),
        //         BIT => self.bit(&ins.mode),
        //         BCS => self.branch(self.status.is_set(Status::Carry)),
        //         BCC => self.branch(!self.status.is_set(Status::Carry)),
        //         BEQ => self.branch(self.status.is_set(Status::Zero)),
        //         BNE => self.branch(!self.status.is_set(Status::Zero)),
        //         BMI => self.branch(self.status.is_set(Status::Negative)),
        //         BPL => self.branch(!self.status.is_set(Status::Negative)),
        //         BVS => self.branch(self.status.is_set(Status::Overflow)),
        //         BVC => self.branch(!self.status.is_set(Status::Overflow)),
        //         BRK => return,
        //         CLC => self.clear_status(Status::Carry),
        //         CLD => self.clear_status(Status::Decimal),
        //         CLI => self.clear_status(Status::InterruptDisable),
        //         CLV => self.clear_status(Status::Overflow),
        //         CMP => self.compare(self.a.value(), &ins.mode),
        //         CPX => self.compare(self.x.value(), &ins.mode),
        //         CPY => self.compare(self.y.value(), &ins.mode),
        //         DEC => self.dec(&ins.mode),
        //         DEX => self.dex(),
        //         DEY => self.dey(),
        //         EOR => self.eor(&ins.mode),
        //         INC => self.inc(&ins.mode),
        //         INX => self.inx(),
        //         INY => self.iny(),
        //         JMP => self.jmp(&ins.mode),
        //         JSR => self.jsr(),
        //         LDA => self.lda(&ins.mode),
        //         LDX => self.ldx(&ins.mode),
        //         LDY => self.ldy(&ins.mode),
        //         LSR_A => self.lsr_a(),
        //         LSR => self.lsr(&ins.mode),
        //         NOP => /* no change */ (),
        //         ORA => self.ora(&ins.mode),
        //         PHA => self.pha(),
        //         PHP => self.php(),
        //         PLA => self.pla(),
        //         PLP => self.plp(),
        //         ROL_A => self.rol_a(),
        //         ROL => self.rol(&ins.mode),
        //         ROR_A => self.ror_a(),
        //         ROR => self.ror(&ins.mode),
        //         RTI => self.rti(),
        //         RTS => self.rts(),
        //         SBC => {
        //             // TODO
        //             self.sbc(&ins.mode)
        //         },
        //         SEC => self.set_status(Status::Carry),
        //         SED => self.set_status(Status::Decimal),
        //         SEI => self.set_status(Status::InterruptDisable),
        //         STA => self.sta(&ins.mode),
        //         STX => self.stx(&ins.mode),
        //         STY => self.sty(&ins.mode),
        //         TAX => self.tax(),
        //         TAY => self.tay(),
        //         TSX => self.tsx(),
        //         TXA => self.txa(&ins.mode),
        //         TXS => self.txs(),
        //         TYA => self.tya(),
        //     }
        //
        //     // increase prog_counter
        //     // (ins.bytes - 1) because we already increased it by 1 at the beginning
        //     self.prog_counter += (ins.bytes - 1) as u16;
        //
        // }
    }

    pub fn interpret_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut CPU)
    {
        loop {
            let ins_code = self.memory.read(self.prog_counter);
            self.prog_counter += 1;
            let prog_counter_state = self.prog_counter;

            // let ins: &Instruction = INSTRUCTION_MAP.get(&ins_code).expect(format!("Code {:X} not recognized", ins_code).as_str());
            let ins: &Instruction = match INSTRUCTION_MAP.get(&ins_code) {
                Some(instruction) => instruction,
                None => {
                    eprintln!("Unrecognized opcode: 0x{:X}", ins_code);
                    return;
                }
            };

            println!(
                "PC: {:X} OP: {:?} A: {} X: {} Y: {}",
                self.prog_counter, ins, self.a.value(), self.x.value(), self.y.value()
            );

            match ins.name {
                ADC => self.adc(&ins.mode),
                AND => self.and(&ins.mode),
                ASL_A => self.asl_a(),
                ASL => self.asl(&ins.mode),
                BIT => self.bit(&ins.mode),
                BCS => self.branch(self.status.is_set(Status::Carry)),
                BCC => self.branch(!self.status.is_set(Status::Carry)),
                BEQ => self.branch(self.status.is_set(Status::Zero)),
                BNE => self.branch(!self.status.is_set(Status::Zero)),
                BMI => self.branch(self.status.is_set(Status::Negative)),
                BPL => self.branch(!self.status.is_set(Status::Negative)),
                BVS => self.branch(self.status.is_set(Status::Overflow)),
                BVC => self.branch(!self.status.is_set(Status::Overflow)),
                BRK => return,
                CLC => self.clear_status(Status::Carry),
                CLD => self.clear_status(Status::Decimal),
                CLI => self.clear_status(Status::InterruptDisable),
                CLV => self.clear_status(Status::Overflow),
                CMP => self.compare(self.a.value(), &ins.mode),
                CPX => self.compare(self.x.value(), &ins.mode),
                CPY => self.compare(self.y.value(), &ins.mode),
                DEC => self.dec(&ins.mode),
                DEX => self.dex(),
                DEY => self.dey(),
                EOR => self.eor(&ins.mode),
                INC => self.inc(&ins.mode),
                INX => self.inx(),
                INY => self.iny(),
                JMP_ABS => self.jmp_abs(),
                JMP_IND => self.jmp_ind(),
                JSR => self.jsr(),
                LDA => self.lda(&ins.mode),
                LDX => self.ldx(&ins.mode),
                LDY => self.ldy(&ins.mode),
                LSR_A => self.lsr_a(),
                LSR => self.lsr(&ins.mode),
                NOP => /* no change */ (),
                ORA => self.ora(&ins.mode),
                PHA => self.pha(),
                PHP => self.php(),
                PLA => self.pla(),
                PLP => self.plp(),
                ROL_A => self.rol_a(),
                ROL => self.rol(&ins.mode),
                ROR_A => self.ror_a(),
                ROR => self.ror(&ins.mode),
                RTI => self.rti(),
                RTS => self.rts(),
                SBC => self.sbc(&ins.mode),
                SEC => self.set_status(Status::Carry),
                SED => self.set_status(Status::Decimal),
                SEI => self.set_status(Status::InterruptDisable),
                STA => self.sta(&ins.mode),
                STX => self.stx(&ins.mode),
                STY => self.sty(&ins.mode),
                TAX => self.tax(),
                TAY => self.tay(),
                TSX => self.tsx(),
                TXA => self.txa(&ins.mode),
                TXS => self.txs(),
                TYA => self.tya(),
            }

            if self.prog_counter == prog_counter_state {
                // increase prog_counter
                // (ins.bytes - 1) because we already increased it by 1 at the beginning
                self.prog_counter += (ins.bytes - 1) as u16;
            }

            callback(self);
        }
    }

    pub fn mem_write(&mut self, address: u16, val: u8) {
        self.memory.write(address, val);
    }

    pub fn mem_read(&self, address: u16) -> u8 {
        self.memory.read(address)
    }
}
