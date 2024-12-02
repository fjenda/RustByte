use rust_byte::ppu::ppu::PPU;
#[cfg(test)]
pub mod test {
    use rust_byte::ppu::mirroring::Mirroring;
    use super::*;

    #[test]
    fn verify_vram_write_behavior() {
        let mut ppu = PPU::new_empty_rom();

        ppu.write_address_register(0x22);
        ppu.write_address_register(0x15);
        ppu.write(0x38);

        assert_eq!(ppu.ram[0x0215], 0x38);
    }

    #[test]
    fn verify_vram_read_sequence() {
        let mut ppu = PPU::new_empty_rom();

        ppu.write_control_register(0);
        ppu.ram[0x02F4] = 0x34;

        ppu.write_address_register(0x22);
        ppu.write_address_register(0xF4);

        // pre-fetch
        ppu.read();

        assert_eq!(ppu.address_register.get(), 0x22F5);
        assert_eq!(ppu.read(), 0x34);
    }

    #[test]
    fn verify_vram_read_wraparound() {
        let mut ppu = PPU::new_empty_rom();

        ppu.ram[0x01FE] = 0x11;
        ppu.ram[0x01FF] = 0x22;

        ppu.write_address_register(0x21);
        ppu.write_address_register(0xFE);

        // pre-fetch
        ppu.read();
        assert_eq!(ppu.read(), 0x11);
        assert_eq!(ppu.read(), 0x22);
    }

    #[test]
    fn verify_vram_read_step_size() {
        let mut ppu = PPU::new_empty_rom();
        ppu.write_control_register(0b100); // Step 32
        ppu.ram[0x01E0] = 0x33;
        ppu.ram[0x01E0 + 32] = 0x44;
        ppu.ram[0x01E0 + 64] = 0x55;

        ppu.write_address_register(0x21);
        ppu.write_address_register(0xE0);

        // pre-fetch
        ppu.read();
        assert_eq!(ppu.read(), 0x33);
        assert_eq!(ppu.read(), 0x44);
        assert_eq!(ppu.read(), 0x55);
    }

    #[test]
    fn test_horizontal_mirroring_logic() {
        let mut ppu = PPU::new_empty_rom();
        ppu.write_address_register(0x25);
        ppu.write_address_register(0x10);

        // write to mirrored region A
        ppu.write(0xAA);

        ppu.write_address_register(0x29);
        ppu.write_address_register(0x10);

        // write to mirrored region B
        ppu.write(0xBB);

        ppu.write_address_register(0x21);
        ppu.write_address_register(0x10);

        // pre-fetch
        ppu.read();

        // verify read from region A
        assert_eq!(ppu.read(), 0xAA);

        ppu.write_address_register(0x2D);
        ppu.write_address_register(0x10);

        // pre-fetch
        ppu.read();

        // verify read from region B
        assert_eq!(ppu.read(), 0xBB);
    }

    #[test]
    fn test_vertical_mirroring_logic() {
        let mut ppu = PPU::new(vec![0; 2048], Mirroring::Vertical);

        ppu.write_address_register(0x20);
        ppu.write_address_register(0x07);

        // write to mirrored region A
        ppu.write(0xCC);

        ppu.write_address_register(0x2C);
        ppu.write_address_register(0x07);

        // write to mirrored region B
        ppu.write(0xDD);

        ppu.write_address_register(0x28);
        ppu.write_address_register(0x07);

        // pre-fetch
        ppu.read();

        // verify read from region A
        assert_eq!(ppu.read(), 0xCC);

        ppu.write_address_register(0x24);
        ppu.write_address_register(0x07);

        // pre-fetch
        ppu.read();

        // verify read from region B
        assert_eq!(ppu.read(), 0xDD);
    }


    #[test]
    fn validate_status_latch_reset() {
        let mut ppu = PPU::new_empty_rom();
        ppu.ram[0x0307] = 0x88;

        ppu.write_address_register(0x21);
        ppu.write_address_register(0x23);
        ppu.write_address_register(0x07);

        // pre-fetch
        ppu.read();
        assert_ne!(ppu.read(), 0x88);

        // reset latch
        ppu.read_status_register();

        ppu.write_address_register(0x23);
        ppu.write_address_register(0x07);

        // pre-fetch
        ppu.read();
        assert_eq!(ppu.read(), 0x88);
    }

    #[test]
    fn validate_oam_read_write() {
        let mut ppu = PPU::new_empty_rom();
        ppu.write_oam_address(0x12);
        ppu.write_oam_data(0x99);
        ppu.write_oam_data(0xAA);

        ppu.write_oam_address(0x12);
        assert_eq!(ppu.read_oam_data(), 0x99);

        ppu.write_oam_address(0x13);
        assert_eq!(ppu.read_oam_data(), 0xAA);
    }

    #[test]
    fn validate_oam_dma_transfer() {
        let mut ppu = PPU::new_empty_rom();

        let mut dma_buffer = [0x55; 256];
        dma_buffer[0] = 0xAA;
        dma_buffer[255] = 0xBB;

        ppu.write_oam_address(0x00);
        ppu.write_oam_dma(&dma_buffer);

        ppu.write_oam_address(0x00);
        assert_eq!(ppu.read_oam_data(), 0xAA);

        ppu.write_oam_address(0x80);
        assert_eq!(ppu.read_oam_data(), 0x55);

        ppu.write_oam_address(0xFF);
        assert_eq!(ppu.read_oam_data(), 0xBB);
    }
}