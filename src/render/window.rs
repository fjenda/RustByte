extern crate sdl2;

use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

/// Class representing a window
pub struct Window {
    /// The window title
    title: String,

    /// The window width
    width: u32,

    /// The window height
    height: u32,

    /// Input Handler
    input_handler: crate::render::input_handler::InputHandler,

    /// The SDL canvas
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Window {
    /// Creation of a new window
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        // initializing the SDL2 context
        let context = sdl2::init().expect("Failed to initialize SDL2");

        // initializing the video subsystem
        let video_subsystem = context.video().expect("Failed to initialize video subsystem");

        // creating the window
        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .opengl()
            .build()
            .expect("Failed to create window");

        // creating the canvas
        let mut canvas = window.into_canvas().build().expect("Failed to create canvas");

        // input handler
        let input_handler = crate::render::input_handler::InputHandler::new(context);

        canvas.set_scale(10.0, 10.0).expect("Failed to set scale");

        Self {
            title: title.to_string(),
            width,
            height,
            input_handler,
            canvas,
        }
    }

    /// Draw the window
    pub fn draw(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        self.canvas.clear();
        self.canvas.present();

        'running: loop {
            if self.input_handler.handle_input() {
                break 'running;
            }

            self.canvas.clear();
            self.canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
            // The rest of the game loop goes here...
        }

        Ok(())
    }
}