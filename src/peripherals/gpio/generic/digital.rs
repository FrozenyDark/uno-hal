use crate::peripherals::gpio::generic::GenericPin;
use uno_hal_peripherals::{
    atomic_block,
    gpio::{
        pins::*,
        ports::{PortB, PortC, PortD},
    },
    register::{BitRO, BitRW},
};

macro_rules! init_digital {
    ($name:ident, $port:ident($read:ident, $mode:ident, $write:ident), $bit:literal) => {
        impl GenericPin for $name {
            #[inline]
            unsafe fn to_input(&mut self, pullup: bool) {
                let mut port = unsafe { $port::take() };

                atomic_block! {
                    port.$mode.clear_bit($bit);
                    match pullup {
                        true => port.$write.set_bit($bit),
                        false => port.$write.clear_bit($bit),
                    }
                };
            }

            #[inline]
            unsafe fn to_output(&mut self) {
                atomic_block! { $port::take().$mode.set_bit($bit) };
            }

            #[inline]
            fn input_get(&self) -> bool {
                unsafe { $port::take().$read.is_set_bit($bit) }
            }

            #[inline]
            unsafe fn output_set(&mut self) {
                atomic_block! { $port::take().$write.set_bit($bit) };
            }

            #[inline]
            unsafe fn output_clear(&mut self) {
                atomic_block! { $port::take().$write.clear_bit($bit) };
            }

            #[inline]
            fn erase(self) -> ErasedPin {
                self.erase()
            }
        }
    };
}

init_digital!(PD0, PortD(pind, ddrd, portd), 0); // Digital Pin 0
init_digital!(PD1, PortD(pind, ddrd, portd), 1); // Digital Pin 1
init_digital!(PD2, PortD(pind, ddrd, portd), 2); // Digital Pin 2
init_digital!(PD3, PortD(pind, ddrd, portd), 3); // Digital Pin 3, PWM
init_digital!(PD4, PortD(pind, ddrd, portd), 4); // Digital Pin 4
init_digital!(PD5, PortD(pind, ddrd, portd), 5); // Digital Pin 5, PWM
init_digital!(PD6, PortD(pind, ddrd, portd), 6); // Digital Pin 6, PWM
init_digital!(PD7, PortD(pind, ddrd, portd), 7); // Digital Pin 7

init_digital!(PB0, PortB(pinb, ddrb, portb), 0); // Digital Pin 8
init_digital!(PB1, PortB(pinb, ddrb, portb), 1); // Digital Pin 9, PWM
init_digital!(PB2, PortB(pinb, ddrb, portb), 2); // Digital Pin 10, PWM
init_digital!(PB3, PortB(pinb, ddrb, portb), 3); // Digital Pin 11, PWM
init_digital!(PB4, PortB(pinb, ddrb, portb), 4); // Digital Pin 12
init_digital!(PB5, PortB(pinb, ddrb, portb), 5); // Digital Pin 13

init_digital!(PC0, PortC(pinc, ddrc, portc), 0); // Digital Pin 14, Analog Pin 0
init_digital!(PC1, PortC(pinc, ddrc, portc), 1); // Digital Pin 15, Analog Pin 1
init_digital!(PC2, PortC(pinc, ddrc, portc), 2); // Digital Pin 16, Analog Pin 2
init_digital!(PC3, PortC(pinc, ddrc, portc), 3); // Digital Pin 17, Analog Pin 3
init_digital!(PC4, PortC(pinc, ddrc, portc), 4); // Digital Pin 18, Analog Pin 4
init_digital!(PC5, PortC(pinc, ddrc, portc), 5); // Digital Pin 19, Analog Pin 5

impl GenericPin for ErasedPin {
    #[inline]
    unsafe fn to_input(&mut self, pullup: bool) {
        atomic_block! {
            self.port.ddr.clear(self.mask);
            match pullup {
                true => self.port.port.set(self.mask),
                false => self.port.port.clear(self.mask),
            }
        };
    }

    #[inline]
    unsafe fn to_output(&mut self) {
        atomic_block! { self.port.ddr.set(self.mask) };
    }

    #[inline]
    fn input_get(&self) -> bool {
        self.port.pin.is_set(self.mask)
    }

    #[inline]
    unsafe fn output_set(&mut self) {
        atomic_block! { self.port.port.set(self.mask) };
    }

    #[inline]
    unsafe fn output_clear(&mut self) {
        atomic_block! { self.port.port.clear(self.mask) };
    }

    #[inline]
    fn erase(self) -> ErasedPin {
        self
    }
}
