use crate::{
    init_bits, init_register,
    register::{BitRO, BitRW, RegRO, RegRW},
};

init_bits! {
    Ucsr0ABits {
        #[doc = "Multi-Processor Communication Mode"]
        MPCM0 = 0,
        #[doc = "Double the USART Transmission Speed"]
        U2X0 = 1,
        #[doc = "USART Parity Error"]
        UPE0 = 2,
        #[doc = "Data OverRun"]
        DOR0 = 3,
        #[doc = "Frame Error"]
        FE0 = 4,
        #[doc = "USART Data Register Empty"]
        UDRE0 = 5,
        #[doc = "USART Transmit Complete"]
        TXC0 = 6,
        #[doc = "USART Receive Complete"]
        RXC0 = 7,
    }
}

init_bits! {
    Ucsr0BBits {
        #[doc = "Transmit Data Bit 8"]
        TXB80 = 0,
        #[doc = "Receive Data Bit 8"]
        RXB80 = 1,
        #[doc = "Character Size Bit"]
        UCSZ02 = 2,
        #[doc = "Transmitter Enable"]
        TXEN0 = 3,
        #[doc = "Receiver Enable"]
        RXEN0 = 4,
        #[doc = "USART Data Register Empty Interrupt Enable"]
        UDRIE0 = 5,
        #[doc = "TX Complete Interupt Enable"]
        TXCIE0 = 6,
        #[doc = "RX Complete Interupt Enable"]
        RXCIE0 = 7,
    }
}

init_bits! {
    Ucsr0CBits {
        #[doc = "Clock Polarity"]
        UCPOL0 = 0,
        #[doc = "Character Size Bit"]
        UCSZ00 = 1,
        #[doc = "Character Size Bit"]
        UCSZ01 = 2,
        #[doc = "Stop Bit Select"]
        USBS0 = 3,
        #[doc = "USART Parity Mode Bit"]
        UPM00 = 4,
        #[doc = "USART Parity Mode Bit"]
        UPM01 = 5,
        #[doc = "USART Mode Select Bit"]
        UMSEL00 = 6,
        #[doc = "USART Mode Select Bit"]
        UMSEL01 = 7,
    }
}

init_register! {
    #[doc = "USART Control and Status Register A"]
    Ucsr0A: RegRW<u8> = new_mem8(0xC0) + Ucsr0ABits;
}

init_register!(
    #[doc = "USART Control and Status Register B"]
    Ucsr0B: RegRW<u8> = new_mem8(0xC1) + Ucsr0BBits;
);

init_register!(
    #[doc = "USART Control and Status Register C"]
    Ucsr0C: RegRW<u8> = new_mem8(0xC2) + Ucsr0CBits;
);
