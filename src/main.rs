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
use framebuffer::{ Framebuffer };
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

    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
    let mode_button = gpiob.pb10.into_pull_up_input(&mut gpiob.crh);

    let mut framebuffer = Framebuffer::new(&mut rcc.apb2, dp.GPIOA);
    let mut frame = 0;
    let mut cur_pattern = 1;
    let mut rainbow_pattern = RainbowPattern::new();
    let mut flame_pattern = FlamePattern::new();
    let mut mode_pressed_last_frame = false;

    loop {
        if mode_button.is_low() {
            if !mode_pressed_last_frame {
                cur_pattern = (cur_pattern + 1) % 2;
            }
            mode_pressed_last_frame = true;
        } else {
            mode_pressed_last_frame = false;
        }

        match cur_pattern {
            0 => rainbow_pattern.render(&mut framebuffer, frame),
            1 => flame_pattern.render(&mut framebuffer, frame),
            _ => {}, // should be impossible
        }

        frame = frame + 1;
        framebuffer.dump();

        block!(timer.wait()).unwrap();
    }
}
