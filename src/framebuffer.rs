use stm32f1xx_hal::{
    prelude::*,
    rcc::APB2,
    stm32::{
        GPIOA,
        gpioa::RegisterBlock,
    },
};

const PIN1: u32 = 0;
const PIN2: u32 = 1;
const PIN3: u32 = 2;
const ALL_PINS_ON: u32 = (1 << PIN1) + (1 << PIN2) + (1 << PIN3);

pub struct Framebuffer {
    pub buffer: [u8; Framebuffer::size()*3],
    registers: &'static RegisterBlock,
}

impl Framebuffer {
    pub const fn width() -> usize { 48 }
    pub const fn height() -> usize { 16 }
    pub const fn size() -> usize { Framebuffer::width() * Framebuffer::height() }

    const fn pin1_start() -> usize { Framebuffer::size() }
    const fn pin2_start() -> usize { Framebuffer::pin1_start()*2 }

    pub fn new(apb2: &mut APB2, gpioa: GPIOA) -> Framebuffer {
        let mut gpioa = gpioa.split(apb2);
        let _ = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);
        let _ = gpioa.pa1.into_push_pull_output(&mut gpioa.crl);
        let _ = gpioa.pa2.into_push_pull_output(&mut gpioa.crl);
        let _ = gpioa.pa3.into_push_pull_output(&mut gpioa.crl);
        let _ = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);
        let _ = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);
        let _ = gpioa.pa6.into_push_pull_output(&mut gpioa.crl);
        let _ = gpioa.pa7.into_push_pull_output(&mut gpioa.crl);
        let _ = gpioa.pa8.into_push_pull_output(&mut gpioa.crh);
        let _ = gpioa.pa9.into_push_pull_output(&mut gpioa.crh);
        let _ = gpioa.pa10.into_push_pull_output(&mut gpioa.crh);
        let _ = gpioa.pa11.into_push_pull_output(&mut gpioa.crh);
        let _ = gpioa.pa12.into_push_pull_output(&mut gpioa.crh);
        let _ = gpioa.pa13.into_push_pull_output(&mut gpioa.crh);
        let _ = gpioa.pa14.into_push_pull_output(&mut gpioa.crh);
        let _ = gpioa.pa15.into_push_pull_output(&mut gpioa.crh);

        Framebuffer {
            buffer: [0; Framebuffer::size()*3],
            registers: unsafe { &*GPIOA::ptr() },
        }
    }

    pub fn set_pixel(&mut self, index: usize, r: u8, g: u8, b: u8) {
        self.buffer[index*3] = g;
        self.buffer[index*3+1] = r;
        self.buffer[index*3+2] = b;
    }

    pub fn dump(&self) {
        for i in 0..Framebuffer::pin1_start() {
            let mut byte1 = self.buffer[i];
            let mut byte2 = self.buffer[i + Framebuffer::pin1_start()];
            let mut byte3 = self.buffer[i + Framebuffer::pin2_start()];
            for _ in 0..8 {
                let bits =
                    (((byte1 as u32) & 0b1000_0000) >> 7 << PIN1) +
                    (((byte2 as u32) & 0b1000_0000) >> 7 << PIN2) +
                    (((byte3 as u32) & 0b1000_0000) >> 7 << PIN3);
                self.registers.odr.write(|w| unsafe { w.bits(ALL_PINS_ON) });
                self.registers.odr.write(|w| unsafe { w.bits(bits) });
                cortex_m::asm::nop();
                cortex_m::asm::nop();
                self.registers.odr.write(|w| unsafe { w.bits(0) });
                cortex_m::asm::nop();
                cortex_m::asm::nop();
                byte1 = byte1 << 1;
                byte2 = byte2 << 1;
                byte3 = byte3 << 1;
            }
        }
    }
}
