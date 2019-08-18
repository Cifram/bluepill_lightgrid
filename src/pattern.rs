use crate::framebuffer::{Framebuffer};

pub trait Pattern {
    fn render(&mut self, buffer: &mut Framebuffer, frame: u32);
}

pub trait StatelessPattern {
    fn get_pixel(x: usize, y: usize, frame: u32) -> (u8, u8, u8);
}

impl<T: StatelessPattern> Pattern for T {
    fn render(&mut self, buffer: &mut Framebuffer, frame: u32) {
        for i in 0..Framebuffer::size() {
            let height = Framebuffer::height();
            let x = i / height;
            let y = if x % 2 == 0 { i % height } else { (height-1) - (i % height) };
            let (r, g, b) = T::get_pixel(x, y, frame);
            buffer.set_pixel(i, r, g, b);
        }
    }
}