use crate::ppu::mirroring::Mirroring;
use crate::ppu::ppu::PPU;
use crate::render::color_palette::PALETTE;
use crate::render::frame::Frame;
use crate::render::tile::Slice;

/// Renderer struct responsible for rendering the game state to the screen
pub struct Renderer { }

impl Renderer {
    pub fn new() -> Self {
        Renderer {}
    }

    pub fn render(ppu: &PPU, frame: &mut Frame) {
        let offset_x = ppu.scroll_register.scroll_x as usize;
        let offset_y = ppu.scroll_register.scroll_y as usize;
        
        let (main_name_table, second_name_table) = match (&ppu.mirroring, ppu.controller_register.nametable()) {
            (Mirroring::Vertical, 0x2000) | (Mirroring::Vertical, 0x2800) | (Mirroring::Horizontal, 0x2000) | (Mirroring::Horizontal, 0x2400) => {
                (&ppu.ram[0..0x400], &ppu.ram[0x400..0x800])
            },
        
            (Mirroring::Vertical, 0x2400) | (Mirroring::Vertical, 0x2C00) | (Mirroring::Horizontal, 0x2800) | (Mirroring::Horizontal, 0x2C00) => {
                (&ppu.ram[0x400..0x800], &ppu.ram[0..0x400])
            },
        
            (_,_) => panic!("unsupported mirroring mode"),
        };
        
        Self::render_slice(ppu, frame, main_name_table, Slice::new(offset_x, offset_y, 256, 240), -(offset_x as isize), -(offset_y as isize));
        
        if offset_x > 0 {
            Self::render_slice(ppu, frame, second_name_table, Slice::new(0, 0, offset_x, 240), (256 - offset_x) as isize, 0);
        } else if offset_y > 0 {
            Self::render_slice(ppu, frame, second_name_table, Slice::new(0, 0, 256, offset_y), 0, (240 - offset_y) as isize);
        }
        
        for i in (0..ppu.oam.len()).step_by(4).rev() {
            let tile_idx = ppu.oam[i + 1] as u16;
            let tile_x = ppu.oam[i + 3] as usize;
            let tile_y = ppu.oam[i] as usize;

            let flip_vertical = ppu.oam[i + 2] >> 7 & 1 == 1;
            let flip_horizontal = ppu.oam[i + 2] >> 6 & 1 == 1;
            
            let pallette_idx = ppu.oam[i + 2] & 0b11;
            let sprite_palette = Self::sprite_pal(ppu, pallette_idx);
            let bank: u16 = ppu.controller_register.sprite_pattern_table();

            let tile = &ppu.chr[(bank + tile_idx * 16) as usize..=(bank + tile_idx * 16 + 15) as usize];


            for y in 0..=7 {
                let mut upper = tile[y];
                let mut lower = tile[y + 8];
                'ololo: for x in (0..=7).rev() {
                    let value = (1 & lower) << 1 | (1 & upper);
                    upper >>= 1;
                    lower >>= 1;
                    let rgb = match value {
                        0 => continue 'ololo, // skip coloring the pixel
                        1 => PALETTE[sprite_palette[1] as usize],
                        2 => PALETTE[sprite_palette[2] as usize],
                        3 => PALETTE[sprite_palette[3] as usize],
                        _ => panic!("can't be"),
                    };
                    match (flip_horizontal, flip_vertical) {
                        (false, false) => frame.set_pixel(tile_x + x, tile_y + y, rgb),
                        (true, false) => frame.set_pixel(tile_x + 7 - x, tile_y + y, rgb),
                        (false, true) => frame.set_pixel(tile_x + x, tile_y + 7 - y, rgb),
                        (true, true) => frame.set_pixel(tile_x + 7 - x, tile_y + 7 - y, rgb),
                    }
                }
            }
        }
    }

    fn bg_pal(ppu: &PPU, attribute_table: &[u8], tile_column: usize, tile_row : usize) -> [u8; 4] {
        let attr_table_idx = tile_row / 4 * 8 +  tile_column / 4;
        let attr_byte = attribute_table[attr_table_idx];

        let palette_idx = match (tile_column % 4 / 2, tile_row % 4 / 2) {
            (0,0) => attr_byte & 0b11,
            (1,0) => (attr_byte >> 2) & 0b11,
            (0,1) => (attr_byte >> 4) & 0b11,
            (1,1) => (attr_byte >> 6) & 0b11,
            (_,_) => panic!("wrong palette index"),
        };

        let palette_start: usize = 1 + (palette_idx as usize) * 4;
        [ppu.palette[0], ppu.palette[palette_start], ppu.palette[palette_start + 1], ppu.palette[palette_start + 2]]
    }

    fn sprite_pal(ppu: &PPU, pallete_idx: u8) -> [u8; 4] {
        let start = 0x11 + (pallete_idx * 4) as usize;
        [
            0,
            ppu.palette[start],
            ppu.palette[start + 1],
            ppu.palette[start + 2],
        ]
    }
    
    fn render_slice(ppu: &PPU, frame: &mut Frame, name_table: &[u8], slice: Slice, offset_x: isize, offset_y: isize) {
        let background = ppu.controller_register.background_pattern_table();
        let attr = &name_table[0x3C0 .. 0x400];
        
        for i in 0 .. 0x3C0 {
            let col = i % 32;
            let row = i / 32;
            let idx = name_table[i] as u16;
            let tile = &ppu.chr[(background + idx * 16) as usize ..= (background + idx * 16 + 15) as usize];
            let palette = Self::bg_pal(ppu, attr, col, row);
            
            for y in 0 ..= 7 {
                let mut upper = tile[y];
                let mut lower = tile[y + 8];
                
                for x in (0 ..= 7).rev() {
                    let value = (1 & lower) << 1 | (1 & upper);
                    upper >>= 1;
                    lower >>= 1;
                    
                    let rgb = match value {
                        0 => PALETTE[ppu.palette[0] as usize],
                        1 => PALETTE[palette[1] as usize],
                        2 => PALETTE[palette[2] as usize],
                        3 => PALETTE[palette[3] as usize],
                        _ => panic!("can't be"),
                    };
                    
                    let pixel_x = col * 8 + x;
                    let pixel_y = row * 8 + y;
                    
                    if pixel_x >= slice.x1 && pixel_x < slice.x2 && pixel_y >= slice.y1 && pixel_y < slice.y2 {
                        frame.set_pixel((offset_x + pixel_x as isize) as usize, (offset_y + pixel_y as isize) as usize, rgb);
                    }
                }
            }
        }
    } 
}