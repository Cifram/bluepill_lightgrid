#![no_main]
#![no_std]

extern crate panic_itm;

use stm32f1xx_hal::{
    prelude::*,
    pac,
};
use core::ptr;
use cortex_m_rt::entry;

const GPIOC_ODR: u32 = 0x4001100C;

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

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let _ = rcc.cfgr.use_hse(8.mhz()).sysclk(24.mhz()).freeze(&mut flash.acr);

    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);
    let _ = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    loop {
        for _ in 0..144 {
            let mut byte1: u8 = 16;
            let mut byte2: u8 = 0;
            let mut byte3: u8 = 0;
            ws2812_byte!(byte1);
            ws2812_byte!(byte2);
            ws2812_byte!(byte3);
        }

        for _ in 0..200 {
            cortex_m::asm::nop();
        }
    }
}
