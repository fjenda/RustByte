use crate::ppu::mirroring::Mirroring;
use crate::ppu::address_register::AddressRegister;
use crate::ppu::controller_register::ControllerRegister;
use crate::ppu::mask_register::MaskRegister;

/// Class representing the PPU
/// https://www.nesdev.org/wiki/PPU
/// https://www.nesdev.org/wiki/PPU_registers
#[derive(Debug)]
pub struct PPU {
    /// PPU Memory
    /// 2kB of RAM dedicated to PPU
    ram: [u8; 2048],

    /// Palette tables
    /// 32 bytes of palette data
    palette: [u8; 32],

    /// Visuals of the cartridge
    chr: Vec<u8>,

    /// Internal memory storing sprites
    /// max. 64 sprites (4 bytes each) = 256 bytes
    /// https://www.nesdev.org/wiki/PPU_OAM
    oam: [u8; 256],

    /// Mirroring mode
    /// https://www.nesdev.org/wiki/Mirroring
    mirroring: Mirroring,

    /// PPUCTRL - Controller Register ($2000)
    pub controller_register: ControllerRegister,

    /// PPUMASK - Mask Register ($2001)
    pub mask_register: MaskRegister,

    /// PPUSTATUS - Status Register ($2002)
    pub status_register: StatusRegister,

    /// PPUADDR - Address Register ($2006)
    pub address_register: AddressRegister,

    /// Internal buffer for reading and writing
    internal_buffer: u8,
}

impl PPU {
    /// Create a new PPU
    pub fn new(chr: Vec<u8>, mirroring: Mirroring) -> Self {
        PPU {
            ram: [0; 2048],
            palette: [0; 32],
            chr,
            oam: [0; 256],
            mirroring,
            address_register: AddressRegister::new(),
            controller_register: ControllerRegister::new(),
            mask_register: MaskRegister::new(),
            internal_buffer: 0,
        }
    }

    /// Handle mirroring of the PPU
    /// https://www.nesdev.org/wiki/Mirroring#Nametable_Mirroring
    pub fn mirror(&self, addr: u16) -> u16 {
        // mirror down addresses in the range 0x3000-0x3EFF to 0x2000-0x2EFF
        let mirrored_vram = addr & 0x2FFF;

        // convert the address to a VRAM index (0x0000 - 0x0FFF)
        let vram_index = mirrored_vram - 0x2000;

        // determine the name table
        let name_table = vram_index / 0x400;

        // calculate the effective VRAM index based on mirroring mode and name table
        let effective_index = match (&self.mirroring, name_table) {
            // vertical mirroring: map tables 2 and 3 back to 0 and 1
            (Mirroring::Vertical, 2) | (Mirroring::Vertical, 3) => vram_index - 0x800,

            // horizontal mirroring:
            // table 2 maps to table 0
            (Mirroring::Horizontal, 2) => vram_index - 0x400,

            // table 1 maps back to table 0, and table 3 to table 1
            // (Mirroring::Horizontal, 1) | (Mirroring::Horizontal, 3) => vram_index - 0x400 * name_table,
            (Mirroring::Horizontal, 1) | (Mirroring::Horizontal, 3) => vram_index - 0x800,

            // no adjustment needed for tables 0 and 1 in both mirroring types
            _ => vram_index,
        };

        effective_index
    }

    // dummy read/write operations
    // PPU can't access rom and ram directly

    /// Read from the PPU
    pub fn read(&mut self) -> u8 {
        // get the address from the address register
        let addr = self.address_register.get();

        // PPUADDR is incremented by 1 or 32 depending on the value of PPUCTRL
        self.address_register.add(self.controller_register.vram_increment());

        // https://www.nesdev.org/wiki/PPU_memory_map
        match addr {
            0x0000 .. 0x1FFF => {
                // pattern tables
                let res = self.internal_buffer;
                self.internal_buffer = self.chr[addr as usize];
                res
            },
            0x2000 .. 0x2FFF => {
                // name tables
                let res = self.internal_buffer;
                self.internal_buffer = self.ram[addr as usize];
                res
            },
            0x3000 .. 0x3EFF => {
                // unused
                panic!("Reading from 0x3000 - 0x3EFF is not expected");
            },
            0x3F00 .. 0x3FFF => {
                // palette
                self.palette[(addr - 0x3F00) as usize]
            },
            _ => {
                panic!("Reading from address {:04X} is not expected", addr);
            }
        }
    }

    /// Write to the PPU
    pub fn write(&mut self, val: u8) {
        // get the address from the address register
        let addr = self.address_register.get();

        match addr {
            0x0000 .. 0x1FFF => {
                // pattern tables
                panic!("Writing to 0x0000 - 0x1FFF (CHR) is not expected");
            },
            0x2000 .. 0x2FFF => {
                // name tables
                self.ram[self.mirror(addr) as usize] = val;
            },
            0x3000 .. 0x3EFF => {
                // unused
                panic!("Writing to 0x3000 - 0x3EFF is not expected");
            },
            0x3F00 .. 0x3FFF => {
                // palette
                self.palette[(addr - 0x3F00) as usize] = val;
            },
            _ => {
                panic!("Writing to address {:04X} is not expected", addr);
            }
        }

        // PPUADDR is incremented by 1 or 32 depending on the value of PPUCTRL
        self.address_register.add(self.controller_register.vram_increment());
    }
}