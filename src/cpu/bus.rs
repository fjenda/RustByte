
#[derive(Debug)]
pub struct Bus {
    vram: [u8; 2048],
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            vram: [0; 2048],
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000 ..= 0x1FFF => {
                // ram
                // this makes sure that the ram is mirrored every 0x0800 bytes
                self.vram[(addr & 0x07FF) as usize]
            },
            0x2000 ..= 0x3FFF => {
                // ppu registers
                // self.vram[(addr & 0x2007) as usize]
                0
            },
            _ => {
                // invalid read
                println!("Invalid read at address: {:#06X}", addr);
                0
            }
        }
    }

    pub fn read_u16(&self, addr: u16) -> u16 {
        let low = self.read(addr);
        let high = self.read(addr + 1);
        u16::from_le_bytes([low, high])
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000 ..= 0x1FFF => {
                // ram
                // this makes sure that the ram is mirrored every 0x0800 bytes
                self.vram[(addr & 0x07FF) as usize] = val;
            },
            0x2000 ..= 0x3FFF => {
                // ppu registers
                // self.vram[(addr & 0x2007) as usize] = val;
            },
            _ => {
                // invalid write
                println!("Invalid write at address: {:#06X}", addr);
            }
        }
    }

    pub fn write_u16(&mut self, addr: u16, val: u16) {
        let bytes = val.to_le_bytes();
        self.write(addr, bytes[0]);
        self.write(addr + 1, bytes[1]);
    }
}