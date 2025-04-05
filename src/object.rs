use sdl3::render::WindowCanvas;
use crate::drawable::Drawable;
use crate::transformable::Transformable;

pub struct Object {
    pub children : Vec<Box<dyn Drawable>>,
    pub position : (i32, i32),
}

impl Drawable for Object {
    fn draw(&self, window_canvas: &mut WindowCanvas) {
        for i in &self.children {
            i.draw(window_canvas);
        }
    }
}

impl Transformable for Object {
    fn rotate(&self, by: f32) {

    }

    fn translate(&self, by: (i32, i32)) {
    }
}
