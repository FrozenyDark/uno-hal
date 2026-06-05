use uno_hal_peripherals::{
    register::RW,
    timers::{
        registers::tccr::{Tccr0ABits, Tccr1ABits, Tccr2ABits},
        Timer0, Timer1, Timer2,
    },
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
    ($name:ident($tccr:ident, $bits:ident::($com_a:ident, $com_b:ident), $ocr_a:ident, $ocr_b: ident): $type:ty) => {
        impl TimerPWM for $name {
            #[inline]
            unsafe fn set_pwm_a(&mut self) {
                self.$tccr.set_bit($bits::$com_a);
            }

            #[inline]
            unsafe fn clear_pwm_a(&mut self) {
                self.$tccr.clear_bit($bits::$com_a);
            }

            #[inline]
            unsafe fn set_pwm_b(&mut self) {
                self.$tccr.set_bit($bits::$com_b);
            }

            #[inline]
            unsafe fn clear_pwm_b(&mut self) {
                self.$tccr.clear_bit($bits::$com_b);
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

impl_pwm!(Timer0(tccr0a, Tccr0ABits::(COM0A1, COM0B1), ocr0a, ocr0b): u8);
impl_pwm!(Timer1(tccr1a, Tccr1ABits::(COM1A1, COM1B1), ocr1a, ocr1b): u16);
impl_pwm!(Timer2(tccr2a, Tccr2ABits::(COM2A1, COM2B1), ocr2a, ocr2b): u8);
