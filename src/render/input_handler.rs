use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct InputHandler {
    event_pump: sdl2::EventPump,
}

impl InputHandler {
    pub fn new(context: sdl2::Sdl) -> Self {
        // initializing the event pump
        let event_pump = context.event_pump().expect("Failed to get event pump");

        Self { event_pump }
    }

    pub fn handle_input(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return true,
                _ => {}
            }
        }

        false
    }
}