use crate::peripherals::gpio::generic::GenericPinAnalog;
use uno_hal_peripherals::{analog::Analog, gpio::pins::*};

macro_rules! init_analog {
    ($name:ident, $pin:literal) => {
        impl GenericPinAnalog for $name {
            #[inline]
            fn input_analog(&mut self, analog: &mut Analog) -> u16 {
                analog.select_pin($pin);
                analog.read();

                analog.get_result()
            }
        }
    };
}

init_analog!(PC0, 0);
init_analog!(PC1, 1);
init_analog!(PC2, 2);
init_analog!(PC3, 3);
init_analog!(PC4, 4);
init_analog!(PC5, 5);
