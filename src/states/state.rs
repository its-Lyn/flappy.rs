pub trait State {
    fn init(&mut self);
    fn update(&mut self);
    fn draw(&self);
}