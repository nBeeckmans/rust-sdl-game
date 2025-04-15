mod scene;
mod square;
mod drawable;
mod object;
mod transformable;

extern crate sdl3;

use std::ops::Deref;
use sdl3::pixels::Color;
use sdl3::keyboard::Keycode;
use sdl3::event::Event;
use std::time::Duration;
use std::f32::consts::PI;
use crate::square::Square;
use crate::scene::Scene;
use crate::drawable::Drawable;
use crate::object::Object;
use crate::transformable::Transformable;
use std::borrow::BorrowMut;

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
    let mut object = Object::new((0,0), 0);
    (*object).borrow_mut().rotate(PI / (3f32));
    scene.add(Box::new((*object).borrow_mut().clone()));
    scene.add(Box::new(square));
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
