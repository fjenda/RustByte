// https://www.nesdev.org/wiki/CPU_memory_map

// [0x2000 - 0x4020] => redirected to hardware modules
// [0x4020 - 0x6000] => unmapped, for cartridge use
// [0x6000 - 0x8000] => cartridge RAM
// [0x8000 - 0xFFFF] => Program ROM

// Special addresses
// [0xFFFC - 0xFFFD] => Reset vector

/// Class representing the memory in the CPU
pub struct Memory {
    data: [u8; 0xFFFF],
}

impl Memory {
    /// Constructor for the class
    pub fn new() -> Self {
        Memory {
            data: [0; 0xFFFF],
        }
    }

    /// Load the Program ROM into the memory
    pub fn load(&mut self, prog: Vec<u8>) -> Result<(), &'static str> {
        // set data from 0x8000 to prog.len() to prog
        let start: u16 = 0x8000;
        let max_size: u16 = 0xFFFF - start;

        // if program is too big (shouldn't happen really)
        if prog.len() > max_size as usize {
            return Err("Program size exceeds available memory space from 0x8000 to 0xFFFF");
        }

        // set data
        self.data[start as usize .. (start as usize + prog.len())].copy_from_slice(&prog);

        // set the 0xFFFC address
        self.write_u16(0xFFFC, start);

        Ok(())
    }

    /// Function that returns a value read from the memory at a given address
    pub fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    /// Function that writes a value into the memory at a given address
    pub fn write(&mut self, address: u16, val: u8) {
        self.data[address as usize] = val;
    }

    /// Function that returns a u16 value read from the memory at a given address
    /// NES uses Little-Endian addressing
    /// (8 LSBs will be stored before the 8 MSBs)
    /// https://doc.rust-lang.org/std/primitive.f16.html#method.to_le_bytes
    pub fn read_u16(&mut self, address: u16) -> u16 {
        let low = self.read(address);
        let high = self.read(address + 1);
        u16::from_le_bytes([low, high])
    }

    /// Function that writes a u16 value into the memory at a given address
    /// NES uses Little-Endian addressing
    /// (8 LSBs will be stored before the 8 MSBs)
    /// https://doc.rust-lang.org/std/primitive.f16.html#method.to_le_bytes
    pub fn write_u16(&mut self, address: u16, val: u16) {
        let bytes = val.to_le_bytes();
        self.write(address, bytes[0]);
        self.write(address + 1, bytes[1]);
    }
}
