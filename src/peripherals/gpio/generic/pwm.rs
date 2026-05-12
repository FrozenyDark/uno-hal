use crate::peripherals::{gpio::generic::GenericPinPWM, timers::pwm::TimerPWM};
use uno_hal_peripherals::{
    gpio::pins::*,
    timers::{Timer0, Timer1, Timer2},
};

macro_rules! init_pwm_on_off {
    ($timer:ident:a) => {
        #[inline]
        fn enable_pwm(&mut self) {
            unsafe { $timer::take().set_pwm_a() };
        }

        #[inline]
        fn disable_pwm(&mut self) {
            unsafe { $timer::take().clear_pwm_a() };
        }
    };

    ($timer:ident:b) => {
        #[inline]
        fn enable_pwm(&mut self) {
            unsafe { $timer::take().set_pwm_b() };
        }

        #[inline]
        fn disable_pwm(&mut self) {
            unsafe { $timer::take().clear_pwm_b() };
        }
    };
}

macro_rules! init_pwm {
    ($name:ident, $timer:ident:a) => {
        impl GenericPinPWM for $name {
            init_pwm_on_off!($timer:a);

            #[inline]
            unsafe fn output_pwm(&mut self, value: u8) {
                $timer::take().set_value_a(value);
            }
        }
    };

    ($name:ident, $timer:ident:b) => {
        impl GenericPinPWM for $name {
            init_pwm_on_off!($timer:b);

            #[inline]
            unsafe fn output_pwm(&mut self, value: u8) {
                $timer::take().set_value_b(value);
            }
        }
    };
}

init_pwm!(PD3, Timer2:b);
init_pwm!(PD5, Timer0:b);
init_pwm!(PD6, Timer0:a);

init_pwm!(PB1, Timer1:a);
init_pwm!(PB2, Timer1:b);
init_pwm!(PB3, Timer2:a);
