use crate::registers::{analog::*, timers::*, usart::UCSR0B, Register};

pub fn init() {
    unsafe { crate::interrupt::enable() };

    TCCR0A.wgm01.set();
    TCCR0A.wgm00.set();

    TCCR0B.cs01.set();
    TCCR0B.cs00.set();

    TIMSK0.toie0.set();

    TCCR1B.write(0);

    TCCR1B.cs11.set();
    TCCR1B.cs10.set();

    TCCR1A.wgm10.set();

    TCCR2B.cs22.set();

    TCCR2A.wgm20.set();

    ADCSRA.adps2.set();
    ADCSRA.adps1.set();
    ADCSRA.adps0.set();

    ADCSRA.aden.set();

    UCSR0B.write(0);
}
