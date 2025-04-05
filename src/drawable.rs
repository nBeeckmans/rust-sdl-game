use sdl3::render::WindowCanvas;

pub trait Drawable {
    fn draw(&self,window_canvas: &mut WindowCanvas);
}
