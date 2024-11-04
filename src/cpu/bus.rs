use crate::cpu::cartridge::Cartridge;
#[derive(Debug)]
pub struct Bus {
    v_ram: [u8; 2048],
    cartridge: Cartridge
}

impl Bus {
    pub fn new(cartridge: Cartridge) -> Self {
        Bus {
            v_ram: [0; 2048],
            cartridge,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000 ..= 0x1FFF => {
                // ram
                // this makes sure that the ram is mirrored every 0x0800 bytes
                self.v_ram[(addr & 0x07FF) as usize]
            },
            0x2000 ..= 0x3FFF => {
                // ppu registers
                // self.vram[(addr & 0x2007) as usize]
                0
            },
            0x8000 ..= 0xFFFF => {
                // cartridge
                self.read_from_rom(addr)
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
                self.v_ram[(addr & 0x07FF) as usize] = val;
            },
            0x2000 ..= 0x3FFF => {
                // ppu registers
                // self.vram[(addr & 0x2007) as usize] = val;
            },
            0x8000 ..= 0xFFFF => {
                // cartridge
                panic!("Write to ROM is not supported");
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

    fn read_from_rom(&self, addr: u16) -> u8 {
        // adjust address by subtractiong the base address
        let mut adjusted_addr = addr.wrapping_sub(0x8000);

        // check if we need to handle mirroring
        if self.cartridge.prg_rom.len() == 0x4000 && adjusted_addr >= 0x4000 {
            // wrap address
            adjusted_addr %=  0x4000;
        }

        // return the value at the adjusted address
        self.cartridge.prg_rom[adjusted_addr as usize]
    }
}