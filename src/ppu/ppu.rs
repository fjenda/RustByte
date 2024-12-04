use crate::byte_status::ByteStatus;
use crate::flags::PPUStatus;
use crate::ppu::mirroring::Mirroring;
use crate::ppu::address_register::AddressRegister;
use crate::ppu::controller_register::ControllerRegister;
use crate::ppu::mask_register::MaskRegister;
use crate::ppu::scroll_register::ScrollRegister;
use crate::ppu::status_register::StatusRegister;

/// Class representing the PPU
/// https://www.nesdev.org/wiki/PPU
/// https://www.nesdev.org/wiki/PPU_registers
#[derive(Debug)]
pub struct PPU {
    /// PPU Memory
    /// 2kB of RAM dedicated to PPU
    pub ram: [u8; 2048],

    /// Palette tables
    /// 32 bytes of palette data
    palette: [u8; 32],

    /// Visuals of the cartridge
    chr: Vec<u8>,

    /// Internal memory storing sprites
    /// max. 64 sprites (4 bytes each) = 256 bytes
    /// https://www.nesdev.org/wiki/PPU_OAM
    oam: [u8; 256],
    oam_address: u8,

    /// Mirroring mode
    /// https://www.nesdev.org/wiki/Mirroring
    mirroring: Mirroring,

    /// PPUCTRL - Controller Register ($2000)
    controller_register: ControllerRegister,

    /// PPUMASK - Mask Register ($2001)
    mask_register: MaskRegister,

    /// PPUSTATUS - Status Register ($2002)
    pub status_register: StatusRegister,

    /// PPUSCROLL - Scroll Register ($2005)
    scroll_register: ScrollRegister,

    /// PPUADDR - Address Register ($2006)
    pub address_register: AddressRegister,

    /// Internal buffer for reading and writing
    internal_buffer: u8,

    /// Cycle counter
    cycles: usize,

    /// Scanline counter
    scanline: u16,

    /// NMI Interrupt
    pub nmi: bool,
}

impl PPU {
    /// Create a new PPU
    pub fn new(chr: Vec<u8>, mirroring: Mirroring) -> Self {
        PPU {
            ram: [0; 2048],
            palette: [0; 32],
            chr,
            oam: [0; 256],
            oam_address: 0,
            mirroring,
            controller_register: ControllerRegister::new(),
            mask_register: MaskRegister::new(),
            status_register: StatusRegister::new(),
            scroll_register: ScrollRegister::new(),
            address_register: AddressRegister::new(),
            internal_buffer: 0,
            cycles: 0,
            scanline: 0,
            nmi: false,
        }
    }

    /// Create a new PPU with an empty ROM
    pub fn new_empty_rom() -> Self {
        PPU::new(vec![0; 2048], Mirroring::Horizontal)
    }

    /// Function that ticks the PPU
    /// It ticks 3 times faster than the CPU
    /// It's used to determine which scanline the PPU is currently rendering
    /// https://wiki.nesdev.com/w/index.php/PPU_rendering
    pub fn tick(&mut self, cycles: u8) -> bool {
        // total scanlines: 262
        // one scanline -> 341 cycles
        // nmi interrupt on scanline 241

        self.cycles += cycles as usize;

        if self.cycles >= 341 {
            self.cycles = self.cycles - 341;
            self.scanline += 1;

            if self.scanline == 241 {
                // set the vblank flag
                self.status_register.add(PPUStatus::Vblank.as_u8());
                self.status_register.add(PPUStatus::Sprite0Hit.as_u8());

                // trigger NMI
                if self.controller_register.vblank() {
                    self.nmi = true;
                }
            }

            if self.scanline >= 262 {
                self.scanline = 0;
                self.status_register.remove(PPUStatus::Vblank.as_u8());
                self.status_register.remove(PPUStatus::Sprite0Hit.as_u8());
                self.nmi = false;
                return true;
            }
        }

        false
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

            // table 3 maps to table 1
            (Mirroring::Horizontal, 1) => vram_index - 0x400,

            // table 3 maps to table 1
            (Mirroring::Horizontal, 3) => vram_index - 0x800,

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
            0x0000 ..= 0x1FFF => {
                // pattern tables
                let res = self.internal_buffer;
                self.internal_buffer = self.chr[addr as usize];
                res
            },
            0x2000 ..= 0x2FFF => {
                // name tables
                let res = self.internal_buffer;
                self.internal_buffer = self.ram[self.mirror(addr) as usize];
                res
            },
            0x3000 ..= 0x3EFF => {
                // unused
                panic!("Reading from 0x3000 - 0x3EFF is not expected");
            },
            0x3F10 | 0x3F14 | 0x3F18 | 0x3F1C => {
                // mirror of 0x3F00 - 0x3F0F
                self.palette[(addr - 0x3F00 - 0x10) as usize]
            },
            0x3F00 ..= 0x3FFF => {
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
            0x0000 ..= 0x1FFF => {
                // pattern tables
                panic!("Writing to 0x0000 - 0x1FFF (CHR) is not expected");
            },
            0x2000 ..= 0x2FFF => {
                // name tables
                self.ram[self.mirror(addr) as usize] = val;
            },
            0x3000 ..= 0x3EFF => {
                // unused
                panic!("Writing to 0x3000 - 0x3EFF is not expected");
            },
            0x3F10 | 0x3F14 | 0x3F18 | 0x3F1C => {
                // mirror of 0x3F00 - 0x3F0F
                self.palette[(addr - 0x3F00 - 0x10) as usize] = val;
            },
            0x3F00 ..= 0x3FFF => {
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

    pub fn read_status_register(&mut self) -> u8 {
        let res = self.status_register.value;

        // clear the vblank flag
        self.status_register.remove(PPUStatus::Vblank.as_u8());

        // clear the address latch
        self.address_register.reset_high_byte();

        // clear the scroll latch
        self.scroll_register.reset_latch();

        res
    }

    // Helper functions for reading and writing to the PPU registers
    pub fn write_oam_dma(&mut self, data: &[u8; 256]) {
        for x in data.iter() {
            self.oam[self.oam_address as usize] = *x;
            self.oam_address = self.oam_address.wrapping_add(1);
        }
    }

    pub fn read_oam_data(&mut self) -> u8 {
        self.oam[self.oam_address as usize]
    }

    pub fn write_oam_data(&mut self, val: u8) {
        self.oam[self.oam_address as usize] = val;
        self.oam_address = self.oam_address.wrapping_add(1);
    }

    pub fn write_oam_address(&mut self, val: u8) {
        self.oam_address = val;
    }

    pub fn write_control_register(&mut self, val: u8) {
        let before_nmi = self.controller_register.vblank();
        self.controller_register.set_bits(val);

        if !before_nmi && self.controller_register.vblank() && self.status_register.is_set(PPUStatus::Vblank.as_u8()) {
            self.nmi = true;
        }
    }

    pub fn write_mask_register(&mut self, val: u8) {
        self.mask_register.set_bits(val);
    }

    pub fn write_scroll_register(&mut self, val: u8) {
        self.scroll_register.write(val);
    }

    pub fn write_address_register(&mut self, val: u8) {
        self.address_register.set(val);
    }
}

