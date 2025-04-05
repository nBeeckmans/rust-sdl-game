pub trait Transformable {
    fn rotate(&self, by : f32);
    fn translate(&self, by : (i32,i32));
}