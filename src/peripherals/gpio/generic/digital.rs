use crate::peripherals::gpio::generic::GenericPin;
use uno_hal_peripherals::{
    atomic_block,
    gpio::{
        pins::*,
        ports::{Port, PortB, PortC, PortD},
    },
};

macro_rules! init_digital {
    ($name:ident, $port:ident, $bit:literal) => {
        impl GenericPin for $name {
            #[inline]
            unsafe fn to_input(&mut self, pullup: bool) {
                let mut port = unsafe { $port::take() };

                atomic_block! {
                    port.mode_reg().clear($bit);
                    match pullup {
                        true => port.write_reg().set($bit),
                        false => port.write_reg().clear($bit),
                    }
                };
            }

            #[inline]
            unsafe fn to_output(&mut self) {
                atomic_block! {
                    $port::take().mode_reg().set($bit);
                }
            }

            #[inline]
            fn input_get(&self) -> bool {
                unsafe { $port::take().read_reg().is_set($bit) }
            }

            #[inline]
            unsafe fn output_set(&mut self) {
                atomic_block! {
                    $port::take().write_reg().set($bit);
                }
            }

            #[inline]
            unsafe fn output_clear(&mut self) {
                atomic_block! {
                    $port::take().write_reg().clear($bit);
                }
            }
        }
    };
}

init_digital!(PD0, PortD, 0); // Digital Pin 0
init_digital!(PD1, PortD, 1); // Digital Pin 1
init_digital!(PD2, PortD, 2); // Digital Pin 2
init_digital!(PD3, PortD, 3); // Digital Pin 3, PWM
init_digital!(PD4, PortD, 4); // Digital Pin 4
init_digital!(PD5, PortD, 5); // Digital Pin 5, PWM
init_digital!(PD6, PortD, 6); // Digital Pin 6, PWM
init_digital!(PD7, PortD, 7); // Digital Pin 7

init_digital!(PB0, PortB, 0); // Digital Pin 8
init_digital!(PB1, PortB, 1); // Digital Pin 9, PWM
init_digital!(PB2, PortB, 2); // Digital Pin 10, PWM
init_digital!(PB3, PortB, 3); // Digital Pin 11, PWM
init_digital!(PB4, PortB, 4); // Digital Pin 12
init_digital!(PB5, PortB, 5); // Digital Pin 13

init_digital!(PC0, PortC, 0); // Digital Pin 14, Analog Pin 0
init_digital!(PC1, PortC, 1); // Digital Pin 15, Analog Pin 1
init_digital!(PC2, PortC, 2); // Digital Pin 16, Analog Pin 2
init_digital!(PC3, PortC, 3); // Digital Pin 17, Analog Pin 3
init_digital!(PC4, PortC, 4); // Digital Pin 18, Analog Pin 4
init_digital!(PC5, PortC, 5); // Digital Pin 19, Analog Pin 5
