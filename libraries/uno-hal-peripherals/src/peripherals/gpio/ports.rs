use crate::{
    addr::{RegRO, RegRW},
    peripherals::gpio::pins::*,
};

pub trait Port {
    type Pins;

    fn split(&self) -> Self::Pins;

    fn read_reg(&self) -> &RegRO<u8>;
    fn mode_reg(&mut self) -> &mut RegRW<u8>;
    fn write_reg(&mut self) -> &mut RegRW<u8>;
}

pub struct PortDPins {
    pub pd0: PD0,
    pub pd1: PD1,
    pub pd2: PD2,
    pub pd3: PD3,
    pub pd4: PD4,
    pub pd5: PD5,
    pub pd6: PD6,
    pub pd7: PD7,
}

pub struct PortBPins {
    pub pb0: PB0,
    pub pb1: PB1,
    pub pb2: PB2,
    pub pb3: PB3,
    pub pb4: PB4,
    pub pb5: PB5,
}

pub struct PortCPins {
    pub pc0: PC0,
    pub pc1: PC1,
    pub pc2: PC2,
    pub pc3: PC3,
    pub pc4: PC4,
    pub pc5: PC5,
}

pub struct PortD {
    pind: RegRO<u8>,
    ddrd: RegRW<u8>,
    portd: RegRW<u8>,
}

pub struct PortB {
    pinb: RegRO<u8>,
    ddrb: RegRW<u8>,
    portb: RegRW<u8>,
}

pub struct PortC {
    pinc: RegRO<u8>,
    ddrc: RegRW<u8>,
    portc: RegRW<u8>,
}

impl PortD {
    pub(crate) const fn new() -> Self {
        Self {
            pind: RegRO::new_io8::<0x09>(),
            ddrd: RegRW::new_io8::<0x0A>(),
            portd: RegRW::new_io8::<0x0B>(),
        }
    }

    #[inline]
    pub unsafe fn take() -> Self {
        Self::new()
    }
}

impl PortB {
    pub(crate) const fn new() -> Self {
        Self {
            pinb: RegRO::new_io8::<0x03>(),
            ddrb: RegRW::new_io8::<0x04>(),
            portb: RegRW::new_io8::<0x05>(),
        }
    }

    #[inline]
    pub unsafe fn take() -> Self {
        Self::new()
    }
}

impl PortC {
    pub(crate) const fn new() -> Self {
        Self {
            pinc: RegRO::new_io8::<0x06>(),
            ddrc: RegRW::new_io8::<0x07>(),
            portc: RegRW::new_io8::<0x08>(),
        }
    }

    #[inline]
    pub unsafe fn take() -> Self {
        Self::new()
    }
}

impl Port for PortD {
    type Pins = PortDPins;

    #[inline]
    fn split(&self) -> Self::Pins {
        Self::Pins {
            pd0: PD0::new(),
            pd1: PD1::new(),
            pd2: PD2::new(),
            pd3: PD3::new(),
            pd4: PD4::new(),
            pd5: PD5::new(),
            pd6: PD6::new(),
            pd7: PD7::new(),
        }
    }

    #[inline]
    fn read_reg(&self) -> &RegRO<u8> {
        &self.pind
    }

    #[inline]
    fn mode_reg(&mut self) -> &mut RegRW<u8> {
        &mut self.ddrd
    }

    #[inline]
    fn write_reg(&mut self) -> &mut RegRW<u8> {
        &mut self.portd
    }
}

impl Port for PortB {
    type Pins = PortBPins;

    #[inline]
    fn split(&self) -> Self::Pins {
        Self::Pins {
            pb0: PB0::new(),
            pb1: PB1::new(),
            pb2: PB2::new(),
            pb3: PB3::new(),
            pb4: PB4::new(),
            pb5: PB5::new(),
        }
    }

    #[inline]
    fn read_reg(&self) -> &RegRO<u8> {
        &self.pinb
    }

    #[inline]
    fn mode_reg(&mut self) -> &mut RegRW<u8> {
        &mut self.ddrb
    }

    #[inline]
    fn write_reg(&mut self) -> &mut RegRW<u8> {
        &mut self.portb
    }
}

impl Port for PortC {
    type Pins = PortCPins;

    #[inline]
    fn split(&self) -> Self::Pins {
        Self::Pins {
            pc0: PC0::new(),
            pc1: PC1::new(),
            pc2: PC2::new(),
            pc3: PC3::new(),
            pc4: PC4::new(),
            pc5: PC5::new(),
        }
    }

    #[inline]
    fn read_reg(&self) -> &RegRO<u8> {
        &self.pinc
    }

    #[inline]
    fn mode_reg(&mut self) -> &mut RegRW<u8> {
        &mut self.ddrc
    }

    #[inline]
    fn write_reg(&mut self) -> &mut RegRW<u8> {
        &mut self.portc
    }
}
