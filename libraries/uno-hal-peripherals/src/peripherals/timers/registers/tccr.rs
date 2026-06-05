use crate::{
    init_bits, init_register,
    register::{BitRO, BitRW, RegRO, RegRW},
};

init_bits! {
    Tccr0ABits {
        #[doc = "Waveform Generation Mode Bit"]
        WGM00 = 0,
        #[doc = "Waveform Generation Mode Bit"]
        WGM01 = 1,
        #[doc = "Compare Match Output B Mode Bit"]
        COM0B0 = 4,
        #[doc = "Compare Match Output B Mode Bit"]
        COM0B1 = 5,
        #[doc = "Compare Match Output A Mode Bit"]
        COM0A0 = 6,
        #[doc = "Compare Match Output A Mode Bit"]
        COM0A1 = 7,
    }
}

init_bits! {
    Tccr0BBits {
        #[doc = "Clock Select Bit"]
        CS00 = 0,
        #[doc = "Clock Select Bit"]
        CS01 = 1,
        #[doc = "Clock Select Bit"]
        CS02 = 2,
        #[doc = "Waveform Generation Mode Bit"]
        WGM02 = 3,
        #[doc = "Force Output Compare B"]
        FOC0B = 6,
        #[doc = "Force Output Compare A"]
        FOC0A = 7,
    }
}

init_bits! {
    Tccr1ABits {
        #[doc = "Waveform Generation Mode Bit"]
        WGM10 = 0,
        #[doc = "Waveform Generation Mode Bit"]
        WGM11 = 1,
        #[doc = "Compare Match Output B Mode Bit"]
        COM1B0 = 4,
        #[doc = "Compare Match Output B Mode Bit"]
        COM1B1 = 5,
        #[doc = "Compare Match Output A Mode Bit"]
        COM1A0 = 6,
        #[doc = "Compare Match Output A Mode Bit"]
        COM1A1 = 7,
    }
}

init_bits! {
    Tccr1BBits {
        #[doc = "Clock Select Bit"]
        CS10 = 0,
        #[doc = "Clock Select Bit"]
        CS11 = 1,
        #[doc = "Clock Select Bit"]
        CS12 = 2,
        #[doc = "Waveform Generation Mode Bit"]
        WGM12 = 3,
        #[doc = "Waveform Generation Mode Bit"]
        WGM13 = 4,
        #[doc = "Input Capture Edge Select"]
        ICES1 = 6,
        #[doc = "Input Capture Noise Canceler"]
        ICNC1 = 7,
    }
}

init_bits! {
    Tccr2ABits {
        #[doc = "Waveform Generation Mode Bit"]
        WGM20 = 0,
        #[doc = "Waveform Generation Mode Bit"]
        WGM21 = 1,
        #[doc = "Compare Match Output B Mode Bit"]
        COM2B0 = 4,
        #[doc = "Compare Match Output B Mode Bit"]
        COM2B1 = 5,
        #[doc = "Compare Match Output A Mode Bit"]
        COM2A0 = 6,
        #[doc = "Compare Match Output A Mode Bit"]
        COM2A1 = 7,
    }
}

init_bits! {
    Tccr2BBits {
        #[doc = "Clock Select Bit"]
        CS20 = 0,
        #[doc = "Clock Select Bit"]
        CS21 = 1,
        #[doc = "Clock Select Bit"]
        CS22 = 2,
        #[doc = "Waveform Generation Mode Bit"]
        WGM22 = 3,
        #[doc = "Force Output Compare B"]
        FOC2B = 6,
        #[doc = "Force Output Compare A"]
        FOC2A = 7,
    }
}

init_register! {
    #[doc = "Timer/Counter 0 Control Register A"]
    Tccr0A: RegRW<u8> = new_io8(0x24) + Tccr0ABits;
}

init_register! {
    #[doc = "Timer/Counter 0 Control Register B"]
    Tccr0B: RegRW<u8> = new_io8(0x25) + Tccr0BBits;
}

init_register! {
    #[doc = "Timer/Counter 1 Control Register A"]
    Tccr1A: RegRW<u8> = new_mem8(0x80) + Tccr1ABits;
}

init_register! {
    #[doc = "Timer/Counter 1 Control Register B"]
    Tccr1B: RegRW<u8> = new_mem8(0x81) + Tccr1BBits;
}

init_register! {
    #[doc = "Timer/Counter 2 Control Register A"]
    Tccr2A: RegRW<u8> = new_mem8(0xB0) + Tccr2ABits;
}

init_register! {
    #[doc = "Timer/Counter 2 Control Register B"]
    Tccr2B: RegRW<u8> = new_mem8(0xB1) + Tccr2BBits;
}
