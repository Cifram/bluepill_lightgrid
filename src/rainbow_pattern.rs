use crate::pattern::Pattern;

pub struct RainbowPattern {}

impl RainbowPattern {
    pub fn new() -> RainbowPattern {
        RainbowPattern {}
    }
}

const DIMMER_TABLE: [[u8; 16]; 16] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  0,  0,  0,  0,  0,  0],
    [0, 0, 0, 0, 0, 0, 0, 0, 1, 1,  1,  1,  1,  1,  1,  1],
    [0, 0, 0, 0, 1, 1, 1, 1, 1, 1,  1,  1,  2,  2,  2,  2],
    [0, 0, 0, 1, 1, 1, 1, 1, 2, 2,  2,  2,  2,  3,  3,  3],
    [0, 0, 1, 1, 1, 1, 2, 2, 2, 2,  3,  3,  3,  3,  4,  4],
    [0, 0, 1, 1, 1, 2, 2, 2, 3, 3,  3,  4,  4,  4,  5,  5],
    [0, 0, 1, 1, 2, 2, 2, 3, 3, 4,  4,  4,  5,  5,  6,  6],
    [0, 0, 1, 1, 2, 2, 3, 3, 4, 4,  5,  5,  6,  6,  7,  7],
    [0, 1, 1, 2, 2, 3, 3, 4, 4, 5,  5,  6,  6,  7,  7,  8],
    [0, 1, 1, 2, 2, 3, 4, 4, 5, 5,  6,  7,  7,  8,  8,  9],
    [0, 1, 1, 2, 3, 3, 4, 5, 5, 6,  7,  7,  8,  9,  9, 10],
    [0, 1, 1, 2, 3, 4, 4, 5, 6, 7,  7,  8,  9, 10, 10, 11],
    [0, 1, 2, 2, 3, 4, 5, 6, 6, 7,  8,  9, 10, 10, 11, 12],
    [0, 1, 2, 3, 3, 4, 5, 6, 7, 8,  9, 10, 10, 11, 12, 13],
    [0, 1, 2, 3, 4, 5, 6, 7, 7, 8,  9, 10, 11, 12, 13, 14],
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
];

impl Pattern for RainbowPattern {
    fn update(&mut self) {}

    fn get_pixel(&self, x: usize, y: usize, frame: u32) -> (u8, u8, u8) {
        let cast_frame = frame as usize;
        let brightness_progression = x-y - cast_frame+100;
        let brightness_gradient = brightness_progression % 16;
        let brightness_segment = brightness_progression / 16;
        let brightness = if brightness_gradient < 8 {
            brightness_gradient*2
        } else {
            (15 - brightness_gradient)*2
        };

        let color_gradient = (x+y + cast_frame + brightness_segment*8) % 48;
        let base_r = if color_gradient < 16 { color_gradient } else if color_gradient >= 32 { 0 } else { 31-color_gradient };
        let r = DIMMER_TABLE[brightness][base_r];
        let base_g = if color_gradient < 16 { 0 } else if color_gradient >= 32 { 47-color_gradient } else { color_gradient-16 };
        let g = DIMMER_TABLE[brightness][base_g];
        let base_b = if color_gradient < 16 { 15-color_gradient } else if color_gradient >= 32 { color_gradient-32 } else { 0 };
        let b = DIMMER_TABLE[brightness][base_b];

        (r, g, b)
    }
}
