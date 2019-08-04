use stm32f1xx_hal::{
    prelude::*,
    rcc::APB2,
    stm32::{
        GPIOB,
        gpioa::RegisterBlock,
    },
};

pub const FRAMEBUFFER_SIZE: usize = 768;

const PIN1_START: usize = (FRAMEBUFFER_SIZE*3)/3;
const PIN2_START: usize = PIN1_START*2;

const PIN1: u32 = 13;
const PIN2: u32 = 14;
const PIN3: u32 = 15;
const PIN1_OFFSET: i32 = (PIN1 as i32) - 7;
const PIN2_OFFSET: i32 = (PIN2 as i32) - 7;
const PIN3_OFFSET: i32 = (PIN3 as i32) - 7;
const ALL_PINS_ON: u32 = (1 << PIN1) + (1 << PIN2) + (1 << PIN3);

pub struct Framebuffer {
    pub buffer: [u8; FRAMEBUFFER_SIZE*3],
    registers: &'static RegisterBlock,
}

impl Framebuffer {
    pub fn new(apb2: &mut APB2, gpiob: GPIOB) -> Framebuffer {
        let mut gpiob = gpiob.split(apb2);
        let _ = gpiob.pb0.into_push_pull_output(&mut gpiob.crl);
        let _ = gpiob.pb1.into_push_pull_output(&mut gpiob.crl);
        let _ = gpiob.pb2.into_push_pull_output(&mut gpiob.crl);
        let _ = gpiob.pb3.into_push_pull_output(&mut gpiob.crl);
        let _ = gpiob.pb4.into_push_pull_output(&mut gpiob.crl);
        let _ = gpiob.pb5.into_push_pull_output(&mut gpiob.crl);
        let _ = gpiob.pb6.into_push_pull_output(&mut gpiob.crl);
        let _ = gpiob.pb7.into_push_pull_output(&mut gpiob.crl);
        let _ = gpiob.pb8.into_push_pull_output(&mut gpiob.crh);
        let _ = gpiob.pb9.into_push_pull_output(&mut gpiob.crh);
        let _ = gpiob.pb10.into_push_pull_output(&mut gpiob.crh);
        let _ = gpiob.pb11.into_push_pull_output(&mut gpiob.crh);
        let _ = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);
        let _ = gpiob.pb13.into_push_pull_output(&mut gpiob.crh);
        let _ = gpiob.pb14.into_push_pull_output(&mut gpiob.crh);
        let _ = gpiob.pb15.into_push_pull_output(&mut gpiob.crh);

        Framebuffer {
            buffer: [0; FRAMEBUFFER_SIZE*3],
            registers: unsafe { &*GPIOB::ptr() },
        }
    }

    pub fn set_pixel(&mut self, index: usize, r: u8, g: u8, b: u8) {
        self.buffer[index*3] = g;
        self.buffer[index*3+1] = r;
        self.buffer[index*3+2] = b;
    }

    pub fn dump(&self) {
        for i in 0..PIN1_START {
            let mut byte1 = self.buffer[i];
            let mut byte2 = self.buffer[i + PIN1_START];
            let mut byte3 = self.buffer[i + PIN2_START];
            for _ in 0..8 {
                let bits =
                    (((byte1 as u32) & 0b1000_0000) << PIN1_OFFSET) +
                    (((byte2 as u32) & 0b1000_0000) << PIN2_OFFSET) +
                    (((byte3 as u32) & 0b1000_0000) << PIN3_OFFSET);
                self.registers.odr.write(|w| unsafe { w.bits(ALL_PINS_ON) });
                cortex_m::asm::nop();
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
