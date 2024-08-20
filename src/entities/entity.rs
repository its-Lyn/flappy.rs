pub trait Entity {
    fn init(&mut self);
    fn update(&mut self, paused: bool);
    fn draw(&self);
}