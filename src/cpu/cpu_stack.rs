/// https://www.nesdev.org/wiki/CPU_registers
/// https://www.nesdev.org/wiki/Stack
/// 0x0100 - 0x01FF

/// Class to represent the CPU stack
pub struct CPUStack {
    /// Stack memory
    stack: [u8; 0x100],

    /// Stack pointer
    pub pointer: u8,
}

impl CPUStack {
    /// Create a new CPU stack
    pub fn new() -> Self {
        CPUStack {
            stack: [0; 0x100],
            pointer: 0xfd,
        }
    }

    /// Push a value onto the stack
    pub fn push(&mut self, value: u8) {
        self.stack[self.pointer as usize] = value;
        self.pointer = self.pointer.wrapping_sub(1);
    }

    /// Pop a value from the stack
    pub fn pop(&mut self) -> u8 {
        self.pointer = self.pointer.wrapping_add(1);
        self.stack[self.pointer as usize]
    }

    /// Pop a u16 value from the stack
    pub fn pop_u16(&mut self) -> u16 {
        let low = self.pop();
        let high = self.pop();
        u16::from_le_bytes([low, high])
    }

    /// Push a u16 value onto the stack
    pub fn push_u16(&mut self, value: u16) {
        let bytes = value.to_le_bytes();
        self.push(bytes[1]);
        self.push(bytes[0]);
    }

    /// Peek at the top of the stack
    pub fn peek(&self) -> u8 {
        self.stack[self.pointer as usize]
    }

    /// Reset the stack pointer
    pub fn reset(&mut self) {
        self.pointer = 0xfd;
    }
}