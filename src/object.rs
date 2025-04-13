use std::fmt::Pointer;
use std::ops::Deref;
use std::rc::Rc;
use std::cell::{Ref, RefCell};
use sdl3::render::WindowCanvas;
use crate::drawable::Drawable;
use crate::transformable::Transformable;


pub struct Object {
    primitives : RefCell<Vec<Box<dyn Drawable>>>,
    children : RefCell<Vec<Rc<Object>>>,
    position : (i32, i32),
    z: i32,
}

impl Drawable for Object {
    fn draw(&self, window_canvas: &mut WindowCanvas) {
        for i in self.primitives.borrow().iter() {
            i.draw(window_canvas);
        }
    }
}

impl Drawable for &Object {
    fn draw(&self, window_canvas: &mut WindowCanvas) {
        (**self).draw(window_canvas);
    }
}

impl Transformable for Object {
    fn rotate(&mut self, by: f32) {
        let mut r = (self.position.0 * self.position.0 + self.position.1 * self.position.1) as f32;
        r = r.sqrt();
        self.position.0 += (r * by.cos()).round() as i32;
        self.position.1 += (r * by.sin()).round() as i32;
    }

    fn translate(&mut self, by: (i32, i32)) {
        self.position.0 += by.0;
        self.position.1 += by.1;
    }
}

impl Clone for Box<dyn Drawable> {
    fn clone(&self) -> Self {
        (*self).clone()
    }
}

impl Clone for Object {
    fn clone(&self) -> Self {
        Object {
            primitives : RefCell::new(
                self.primitives
                    .borrow()
                    .iter()
                    .map(|primitive| {
                        (*primitive).clone()
                    })
                    .collect(),
            ),
            children : RefCell::new(
                self.children
                    .borrow()
                    .iter()
                    .map(|child| {
                        Rc::clone(child)
                    })
                    .collect(),
            ),
            position : self.position,
            z : self.z,
        }
    }
}


impl Object {
    pub fn new(position : (i32, i32), z : i32 ) -> Rc<Object> {
        let mut object = Rc::new(Object {
            primitives : RefCell::new(Vec::new()),
            children : RefCell::new(Vec::new()),
            position,
            z,
        });
        object.children.borrow_mut().push(object.clone());
        return object;
    }
    pub fn add_primitive(&mut self, primitive : Box<dyn Drawable>) {
        self.primitives.borrow_mut().push(primitive);
    }
    pub fn add_child(&mut self, child: Rc<Object>) {
        let mut index = -1;
        for current_child in self.children.borrow_mut().iter() {
            index += 1;
            if (current_child.z > child.z ) {

            }
        }
        if (index == -1) {
            self.children.get_mut().push(child);
        }
    }
}