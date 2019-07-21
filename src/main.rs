#![no_main]
#![no_std]

extern crate panic_itm;

use stm32f1xx_hal::{
    prelude::*,
    pac,
    stm32::{ RCC, TIM2, tim2 }
};
use cortex_m_rt::entry;

fn delay(timer: &tim2::RegisterBlock, ms: u16) {
    timer.arr.write(|w| w.arr().bits(ms));
    timer.cr1.write(|w| w.cen().set_bit());
    while !timer.sr.read().uif().bit_is_set() {}
    timer.sr.write(|w| w.uif().clear_bit());
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();

    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let rcc = unsafe { &*RCC::ptr() };
    let tim2 = unsafe { &*TIM2::ptr() };

    rcc.apb1enr.write(|w| w.tim2en().set_bit());
    tim2.cr1.write(|w| w.opm().set_bit().cen().clear_bit());
    tim2.psc.write(|w| unsafe { w.psc().bits(7_199) });

    loop {
        led.set_high();
        delay(tim2, 1000);
        led.set_low();
        delay(tim2, 1000);
    }
}
