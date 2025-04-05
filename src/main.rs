mod scene;
mod square;
mod drawable;
mod object;
mod transformable;

extern crate sdl3;

use sdl3::pixels::Color;
use sdl3::keyboard::Keycode;
use sdl3::event::Event;
use std::time::Duration;
use crate::square::Square;
use crate::scene::Scene;
use crate::drawable::Drawable;

const FRAME_RATE : u32 = 1_000_000_000u32/60;

fn main() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem =
        sdl_context.video().unwrap();

    let window =
        video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
            .position_centered()
            .build()
            .unwrap();

    let mut canvas = window.into_canvas();

    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut scene = Scene::new();
    let square = Square::new(400, 300, 60, 255,255,255,255);
    scene.add(square);
    'running: loop {
        canvas.set_draw_color(Color::RGB(10,10,10));
        canvas.clear();
        scene.draw(&mut canvas);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode : Some(Keycode::Escape), ..} => {
                    break 'running;
                },
                _ => {}
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0,FRAME_RATE));
    }
}
