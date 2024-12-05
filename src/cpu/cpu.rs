// https://www.nesdev.org/obelisk-6502-guide/reference.html

use std::fmt::Formatter;
use sdl2::log::log;
use crate::cpu::addressing::Addressing;
use crate::flags::Status;
use crate::byte_status::ByteStatus;
use crate::cpu::bus::Bus;
use crate::cpu::cpu_register::CPURegister;
use crate::cpu::cpu_status::{CPUStatus};
use crate::cpu::instructions::{Instruction, INSTRUCTION_MAP, OpName::*};
use crate::cpu::cpu_stack::CPUStack;
use crate::cpu::interrupt::{Interrupt, NMI};

/// This class represents the CPU
pub struct CPU<'a> {
    /// 3x 8-bit registers A (accumulator), X, Y (indexes)
    pub a: CPURegister,
    pub x: CPURegister,
    pub y: CPURegister,

    /// CPU Status with Flags
    pub status: CPUStatus,

    /// This is also a register, but it's simpler to represent it using u16
    pub prog_counter: u16,

    /// CPU Memory
    // pub memory: Memory,

    /// CPU BUS
    pub bus: Bus<'a>,

    // 0x0100 - 0x01FF
    // pub stack: CPUStack
    pub stack_pointer: u8,
}

