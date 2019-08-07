use crate::pattern::Pattern;

pub struct FlamePattern {}

impl FlamePattern {
    pub fn new() -> FlamePattern {
        FlamePattern {}
    }
}

const FRAME_COUNT: usize = 7;
const ANIMATION: [[(u8, u8, u8); 16]; FRAME_COUNT] = [
    [ (16, 12, 0), (16, 8, 0), (16, 4, 0), (16, 2, 0), (16, 1, 0), (10, 0, 0), (4, 0, 0), (2, 0, 0), (1, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0) ],
    [ (16, 14, 0), (16, 12, 0), (16, 8, 0), (16, 4, 0), (16, 2, 0), (16, 1, 0), (10, 0, 0), (4, 0, 0), (2, 0, 0), (1, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0) ],
    [ (16, 14, 0), (16, 14, 0), (16, 12, 0), (16, 8, 0), (16, 4, 0), (16, 2, 0), (16, 1, 0), (10, 0, 0), (4, 0, 0), (2, 0, 0), (1, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0)],
    [ (16, 14, 0), (16, 14, 0), (16, 14, 0), (16, 12, 0), (16, 8, 0), (16, 4, 0), (16, 2, 0), (16, 1, 0), (10, 0, 0), (4, 0, 0), (2, 0, 0), (1, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0)],
    [ (16, 14, 0), (16, 14, 0), (16, 12, 0), (16, 8, 0), (16, 4, 0), (16, 2, 0), (16, 1, 0), (10, 0, 0), (4, 0, 0), (2, 0, 0), (1, 0, 0), (0, 0, 0), (1, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0)],
    [ (16, 14, 0), (16, 12, 0), (16, 8, 0), (16, 4, 0), (16, 2, 0), (16, 1, 0), (10, 0, 0), (4, 0, 0), (2, 0, 0), (1, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0), (1, 0, 0), (0, 0, 0), (0, 0, 0) ],
    [ (16, 12, 0), (16, 8, 0), (16, 4, 0), (16, 2, 0), (16, 1, 0), (10, 0, 0), (4, 0, 0), (2, 0, 0), (1, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0), (1, 0, 0), (0, 0, 0) ],
];
const RANDOMIZER: [usize; 16] = [ 14, 6, 9, 5, 3, 7, 15, 11, 10, 12, 4, 13, 1, 2, 8, 0 ];

impl Pattern for FlamePattern {
    fn update(&mut self) {}

    fn get_pixel(&self, x: usize, y: usize, frame: u32) -> (u8, u8, u8) {
        ANIMATION[(((frame/2) as usize)+RANDOMIZER[x%16])%FRAME_COUNT][y]
    }
}