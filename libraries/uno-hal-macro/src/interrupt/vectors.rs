pub fn get_vector_id(chip: &str, name: &str) -> Option<u8> {
    match chip {
        "atmega328p" => match name {
            "RESET" => Some(0),         // Reset Vector
            "INT0" => Some(1),          // External Interrupt Request 0
            "INT1" => Some(2),          // External Interrupt Request 1
            "PCINT0" => Some(3),        // Pin Change Interrupt Request 0
            "PCINT1" => Some(4),        // Pin Change Interrupt Request 1
            "PCINT2" => Some(5),        // Pin Change Interrupt Request 2
            "WDT" => Some(6),           // Watchdog Time-out Interrupt
            "TIMER2_COMPA" => Some(7),  // Timer/Counter2 Compare Match A
            "TIMER2_COMPB" => Some(8),  // Timer/Counter2 Compare Match B
            "TIMER2_OVF" => Some(9),    // Timer/Counter2 Overflow
            "TIMER1_CAPT" => Some(10),  // Timer/Counter1 Capture Event
            "TIMER1_COMPA" => Some(11), // Timer/Counter1 Compare Match A
            "TIMER1_COMPB" => Some(12), // Timer/Counter1 Compare Match B
            "TIMER1_OVF" => Some(13),   // Timer/Counter1 Overflow
            "TIMER0_COMPA" => Some(14), // Timer/Counter0 Compare Match A
            "TIMER0_COMPB" => Some(15), // Timer/Counter0 Compare Match B
            "TIMER0_OVF" => Some(16),   // Timer/Counter0 Overflow
            "SPI_STC" => Some(17),      // SPI Serial Transfer Complete
            "USART_RX" => Some(18),     // USART Rx Complete
            "USART_UDRE" => Some(19),   // USART, Data Register Empty
            "USART_TX" => Some(20),     // USART Tx Complete
            "ADC" => Some(21),          // ADC Conversion Complete
            "EE_READY" => Some(22),     // EEPROM Ready
            "ANALOG_COMP" => Some(23),  // Analog Comparator
            "TWI" => Some(24),          // Two-wire Serial Interface
            "SPM_READY" => Some(25),    // Store Program Memory Read
            _ => None,
        },
        _ => None,
    }
}