impl<'a> CPU<'a> {
    /// Creates an instance of CPU
    pub fn new<'b>(bus: Bus<'b>) -> CPU<'b> {
        CPU {
            a: CPURegister::new(),
            x: CPURegister::new(),
            y: CPURegister::new(),
            status: CPUStatus::new(),
            prog_counter: 0,
            // memory: Memory::new(),
            bus,
            // stack: CPUStack::new(),
            stack_pointer: 0xFD,
        }
    }

    /// Function that loads the Program ROM into memory and resets the CPU
    /// Unused for now since we are using the bus
    // pub fn load_program(&mut self, program: Vec<u8>) {
    //     // load new program into the memory
    //     for (i, byte) in program.iter().enumerate() {
    //         self.write(0x0600 + i as u16, *byte);
    //     }
    //
    //     // start of the Program ROM
    //     // (actually it can be anything from 0x8000 to 0xFFFF)
    //     self.write_u16(0xFFFC, 0x0600);
    //
    //     // reset the cpu
    //     // self.reset();
    // }

    /// Function that resets the CPU
    pub fn reset(&mut self) {
        // reset the registers
        self.a.reset();
        self.x.reset();
        self.y.reset();

        // reset the status
        self.status.reset();

        // reset the stack
        self.stack_pointer = 0xFD;

        // set prog_counter to address at 0xFFFC
        self.prog_counter = self.read_u16(0xFFFC);
    }

    /// Function that handles an interrupt
    pub fn interrupt(&mut self, interrupt: Interrupt) {
        // push the program counter to the stack
        self.stack_push_u16(self.prog_counter);

        let mut status = self.status.clone();

        // set the break flags
        status.set(Status::Break.as_u8(), interrupt.flag_mask & 0b010000 == 1);
        status.set(Status::Break2.as_u8(), interrupt.flag_mask & 0b100000 == 1);


        // push the status register to the stack
        self.stack_push(status.value);
        self.status.add(Status::InterruptDisable.as_u8());

        self.bus.tick(interrupt.cycles);
        self.prog_counter = self.read_u16(interrupt.address);
    }

    pub fn read(&mut self, address: u16) -> u8 {
        // self.memory.read(address)
        self.bus.read(address)
    }

    pub fn read_u16(&mut self, address: u16) -> u16 {
        // self.memory.read_u16(address)
        self.bus.read_u16(address)
    }

    pub fn write(&mut self, address: u16, val: u8) {
        // self.memory.write(address, val);
        self.bus.write(address, val);
    }

    pub fn write_u16(&mut self, address: u16, val: u16) {
        // self.memory.write_u16(address, val);
        self.bus.write_u16(address, val);
    }

    /// Function that handles the logic of setting Zero and Negative flags
    fn zero_negative(&mut self, res: u8) {
        // Zero Flag
        match res {
            0 => self.status.add(Status::Zero.as_u8()),
            _ => self.status.remove(Status::Zero.as_u8()),
        }

        // Negative Flag
        match res & Status::Negative.as_u8() {
            0 => self.status.remove(Status::Negative.as_u8()),
            _ => self.status.add(Status::Negative.as_u8()),
        }
    }

    /// Function that determines if a page was crossed
    fn crossed_page(addr_1: u16, addr_2: u16) -> bool {
        addr_1 & 0xFF00 != addr_2 & 0xFF00
    }

    // https://www.nesdev.org/obelisk-6502-guide/addressing.html
    /// Function that gets the parameter address for a function using its addressing mode
    pub fn get_param_address(&mut self, mode: &Addressing, addr: u16) -> (u16, bool) {
        match mode {
            // Zero Page
            Addressing::ZeroPage => (self.read(addr) as u16, false),
            Addressing::ZeroPageX => {
                // u8 value from memory
                let val = self.read(addr);

                // add register x value to it (wrap around if needed)
                let addr = val.wrapping_add(self.x.value()) as u16;
                (addr, false)
            },
            Addressing::ZeroPageY => {
                // u8 value from memory
                let val = self.read(addr);

                // add register y value to it (wrap around if needed)
                let addr = val.wrapping_add(self.y.value()) as u16;
                (addr, false)
            },

            // Absolute
            Addressing::Absolute => (self.read_u16(addr), false),
            Addressing::AbsoluteX => {
                // u16 value from memory
                let val = self.read_u16(addr);

                // add register x value to it (wrap around if needed)
                let addr = val.wrapping_add(self.x.value() as u16);
                (addr, CPU::crossed_page(val, addr))
            },
            Addressing::AbsoluteY => {
                // u16 value from memory
                let val = self.read_u16(addr);

                // add register y value to it (wrap around if needed)
                let addr = val.wrapping_add(self.y.value() as u16);
                (addr, CPU::crossed_page(val, addr))
            },

            // Indirect
            Addressing::IndirectX => {
                // u8 value from memory
                let val = self.read(addr);

                // index into the memory
                let index: u8 = val.wrapping_add(self.x.value());

                // high and low
                let low = self.read(index as u16);
                let high = self.read(index.wrapping_add(1) as u16);
                (u16::from_le_bytes([low, high]), false)
            },
            Addressing::IndirectY => {
                // u8 value from memory
                let val = self.read(addr);

                // high and low
                let low = self.read(val as u16);
                let high = self.read(val.wrapping_add(1) as u16);

                let tmp = u16::from_le_bytes([low, high]);
                let addr = tmp.wrapping_add(self.y.value() as u16);
                (addr, CPU::crossed_page(tmp, addr))
            },

            // None
            _ => {
                panic!("mode {:?} not supported", mode);
            }
        }
    }

    fn get_param_address_internal(&mut self, mode: &Addressing) -> (u16, bool) {
        match mode {
            Addressing::Immediate => (self.prog_counter, false),
            _ => self.get_param_address(&mode, self.prog_counter),
        }
    }

    fn add_to_a(&mut self, val: u8) {
        let res = self.a.value() as u16 + val as u16 + self.status.is_set(Status::Carry.as_u8()) as u16;
        let carry = res > 0xFF;

        // set carry flag
        match carry {
            true => self.status.add(Status::Carry.as_u8()),
            false => self.status.remove(Status::Carry.as_u8()),
        }

        let res = res as u8;

        // set overflow flag
        match (val ^ res) & (res ^ self.a.value()) & 0x80 != 0 {
            true => self.status.add(Status::Overflow.as_u8()),
            false => self.status.remove(Status::Overflow.as_u8()),
        }

        self.a.set(res);
        self.zero_negative(res);
    }

    fn adc(&mut self, mode: &Addressing) {
        let (address, cross) = self.get_param_address_internal(mode);
        let param = self.read(address);

        self.add_to_a(param);

        if cross {
            self.bus.tick(1);
        }
    }

    fn and(&mut self, mode: &Addressing) {
        let (address, cross) = self.get_param_address_internal(mode);
        let param = self.read(address);

        self.a.set(param & self.a.value());
        self.zero_negative(self.a.value());

        if cross {
            self.bus.tick(1);
        }
    }

    fn asl_a(&mut self) {
        let param = self.a.value();

        // set carry flag
        match param >> 7 {
            1 => self.status.add(Status::Carry.as_u8()),
            _ => self.status.remove(Status::Carry.as_u8()),
        }

        // shift left
        let res = param << 1;
        self.a.set(res);
        self.zero_negative(res);
    }

    fn asl(&mut self, mode: &Addressing) -> u8 {
        let (address, _) = self.get_param_address_internal(mode);
        let param = self.read(address);

        // set carry flag
        match param >> 7 {
            1 => self.status.add(Status::Carry.as_u8()),
            _ => self.status.remove(Status::Carry.as_u8()),
        }

        // shift left
        let res = param << 1;
        self.write(address, res);
        self.zero_negative(res);
        param
    }

    fn branch(&mut self, condition: bool) {
        if condition {
            self.bus.tick(1);

            // get the offset
            let offset = self.read(self.prog_counter) as i8;
            let jump_addr = self.prog_counter.wrapping_add(1).wrapping_add(offset as u16);

            if self.prog_counter.wrapping_add(1) & 0xFF00 != jump_addr & 0xFF00 {
                self.bus.tick(1);
            }

            self.prog_counter = jump_addr;
        }
    }

    fn clear_status(&mut self, status: Status) {
        self.status.remove(status.as_u8());
    }

    fn set_status(&mut self, status: Status) {
        self.status.add(status.as_u8());
    }

    fn bit(&mut self, mode: &Addressing) {
        let (address, _) = self.get_param_address_internal(mode);
        let param = self.read(address);

        match self.a.value() & param {
            0 => self.status.add(Status::Zero.as_u8()),
            _ => self.status.remove(Status::Zero.as_u8()),
        }

        self.status.set(Status::Negative.as_u8(), param & Status::Negative.as_u8() > 0);
        self.status.set(Status::Overflow.as_u8(), param & Status::Overflow.as_u8() > 0);
    }

    fn compare(&mut self, reg_val: u8, mode: &Addressing) {
        let (address, cross) = self.get_param_address_internal(mode);
        let param = self.read(address);

        match param <= reg_val {
            true => self.status.add(Status::Carry.as_u8()),
            false => self.status.remove(Status::Carry.as_u8()),
        }

        self.zero_negative(reg_val.wrapping_sub(param));

        if cross {
            self.bus.tick(1);
        }
    }
    fn dec(&mut self, mode: &Addressing) -> u8 {
        let (address, _) = self.get_param_address_internal(mode);
        let mut param = self.read(address);

        param = param.wrapping_sub(1);
        self.write(address, param);
        self.zero_negative(param);
        param
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
        let (address, cross) = self.get_param_address_internal(mode);
        let param = self.read(address);

        self.a.set(param ^ self.a.value());
        self.zero_negative(self.a.value());

        if cross {
            self.bus.tick(1);
        }
    }

    fn inc(&mut self, mode: &Addressing) -> u8 {
        let (address, _) = self.get_param_address_internal(mode);
        let mut param = self.read(address);

        param = param.wrapping_add(1);
        self.write(address, param);
        self.zero_negative(param);
        param
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
        let address = self.read_u16(self.prog_counter);
        self.prog_counter = address;
    }

    // An original 6502 has does not correctly fetch the target address if the indirect vector falls on a page boundary
    // (e.g. $xxFF where xx is any value from $00 to $FF).
    // In this case fetches the LSB from $xxFF as expected but takes the MSB from $xx00.
    // This is fixed in some later chips like the 65SC02 so for compatibility always ensure the indirect vector is not at the end of the page.
    fn jmp_ind(&mut self) {
        let address = self.read_u16(self.prog_counter);

        let indirect_ref = if CPU::is_page_boundary(address) {
            self.read_indirect_address(address)
        } else {
            self.read_u16(address)
        };

        self.prog_counter = indirect_ref;
    }


    // helper function for indirect jump
    fn is_page_boundary(address: u16) -> bool {
        address & 0x00FF == 0x00FF
    }

    // helper function for indirect jump
    fn read_indirect_address(&mut self, mem_address: u16) -> u16 {
        let lo = self.read(mem_address);
        let hi = self.read(mem_address & 0xFF00);
        u16::from_le_bytes([lo, hi])
    }

    fn jsr(&mut self) {
        self.stack_push_u16(self.prog_counter + 2 - 1);
        let address = self.read_u16(self.prog_counter);
        self.prog_counter = address;
    }

    fn lda(&mut self, mode: &Addressing) {
        // get param from memory
        let (address, cross) = self.get_param_address_internal(mode);
        let param = self.read(address);

        // log(format!("LDA - Address: 0x{:X} | Value: 0x{:X} ({:?}", address, param, mode).as_str());

        // set param
        self.a.set(param);
        self.zero_negative(self.a.value());

        if cross {
            self.bus.tick(1);
        }
    }

    fn ldx(&mut self, mode: &Addressing) {
        // get param from memory
        let (address, cross) = self.get_param_address_internal(mode);
        let param = self.read(address);

        // set param
        self.x.set(param);
        self.zero_negative(self.x.value());

        if cross {
            self.bus.tick(1);
        }
    }

    fn ldy(&mut self, mode: &Addressing) {
        // get param from memory
        let (address, cross) = self.get_param_address_internal(mode);
        let param = self.read(address);

        // set param
        self.y.set(param);
        self.zero_negative(self.y.value());

        if cross {
            self.bus.tick(1);
        }
    }

    fn lsr_a(&mut self) {
        let param = self.a.value();

        // set carry flag
        match param & 1 {
            1 => self.status.add(Status::Carry.as_u8()),
            _ => self.status.remove(Status::Carry.as_u8()),
        }

        // shift right
        let res = param >> 1;
        self.a.set(res);
        self.zero_negative(res);
    }

    fn lsr(&mut self, mode: &Addressing) {
        let (address, _) = self.get_param_address_internal(mode);
        let param = self.read(address);

        // set carry flag
        match param & 1 {
            1 => self.status.add(Status::Carry.as_u8()),
            _ => self.status.remove(Status::Carry.as_u8()),
        }

        // shift right
        let res = param >> 1;
        self.write(address, res);
        self.zero_negative(res);
    }

    fn ora(&mut self, mode: &Addressing) {
        let (address, cross) = self.get_param_address_internal(mode);
        let param = self.read(address);

        self.a.set(param | self.a.value());
        self.zero_negative(self.a.value());

        if cross {
            self.bus.tick(1);
        }
    }

    fn pha(&mut self) {
        self.stack_push(self.a.value());
    }

    fn php(&mut self) {
        let mut flags = self.status.clone();
        flags.add(Status::Break.as_u8());
        flags.add(Status::Break2.as_u8());
        self.stack_push(flags.value);
    }

    fn pla(&mut self) {
        let data = self.stack_pop();
        self.a.set(data);
        self.zero_negative(self.a.value());
    }

    fn plp(&mut self) {
        let val = self.stack_pop();
        self.status.set_bits(val);
        self.status.remove(Status::Break.as_u8());
        self.status.add(Status::Break2.as_u8());
    }

    fn rol_a(&mut self) {
        let mut param = self.a.value();
        let old_carry = self.status.is_set(Status::Carry.as_u8());

        // set carry flag
        match param >> 7 {
            1 => self.status.add(Status::Carry.as_u8()),
            _ => self.status.remove(Status::Carry.as_u8()),
        }

        // shift left
        param = param << 1;
        if old_carry {
            param = param | 1;
        }

        self.a.set(param);
        self.zero_negative(param);
    }

    fn rol(&mut self, mode: &Addressing) -> u8 {
        let (address, _) = self.get_param_address_internal(mode);
        let mut param = self.read(address);
        let old_carry = self.status.is_set(Status::Carry.as_u8());

        // set carry flag
        match param >> 7 {
            1 => self.status.add(Status::Carry.as_u8()),
            _ => self.status.remove(Status::Carry.as_u8()),
        }

        // shift left
        param = param << 1;
        if old_carry {
            param = param | 1;
        }

        self.write(address, param);
        self.zero_negative(param);
        param
    }

    fn ror_a(&mut self) {
        let mut param = self.a.value();
        let old_carry = self.status.is_set(Status::Carry.as_u8());

        // set carry flag
        match param & 1 {
            1 => self.status.add(Status::Carry.as_u8()),
            _ => self.status.remove(Status::Carry.as_u8()),
        }

        // shift right
        param = param >> 1;
        if old_carry {
            param = param | 0x80;
        }

        self.a.set(param);
        self.zero_negative(param);
    }

    fn ror(&mut self, mode: &Addressing) -> u8 {
        let (address, _) = self.get_param_address_internal(mode);
        let mut param = self.read(address);
        let old_carry = self.status.is_set(Status::Carry.as_u8());

        // set carry flag
        match param & 1 {
            1 => self.status.add(Status::Carry.as_u8()),
            _ => self.status.remove(Status::Carry.as_u8()),
        }

        // shift right
        param = param >> 1;
        if old_carry {
            param = param | 0x80;
        }

        self.write(address, param);
        self.zero_negative(param);
        param
    }

    fn rti(&mut self) {
        let val = self.stack_pop();
        self.status.set_bits(val);
        self.status.remove(Status::Break.as_u8());
        self.status.add(Status::Break2.as_u8());

        self.prog_counter = self.stack_pop_u16();
    }

    fn rts(&mut self) {
        self.prog_counter = self.stack_pop_u16() + 1;
    }

    fn sbc(&mut self, mode: &Addressing) {
        let (address, cross) = self.get_param_address_internal(mode);
        let param = self.read(address);
        self.add_to_a((param as i8).wrapping_neg().wrapping_sub(1) as u8);

        if cross {
            self.bus.tick(1);
        }
    }

    fn sta(&mut self, mode: &Addressing) {
        let (address, _) = self.get_param_address_internal(mode);
        self.write(address, self.a.value());
    }

    fn stx(&mut self, mode: &Addressing) {
        let (address, _) = self.get_param_address_internal(mode);
        self.write(address, self.x.value());
    }

    fn sty(&mut self, mode: &Addressing) {
        let (address, _) = self.get_param_address_internal(mode);
        self.write(address, self.y.value());
    }

    fn tax(&mut self) {
        self.x.set(self.a.value());
        self.zero_negative(self.x.value());
    }

    fn tay(&mut self) {
        self.y.set(self.a.value());
        self.zero_negative(self.y.value());
    }


    // TODO: check if this is correct
    fn tsx(&mut self) {
        self.x.set(self.stack_pointer);
        self.zero_negative(self.x.value());
    }

    fn txa(&mut self) {
        self.a.set(self.x.value());
        self.zero_negative(self.a.value());
    }

    fn txs(&mut self) {
        self.stack_pointer = self.x.value();
    }

    fn tya(&mut self) {
        self.a.set(self.y.value());
        self.zero_negative(self.a.value());
    }

    /// Function that interprets the given program
    pub fn interpret(&mut self) {
        self.interpret_callback(|_| {});
    }

    /// Function that interprets the given program with a callback function
    pub fn interpret_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut CPU)
    {
        loop {
            if self.bus.nmi_status() {
                self.interrupt(NMI);
            }

            callback(self);

            let ins_code = self.read(self.prog_counter);
            self.prog_counter += 1;
            let prog_counter_state = self.prog_counter;

            let ins: &Instruction = match INSTRUCTION_MAP.get(&ins_code) {
                Some(instruction) => instruction,
                None => {
                    eprintln!("Unrecognized opcode: 0x{:X}", ins_code);
                    return;
                }
            };

            // println!("Before PC: {:X} | {} | A: {} X: {} Y: {}", self.prog_counter, self.status, self.a.value(), self.x.value(), self.y.value());

            // println!("Executing: {:?} - {:?} (0x{:X}, {} bytes)", ins.name, ins.mode, ins.address, ins.bytes);

            match ins.name {
                ADC => self.adc(&ins.mode),
                AND => self.and(&ins.mode),
                ASL_A => self.asl_a(),
                ASL => { self.asl(&ins.mode); },
                BIT => self.bit(&ins.mode),
                BCS => self.branch(self.status.is_set(Status::Carry.as_u8())),
                BCC => self.branch(!self.status.is_set(Status::Carry.as_u8())),
                BEQ => self.branch(self.status.is_set(Status::Zero.as_u8())),
                BNE => self.branch(!self.status.is_set(Status::Zero.as_u8())),
                BMI => self.branch(self.status.is_set(Status::Negative.as_u8())),
                BPL => self.branch(!self.status.is_set(Status::Negative.as_u8())),
                BVS => self.branch(self.status.is_set(Status::Overflow.as_u8())),
                BVC => self.branch(!self.status.is_set(Status::Overflow.as_u8())),
                BRK => return,
                CLC => self.clear_status(Status::Carry),
                CLD => self.clear_status(Status::Decimal),
                CLI => self.clear_status(Status::InterruptDisable),
                CLV => self.clear_status(Status::Overflow),
                CMP => self.compare(self.a.value(), &ins.mode),
                CPX => self.compare(self.x.value(), &ins.mode),
                CPY => self.compare(self.y.value(), &ins.mode),
                DEC => { self.dec(&ins.mode); },
                DEX => self.dex(),
                DEY => self.dey(),
                EOR => self.eor(&ins.mode),
                INC => { self.inc(&ins.mode); },
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
                ROL => { self.rol(&ins.mode); },
                ROR_A => self.ror_a(),
                ROR => { self.ror(&ins.mode); },
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
                TXA => self.txa(),
                TXS => self.txs(),
                TYA => self.tya(),
            }

            self.bus.tick(ins.cycles);

            if self.prog_counter == prog_counter_state {
                // increase prog_counter
                // (ins.bytes - 1) because we already increased it by 1 at the beginning
                self.prog_counter += (ins.bytes - 1) as u16;
            }

            // println!("After PC: {:X} | {} | A: {} X: {} Y: {}", self.prog_counter, self.status, self.a.value(), self.x.value(), self.y.value());
            // println!("Status: {} SP: {:X} CYC: {}", self.status, self.stack.pointer, self.bus.cycles);
        }
    }

    pub fn stack_push(&mut self, val: u8) {
        self.write(0x100 + self.stack_pointer as u16, val);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    pub fn stack_pop(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        self.read(0x100 + self.stack_pointer as u16)
    }

    pub fn stack_push_u16(&mut self, val: u16) {
        self.stack_push((val >> 8) as u8);
        self.stack_push(val as u8);
    }

    pub fn stack_pop_u16(&mut self) -> u16 {
        let low = self.stack_pop() as u16;
        let high = self.stack_pop() as u16;
        (high << 8) | low
    }
}
