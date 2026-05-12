use core::marker::PhantomData;

use crate::peripherals::gpio::{Floating, InputMode, Pin};
use uno_hal_peripherals::gpio::{
    pins::*,
    ports::{Port, PortB, PortC, PortD},
};

type PinType<P> = Pin<InputMode<Floating>, P>;

macro_rules! build_pin {
    ($port:ident.$pin:ident) => {
        Pin {
            pin: $port.$pin,
            _state: PhantomData,
        }
    };
}

pub struct Pins {
    pub a0: PinType<PC0>,
    pub a1: PinType<PC1>,
    pub a2: PinType<PC2>,
    pub a3: PinType<PC3>,
    pub a4: PinType<PC4>,
    pub a5: PinType<PC5>,

    pub d1: PinType<PD1>,
    pub d0: PinType<PD0>,
    pub d2: PinType<PD2>,
    pub d3: PinType<PD3>,
    pub d4: PinType<PD4>,
    pub d5: PinType<PD5>,
    pub d6: PinType<PD6>,
    pub d7: PinType<PD7>,

    pub d8: PinType<PB0>,
    pub d9: PinType<PB1>,
    pub d10: PinType<PB2>,
    pub d11: PinType<PB3>,
    pub d12: PinType<PB4>,
    pub d13: PinType<PB5>,
}

impl Pins {
    #[inline]
    pub fn new(portd: PortD, portb: PortB, portc: PortC) -> Self {
        let d = portd.split();
        let b = portb.split();
        let c = portc.split();

        Self {
            a0: build_pin!(c.pc0),
            a1: build_pin!(c.pc1),
            a2: build_pin!(c.pc2),
            a3: build_pin!(c.pc3),
            a4: build_pin!(c.pc4),
            a5: build_pin!(c.pc5),
            d0: build_pin!(d.pd0),
            d1: build_pin!(d.pd1),
            d2: build_pin!(d.pd2),
            d3: build_pin!(d.pd3),
            d4: build_pin!(d.pd4),
            d5: build_pin!(d.pd5),
            d6: build_pin!(d.pd6),
            d7: build_pin!(d.pd7),
            d8: build_pin!(b.pb0),
            d9: build_pin!(b.pb1),
            d10: build_pin!(b.pb2),
            d11: build_pin!(b.pb3),
            d12: build_pin!(b.pb4),
            d13: build_pin!(b.pb5),
        }
    }
}

#[macro_export]
macro_rules! make_pins {
    ($p:ident) => {{
        $crate::peripherals::gpio::Pins::new($p.portd, $p.portb, $p.portc)
    }};
}
