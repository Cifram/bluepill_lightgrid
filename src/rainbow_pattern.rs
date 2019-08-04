use crate::pattern::Pattern;

pub struct RainbowPattern {}

impl RainbowPattern {
    pub fn new() -> RainbowPattern {
        RainbowPattern {}
    }
}

impl Pattern for RainbowPattern {
    fn update(&mut self) {}

    fn get_pixel(&self, x: usize, y: usize, frame: u32) -> (u8, u8, u8) {
        let color = (((x+y) as u32 + frame) % 48) as u8;
        (
            if color < 16 { color } else if color >= 32 { 0 } else { 32-color },
            if color < 16 { 0 } else if color >= 32 { 48-color } else { color-16 },
            if color < 16 { 16-color } else if color >= 32 { color-32 } else { 0 },
        )
    }
}
