/// Struct representing a frame
/// It serves as an abstraction layer for SLD2
pub struct Frame {
    pub data: Vec<u8>,
}

impl Default for Frame {
    fn default() -> Self {
        Self::new()
    }
}

impl Frame {
    const WIDTH: usize = 256;
    const HEIGHT: usize = 240;

    pub fn new() -> Self {
        Frame {
            data: vec![0; Frame::WIDTH * Frame::HEIGHT * 3],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: (u8, u8, u8)) {
        let index = (y * Frame::WIDTH + x) * 3;

        if index + 2 < self.data.len() {
            self.data[index] = color.0;
            self.data[index + 1] = color.1;
            self.data[index + 2] = color.2;
        }
    }
}