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

    let mut framebuffer = Framebuffer::new(&mut rcc.apb2, dp.GPIOC);
    let mut chase_pos = 0;

    loop {
        for i in 0..FRAMEBUFFER_SIZE {
            let color = ((i + chase_pos/2) % 48) as u8;
            framebuffer.set_pixel(i,
                if color < 16 { color } else if color >= 32 { 0 } else { 32-color },
                if color < 16 { 0 } else if color >= 32 { 48-color } else { color-16 },
                if color < 16 { 16-color } else if color >= 32 { color-32 } else { 0 },
            );
        }

        chase_pos = (chase_pos + 1) % 96;
        framebuffer.dump();
    }
}
