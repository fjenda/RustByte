use crate::byte_status::ByteStatus;
use crate::flags::Button;
use crate::render::input::button_status::ButtonStatus;

pub struct Joypad {
    strobe: bool,
    index: u8,
    status: ButtonStatus
}

impl Default for Joypad {
    fn default() -> Self {
        Self::new()
    }
}

impl Joypad {
    fn new() -> Self {
        Joypad {
            strobe: false,
            index: 0,
            status: ButtonStatus::default(),
        }
    }
    
    pub fn write(&mut self, val: u8) {
        self.strobe = val & 1 == 1;
        
        if self.strobe {
            self.index = 0;
        }
    }
    
    pub fn read(&mut self) -> u8 {
        if self.index > 7 {
            return 1;
        }
        
        let res = (self.status.value & (1 << self.index)) >> self.index;
        if !self.strobe && self.index <= 7 {
            self.index += 1;
        }
        
        res
    }
    
    pub fn add(&mut self, button: Button) {
        self.status.add(button.as_u8());
    }
    
    pub fn remove(&mut self, button: Button) {
        self.status.remove(button.as_u8());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_strobe_mode() {
        let mut joypad = Joypad::new();
        joypad.write(1);
        joypad.add(Button::A);
        for _x in 0..10 {
            assert_eq!(joypad.read(), 1);
        }
    }

    #[test]
    fn test_strobe_mode_on_off() {
        let mut joypad = Joypad::new();

        joypad.write(0);
        joypad.add(Button::RIGHT);
        joypad.add(Button::LEFT);
        joypad.add(Button::SELECT);
        joypad.add(Button::B);

        for _ in 0..=1 {
            assert_eq!(joypad.read(), 0);
            assert_eq!(joypad.read(), 1);
            assert_eq!(joypad.read(), 1);
            assert_eq!(joypad.read(), 0);
            assert_eq!(joypad.read(), 0);
            assert_eq!(joypad.read(), 0);
            assert_eq!(joypad.read(), 1);
            assert_eq!(joypad.read(), 1);

            for _x in 0..10 {
                assert_eq!(joypad.read(), 1);
            }
            joypad.write(1);
            joypad.write(0);
        }
    }
}