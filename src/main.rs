#![no_main]
#![no_std]

extern crate panic_itm;

mod framebuffer;

use stm32f1xx_hal::{
    prelude::*,
    pac,
};
use cortex_m_rt::entry;
use framebuffer::{ Framebuffer, FRAMEBUFFER_SIZE };

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let _ = rcc.cfgr.use_hse(8.mhz()).sysclk(24.mhz()).freeze(&mut flash.acr);

    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);
    let _ = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let mut framebuffer = Framebuffer::new();
    let mut chase_pos = 0;

    loop {
        for i in (0..FRAMEBUFFER_SIZE).step_by(3) {
            if i % 64 == chase_pos {
                framebuffer.buffer[i] = 16;
                framebuffer.buffer[i+1] = 0;
                framebuffer.buffer[i+2] = 0;
            } else {
                framebuffer.buffer[i] = 0;
                framebuffer.buffer[i+1] = 0;
                framebuffer.buffer[i+2] = 0;
            }
        }

        chase_pos = (chase_pos + 1) % 64;
        framebuffer.dump();
    }
}
