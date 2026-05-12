use core::marker::PhantomData;

use crate::peripherals::gpio::{Floating, Input, InputState, Output, PinState, PullUp};

pub trait GenericPin {
    unsafe fn to_output(&mut self);
    unsafe fn to_input(&mut self, pullup: bool);

    unsafe fn in_get(&self) -> bool;

    unsafe fn set(&mut self);
    unsafe fn clear(&mut self);
    unsafe fn toggle(&mut self);
    unsafe fn out_get(&self) -> bool;
}

pub struct Pin<S: PinState, P: GenericPin> {
    pin: P,
    _marker: PhantomData<S>,
}

impl<S: PinState, P: GenericPin> Pin<S, P> {
    pub fn into_input(mut self) -> Pin<Input<Floating>, P> {
        unsafe { self.pin.to_input(false) };

        Pin {
            pin: self.pin,
            _marker: PhantomData::<Input<Floating>>,
        }
    }

    pub fn into_pullup_input(mut self) -> Pin<Input<PullUp>, P> {
        unsafe { self.pin.to_input(true) };

        Pin {
            pin: self.pin,
            _marker: PhantomData::<Input<PullUp>>,
        }
    }

    pub fn into_output(mut self) -> Pin<Output, P> {
        unsafe { self.pin.to_output() }

        Pin {
            pin: self.pin,
            _marker: PhantomData::<Output>,
        }
    }
}

impl<T: InputState, P: GenericPin> Pin<Input<T>, P> {
    pub fn is_high(&self) -> bool {
        unsafe { self.pin.in_get() }
    }

    pub fn is_low(&self) -> bool {
        unsafe { !self.pin.in_get() }
    }
}

impl<P: GenericPin> Pin<Output, P> {
    pub fn set_high(&mut self) {
        unsafe { self.pin.set() };
    }

    pub fn set_low(&mut self) {
        unsafe { self.pin.clear() };
    }

    pub fn toggle(&mut self) {
        unsafe { self.pin.toggle() };
    }

    pub fn is_high(&self) -> bool {
        unsafe { self.pin.out_get() }
    }

    pub fn is_low(&self) -> bool {
        unsafe { !self.pin.out_get() }
    }
}
