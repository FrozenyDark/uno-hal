use uno_hal_peripherals::{
    addr::RW,
    timers::{Timer0, Timer1, Timer2},
};

pub trait TimerPWM {
    unsafe fn set_pwm_a(&mut self);
    unsafe fn clear_pwm_a(&mut self);

    unsafe fn set_pwm_b(&mut self);
    unsafe fn clear_pwm_b(&mut self);

    unsafe fn set_value_a(&mut self, value: u8);
    unsafe fn set_value_b(&mut self, value: u8);
}

macro_rules! impl_pwm {
    ($name:ident($tccr:ident, $com_a:ident, $com_b:ident, $ocr_a:ident, $ocr_b: ident): $type:ty) => {
        impl TimerPWM for $name {
            #[inline]
            unsafe fn set_pwm_a(&mut self) {
                self.$tccr.$com_a.set();
            }

            #[inline]
            unsafe fn clear_pwm_a(&mut self) {
                self.$tccr.$com_a.clear();
            }

            #[inline]
            unsafe fn set_pwm_b(&mut self) {
                self.$tccr.$com_b.set();
            }

            #[inline]
            unsafe fn clear_pwm_b(&mut self) {
                self.$tccr.$com_b.clear();
            }

            #[inline]
            unsafe fn set_value_a(&mut self, value: u8) {
                self.$ocr_a.reg_mut().write(value as $type);
            }

            #[inline]
            unsafe fn set_value_b(&mut self, value: u8) {
                self.$ocr_a.reg_mut().write(value as $type);
            }
        }
    };
}

impl_pwm!(Timer0(tccr0a, com0a1, com0b1, ocr0a, ocr0b): u8);
impl_pwm!(Timer1(tccr1a, com1a1, com1b1, ocr1a, ocr1b): u16);
impl_pwm!(Timer2(tccr2a, com2a1, com2b1, ocr2a, ocr2b): u8);
