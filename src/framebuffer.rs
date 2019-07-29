use core::ptr;

const GPIOC_ODR: u32 = 0x4001100C;
pub const FRAMEBUFFER_SIZE: usize = 144*3;

macro_rules! ws2812_byte {
    ($byte:ident) => {
        for _ in 0..8 {
            let bit = (($byte as u32) & 0b1000_0000) >> 7;
            unsafe { ptr::write_volatile(GPIOC_ODR as *mut u32, 1 << 13); }
            cortex_m::asm::nop();
            unsafe { ptr::write_volatile(GPIOC_ODR as *mut u32, bit << 13); }
            cortex_m::asm::nop();
            cortex_m::asm::nop();
            unsafe { ptr::write_volatile(GPIOC_ODR as *mut u32, 0); }
            cortex_m::asm::nop();
            cortex_m::asm::nop();
            $byte = $byte << 1;
        }
    };
}

pub struct Framebuffer {
    pub buffer: [u8; FRAMEBUFFER_SIZE],
}

impl Framebuffer {
    pub fn new() -> Framebuffer {
        Framebuffer { buffer: [0; FRAMEBUFFER_SIZE] }
    }

    pub fn dump(&self) {
        for i in 0..FRAMEBUFFER_SIZE {
            let mut byte = self.buffer[i];
            ws2812_byte!(byte);
        }
    }
}
