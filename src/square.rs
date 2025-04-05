use sdl3::rect::Point;
use sdl3::render::WindowCanvas;
use crate::drawable::Drawable;

pub struct Square {
    pub position : (i32, i32),  // left right
    pub side : i32,
    pub color : (u8,u8,u8,u8) // RGBA
}

impl Square {
    pub fn new(px : i32, py : i32, side : i32, r : u8, g: u8, b: u8, a: u8) -> Self{
        Self {position : (px, py), side, color : (r, g, b, a)}
    }
}

impl Drawable for Square {
    fn draw(&self, canvas:  &mut WindowCanvas) {
        canvas.set_draw_color(self.color);
        for i in 0..self.side {
            for j in 0..self.side{
                let position: (i32, i32) = (self.position.0 + i, self.position.1 + j);
                if canvas.draw_point(Point::new(position.0, position.1)).is_err() {
                    println!("error while drawing point \n");
                }
            }
        }
    }
}
