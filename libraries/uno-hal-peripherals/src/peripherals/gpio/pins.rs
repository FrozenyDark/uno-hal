use core::marker::PhantomData;

use crate::gpio::ports::{ErasedPort, PortB, PortC, PortD};

macro_rules! make_pin {
    ($name:ident: $port:ident = $pin:literal) => {
        pub struct $name(PhantomData<*const ()>);

        impl $name {
            pub(super) const fn new() -> Self {
                Self(PhantomData)
            }

            pub const fn erase(self) -> ErasedPin {
                ErasedPin {
                    port: $port::new().erase(),
                    bit: $pin,
                }
            }
        }
    };
}

make_pin!(PD0: PortD = 0); // 0
make_pin!(PD1: PortD = 1); // 1
make_pin!(PD2: PortD = 2); // 2
make_pin!(PD3: PortD = 3); // 3
make_pin!(PD4: PortD = 4); // 4
make_pin!(PD5: PortD = 5); // 5
make_pin!(PD6: PortD = 6); // 6
make_pin!(PD7: PortD = 7); // 7

make_pin!(PB0: PortB = 0); // 8
make_pin!(PB1: PortB = 1); // 9
make_pin!(PB2: PortB = 2); // 10
make_pin!(PB3: PortB = 3); // 11
make_pin!(PB4: PortB = 4); // 12
make_pin!(PB5: PortB = 5); // 13

make_pin!(PC0: PortC = 0); // 14, A0
make_pin!(PC1: PortC = 1); // 15, A1
make_pin!(PC2: PortC = 2); // 16, A2
make_pin!(PC3: PortC = 3); // 17, A3
make_pin!(PC4: PortC = 4); // 18, A4
make_pin!(PC5: PortC = 5); // 19, A5

pub struct ErasedPin {
    pub port: ErasedPort,
    pub bit: u8,
}
