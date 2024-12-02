// https://www.nesdev.org/wiki/CPU_memory_map

// [0x2000 - 0x4020] => redirected to hardware modules
// [0x4020 - 0x6000] => unmapped, for cartridge use
// [0x6000 - 0x8000] => cartridge RAM
// [0x8000 - 0xFFFF] => Program ROM

// Special addresses
// [0xFFFC - 0xFFFD] => Reset vector

use crate::ppu::cartridge::Cartridge;
use crate::ppu::ppu::PPU;

#[derive(Debug)]
pub struct Bus {
    ram: [u8; 2048],
    prg: Vec<u8>,
    ppu: PPU,
}

/// Implementation of the Bus.
/// The bus is the component that connects all the different parts of the NES
/// It is responsible for reading and writing to the different memory regions
/// https://wiki.nesdev.com/w/index.php/CPU_memory_map
impl Bus {
    /// Create a new Bus
    pub fn new(cartridge: Cartridge) -> Self {
        let ppu = PPU::new(cartridge.chr_rom, cartridge.mirroring);

        Bus {
            ram: [0; 2048],
            prg: cartridge.prg_rom,
            ppu,
        }
    }

    /// Function that returns a value read from the memory at a given address
    /// This function will handle the different memory regions
    pub fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000 ..= 0x1FFF => {
                // ram
                // this makes sure that the ram is mirrored every 0x0800 bytes
                self.ram[(addr & 0x07FF) as usize]
            },
            0x2000 | 0x2001 | 0x2003 | 0x2005 | 0x2006 | 0x4014 => {
                // write only registers
                panic!("Write only register at address: {:#06X}", addr);
            },
            0x2002 => {
                // ppu status register
                self.ppu.read_status_register()
            },
            0x2004 => {
                // ppu oam data register
                self.ppu.read_oam_data()
            }
            0x2007 => {
                // ppu data register
                self.ppu.read()
            },
            0x2008 ..= 0x3FFF => {
                let mirror_addr = addr & 0x2007;
                self.read(mirror_addr)
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

    /// Function that returns a u16 value read from the memory at a given address
    /// CPU uses Little-Endian addressing
    pub fn read_u16(&mut self, addr: u16) -> u16 {
        let low = self.read(addr);
        let high = self.read(addr + 1);
        u16::from_le_bytes([low, high])
    }

    /// Function that writes a value into the memory at a given address
    /// This function will handle the different memory regions
    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000 ..= 0x1FFF => {
                // ram
                // this makes sure that the ram is mirrored every 0x0800 bytes
                self.ram[(addr & 0x07FF) as usize] = val;
            },
            0x2000 => {
                // PPUCTRL
                self.ppu.write_control_register(val);
            },
            0x2001 => {
                // PPUMASK
                self.ppu.write_mask_register(val);
            },
            0x2003 => {
                // OAMADDR
                self.ppu.write_oam_address(val);
            },
            0x2004 => {
                // OAMDATA
                self.ppu.write_oam_data(val);
            },
            0x2005 => {
                // PPUSCROLL
                self.ppu.write_scroll_register(val);
            },
            0x2006 => {
                // ppu address register
                self.ppu.write_address_register(val);
            },
            0x2007 => {
                // ppu data register
                self.ppu.write(val);
            },
            0x2000 ..= 0x3FFF => {
                // ppu registers
                let mirror_addr = addr & 0x2007;
                self.write(mirror_addr, val);
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

    /// Function that writes a u16 value into the memory at a given address
    /// CPU uses Little-Endian addressing
    pub fn write_u16(&mut self, addr: u16, val: u16) {
        let bytes = val.to_le_bytes();
        self.write(addr, bytes[0]);
        self.write(addr + 1, bytes[1]);
    }

    /// Function that reads from the ROM
    fn read_from_rom(&mut self, addr: u16) -> u8 {
        // adjust address by subtracting the base address
        let mut adjusted_addr = addr.wrapping_sub(0x8000);

        // check if we need to handle mirroring
        if self.prg.len() == 0x4000 && adjusted_addr >= 0x4000 {
            // wrap address
            adjusted_addr %=  0x4000;
        }

        // return the value at the adjusted address
        self.prg[adjusted_addr as usize]
    }
}