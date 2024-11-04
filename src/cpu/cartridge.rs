// https://www.nesdev.org/wiki/Mirroring#Nametable_Mirroring
// https://formats.kaitai.io/ines/index.html
// https://www.nesdev.org/wiki/INES#iNES_file_format

use crate::cpu::mirroring::Mirroring;

#[derive(Debug)]
pub struct Cartridge {
    pub prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    mapper: u8,
    mirroring: Mirroring,
}

impl Cartridge {
    pub fn new(data: Vec<u8>) -> Result<Cartridge, &'static str> {
        // check if file is valid iNES file
        if data[0..4] != [0x4E, 0x45, 0x53, 0x1A] {
            return Err("Invalid iNES file");
        }

        // mapper information
        let mapper_id = (data[7] & 0xF0) | ((data[6] & 0xF0) >> 4);

        // iNes version
        let ines_version = data[7] & 3;
        if ines_version != 0 {
            return Err("Only iNES version 0 is supported");
        }

        // mirroring
        let four = data[6] & 4 != 0;
        let vert = data[6] & 1 != 0;

        let mirr = match (four, vert) {
            (false, false) => Mirroring::Horizontal,
            (false, true) => Mirroring::Vertical,
            (true, _) => Mirroring::FourScreen,
        };

        // rom sizes
        let prg_rom_size = data[4] as usize * 0x4000;
        let chr_rom_size = data[5] as usize * 0x2000;

        // trainer
        let has_trainer = data[6] & 4 != 0;

        // starting indices
        let prg_rom_start = 16 + if has_trainer { 512 } else { 0 };
        let chr_rom_start = prg_rom_start + prg_rom_size;

        Ok(Cartridge {
            prg_rom: data[prg_rom_start..(prg_rom_start + prg_rom_size)].to_vec(),
            chr_rom: data[chr_rom_start..(chr_rom_start + chr_rom_size)].to_vec(),
            mapper: mapper_id,
            mirroring: mirr,
        })
    }
}