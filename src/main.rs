#![no_main]
#![no_std]

extern crate panic_itm;

mod framebuffer;
mod pattern;
mod rainbow_pattern;
mod flame_pattern;

use nb::block;
use stm32f1xx_hal::{
    prelude::*,
    pac,
    timer::Timer,
};
use cortex_m_rt::entry;
use framebuffer::{ Framebuffer, FRAMEBUFFER_SIZE };
use pattern::Pattern;
use rainbow_pattern::RainbowPattern;
use flame_pattern::FlamePattern;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.mhz()).sysclk(24.mhz()).freeze(&mut flash.acr);
    let mut timer = Timer::syst(cp.SYST, 30.hz(), clocks);

    let mut framebuffer = Framebuffer::new(&mut rcc.apb2, dp.GPIOA);
    let mut frame = 0;
    let mut rainbow_pattern = RainbowPattern::new();
    let mut flame_pattern = FlamePattern::new();

    loop {
        rainbow_pattern.update();

        for i in 0..FRAMEBUFFER_SIZE {
            let x = i / 16;
            let y = if x % 2 == 0 { i % 16 } else { 15 - (i % 16) };
            let (r, g, b) = rainbow_pattern.get_pixel(x, y, frame);
            framebuffer.set_pixel(i, r, g, b);
        }

        frame = frame + 1;
        framebuffer.dump();

        block!(timer.wait()).unwrap();
    }
}
