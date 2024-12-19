extern crate sdl2;

use std::collections::HashMap;
use rust_byte::cpu::bus::Bus;
use rust_byte::cpu::cpu::CPU;
use rust_byte::ppu::cartridge::Cartridge;
use rust_byte::ppu::ppu::PPU;
use rust_byte::render::frame::Frame;
use rust_byte::render::renderer::Renderer;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use rust_byte::flags::Button;
use rust_byte::render::input::joypad::Joypad;

fn main() {
    // init sdl2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Tile viewer", (256.0 * 3.0) as u32, (240.0 * 3.0) as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(3.0, 3.0).unwrap();

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 256, 240)
        .unwrap();

    // load the game
    let bytes: Vec<u8> = std::fs::read("assets/pacman.nes").unwrap();
    let rom = Cartridge::new(bytes).unwrap();
    let mut frame = Frame::new();
    
    // map keyboard to joypad
    let mut keys = HashMap::new();
    keys.insert(Keycode::S, Button::DOWN);
    keys.insert(Keycode::W, Button::UP);
    keys.insert(Keycode::D, Button::RIGHT);
    keys.insert(Keycode::A, Button::LEFT);
    keys.insert(Keycode::Space, Button::SELECT);
    keys.insert(Keycode::LCtrl, Button::START);
    keys.insert(Keycode::Q, Button::A);
    keys.insert(Keycode::E, Button::B);

    let bus = Bus::new(rom, move |ppu: &PPU, joy: &mut Joypad| {
        Renderer::render(ppu, &mut frame);
        texture.update(None, &frame.data, 256 * 3).unwrap();

        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(0),
                
                Event::KeyDown { keycode, .. } => {
                    if let Some(key) = keys.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                        joy.add(*key);
                    }
                },
                
                Event::KeyUp { keycode, .. } => {
                    if let Some(key) = keys.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                        joy.remove(*key);
                    }
                },
                
                _ => { }
            }
        }
    });

    let mut cpu = CPU::new(bus);
    cpu.reset();
    cpu.interpret_callback(|cpu| {});
}