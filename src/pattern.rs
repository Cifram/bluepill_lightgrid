pub trait Pattern {
    fn update(&mut self);
    fn get_pixel(&self, x: usize, y: usize, frame: u32) -> (u8, u8, u8);
}
