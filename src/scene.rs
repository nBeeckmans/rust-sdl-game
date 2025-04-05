use sdl3::render::WindowCanvas;
use crate::drawable::Drawable;

pub struct Scene {
    pub list : Vec<Box<dyn Drawable>>,
}

impl Scene {
    pub fn new() -> Self {
        Self {list: Vec::new()}
    }


    pub fn add(&mut self, element : impl Drawable + 'static) {
        self.list.push(Box::new(element));
    }
}

impl Drawable for Scene {
    fn draw(&self, canvas : &mut WindowCanvas) {
        for i in &self.list {
            i.draw(canvas);
        }
    }
}