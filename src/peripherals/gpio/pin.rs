use core::marker::PhantomData;

use uno_hal_peripherals::analog::Analog;

use crate::peripherals::gpio::{
    generic::{GenericPin, GenericPinAnalog, GenericPinPWM},
    AnalogMode, Floating, IOMode, InputMode, InputState, OutputMode, PinMode, PullUp, PwmMode,
};

pub struct Pin<M: PinMode, P: GenericPin> {
    pub(super) pin: P,
    pub(super) _state: PhantomData<M>,
}

impl<M: IOMode, P: GenericPin> Pin<M, P> {
    #[inline]
    pub fn into_floating_input(mut self) -> Pin<InputMode<Floating>, P> {
        unsafe { self.pin.to_input(false) };

        Pin {
            pin: self.pin,
            _state: PhantomData,
        }
    }

    #[inline]
    pub fn into_pullup_input(mut self) -> Pin<InputMode<PullUp>, P> {
        unsafe { self.pin.to_input(true) };

        Pin {
            pin: self.pin,
            _state: PhantomData,
        }
    }

    #[inline]
    pub fn into_output(mut self) -> Pin<OutputMode, P> {
        unsafe { self.pin.to_output() };

        Pin {
            pin: self.pin,
            _state: PhantomData,
        }
    }
}

impl<S: InputState, P: GenericPin> Pin<InputMode<S>, P> {
    #[inline]
    pub fn is_high(&self) -> bool {
        self.pin.input_get()
    }

    #[inline]
    pub fn is_low(&self) -> bool {
        !self.pin.input_get()
    }
}

impl<P: GenericPin> Pin<OutputMode, P> {
    #[inline]
    pub fn set_high(&mut self) {
        unsafe { self.pin.output_set() };
    }

    #[inline]
    pub fn set_low(&mut self) {
        unsafe { self.pin.output_clear() };
    }
}

impl<M: IOMode, P: GenericPinPWM> Pin<M, P> {
    #[inline]
    pub fn into_pwm(mut self) -> Pin<PwmMode, P> {
        unsafe { self.pin.to_output() };

        Pin {
            pin: self.pin,
            _state: PhantomData,
        }
    }
}

impl<P: GenericPinPWM> Pin<PwmMode, P> {
    pub fn set(&mut self, value: u8) {
        match value {
            u8::MIN => {
                self.pin.disable_pwm();
                unsafe { self.pin.output_clear() };
            }
            u8::MAX => {
                self.pin.disable_pwm();
                unsafe { self.pin.output_set() };
            }
            x => {
                self.pin.enable_pwm();
                unsafe { self.pin.output_pwm(x) };
            }
        }
    }
}

impl<M: IOMode, P: GenericPinAnalog> Pin<M, P> {
    #[inline]
    pub fn into_analog(mut self) -> Pin<AnalogMode, P> {
        unsafe { self.pin.to_input(false) };

        Pin {
            pin: self.pin,
            _state: PhantomData,
        }
    }
}

impl<P: GenericPinAnalog> Pin<AnalogMode, P> {
    #[inline]
    pub fn read(&mut self, analog: &mut Analog) -> u16 {
        self.pin.input_analog(analog)
    }
}
