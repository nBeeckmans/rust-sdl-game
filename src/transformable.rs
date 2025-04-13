pub trait Transformable {
    fn rotate(&mut self, by : f32);
    fn translate(&mut self, by : (i32,i32));
}