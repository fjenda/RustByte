// https://www.nesdev.org/wiki/CPU_registers

/// Class representing one register in the CPU
pub struct Register {
    value: u8
}

impl Register {
    /// Creates new instance of the object
    pub fn new() -> Self {
        Register {
            value: 0
        }
    }

    /// Sets the value of the register
    pub fn set(&mut self, val: u8) {
        self.value = val;
    }

    /// Adds val to the value of the register
    pub fn add(&mut self, val: u8) {
        self.value = self.value.wrapping_add(val);
    }

    /// Subtracts val from the value of the register
    pub fn subtract(&mut self, val: u8) {
        self.value = self.value.wrapping_sub(val);
    }

    /// Getter for the value
    pub fn value(&self) -> u8 {
        self.value
    }

    /// Function that resets the register to 0
    pub fn reset(&mut self) {
        self.value = 0;
    }
}
