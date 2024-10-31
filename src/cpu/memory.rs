// https://www.nesdev.org/wiki/CPU_memory_map

// [0x2000 - 0x4020] => redirected to hardware modules
// [0x4020 - 0x6000] => unmapped, for cartridge use
// [0x6000 - 0x8000] => cartridge RAM
// [0x8000 - 0xFFFF] => Program ROM

pub struct Memory {
    data: [u8; 0xFFFF],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: [0; 0xFFFF],
        }
    }

    pub fn load(&mut self, prog: Vec<u8>) -> Result<(), &'static str> {
        // set data from 0x8000 to prog.len() to prog
        let start = 0x8000;
        let max_size = 0xFFFF - start;

        // if program is too big (shouldn't happen really)
        if prog.len() > max_size {
            return Err("Program size exceeds available memory space from 0x8000 to 0xFFFF");
        }

        // set data
        self.data[start .. prog.len()].copy_from_slice(&prog);
        Ok(())
    }

    pub fn read(&self, address: u16) -> u8 {
        self.data[address]
    }

    pub fn write(&mut self, address: u16, val: u8) {
        self.data[address] = val;
    }
}
