use core::marker::PhantomData;

use crate::gpio::ports::{Port, PortB, PortC, PortD};

macro_rules! make_pin {
    ($name:ident: $port:ident = $pin:literal) => {
        pub struct $name(PhantomData<*const ()>);

        impl $name {
            pub(super) const fn new() -> Self {
                Self(PhantomData)
            }

            pub const fn erase_pin(self) -> $port {
                $port { mask: 1 << $pin }
            }
        }
    };
}

macro_rules! make_erased_pin {
    ($name:ident: $port:ident) => {
        pub struct $name {
            pub mask: u8,
        }

        impl $name {
            pub const fn erase_port(self) -> Pin<$port> {
                Pin {
                    port: $port::new(),
                    mask: self.mask,
                }
            }
        }
    };
}

make_pin!(PD0: PD = 0); // 0
make_pin!(PD1: PD = 1); // 1
make_pin!(PD2: PD = 2); // 2
make_pin!(PD3: PD = 3); // 3
make_pin!(PD4: PD = 4); // 4
make_pin!(PD5: PD = 5); // 5
make_pin!(PD6: PD = 6); // 6
make_pin!(PD7: PD = 7); // 7

make_pin!(PB0: PB = 0); // 8
make_pin!(PB1: PB = 1); // 9
make_pin!(PB2: PB = 2); // 10
make_pin!(PB3: PB = 3); // 11
make_pin!(PB4: PB = 4); // 12
make_pin!(PB5: PB = 5); // 13

make_pin!(PC0: PC = 0); // 14, A0
make_pin!(PC1: PC = 1); // 15, A1
make_pin!(PC2: PC = 2); // 16, A2
make_pin!(PC3: PC = 3); // 17, A3
make_pin!(PC4: PC = 4); // 18, A4
make_pin!(PC5: PC = 5); // 19, A5

make_erased_pin!(PD: PortD);
make_erased_pin!(PB: PortB);
make_erased_pin!(PC: PortC);

pub struct Pin<P: Port> {
    pub port: P,
    pub mask: u8,
}
