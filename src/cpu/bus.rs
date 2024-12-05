// https://www.nesdev.org/wiki/CPU_memory_map

// [0x2000 - 0x4020] => redirected to hardware modules
// [0x4020 - 0x6000] => unmapped, for cartridge use
// [0x6000 - 0x8000] => cartridge RAM
// [0x8000 - 0xFFFF] => Program ROM

// Special addresses
// [0xFFFC - 0xFFFD] => Reset vector

use crate::ppu::cartridge::Cartridge;
use crate::ppu::ppu::PPU;

pub struct Bus<'callback> {
    /// 2kB of RAM
    ram: [u8; 2048],

    /// Program ROM
    prg: Vec<u8>,

    /// PPU
    ppu: PPU,

    /// Number of cycles
    pub cycles: usize,

    /// Game callback
    game: Box<dyn FnMut(&PPU) + 'callback>
}

/// Implementation of the Bus.
/// The bus is the component that connects all the different parts of the NES
/// It is responsible for reading and writing to the different memory regions
/// https://wiki.nesdev.com/w/index.php/CPU_memory_map
impl<'a> Bus<'a> {
    /// Create a new Bus
    pub fn new<'callback, F>(cartridge: Cartridge, callback: F) -> Bus<'callback>
    where
        F: FnMut(&PPU) + 'callback,
    {
        let ppu = PPU::new(cartridge.chr_rom, cartridge.mirroring);

        Bus {
            ram: [0; 2048],
            prg: cartridge.prg_rom,
            ppu,
            cycles: 0,
            game: Box::from(callback),
        }
    }
    //
    // pub fn new<'callback>(cartridge: Cartridge) -> Bus<'callback> {
    //     let ppu = PPU::new(cartridge.chr_rom, cartridge.mirroring);
    //
    //     Bus {
    //         ram: [0; 2048],
    //         prg: cartridge.prg_rom,
    //         ppu,
    //         cycles: 0,
    //         game: Box::from(|_ppu: &PPU| {}),
    //     }
    // }

    /// Function that ticks the bus, updating the number of cycles and the PPU
    pub fn tick(&mut self, cycles: u8) {
        // update cycles
        self.cycles += cycles as usize;

        let nmi_before = self.ppu.nmi;

        // PPU ticks 3 times faster than the CPU
        self.ppu.tick(cycles * 3);

        let nmi_after = self.ppu.nmi;

        if !nmi_before && nmi_after {
            // call the game callback
            (self.game)(&self.ppu);
        }
    }

    /// Function that gets the NMI status from the PPU
    pub fn nmi_status(&mut self) -> bool {
        self.ppu.nmi
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
                // panic!("Write only register at address: {:#06X}", addr);
                0
            },
            0x2002 => {
                // PPUSTATUS
                self.ppu.read_status_register()
            },
            0x2004 => {
                // PPUOAMDATA
                self.ppu.read_oam_data()
            }
            0x2007 => {
                // PPUDATA
                self.ppu.read()
            },
            0x4000 ..= 0x4015 => {
                // APU
                0
            },
            0x4016 => {
                // JOYPAD1
                0
            },
            0x4017 => {
                // JOYPAD2
                0
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
        // let low = self.read(addr);
        // let high = self.read(addr + 1);
        // u16::from_le_bytes([low, high])
        let lo = self.read(addr) as u16;
        let hi = self.read(addr + 1) as u16;
        (hi << 8) | (lo as u16)
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
                // PPUADDR
                self.ppu.write_address_register(val);
            },
            0x2007 => {
                // PPUDATA
                self.ppu.write(val);
            },
            0x4000..=0x4013 | 0x4015 => {
                // APU
            },
            0x4016 => {
                // JOYPAD1
            },
            0x4017 => {
                // JOYPAD2
            },
            // https://wiki.nesdev.com/w/index.php/PPU_programmer_reference#OAM_DMA_.28.244014.29_.3E_write
            0x4014 => {
                let mut buffer: [u8; 256] = [0; 256];
                let hi: u16 = (val as u16) << 8;
                for i in 0..256u16 {
                    buffer[i as usize] = self.read(hi + i);
                }

                self.ppu.write_oam_dma(&buffer);

                // todo: handle this eventually
                // let add_cycles: u16 = if self.cycles % 2 == 1 { 514 } else { 513 };
                // self.tick(add_cycles); //todo this will cause weird effects as PPU will have 513/514 * 3 ticks
            },
            0x2008 ..= 0x3FFF => {
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