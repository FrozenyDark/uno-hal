use uno_hal_peripherals::analog::Analog;

mod analog;
mod digital;
mod pwm;

pub trait GenericPin {
    unsafe fn to_input(&mut self, pullup: bool);
    unsafe fn to_output(&mut self);

    fn input_get(&self) -> bool;

    unsafe fn output_set(&mut self);
    unsafe fn output_clear(&mut self);
}

pub trait GenericPinPWM: GenericPin {
    fn enable_pwm(&mut self);
    fn disable_pwm(&mut self);

    unsafe fn output_pwm(&mut self, value: u8);
}

pub trait GenericPinAnalog: GenericPin {
    fn input_analog(&mut self, analog: &mut Analog) -> u16;
}
