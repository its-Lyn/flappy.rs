pub trait Entity {
    fn init(&mut self);
    fn update(&mut self);
    fn draw(&self);
}