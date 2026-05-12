use crate::{
    analog::Analog,
    gpio::ports::{PortB, PortC, PortD},
    timers::{Timer0, Timer1, Timer2},
    usart::Usart0,
};

pub mod addr;
pub mod analog;
pub mod bit;
pub mod gpio;
mod macros;
pub mod status;
pub mod timers;
pub mod usart;

static mut TOOK_PERIPHERALS: bool = false;

pub struct Peripherals {
    pub analog: Analog,
    pub portd: PortD,
    pub portb: PortB,
    pub portc: PortC,
    pub timer0: Timer0,
    pub timer1: Timer1,
    pub timer2: Timer2,
    pub usart0: Usart0,
}

unsafe fn init(p: &mut Peripherals) {
    crate::status::enable_interrupts();

    p.timer0.default_setup();
    p.timer1.default_setup();
    p.timer2.default_setup();

    p.analog
        .setup_prescaler(crate::analog::AnalogPrescaler::DivFactor128);
    p.analog.enable_adc();
}

impl Peripherals {
    pub fn take() -> Option<Self> {
        if unsafe { TOOK_PERIPHERALS } {
            None
        } else {
            unsafe { TOOK_PERIPHERALS = true };
            Some(unsafe { Self::create() })
        }
    }

    pub fn take_init() -> Option<Self> {
        if unsafe { TOOK_PERIPHERALS } {
            None
        } else {
            unsafe { TOOK_PERIPHERALS = true };
            let mut p = unsafe { Self::create() };
            unsafe { init(&mut p) };
            Some(p)
        }
    }

    pub unsafe fn create() -> Self {
        Self {
            analog: Analog::new(),
            portd: PortD::new(),
            portb: PortB::new(),
            portc: PortC::new(),
            timer0: Timer0::new(),
            timer1: Timer1::new(),
            timer2: Timer2::new(),
            usart0: Usart0::new(),
        }
    }
}
